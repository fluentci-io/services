use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/zipkin")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart zipkin",
        ])?
        .wait_on(9411, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "zipkin".to_string()
    } else {
        args
    };

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/zipkin")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
