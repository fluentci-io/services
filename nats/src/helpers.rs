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
        .with_exec(vec!["mkdir", "-p", ".fluentci/nats"])?
        .stdout()?;

    let nats_port = dag().get_env("NATS_PORT")?;
    let nats_addr = dag().get_env("NATS_ADDR")?;

    if nats_port.is_empty() {
        dag().set_envs(vec![("NATS_PORT".into(), "4222".into())])?;
    }

    if nats_addr.is_empty() {
        dag().set_envs(vec![("NATS_ADDR".into(), "0.0.0.0".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/nats")?
        .with_exec(vec![
            "flox",
            "install",
            "nats-server",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q nats Procfile || echo -e 'nats: nats-server --port $NATS_PORT --addr $NATS_ADDR \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
