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
        "https://pkg.fluentci.io/consul@v0.1.0?wasm=1",
        "start",
        vec![],
    )?;

    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let prefix = dag().get_env("ENVCONSUL_PREFIX")?;
    let app = dag().get_env("ENVCONSUL_APP")?;

    if prefix.is_empty() {
        dag().set_envs(vec![("ENVCONSUL_PREFIX".into(), "my-app".into())])?;
    }

    if app.is_empty() {
        dag().set_envs(vec![("ENVCONSUL_APP".into(), "env".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec![
            "flox", "install", "envconsul", "overmind", "tmux"
        ])?
        .with_exec(vec!["consul", "kv", "put", "my-app/address", "1.2.3.4"])?
        .with_exec(vec!["consul", "kv", "put", "my-app/port", "80"])?
        .with_exec(vec!["consul", "kv", "put", "my-app/max_conns", "5"])?
        .with_exec(vec![
            "grep -q envconsul: Procfile || echo -e 'envconsul: envconsul -prefix $ENVCONSUL_PREFIX $ENVCONSUL_APP > envconsul.log \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
