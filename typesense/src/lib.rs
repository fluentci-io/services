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
        .with_exec(vec!["type", "typesense-server"])?
        .with_exec(vec![
            "echo -e \"Typesense starting on port $TYPESENSE_API_PORT\"",
        ])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart typesense",
        ])?
        .with_exec(vec![
            "pkgx",
            "deno",
            "run",
            "-A",
            "npm:wait-port",
            "$TYPESENSE_API_PORT",
        ])?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "typesense".to_string()
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
