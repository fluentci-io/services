use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let meili_port = dag().get_env("MEILI_PORT")?;
    let meili_http_addr = dag().get_env("MEILI_HTTP_ADDR")?;

    if meili_port.is_empty() {
        dag().set_envs(vec![("MEILI_PORT".into(), "7700".into())])?;
    }

    let meili_port = dag().get_env("MEILI_PORT")?;

    if meili_http_addr.is_empty() {
        dag().set_envs(vec![(
            "MEILI_HTTP_ADDR".into(),
            format!("127.0.0.1:{}", meili_port),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["flox", "install", "meilisearch" "overmind", "tmux"])?
        .with_exec(vec![
            "grep -q meilisearch Procfile || echo 'meilisearch: meilisearch' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
