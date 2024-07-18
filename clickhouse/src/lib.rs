use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("CLICKHOUSE_PORT")?;
    let http_port = dag().get_env("CLICKHOUSE_HTTP_PORT")?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/clickhouse")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "clickhouse-server"])?
        .with_exec(vec!["clickhouse-server", "--version"])?
        .with_exec(vec![
            "echo -e \"Clickhouse starting on port $CLICKHOUSE_PORT\"",
        ])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart clickhouse",
        ])?
        .wait_on(port.parse()?, None)?
        .wait_on(http_port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "clickhouse".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/clickhouse")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
