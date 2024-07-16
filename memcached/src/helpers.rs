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
        .with_exec(vec!["mkdir", "-p", ".fluentci/memcached"])?
        .stdout()?;

    let memcached_port = dag().get_env("MEMCACHED_PORT")?;
    let memcached_host = dag().get_env("MEMCACHED_HOST")?;

    if memcached_port.is_empty() {
        dag().set_envs(vec![("MEMCACHED_PORT".into(), "11211".into())])?;
    }

    if memcached_host.is_empty() {
        dag().set_envs(vec![("MEMCACHED_HOST".into(), "127.0.0.1".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/memcached")?
        .with_exec(vec![
            "flox",
            "install",
            "memcached",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q memcached Procfile || echo -e 'memcached: memcached --port=$MEMCACHED_PORT --listen=$MEMCACHED_HOST -u `whoami` $MEMCACHED_OPTS \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
