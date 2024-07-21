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
        .with_exec(vec!["mkdir", "-p", ".fluentci/spicedb"])?
        .stdout()?;

    let spicedb_port = dag().get_env("SPICEDB_PORT")?;
    let spicedb_preshared_key = dag().get_env("SPICEDB_PRESHARED_KEY")?;

    if spicedb_port.is_empty() {
        dag().set_envs(vec![("SPICEDB_PORT".into(), "50051".into())])?;
    }

    if spicedb_preshared_key.is_empty() {
        dag().set_envs(vec![(
            "SPICEDB_PRESHARED_KEY".into(),
            "somerandomkeyhere".into(),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/spicedb")?
        .with_exec(vec![
            "flox",
            "install",
            "spicedb",
            "spicedb-zed",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q spicedb: Procfile || echo -e 'spicedb: spicedb serve --grpc-preshared-key --grpc-addr :$SPICEDB_PORT $SPICEDB_PRESHARED_KEY $SPICEDB_ARGS \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
