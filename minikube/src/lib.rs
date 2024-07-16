use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["minikube", "version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "minikube"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart minikube",
        ])?
        .wait_on(32768, None)?
        .wait_on(32769, None)?
        .wait_on(32770, None)?
        .wait_on(32771, None)?
        .wait_on(32772, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "minikube".to_string()
    } else {
        args
    };

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
