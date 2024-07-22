use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/mailpit"])?
        .stdout()?;

    let smtp_port = dag().get_env("MAILPIT_SMTP_PORT")?;
    let http_port = dag().get_env("MAILPIT_HTTP_PORT")?;

    if smtp_port.is_empty() {
        dag().set_envs(vec![("MAILPIT_SMTP_PORT".into(), "1025".into())])?;
    }

    if http_port.is_empty() {
        dag().set_envs(vec![("MAILPIT_HTTP_PORT".into(), "8085".into())])?;
    }

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/mailpit")?
        .with_exec(vec![
            "devbox",
            "add",
            "mailpit",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec!["mkdir -p ../../mailpit"])?
        .with_exec(vec![
            "grep -q mailpit: Procfile || echo -e 'mailpit: devbox run mailpit --db-file ../../mailpit/db.sqlite3 --listen 127.0.0.1:$MAILPIT_HTTP_PORT --smtp 127.0.0.1:$MAILPIT_SMTP_PORT $MAILPIT_ARGS \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
