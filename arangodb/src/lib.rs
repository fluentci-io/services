use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let os = dag().get_os()?;
    let port = dag().get_env("ARANGODB_PORT")?;

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/arangodb")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec![match os.as_str() {
            "linux" => "type arangod",
            _ => "true",
        }])?
        .with_exec(vec![match os.as_str() {
            "linux" => "arangod --version",
            _ => "true",
        }])?
        .with_exec(vec!["echo -e \"ArangoDB starting on port $ARANGODB_PORT\""])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || devbox run overmind restart arangodb",
        ])?
        .wait_on(port.parse()?, Some(60000 * 10))?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "arangodb".to_string()
    } else {
        args
    };

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/arangodb")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
