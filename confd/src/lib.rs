use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let prefix = dag().get_env("CONFD_APP_PREFIX")?;
    let workdir = format!(".fluentci/{}", prefix);

    let stdout = dag()
        .flox()?
        .with_workdir(&workdir)?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["confd", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "confd"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart",
            &prefix,
        ])?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec!["ls", "-la", "../.."])?
        .with_exec(vec![
            "[ -f ../../sample.conf ] && cat ../../sample.conf || true",
        ])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let prefix = dag().get_env("CONFD_APP_PREFIX")?;
    let workdir = format!(".fluentci/{}", prefix);

    let stdout = dag()
        .flox()?
        .with_workdir(&workdir)?
        .with_exec(vec!["overmind", "stop", &prefix])?
        .stdout()?;
    Ok(stdout)
}
