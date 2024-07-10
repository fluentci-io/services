use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["flox", "install", "redis", "overmind", "tmux"])?
        .with_exec(vec![
            "grep -q redis Procfile || echo 'redis: redis >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
