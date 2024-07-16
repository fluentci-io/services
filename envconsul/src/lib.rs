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
        .with_exec(vec!["envconsul", "-version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "envconsul"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind quit",
        ])?
        .with_exec(vec![
            "[ -f ./.overmind.sock ] || flox activate -- overmind start -f Procfile --daemonize",
        ])?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec!["sleep", "2"])?
        .with_exec(vec![
            "envconsul -pristine -upcase -prefix $ENVCONSUL_PREFIX env",
        ])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "envconsul".to_string()
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
