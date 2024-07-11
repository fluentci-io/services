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
        .with_exec(vec!["mkdir", "-p", ".fluentci/logs"])?
        .with_exec(vec![
            "grep -q logs .fluentci/.gitignore || echo 'logs' >> .fluentci/.gitignore",
        ])?
        .stdout()?;

    let caddy_root_dir = dag().get_env("CADDY_ROOT_DIR")?;
    let caddy_log_dir = dag().get_env("CADDY_LOG_DIR")?;
    let caddy_config = dag().get_env("CADDY_CONFIG")?;
    let caddy_port = dag().get_env("CADDY_PORT")?;

    if caddy_root_dir.is_empty() {
        dag().set_envs(vec![("CADDY_ROOT_DIR".into(), "./".into())])?;
    }

    if caddy_log_dir.is_empty() {
        dag().set_envs(vec![("CADDY_LOG_DIR".into(), ".fluentci/logs".into())])?;
    }

    if caddy_port.is_empty() {
        dag().set_envs(vec![("CADDY_PORT".into(), "8082".into())])?;
    }

    let opts = if caddy_config.is_empty() {
        "--config=$CADDY_CONFIG"
    } else {
        ""
    };

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec!["flox", "install", "caddy", "overmind", "tmux", "wget", "curl"])?
        .with_exec(vec!["[ -f ../Caddyfile ] || wget https://raw.githubusercontent.com/fluentci-io/services/main/caddy/Caddyfile -O ../Caddyfile"])?
        .with_exec(vec!["[ -f ../index.html ] || wget https://raw.githubusercontent.com/fluentci-io/services/main/caddy/web/index.html -O ../index.html"])?
        .with_exec(vec![
            &format!("grep -q caddy Procfile || echo 'caddy: cd .. && caddy run {}' >> Procfile", opts),
        ])?
        .stdout()?;

    Ok(stdout)
}
