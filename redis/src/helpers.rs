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
        .with_exec(vec![
            "flox", "install", "redis", "overmind", "tmux", "openssl",
        ])?
        .with_exec(vec![
            "grep -q redis Procfile || echo 'redis: redis-server' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
