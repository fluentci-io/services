use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let http_port = dag().get_env("MAILCATCHER_HTTP_PORT")?;
    let smtp_port = dag().get_env("MAILCATCHER_SMTP_PORT")?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/mailcatcher")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["PATH=`flox activate -- gem environment gemhome`/bin:$PATH && flox activate -- type mailcatcher"])?
        .with_exec(vec!["PATH=`flox activate -- gem environment gemhome`/bin:$PATH && flox activate -- mailcatcher --version"])?
        .with_exec(vec![
            "echo -e \"MailCatcher starting on port $MAILCATCHER_HTTP_PORT\"",
        ])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart mailcatcher",
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
        "mailcatcher".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/mailcatcher")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
