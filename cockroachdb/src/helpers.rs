use anyhow::Error;
use fluentci_pdk::dag;

pub fn install_cockroachdb() -> Result<(), Error> {
    let os = dag().get_os()?;
    let arch = dag().get_arch()?;

    let os = match os.as_str() {
        "linux" => "linux",
        "macos" => "darwin",
        _ => &os,
    };
    let arch = match arch.as_str() {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => &arch,
    };

    let arch = match os {
        "darwin" => match arch {
            "amd64" => "10.9-amd64",
            "arm64" => "11.0-arm64",
            _ => arch,
        },
        _ => arch,
    };

    dag().set_envs(vec![("OS".into(), os.into()), ("ARCH".into(), arch.into())])?;

    let version = dag().get_env("COCKROACH_VERSION")?;
    if version.is_empty() {
        dag().set_envs(vec![("COCKROACH_VERSION".into(), "v24.1.2".into())])?;
    }

    dag()
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_exec(vec![
            "type cockroach > /dev/null 2> /dev/null || ",
            "pkgx",
            "wget",
            "https://binaries.cockroachdb.com/cockroach-$COCKROACH_VERSION.$OS-$ARCH.tgz",
        ])?
        .with_exec(vec![
            "type cockroach > /dev/null 2> /dev/null || ",
            "tar",
            "-xvzf",
            "cockroach-$COCKROACH_VERSION.$OS-$ARCH.tgz",
        ])?
        .with_exec(vec![
            "[ -d cockroach-$COCKROACH_VERSION.$OS-$ARCH ] && ",
            "mv",
            "cockroach-$COCKROACH_VERSION.$OS-$ARCH/cockroach",
            "$HOME/.local/bin",
            " || true",
        ])?
        .with_exec(vec!["mkdir", "-p", "$HOME/.local/lib"])?
        .with_exec(vec![
            "[ -d cockroach-$COCKROACH_VERSION.$OS-$ARCH/lib ] && ",
            "mv",
            "cockroach-$COCKROACH_VERSION.$OS-$ARCH/lib/*",
            "$HOME/.local/lib",
            " || true",
        ])?
        .with_exec(vec![
            "[ -d cockroach-$COCKROACH_VERSION.$OS-$ARCH ] && ",
            "rm",
            "-rf",
            "cockroach-$COCKROACH_VERSION.$OS-$ARCH*",
            " || true",
        ])?
        .stdout()?;

    Ok(())
}

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/cockroachdb"])?
        .stdout()?;

    install_cockroachdb()?;

    let cockroach_port = dag().get_env("COCKROACH_PORT")?;
    let cockroach_host = dag().get_env("COCKROACH_HOST")?;
    let cockroach_http_port = dag().get_env("COCKROACH_HTTP_PORT")?;
    let cockroach_http_host = dag().get_env("COCKROACH_HTTP_HOST")?;
    let pwd = dag().get_env("PWD")?;
    let cockroach_data = dag().get_env("COCKROACH_DATA")?;

    if cockroach_port.is_empty() {
        dag().set_envs(vec![("COCKROACH_PORT".into(), "26257".into())])?;
    }

    if cockroach_host.is_empty() {
        dag().set_envs(vec![("COCKROACH_HOST".into(), "127.0.0.1".into())])?;
    }

    if cockroach_http_port.is_empty() {
        dag().set_envs(vec![("COCKROACH_HTTP_PORT".into(), "8080".into())])?;
    }

    if cockroach_data.is_empty() {
        dag().set_envs(vec![(
            "COCKROACH_DATA".into(),
            format!("{}/cockroachdb/data", pwd),
        )])?;
    }

    if cockroach_http_host.is_empty() {
        dag().set_envs(vec![("COCKROACH_HTTP_HOST".into(), "127.0.0.1".into())])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/cockroachdb")?
        .with_packages(vec![
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec!["[ -d $COCKROACH_DATA ] || mkdir -p $COCKROACH_DATA"])?
        .with_exec(vec![
            "grep -q cockroachdb: Procfile || echo -e 'cockroachdb: cockroach  start-single-node --insecure --listen-addr=$COCKROACH_HOST:$COCKROACH_PORT --http-addr=$COCKROACH_HTTP_HOST:$COCKROACH_HTTP_PORT --store=path=$COCKROACH_DATA \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
