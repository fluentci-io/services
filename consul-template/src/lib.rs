use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let prefix = dag().get_env("CONSUL_APP_PREFIX")?;
    let workdir = format!(".fluentci/{}", prefix);

    let stdout = dag()
        .pkgx()?
        .with_workdir(&workdir)?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["consul-template", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "consul-template"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart",
            &prefix,
        ])?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec!["cat $CONSUL_TEMPLATE_CONFIG"])?
        .with_exec(vec!["cat $CONSUL_APP_CONFIG"])?
        .with_exec(vec!["echo '[test] updating config ...'"])?
        .with_exec(vec![
            "consul",
            "kv",
            "put",
            "$CONSUL_APP_PREFIX/address",
            "127.0.0.1",
        ])?
        .with_exec(vec![
            "consul",
            "kv",
            "put",
            "$CONSUL_APP_PREFIX/port",
            "5000",
        ])?
        .with_exec(vec![
            "consul",
            "kv",
            "put",
            "$CONSUL_APP_PREFIX/max_conns",
            "6",
        ])?
        .with_exec(vec!["cat $CONSUL_APP_CONFIG"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let prefix = dag().get_env("CONSUL_APP_PREFIX")?;
    let workdir = format!(".fluentci/{}", prefix);

    let stdout = dag()
        .pkgx()?
        .with_workdir(&workdir)?
        .with_exec(vec!["overmind", "stop", &prefix])?
        .stdout()?;
    Ok(stdout)
}
