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
        .with_exec(vec!["mkdir", "-p", ".fluentci/rethinkdb"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let rethinkdb_port = dag().get_env("RETHINKDB_PORT")?;
    let rethinkdb_http_port = dag().get_env("RETHINKDB_HTTP_PORT")?;
    let cluster_port = dag().get_env("RETHINKDB_CLUSTER_PORT")?;
    let data_dir = dag().get_env("RETHINKDB_DATADIR")?;

    if rethinkdb_port.is_empty() {
        dag().set_envs(vec![("RETHINKDB_PORT".into(), "28015".into())])?;
    }

    if rethinkdb_http_port.is_empty() {
        dag().set_envs(vec![("RETHINKDB_HTTP_PORT".into(), "8080".into())])?;
    }

    if cluster_port.is_empty() {
        dag().set_envs(vec![("RETHINKDB_CLUSTER_PORT".into(), "29015".into())])?;
    }

    if data_dir.is_empty() {
        dag().set_envs(vec![(
            "RETHINKDB_DATADIR".into(),
            format!("{}/rethinkdb-data", pwd),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/rethinkdb")?
        .with_exec(vec![
            "flox",
            "install",
            "rethinkdb",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q rethinkdb: Procfile || echo -e 'rethinkdb: rethinkdb --driver-port $RETHINKDB_PORT --directory $RETHINKDB_DATADIR --http-port $RETHINKDB_HTTP_PORT --cluster-port $RETHINKDB_CLUSTER_PORT $RETHINKDB_ARGS \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
