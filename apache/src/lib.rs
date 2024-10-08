use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("HTTPD_PORT")?;
    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/apache")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["httpd", "-v"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "httpd"])?
        .with_exec(vec![
            "echo -e \"Apache starting on port $HTTPD_PORT\\n http://localhost:$HTTPD_PORT\"",
        ])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart web",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec!["curl", "-s", "http://localhost:$HTTPD_PORT"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "web".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/apache")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
