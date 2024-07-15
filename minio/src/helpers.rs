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
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let minio_dir = dag().get_env("MINIO_DIR")?;

    if minio_dir.is_empty() {
        dag().set_envs(vec![("MINIO_DIR".into(), ".".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec![
            "flox",
            "install",
            "minio",
            "minio-client",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q minio Procfile || echo -e 'minio: minio server $MINIO_DIR\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
