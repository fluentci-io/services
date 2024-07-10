use anyhow::Error;
use fluentci_pdk::dag;
use fluentci_types::nix::NixArgs;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["flox", "install", "apacheHttpd", "overmind"])?
        .stdout()?;

    Ok(stdout)
}
