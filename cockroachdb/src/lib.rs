use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("COCKROACH_PORT")?;
    let http_port = dag().get_env("COCKROACH_HTTP_PORT")?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/cockroachdb")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "cockroachdb"])?
        .with_exec(vec!["cockroachdb", "version"])?
        .with_exec(vec![
            "echo -e \"Cockroachdb starting on port $COCKROACH_PORT\"",
        ])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart cockroachdb",
        ])?
        .wait_on(port.parse()?, None)?
        .wait_on(http_port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "cockroachdb".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/cockroachdb")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
