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
    dag().call(
        "https://pkg.fluentci.io/consul@v0.1.1?wasm=1",
        "start",
        vec![],
    )?;

    let prefix = dag().get_env("ENVCONSUL_PREFIX")?;
    if prefix.is_empty() {
        dag().set_envs(vec![("ENVCONSUL_PREFIX".into(), "my-app".into())])?;
    }
    let prefix = dag().get_env("ENVCONSUL_PREFIX")?;

    let workdir = format!(".fluentci/{}", prefix);

    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", &workdir])?
        .stdout()?;

    let app = dag().get_env("ENVCONSUL_APP")?;

    if app.is_empty() {
        dag().set_envs(vec![(
            "ENVCONSUL_APP".into(),
            "pkgx bunx serve -p $PORT".into(),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(&workdir)?
        .with_exec(vec![
            "flox", "install", "envconsul", "overmind", "tmux"
        ])?
        .with_exec(vec!["consul", "kv", "put", "$ENVCONSUL_PREFIX/address", "1.2.3.4"])?
        .with_exec(vec!["consul", "kv", "put", "$ENVCONSUL_PREFIX/port", "4000"])?
        .with_exec(vec!["consul", "kv", "put", "$ENVCONSUL_PREFIX/max_conns", "5"])?
        .with_exec(vec![
            &format!("grep -q {}: Procfile || echo -e 'envconsul: envconsul -upcase $ENVCONSUL_OPTIONS -prefix $ENVCONSUL_PREFIX $ENVCONSUL_APP \\n' >> Procfile", prefix),
        ])?
        .stdout()?;

    Ok(stdout)
}
