use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_packages(vec![
            "redis.io",
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec![
            "grep -q redis Procfile || echo -e 'redis: redis-server\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
