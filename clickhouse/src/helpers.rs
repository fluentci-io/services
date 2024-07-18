use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_flox() -> Result<(), Error> {
    let os = dag().get_os()?;
    if os == "macos" {
        dag()
        .pipeline("setup-flox")?
        .with_exec(vec![r#"type brew > /dev/null 2> /dev/null || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#])?
        .with_exec(vec!["type flox > /dev/null 2> /dev/null || brew install flox"])?
        .stdout()?;
    }
    Ok(())
}

pub fn setup() -> Result<String, Error> {
    setup_flox()?;
    dag()
        .pkgx()?
        .with_exec(vec!["mkdir", "-p", ".fluentci/clickhouse"])?
        .stdout()?;

    let clickhouse_port = dag().get_env("CLICKHOUSE_PORT")?;
    let clickhouse_http_port = dag().get_env("CLICKHOUSE_HTTP_PORT")?;
    let clickhouse_config = dag().get_env("CLICKHOUSE_CONFIG")?;
    let data_dir = dag().get_env("CLICKHOUSE_DATADIR")?;

    if clickhouse_port.is_empty() {
        dag().set_envs(vec![("CLICKHOUSE_PORT".into(), "9000".into())])?;
    }

    if clickhouse_http_port.is_empty() {
        dag().set_envs(vec![("CLICKHOUSE_HTTP_PORT".into(), "8123".into())])?;
    }

    if clickhouse_config.is_empty() {
        dag().set_envs(vec![(
            "CLICKHOUSE_CONFIG".into(),
            "../../clickhouse-config.yaml".into(),
        )])?;
    }

    if data_dir.is_empty() {
        dag().set_envs(vec![(
            "CLICKHOUSE_DATADIR".into(),
            "../../clickhouse-data".into(),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/clickhouse")?
        .with_exec(vec![
            "[ -d $CLICKHOUSE_DATADIR ] || mkdir -p $CLICKHOUSE_DATADIR",
        ])?
        .with_exec(vec![
            "flox",
            "install",
            "clickhouse",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec!["[ -f clickhouse-config.yaml.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/clickhouse/clickhouse-config.yaml.template"])?
        .with_exec(vec![
            "[ -f $CLICKHOUSE_CONFIG ] || CLICKHOUSE_CONFIG_DIR=$(ls -d .flox/run/*/etc) pkgx envsubst < clickhouse-config.yaml.template > $CLICKHOUSE_CONFIG "])?
        .with_exec(vec![
            "grep -q clickhouse: Procfile || echo -e 'clickhouse: clickhouse-server --config-file=$CLICKHOUSE_CONFIG \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
