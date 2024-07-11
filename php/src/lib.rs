use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("PHPFPM_PORT")?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["php", "--version"])?
        .with_exec(vec!["php-fpm", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "php"])?
        .with_exec(vec!["type", "php-fpm"])?
        .with_exec(vec!["overmind", "start", "-f", "Procfile", "--daemonize"])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec!["curl", "-s", "http://localhost:$PHPFPM_PORT"])?
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