use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_flox() -> Result<(), Error> {
    let os = dag().get_os()?;
    if os == "macos" {
        dag()
        .pipeline("setup-flox")?
        .with_exec(vec![r#"type brew > /dev/null 2> /dev/null || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#])?
        .with_exec(vec!["type flox > /dev/null 2> /dev/null || brew install flox"])?
        .stdout()?;
    }
    Ok(())
}

pub fn setup() -> Result<String, Error> {
    setup_flox()?;
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/cockroachdb"])?
        .stdout()?;

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
        .flox()?
        .with_workdir(".fluentci/cockroachdb")?
        .with_exec(vec![
            "flox",
            "install",
            "cockroachdb-bin",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec!["[ -d $COCKROACH_DATA ] || mkdir -p $COCKROACH_DATA"])?
        .with_exec(vec![
            "grep -q cockroachdb Procfile || echo -e 'cockroachdb: cockroachdb  start-single-node --insecure --listen-addr=$COCKROACH_HOST:$COCKROACH_PORT --http-addr=$COCKROACH_DATA --store=path=$COCKROACH_DATA \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
