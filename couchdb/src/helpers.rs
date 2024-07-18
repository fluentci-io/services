use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pkgx()?
        .with_exec(vec!["mkdir", "-p", ".fluentci/couchdb"])?
        .stdout()?;

    let path = dag().get_env("PATH")?;
    dag().set_envs(vec![(
        "PATH".into(),
        format!("/home/linuxbrew/.linuxbrew/bin:{}", path),
    )])?;

    let pwd = dag().get_env("PWD")?;
    let couchdb_host = dag().get_env("COUCHDB_HOST")?;
    let couchdb_port = dag().get_env("COUCHDB_PORT")?;
    let couchdb_config = dag().get_env("COUCHDB_CONFIG")?;
    let data_dir = dag().get_env("COUCHDB_DATADIR")?;

    if couchdb_host.is_empty() {
        dag().set_envs(vec![("COUCHDB_HOST".into(), "127.0.0.1".into())])?;
    }

    if couchdb_port.is_empty() {
        dag().set_envs(vec![("COUCHDB_PORT".into(), "5984".into())])?;
    }

    if couchdb_config.is_empty() {
        dag().set_envs(vec![(
            "COUCHDB_CONFIG".into(),
            format!("{}/couchdb.ini", pwd),
        )])?;
    }

    if data_dir.is_empty() {
        dag().set_envs(vec![(
            "COUCHDB_DATADIR".into(),
            format!("{}/couchdb-data", pwd),
        )])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/couchdb")?
        .with_exec(vec![r#"type brew > /dev/null 2> /dev/null || /bin/bash -c "$(pkgx curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#])?
        .with_exec(vec!["type couchdb > /dev/null 2> /dev/null || brew install couchdb"])?
        .with_exec(vec![
            "[ -d $COUCHDB_DATADIR ] || mkdir -p $COUCHDB_DATADIR",
        ])?
        .with_exec(vec!["touch $COUCHDB_DATADIR/couchdb.uri"])?
        .with_exec(vec!["[ -f $COUCHDB_DATADIR/.erlang.cookie ] || dd if=/dev/random bs=16 count=1 status=none | base64 > $COUCHDB_DATADIR/.erlang.cookie"])?
        .with_packages(vec![
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec!["[ -f couchdb.ini.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/couchdb/couchdb.ini.template"])?
        .with_exec(vec!["[ -f $COUCHDB_CONFIG ] || pkgx envsubst < couchdb.ini.template > $COUCHDB_CONFIG "])?
        .with_exec(vec![
            "grep -q couchdb: Procfile || echo -e 'couchdb: ERL_FLAGS=\"-couch_ini $COUCHDB_CONFIG\" couchdb \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
