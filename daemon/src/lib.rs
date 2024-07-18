use anyhow::Error;
use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(args: String) -> FnResult<String> {
    let mut args = args.split_whitespace();
    let name = args.next().unwrap_or_default();
    let command = args.collect::<Vec<&str>>().join(" ");
    helpers::setup(name, &command)?;

    let workdir = format!(".fluentci/{}", name);

    let stdout = dag()
        .pkgx()?
        .with_workdir(&workdir)?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart",
            &name,
        ])?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    if args.is_empty() {
        return Err(Error::msg("Missing argument").into());
    }
    if args.split_whitespace().clone().count() != 1 {
        return Err(Error::msg("Invalid argument").into());
    }

    let workdir = format!(".fluentci/{}", args);

    let stdout = dag()
        .pkgx()?
        .with_workdir(&workdir)?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
