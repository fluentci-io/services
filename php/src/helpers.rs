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
        .with_exec(vec!["mkdir", "-p", ".fluentci/log"])?
        .with_exec(vec![
            "grep -q log .fluentci/.gitignore || echo 'log/' >> .fluentci/.gitignore",
        ])?
        .stdout()?;

    let phpfpm_port = dag().get_env("PHPFPM_PORT")?;
    let phpfpm_pid_file = dag().get_env("PHPFPM_PID_FILE")?;
    let phpfpm_error_log_file = dag().get_env("PHPFPM_ERROR_LOG_FILE")?;

    if phpfpm_port.is_empty() {
        dag().set_envs(vec![("PHPFPM_PORT".into(), "8080".into())])?;
    }

    if phpfpm_pid_file.is_empty() {
        dag().set_envs(vec![(
            "PHPFPM_PID_FILE".into(),
            ".fluentci/php-fpm.pid".into(),
        )])?;
    }

    if phpfpm_error_log_file.is_empty() {
        dag().set_envs(vec![(
            "PHPFPM_ERROR_LOG_FILE".into(),
            ".fluentci/log/php-fpm.error.log".into(),
        )])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec![
            "flox",
            "install",
            "php83Extensions.xdebug",
            "php83Extensions.imagick",
            "php83Extensions.ds",
            "php83Packages.composer",
            "php83",
            "overmind",
            "tmux",
            "wget",
            "curl",
        ])?
        .with_exec(vec!["cp ../composer.json ."])?
        .with_exec(vec!["cp ../composer.lock ."])?
        .with_exec(vec!["composer", "install"])?
        .with_exec(vec!["rm -rf ../vendor && mv vendor .."])?
        .with_exec(vec!["[ -f ../php-fpm.conf ] || wget https://raw.githubusercontent.com/fluentci-io/services/main/php/php-fpm.conf -O ../php-fpm.conf"])?
        .with_exec(vec![
            "grep -q 'php:' Procfile || echo 'php: cd .. && php public/index.php' >> Procfile",
        ])?
        .with_exec(vec![
            "grep -q php-fpm Procfile || echo 'php-fpm: cd .. && php-fpm -y $PWD/php-fpm.conf --nodaemonize' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
