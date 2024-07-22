use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let http_port = dag().get_env("MAILPIT_HTTP_PORT")?;
    let smtp_port = dag().get_env("MAILPIT_SMTP_PORT")?;

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/mailpit")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type mailpit"])?
        .with_exec(vec!["mailpit version"])?
        .with_exec(vec![
            "echo -e \"Mailpit starting on port $MAILPIT_HTTP_PORT\"",
        ])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || devbox run overmind restart mailpit",
        ])?
        .wait_on(http_port.parse()?, None)?
        .wait_on(smtp_port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "mailpit".to_string()
    } else {
        args
    };

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/mailpit")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
