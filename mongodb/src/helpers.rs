use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let mongodb_data_dir = dag().get_env("MONGODB_DATA_DIR")?;
    let mongodb_port = dag().get_env("MONGODB_PORT")?;
    let mongodb_args = dag().get_env("MONGODB_ARGS")?;
    let mongodb_version = dag().get_env("MONGODB_VERSION")?;

    if mongodb_data_dir.is_empty() {
        dag().set_envs(vec![("MONGODB_DATA_DIR".into(), "./data".into())])?;
    }

    if mongodb_port.is_empty() {
        dag().set_envs(vec![("MONGODB_PORT".into(), "27017".into())])?;
    }

    if mongodb_args.is_empty() {
        dag().set_envs(vec![("MONGODB_ARGS".into(), "--noauth".into())])?;
    }

    if mongodb_version.is_empty() {
        dag().set_envs(vec![("MONGODB_VERSION".into(), "6.0".into())])?;
    }

    let mongodb_version = dag().get_env("MONGODB_VERSION")?.replace(".", "_");

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci")?
        .with_exec(vec![
            "[ -d $MONGODB_DATA_DIR ] || mkdir -p $MONGODB_DATA_DIR",
        ])?
        .with_exec(vec![
            "devbox", "add", &format!("mongodb-{}", mongodb_version), "mongosh", "mongodb-tools", "overmind", "tmux",
        ])?
        .with_exec(vec!["devbox", "install"])?
        .with_exec(vec![
            "grep -q mongodb Procfile || echo 'mongodb: mongod  -dbpath $MONGODB_DATA_DIR --port $MONGODB_PORT $MONGODB_ARGS' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
