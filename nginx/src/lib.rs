use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;
    let port = dag().get_env("NGINX_WEB_PORT")?;
    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["nginx", "-v"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "nginx"])?
        .with_exec(vec![
            "echo -e \"Nginx starting on port $NGINX_WEB_PORT\\n http://localhost:$NGINX_WEB_PORT\"",
        ])?
        .with_exec(vec!["overmind", "start", "-f", "Procfile", "--daemonize"])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec!["curl", "-s", "http://localhost:$NGINX_WEB_PORT"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
