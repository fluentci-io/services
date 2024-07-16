use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/redis"])?
        .stdout()?;

    let redis_port = dag().get_env("REDIS_PORT")?;

    if redis_port.is_empty() {
        dag().set_envs(vec![("REDIS_PORT".into(), "6379".into())])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/redis")?
        .with_packages(vec![
            "redis.io",
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec![
            "grep -q redis Procfile || echo -e 'redis: redis-server --port $REDIS_PORT \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
