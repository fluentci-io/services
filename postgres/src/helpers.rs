use std::vec;

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
        .with_exec(vec!["mkdir", "-p", ".fluentci/postgres"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let is_root = dag()
        .pkgx()?
        .with_exec(vec!["whoami"])?
        .stdout()?
        .contains("root");
    let pg_data_dir = dag().get_env("PGDATA")?;
    let pg_port = dag().get_env("PGPORT")?;
    let lc_all = dag().get_env("LC_ALL")?;
    let lc_ctype = dag().get_env("LC_CTYPE")?;
    let pg_host = dag().get_env("PGHOST")?;

    if pg_port.is_empty() {
        dag().set_envs(vec![("PGPORT".into(), "5432".into())])?;
    }

    if pg_data_dir.is_empty() {
        dag().set_envs(vec![("PGDATA".into(), format!("{}/pg_data", pwd))])?;
    }

    if lc_all.is_empty() {
        dag().set_envs(vec![("LC_ALL".into(), "en_US.UTF-8".into())])?;
    }

    if lc_ctype.is_empty() {
        dag().set_envs(vec![("LC_CTYPE".into(), "en_US.UTF-8".into())])?;
    }

    if pg_host.is_empty() {
        dag().set_envs(vec![("PGHOST".into(), "localhost".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/postgres")?
        .with_exec(vec!["flox", "install", "postgresql", "overmind", "tmux"])?
        .with_exec(vec!["[ -d $PGDATA ] || mkdir -p $PGDATA"])?
        .with_exec(vec!["touch .gitignore"])?
        .with_exec(vec![
            "grep -q $PGDATA .gitignore || echo $PGDATA >> .gitignore",
        ])?
        .with_exec(vec![
            match is_root {
                true => "chown -R fluentci ../.. /nix",
                false => "true"
            }
        ])?
        .with_exec(vec![
            match is_root {
                true => "[ -f $PGDATA/postgresql.conf ] || sudo -H -E -u fluentci PATH=$PATH bash -c 'flox activate -- initdb'",
                false => "[ -f $PGDATA/postgresql.conf ] || flox activate -- initdb"
            }
        ])?
        .with_exec(vec!["chmod", "-R", "700", "$PGDATA"])?
        .with_exec(vec![
            match is_root {
                true => "grep -q postgres: Procfile || echo -e 'postgres: sudo -H -E -u fluentci PATH=$PATH bash -c \"flox activate -- postgres -k $PWD -h $PGHOST -i\"\\n' >> Procfile",
                false => 
            "grep -q postgres: Procfile || echo -e 'postgres: postgres -k $PWD -h $PGHOST -i \\n' >> Procfile"
            }
        ])?
        .stdout()?;

    Ok(stdout)
}
