use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;
    let port = dag().get_env("PGPORT")?;

    let pg_user = dag().get_env("POSTGRES_USER")?;
    let pg_password = dag().get_env("POSTGRES_PASSWORD")?;
    let pg_database = dag().get_env("POSTGRES_DB")?;
    let is_root = dag()
        .pkgx()?
        .with_exec(vec!["whoami"])?
        .stdout()?
        .contains("root");
    let user = match is_root {
        true => "fluentci",
        false => "`whoami`",
    };

    if pg_user.is_empty() {
        dag().set_envs(vec![("POSTGRES_USER".into(), "postgres".into())])?;
    }

    if pg_database.is_empty() {
        dag().set_envs(vec![("POSTGRES_DB".into(), "demo".into())])?;
    }

    let with_password = match pg_password.is_empty() {
        true => "",
        false => "WITH PASSWORD '${POSTGRES_PASSWORD}'",
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/postgres")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["postgres", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "postgres"])?
        .with_exec(vec!["echo -e \"Postgres starting on port $PGPORT\""])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || flox activate -- overmind restart postgres",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec![
            &format!("psql --host=localhost -d postgres -U {} -c \"CREATE DATABASE $POSTGRES_DB;\" || true", user),
        ])?
        .with_exec(vec![
            &format!(
                "psql --host=localhost -d postgres -U {} -c \"CREATE USER $POSTGRES_USER {} CREATEDB CREATEROLE;\" || true", 
                user,
                with_password
        )
        ])?
        .with_exec(vec![
            &format!("psql --host=localhost -d $POSTGRES_DB -U {} -c \"GRANT ALL PRIVILEGES ON DATABASE $POSTGRES_DB TO $POSTGRES_USER;\"", user)])?
        .with_exec(
            vec![
                &format!("psql --host=localhost -d $POSTGRES_DB -U {} -c \"GRANT ALL ON SCHEMA public TO $POSTGRES_USER;\"", user)])?
        .with_exec(vec![
            &format!("psql --host=localhost -d $POSTGRES_DB -U {} -c \"GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO $POSTGRES_USER;\"", user)])?
        .with_exec(vec![&format!("psql --host=localhost -d $POSTGRES_DB -U {} -c \"ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL ON TABLES TO $POSTGRES_USER;\"", user)])?
        .with_exec(vec![&format!("psql --host=localhost -d $POSTGRES_DB -U {} -c \"ALTER DATABASE $POSTGRES_DB OWNER TO $POSTGRES_USER;\"", user)])?
        .with_exec(vec!["overmind", "status"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "postgres".to_string()
    } else {
        args
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/postgres")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
