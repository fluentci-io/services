use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let mongodb_data_dir = dag().get_env("MONGODB_DATA_DIR")?;
    let mongodb_port = dag().get_env("MONGODB_PORT")?;
    let mongodb_version = dag().get_env("MONGODB_VERSION")?;
    let pwd = dag().get_env("PWD")?;

    if mongodb_data_dir.is_empty() {
        dag().set_envs(vec![("MONGODB_DATA_DIR".into(), format!("{}/data", pwd))])?;
    }

    if mongodb_port.is_empty() {
        dag().set_envs(vec![("MONGODB_PORT".into(), "27017".into())])?;
    }

    if mongodb_version.is_empty() {
        dag().set_envs(vec![("MONGODB_VERSION".into(), "latest".into())])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_exec(vec![
            "[ -d $MONGODB_DATA_DIR ] || mkdir -p $MONGODB_DATA_DIR",
        ])?
        .with_packages(vec![
            "overmind", "tmux"
        ])?
        .with_exec(vec![
            "grep -q mongodb Procfile || echo -e 'mongodb: pkgx +docker.com/cli docker run -v $MONGODB_DATA_DIR:/data/db -p $MONGODB_PORT:27017 mongo:$MONGODB_VERSION\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
