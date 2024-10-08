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
        .with_exec(vec!["mkdir", "-p", ".fluentci/nginx"])?
        .with_exec(vec![
            "mkdir",
            "-p",
            ".fluentci/nginx/logs",
            ".fluentci/nginx/temp",
        ])?
        .with_exec(vec![
            "grep -q logs .fluentci/nginx/.gitignore || echo -e 'logs\\ntemp' >> .fluentci/nginx/.gitignore",
        ])?
        .stdout()?;

    let nginx_port = dag().get_env("NGINX_WEB_PORT")?;
    let nginx_web_server_name = dag().get_env("NGINX_WEB_SERVER_NAME")?;
    let nginx_web_root = dag().get_env("NGINX_WEB_ROOT")?;

    if nginx_web_server_name.is_empty() {
        dag().set_envs(vec![("NGINX_WEB_SERVER_NAME".into(), "localhost".into())])?;
    }

    if nginx_port.is_empty() {
        dag().set_envs(vec![("NGINX_WEB_PORT".into(), "8081".into())])?;
    }

    if nginx_web_root.is_empty() {
        dag().set_envs(vec![("NGINX_WEB_ROOT".into(), "../../".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/nginx")?
        .with_exec(vec!["flox", "install", "nginx", "overmind", "tmux", "wget", "curl", "gettext"])?
        .with_exec(vec!["[ -f ../../nginx.template ] || flox activate -- wget https://raw.githubusercontent.com/fluentci-io/services/main/nginx/nginx.template -O ../../nginx.template"])?
        .with_exec(vec!["[ -f fastcgi.conf ] || flox activate -- wget https://raw.githubusercontent.com/fluentci-io/services/main/nginx/fastcgi.conf"])?
        .with_exec(vec!["[ -f ../../index.html ] || flox activate -- wget https://raw.githubusercontent.com/fluentci-io/services/main/nginx/web/index.html -O ../../index.html"])?
        .with_exec(vec!["[ -f ../../nginx.template ] && flox activate -- sh -c \"envsubst < ../../nginx.template\" > nginx.conf"])?
        .with_exec(vec!["cat nginx.conf"])?
        .with_exec(vec![
            "grep -q nginx Procfile || echo -e 'nginx: nginx -p $PWD -c $PWD/nginx.conf -e error.log -g \"pid nginx.pid;daemon off;\"\\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
