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
        .with_exec(vec!["httpd", "-v"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "httpd"])?
        .stdout()?;
    Ok(stdout)
}
