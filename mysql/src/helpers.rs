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
        .with_exec(vec!["mkdir", "-p", ".fluentci/mysql"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let mysql_home = dag().get_env("MYSQL_HOME")?;
    let mysql_data_dir = dag().get_env("MYSQL_DATADIR")?;
    let mysql_port = dag().get_env("MYSQL_PORT")?;

    if mysql_home.is_empty() {
        dag().set_envs(vec![("MYSQL_HOME".into(), format!("{}/mysql", pwd))])?;
    }

    if mysql_data_dir.is_empty() {
        dag().set_envs(vec![(
            "MYSQL_DATADIR".into(),
            format!("{}/mysql/data", pwd),
        )])?;
    }

    if mysql_port.is_empty() {
        dag().set_envs(vec![("MYSQL_PORT".into(), "3306".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/mysql")?
        .with_exec(vec![
            "flox", "install", "mysql84", "overmind", "tmux"
        ])?
        .with_exec(vec!["[ -d $MYSQL_DATADIR ] || mkdir -p $MYSQL_DATADIR"])?
        .with_exec(vec!["[ -f $MYSQL_DATADIR/ca.pem ] || flox activate -- mysqld --initialize-insecure --datadir=$MYSQL_DATADIR --log-error=$MYSQL_HOME/mysql.log"])?
        .with_exec(vec![
            "grep -q mysql Procfile || echo -e 'mysql: mysqld --datadir=$MYSQL_DATADIR --log-error=$MYSQL_HOME/mysql.log --port=$MYSQL_PORT --socket=$MYSQL_HOME/mysql.socket --user=`whoami`\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
