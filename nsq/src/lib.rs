use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/nsq")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "nsqd"])?
        .with_exec(vec!["nsqd", "--version"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart nsqd",
        ])?
        .wait_on(4161, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/nsq")?
        .with_exec(vec!["overmind", "stop", "nsqd,nsqadmin,nsqlookupd"])?
        .stdout()?;
    Ok(stdout)
}
