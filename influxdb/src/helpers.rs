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
        .with_exec(vec!["mkdir", "-p", ".fluentci/influxdb"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let influxdb_port = dag().get_env("INFLUXDB_PORT")?;
    let influxdb_config = dag().get_env("INFLUXDB_CONFIG")?;

    if influxdb_port.is_empty() {
        dag().set_envs(vec![("INFLUXDB_PORT".into(), "8086".into())])?;
    }

    if influxdb_config.is_empty() {
        dag().set_envs(vec![(
            "INFLUXDB_CONFIG".into(),
            format!("{}/influxdb.conf", pwd),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/influxdb")?
        .with_exec(vec![
            "flox",
            "install",
            "influxdb",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec!["[ -f $INFLUXDB_CONFIG ] || touch $INFLUXDB_CONFIG"])?
        .with_exec(vec![
            "grep -q influxdb: Procfile || echo -e 'influxdb: influxd -config $INFLUXDB_CONFIG --http-bind-address $INFLUXDB_PORT \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
