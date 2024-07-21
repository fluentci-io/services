use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("INFLUXDB_PORT")?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/influxdb")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "influxd"])?
        .with_exec(vec!["influxd", "version"])?
        .with_exec(vec!["echo -e \"InfluxDB starting on port $INFLUXDB_PORT\""])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart influxdb",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "influxdb".to_string()
    } else {
        args
    };

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/influxdb")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
