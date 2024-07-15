use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
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
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_packages(vec!["temporal", "overmind", "tmux"])?
        .with_exec(vec![
            &format!("grep -q temporal Procfile || echo -e 'temporal: temporal server start-dev --log-format=pretty --port=$TEMPORAL_PORT {}\\n' >> Procfile", temporal_opts),
        ])?
        .stdout()?;

    Ok(stdout)
}
