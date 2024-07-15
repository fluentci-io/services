use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let rabbitmq_port = dag().get_env("RABBITMQ_NODE_PORT")?;

    if rabbitmq_port.is_empty() {
        dag().set_envs(vec![("RABBITMQ_NODE_PORT".into(), "5672".into())])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_packages(vec!["rabbitmq.com", "overmind", "tmux"])?
        .with_exec(vec![
            "grep -q rabbitmq Procfile || echo -e 'rabbitmq: rabbitmq-server\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
