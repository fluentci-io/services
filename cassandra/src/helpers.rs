use std::vec;

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
        .with_exec(vec!["mkdir", "-p", ".fluentci/cassandra"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let cassandra_port = dag().get_env("CASSANDRA_PORT")?;
    let data_dir = dag().get_env("CASSANDRA_DATADIR")?;
    let cluster_name = dag().get_env("CASSANDRA_CLUSTER_NAME")?;

    if cassandra_port.is_empty() {
        dag().set_envs(vec![("CASSANDRA_PORT".into(), "9042".into())])?;
    }

    if data_dir.is_empty() {
        dag().set_envs(vec![(
            "CASSANDRA_DATADIR".into(),
            format!("{}/cassandra-data", pwd),
        )])?;
    }

    if cluster_name.is_empty() {
        dag().set_envs(vec![(
            "CASSANDRA_CLUSTER_NAME".into(),
            "Test Cluster".into(),
        )])?;
    }

    dag().set_envs(vec![(
        "CASSANDRA_CONFIG".into(),
        format!("{}/cassandra.yaml", pwd),
    )])?;

    let data_dir = dag().get_env("CASSANDRA_DATADIR")?;
    dag().set_envs(vec![(
        "CASSANDRA_LOG_DIR".into(),
        format!("{}/logs", data_dir),
    )])?;

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/cassandra")?
        .with_exec(vec![
            "[ -d $CASSANDRA_DATADIR ] || mkdir -p $CASSANDRA_DATADIR",
        ])?
        .with_exec(vec!["mkdir", "-p", "$CASSANDRA_LOG_DIR"])?
        .with_exec(vec!["flox", "install", "cassandra_4", "overmind", "tmux"])?
        .with_exec(vec!["[ -f cassandra.yaml.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/cassandra/cassandra.yaml.template"])?
        .with_exec(vec!["[ -f $CASSANDRA_CONFIG ] || pkgx envsubst < cassandra.yaml.template > $CASSANDRA_CONFIG "])?
        .with_exec(vec![
            "grep -q cassandra: Procfile || echo -e 'cassandra: JVM_OPTS=\"-Xlog:gc=warning,heap*=warning,age*=warning,safepoint=warning,promotion*=warning\" cassandra -Dcassandra.config=file://$CASSANDRA_CONFIG -R -f \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
