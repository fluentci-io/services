use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/dgraph")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["dgraph", "version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "dgraph"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart dgraph",
        ])?
        .wait_on(5080, None)?
        .wait_on(6080, None)?
        .wait_on(9080, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "dgraph dgraph-zero".to_string()
    } else {
        args
    };

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/dgraph")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
