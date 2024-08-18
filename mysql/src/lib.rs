use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;
    let user = dag().get_env("MYSQL_USER")?;
    let password = dag().get_env("MYSQL_PASSWORD")?;
    let database = dag().get_env("MYSQL_DATABASE")?;
    let port = dag().get_env("MYSQL_PORT")?;

    if user.is_empty() {
        dag().set_envs(vec![("MYSQL_USER".into(), "fluentci".into())])?;
    }

    if password.is_empty() {
        dag().set_envs(vec![("MYSQL_PASSWORD".into(), "fluentci".into())])?;
    }

    if database.is_empty() {
        dag().set_envs(vec![("MYSQL_DATABASE".into(), "demo".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/mysql")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["mysql", "-V"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "mysql"])?
        .with_exec(vec!["echo -e \"MySQL starting on port $MYSQL_PORT\""])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart mysql",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["cat", "$MYSQL_HOME/mysql.log"])?
        .with_exec(vec![
            "mysql",
            "-u",
            "root",
            "--socket=$MYSQL_HOME/mysql.socket -e \"CREATE DATABASE IF NOT EXISTS $MYSQL_DATABASE;\"",
        ])?
        .with_exec(vec![
            "mysql",
            "-u",
            "root",
            "--socket=$MYSQL_HOME/mysql.socket -e \"CREATE USER IF NOT EXISTS '$MYSQL_USER'@'localhost' IDENTIFIED BY '$MYSQL_PASSWORD';\"",
        ])?
        .with_exec(vec![
            "mysql",
            "-u",
            "root",
            "--socket=$MYSQL_HOME/mysql.socket -e \"GRANT ALL PRIVILEGES ON $MYSQL_DATABASE.* TO '$MYSQL_USER'@'localhost';\"",
        ])?
        .with_exec(vec![
            "mysql",
            "-u",
            "root",
            "--socket=$MYSQL_HOME/mysql.socket -e \"FLUSH PRIVILEGES;\"",
        ])?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "mysql".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/mysql")?
        .with_exec(vec!["overmind", "stop", &args])?
        .with_exec(vec![
            "mysqladmin -u root shutdown --socket=$MYSQL_HOME/mysql.socket",
        ])?
        .stdout()?;
    Ok(stdout)
}
