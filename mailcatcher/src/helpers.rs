use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/mailcatcher"])?
        .stdout()?;

    let smtp_port = dag().get_env("MAILCATCHER_SMTP_PORT")?;
    let http_port = dag().get_env("MAILCATCHER_HTTP_PORT")?;

    if smtp_port.is_empty() {
        dag().set_envs(vec![("MAILCATCHER_SMTP_PORT".into(), "1025".into())])?;
    }

    if http_port.is_empty() {
        dag().set_envs(vec![("MAILCATCHER_HTTP_PORT".into(), "1080".into())])?;
    }

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/mailcatcher")?
        .with_exec(vec![
            "devbox",
            "add",
            "ruby",
            "overmind",
            "tmux",
            "pkg-config",
            "openssl"
        ])?
        .with_exec(vec!["gem", "install", "mailcatcher"])?
        .with_exec(vec![
            "grep -q mailcatcher: Procfile || echo -e 'mailcatcher: devbox run mailcatcher --http-port $MAILCATCHER_HTTP_PORT --smtp-port $MAILCATCHER_SMTP_PORT -f \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
