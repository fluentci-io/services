use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("HTTPBIN_PORT")?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/httpbin")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["gunicorn", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "gunicorn"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart httpbin",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec!["curl", "-s", "http://localhost:$HTTPBIN_PORT"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "httpbin".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/httpbin")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
