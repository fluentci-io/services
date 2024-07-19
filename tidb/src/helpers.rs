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
        .with_exec(vec!["mkdir", "-p", ".fluentci/tidb"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let tidb_port = dag().get_env("TIDB_PORT")?;
    let data_dir = dag().get_env("TIDB_DATADIR")?;

    if tidb_port.is_empty() {
        dag().set_envs(vec![("TIDB_PORT".into(), "4000".into())])?;
    }

    if data_dir.is_empty() {
        dag().set_envs(vec![("TIDB_DATADIR".into(), format!("{}/tidb", pwd))])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/tidb")?
        .with_exec(vec![
            "[ -d $TIDB_DATADIR ] || mkdir -p $TIDB_DATADIR",
        ])?
        .with_exec(vec![
            "flox",
            "install",
            "tidb",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q tidb: Procfile || echo -e 'tidb: tidb-server -path $TIDB_DATADIR -temp-dir $TIDB_DATADIR -socket $TIDB_DATADIR/tidb.sock -P $TIDB_PORT \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
