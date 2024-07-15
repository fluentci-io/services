use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "dynamodb-local"])?
        .with_exec(vec!["dynamodb-local", "-version"])?
        .with_exec(vec![
            "echo -e \"DynamoDB Local starting on port $DYNAMODB_PORT\"",
        ])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart dynamodb",
        ])?
        .with_exec(vec![
            "pkgx",
            "deno",
            "run",
            "-A",
            "npm:wait-port",
            "$DYNAMODB_PORT",
        ])?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "dynamodb".to_string()
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
