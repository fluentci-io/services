use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;
    let port = dag().get_env("MYSQL_PORT")?;
    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["mysql", "-V"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "mysql"])?
        .with_exec(vec!["echo -e \"MySQL starting on port $MYSQL_PORT\""])?
        .with_exec(vec!["overmind", "start", "-f", "Procfile", "--daemonize"])?
        .with_exec(vec!["sleep", "2"])?
        .with_exec(vec!["overmind", "status"])?
        .wait_on(port.parse()?, None)?
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