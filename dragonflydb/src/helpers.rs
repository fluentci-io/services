use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let port = dag().get_env("DRAGONFLY_PORT")?;
    if port.is_empty() {
        dag().set_envs(vec![("DRAGONFLY_PORT".into(), "6379".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["flox", "install", "docker", "overmind", "tmux"])?
        .with_exec(vec![
            "grep -q dragonflydb Procfile || echo -e 'dragonflydb: docker run -p $DRAGONFLY_PORT:6379 --ulimit memlock=-1 docker.dragonflydb.io/dragonflydb/dragonfly\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
