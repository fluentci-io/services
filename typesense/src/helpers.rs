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
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let api_port = dag().get_env("TYPESENSE_API_PORT")?;
    let api_host = dag().get_env("TYPESENSE_API_HOST")?;
    let api_key = dag().get_env("TYPESENSE_API_KEY")?;
    let data_dir = dag().get_env("TYPESENSE_DATA_DIR")?;

    if api_port.is_empty() {
        dag().set_envs(vec![("TYPESENSE_API_PORT".into(), "8108".into())])?;
    }

    if api_host.is_empty() {
        dag().set_envs(vec![("TYPESENSE_API_HOST".into(), "127.0.0.1".into())])?;
    }

    if api_key.is_empty() {
        dag().set_envs(vec![("TYPESENSE_API_KEY".into(), "example".into())])?;
    }

    if data_dir.is_empty() {
        dag().set_envs(vec![("TYPESENSE_DATA_DIR".into(), "typesense/data".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["[ -d $TYPESENSE_DATA_DIR ] || mkdir -p $TYPESENSE_DATA_DIR"])?
        .with_exec(vec!["flox", "install", "typesense", "overmind", "tmux"])?
        .with_exec(vec![
            "grep -q typesense Procfile || echo -e 'typesense: typesense-server --data-dir $TYPESENSE_DATA_DIR --api-key $TYPESENSE_API_KEY --api-host $TYPESENSE_API_HOST --api-port $TYPESENSE_API_PORT\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
