use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let httpd_port = dag().get_env("HTTPD_PORT")?;
    let httpd_confdir = dag().get_env("HTTPD_CONFDIR")?;
    let httpd_error_log_file = dag().get_env("HTTPD_ERROR_LOG_FILE")?;
    let httpd_access_log_file = dag().get_env("HTTPD_ACCESS_LOG_FILE")?;

    if httpd_port.is_empty() {
        dag().set_envs(vec![("HTTPD_PORT".into(), "8080".into())])?;
    }

    if httpd_confdir.is_empty() {
        dag().set_envs(vec![("HTTPD_CONFDIR".into(), format!("{}/.fluentci", pwd))])?;
    }

    if httpd_error_log_file.is_empty() {
        dag().set_envs(vec![(
            "HTTPD_ERROR_LOG_FILE".into(),
            format!("{}/.fluentci/log/error.log", pwd),
        )])?;
    }

    if httpd_access_log_file.is_empty() {
        dag().set_envs(vec![(
            "HTTPD_ACCESS_LOG_FILE".into(),
            format!("{}/.fluentci/log/access.log", pwd),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["flox", "install", "apacheHttpd", "overmind", "tmux", "wget", "curl"])?
        .with_exec(vec!["mkdir", "-p", "log"])?
        .with_exec(vec![r#"[ -f log/.gitignore ] || echo -e 'error.log\naccess.log' > log/.gitignore"#])?
        .with_exec(vec!["[ -f httpd.conf ] || wget https://raw.githubusercontent.com/fluentci-io/services/main/apache/httpd.conf"])?
        .with_exec(vec!["[ -f index.html ] || wget https://raw.githubusercontent.com/fluentci-io/services/main/apache/web/index.html"])?
        .with_exec(vec![
            "grep -q web Procfile || echo 'web: apachectl start -f $PWD/httpd.conf -D FOREGROUND' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
