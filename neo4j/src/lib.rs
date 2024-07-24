use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/neo4j")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type neo4j"])?
        .with_exec(vec!["neo4j --version"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || devbox run overmind restart neo4j",
        ])?
        .wait_on(7474, None)?
        .wait_on(7687, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "neo4j".to_string()
    } else {
        args
    };

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/neo4j")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
