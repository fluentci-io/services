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
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/opentelemetry-collector"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let otel_port = dag().get_env("OPENTELEMETRY_PORT")?;
    let otel_http_port = dag().get_env("OPENTELEMETRY_HTTP_PORT")?;
    let otel_config = dag().get_env("OPENTELEMETRY_CONFIG")?;

    if otel_port.is_empty() {
        dag().set_envs(vec![("OPENTELEMETRY_PORT".into(), "4317".into())])?;
    }
    if otel_http_port.is_empty() {
        dag().set_envs(vec![("OPENTELEMETRY_HTTP_PORT".into(), "4318".into())])?;
    }

    if otel_config.is_empty() {
        dag().set_envs(vec![(
            "OPENTELEMETRY_CONFIG".into(),
            format!("{}/otel-config.yaml", pwd),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/opentelemetry-collector")?
        .with_exec(vec![
            "flox",
            "install",
            "opentelemetry-collector-contrib",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec!["[ -f otel-config.yaml.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/opentelemetry-collector/otel-config.yaml.template"])?
        .with_exec(vec!["[ -f $OPENTELEMETRY_CONFIG ] || pkgx envsubst < otel-config.yaml.template > $OPENTELEMETRY_CONFIG "])?
        .with_exec(vec![
            "grep -q opentelemetry-collector: Procfile || echo -e 'opentelemetry-collector: otelcontribcol --config $OPENTELEMETRY_CONFIG \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
