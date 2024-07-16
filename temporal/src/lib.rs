use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/temporal")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["temporal", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "temporal"])?
        .with_exec(vec!["echo -e \"Temporal starting on port $TEMPORAL_PORT\""])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart temporal",
        ])?
        .with_exec(vec![
            "pkgx",
            "deno",
            "run",
            "-A",
            "npm:wait-port",
            "$TEMPORAL_PORT",
        ])?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "temporal".to_string()
    } else {
        args
    };

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/temporal")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
