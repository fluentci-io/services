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

    let temporal_port = dag().get_env("TEMPORAL_PORT")?;
    let temporal_opts = dag().get_env("TEMPORAL_OPTS")?;

    if temporal_port.is_empty() {
        dag().set_envs(vec![("TEMPORAL_PORT".into(), "7233".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["flox", "install", "temporal-cli", "overmind", "tmux"])?
        .with_exec(vec![
            &format!("grep -q temporal Procfile || echo 'temporal: temporal server start-dev --log-format=pretty --port=$TEMPORAL_PORT {}' >> Procfile", temporal_opts),
        ])?
        .stdout()?;

    Ok(stdout)
}
