use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("CONSUL_HTTP_PORT")?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/consul")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["consul", "version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "consul"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart consul",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec!["consul", "kv", "put", "redis/config/minconns", "1"])?
        .with_exec(vec!["consul", "kv", "get", "redis/config/minconns"])?
        .with_exec(vec!["consul", "kv", "delete", "redis/config/minconns"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "consul".to_string()
    } else {
        args
    };

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/consul")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
