use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/mailhog"])?
        .stdout()?;

    let smtp_port = dag().get_env("MAILHOG_SMTP_PORT")?;
    let http_port = dag().get_env("MAILHOG_HTTP_PORT")?;

    if smtp_port.is_empty() {
        dag().set_envs(vec![("MAILHOG_SMTP_PORT".into(), "1025".into())])?;
    }

    if http_port.is_empty() {
        dag().set_envs(vec![("MAILHOG_HTTP_PORT".into(), "8085".into())])?;
    }

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/mailhog")?
        .with_exec(vec![
            "devbox",
            "add",
            "mailhog",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q mailhog: Procfile || echo -e 'mailhog: devbox run MailHog -api-bind-addr 127.0.0.1:$MAILHOG_HTTP_PORT -ui-bind-addr 127.0.0.1:$MAILHOG_HTTP_PORT -smtp-bind-addr 127.0.0.1:$MAILHOG_SMTP_PORT $MAILHOG_ARGS \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
