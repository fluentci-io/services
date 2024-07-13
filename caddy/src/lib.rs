use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("CADDY_PORT")?;

    if port.is_empty() {
        dag().set_envs(vec![("CADDY_PORT".into(), "8082".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["caddy", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "caddy"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart caddy",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec!["curl", "-s", "http://localhost:$CADDY_PORT"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "caddy".to_string()
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
