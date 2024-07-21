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
        .with_exec(vec!["mkdir", "-p", ".fluentci/mailcatcher"])?
        .stdout()?;

    let smtp_port = dag().get_env("MAILCATCHER_SMTP_PORT")?;
    let http_port = dag().get_env("MAILCATCHER_HTTP_PORT")?;

    if smtp_port.is_empty() {
        dag().set_envs(vec![("MAILCATCHER_SMTP_PORT".into(), "1025".into())])?;
    }

    if http_port.is_empty() {
        dag().set_envs(vec![("MAILCATCHER_HTTP_PORT".into(), "1080".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci/mailcatcher")?
        .with_exec(vec![
            "flox",
            "install",
            "ruby",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec!["gem", "install", "mailcatcher"])?
        .with_exec(vec!["[ -d $HOME/.local/bin ] || mkdir -p $HOME/.local/bin"])?
        .with_exec(vec!["ln -s `flox activate -- gem environment gemhome`/bin/mailcatcher $HOME/.local/bin/mailcatcher || true"])?
        .with_exec(vec![
            "grep -q mailcatcher: Procfile || echo -e 'mailcatcher: PATH=`flox activate -- gem environment gemhome`/bin:$PATH mailcatcher --http-port $MAILCATCHER_HTTP_PORT --smtp-port MAILCATCHER_SMTP_PORT -f \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
