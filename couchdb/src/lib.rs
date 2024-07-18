use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("COUCHDB_PORT")?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/couchdb")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "couchdb"])?
        .with_exec(vec!["echo -e \"CouchDB starting on port $COUCHDB_PORT\""])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart couchdb",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec![
            "pkgx",
            "curl",
            "-X",
            "GET",
            "http://localhost:$COUCHDB_PORT",
            " | ",
            "pkgx jq",
        ])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "couchdb".to_string()
    } else {
        args
    };

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/couchdb")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
