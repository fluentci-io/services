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
        .with_exec(vec!["mkdir", "-p", ".fluentci/dynamodb-local"])?
        .stdout()?;

    let dynamodb_port = dag().get_env("DYNAMODB_PORT")?;
    let data_dir = dag().get_env("DYNAMODB_DATA_DIR")?;

    if dynamodb_port.is_empty() {
        dag().set_envs(vec![("DYNAMODB_PORT".into(), "8000".into())])?;
    }

    if data_dir.is_empty() {
        dag().set_envs(vec![("DYNAMODB_DATA_DIR".into(), "dynamodb/data".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/dynamodb-local")?
        .with_exec(vec![
            "[ -d $DYNAMODB_DATA_DIR ] || mkdir -p $DYNAMODB_DATA_DIR",
        ])?
        .with_exec(vec![
            "flox",
            "install",
            "dynamodb-local",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q dynamodb Procfile || echo -e 'dynamodb: dynamodb-local -port $DYNAMODB_PORT -dbPath $DYNAMODB_DATA_DIR -disableTelemetry\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
