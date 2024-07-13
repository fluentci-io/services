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
        .with_exec(vec!["php", "--ini"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "php"])?
        .with_exec(vec!["type", "php-fpm"])?
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

    let args = if args.is_empty() {
        "php-fpm".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
