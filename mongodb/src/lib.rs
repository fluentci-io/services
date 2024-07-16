use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("MONGODB_PORT")?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/mongodb")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "mongod"])?
        .with_exec(vec!["mongod", "--version"])?
        .with_exec(vec!["echo -e \"MongoDB starting on port $MONGODB_PORT\""])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart mongodb",
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
        "mongodb".to_string()
    } else {
        args
    };

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/mongodb")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
