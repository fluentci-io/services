use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;
    let port = dag().get_env("PGPORT")?;
    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/postgres")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["postgres", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "postgres"])?
        .with_exec(vec!["echo -e \"Postgres starting on port $PGPORT\""])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart postgres",
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
        "postgres".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/postgres")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
