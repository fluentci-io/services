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
        .with_exec(vec!["mkdir", "-p", ".fluentci/elasticmq"])?
        .stdout()?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/elasticmq")?
        .with_exec(vec![
            "flox",
            "install",
            "elasticmq-server-bin",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q elasticmq Procfile || echo -e 'elasticmq: elasticmq-server \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
