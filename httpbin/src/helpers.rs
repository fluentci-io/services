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
        .with_exec(vec!["mkdir", "-p", ".fluentci/httpbin"])?
        .stdout()?;

    let httpbin_port = dag().get_env("HTTPBIN_PORT")?;
    if httpbin_port.is_empty() {
        dag().set_envs(vec![("HTTPBIN_PORT".into(), "8080".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/httpbin")?
        .with_exec(vec![
            "flox", "install", "python312Packages.httpbin", "python312Packages.gunicorn", "python312Packages.gevent", "overmind", "tmux", "curl",
        ])?
        .with_exec(vec![
            "grep -q httpin Procfile || echo -e 'httpbin: gunicorn httpbin:app -k gevent -b 127.0.0.1:$HTTPBIN_PORT\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
