use anyhow::Error;
use fluentci_pdk::dag;

pub fn install_influxdb() -> Result<(), Error> {
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

    dag().set_envs(vec![("OS".into(), os.into()), ("ARCH".into(), arch.into())])?;

    let version = dag().get_env("INFLUXDB_VERSION")?;
    if version.is_empty() {
        dag().set_envs(vec![("INFLUXDB_VERSION".into(), "2-2.7.7".into())])?;
    }

    let path = dag().get_env("PATH")?;
    let home = dag().get_env("HOME")?;
    dag().set_envs(vec![
        ("PATH".into(), format!("$HOME/.local/bin:{}", path)),
        ("HOME".into(), home),
    ])?;

    dag()
        .pkgx()?
        .with_exec(vec!["mkdir", "-p", "$HOME/.local/bin"])?
        .with_exec(vec!["type influxd > /dev/null 2> /dev/null || pkgx wget https://dl.influxdata.com/influxdb/releases/influxdb${INFLUXDB_VERSION}_${OS}_${ARCH}.tar.gz"])?
        .with_exec(vec!["type influxd > /dev/null 2> /dev/null || pkgx tar -xvzf influxdb${INFLUXDB_VERSION}_${OS}_${ARCH}.tar.gz"])?
        .with_exec(vec!["type influxd > /dev/null 2> /dev/null || cp influxdb${INFLUXDB_VERSION}/usr/bin/influxd $HOME/.local/bin || true"])?
        .with_exec(vec!["type influxd > /dev/null 2> /dev/null || cp influxdb${INFLUXDB_VERSION}/influxd $HOME/.local/bin || true"])?
        .with_exec(vec![
            "[ -d influxdb$INFLUXDB_VERSION ] && ",
            "rm",
            "-rf",
            "influxdb$INFLUXDB_VERSION*",
            " || true",
        ])?
        .stdout()?;
    Ok(())
}

pub fn setup() -> Result<String, Error> {
    install_influxdb()?;

    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/influxdb"])?
        .stdout()?;

    let influxdb_port = dag().get_env("INFLUXDB_PORT")?;

    if influxdb_port.is_empty() {
        dag().set_envs(vec![("INFLUXDB_PORT".into(), "8086".into())])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/influxdb")?
        .with_packages(vec![
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec![
            "grep -q influxdb: Procfile || echo -e 'influxdb: influxd --http-bind-address $INFLUXDB_PORT $INFLUXDB_ARGS \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
