use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("CASSANDRA_PORT")?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/cassandra")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "cassandra"])?
        .with_exec(vec!["cassandra", "-v", "-R"])?
        .with_exec(vec![
            "echo -e \"Cassandra starting on port $CASSANDRA_PORT\"",
        ])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart cassandra",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "cassandra".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/cassandra")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
