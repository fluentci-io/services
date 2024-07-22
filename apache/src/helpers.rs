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
        .with_exec(vec!["mkdir", "-p", ".fluentci/apache"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let httpd_port = dag().get_env("HTTPD_PORT")?;
    let httpd_confdir = dag().get_env("HTTPD_CONFDIR")?;
    let httpd_error_log_file = dag().get_env("HTTPD_ERROR_LOG_FILE")?;
    let httpd_access_log_file = dag().get_env("HTTPD_ACCESS_LOG_FILE")?;
    let is_root = dag()
        .pkgx()?
        .with_exec(vec!["whoami"])?
        .stdout()?
        .contains("root");

    if httpd_port.is_empty() {
        dag().set_envs(vec![("HTTPD_PORT".into(), "8080".into())])?;
    }

    if httpd_confdir.is_empty() {
        dag().set_envs(vec![(
            "HTTPD_CONFDIR".into(),
            format!("{}/.fluentci/apache", pwd),
        )])?;
    }

    if httpd_error_log_file.is_empty() {
        dag().set_envs(vec![(
            "HTTPD_ERROR_LOG_FILE".into(),
            format!("{}/.fluentci/apache/log/error.log", pwd),
        )])?;
    }

    if httpd_access_log_file.is_empty() {
        dag().set_envs(vec![(
            "HTTPD_ACCESS_LOG_FILE".into(),
            format!("{}/.fluentci/apache/log/access.log", pwd),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/apache")?
        .with_exec(vec!["flox", "install", "apacheHttpd", "overmind", "tmux", "wget", "curl"])?
        .with_exec(vec!["mkdir", "-p", "log"])?
        .with_exec(vec![r#"[ -f log/.gitignore ] || echo -e 'error.log\naccess.log' > log/.gitignore"#])?
        .with_exec(vec!["[ -f httpd.conf ] || flox activate -- wget https://raw.githubusercontent.com/fluentci-io/services/main/apache/httpd.conf"])?
        .with_exec(vec!["[ -f ../../index.html ] || flox activate -- wget https://raw.githubusercontent.com/fluentci-io/services/main/apache/web/index.html -O ../../index.html"])?
        .with_exec(vec![
            match is_root {
                true => "chown -R fluentci ../.. /nix",
                false => "true"
            }
        ])?
        .with_exec(vec![
            match is_root {
                true => "grep -q web Procfile || echo -e 'web: sudo -H -E -u fluentci PATH=$PATH bash -c \"flox activate -- apachectl start -f $PWD/httpd.conf -D FOREGROUND \" \\n' >> Procfile",
                false => "grep -q web Procfile || echo -e 'web: apachectl start -f $PWD/httpd.conf -D FOREGROUND\\n' >> Procfile"
            }
            ,
        ])?
        .stdout()?;

    Ok(stdout)
}
