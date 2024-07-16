use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let prefix = dag().get_env("ENVCONSUL_PREFIX")?;
    let workdir = format!(".fluentci/{}", prefix);

    let stdout = dag()
        .flox()?
        .with_workdir(&workdir)?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["envconsul", "-version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "envconsul"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart",
            &prefix,
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
pub fn stop(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let prefix = dag().get_env("ENVCONSUL_PREFIX")?;
    let workdir = format!(".fluentci/{}", prefix);

    let stdout = dag()
        .flox()?
        .with_workdir(&workdir)?
        .with_exec(vec!["overmind", "stop", &prefix])?
        .stdout()?;
    Ok(stdout)
}
