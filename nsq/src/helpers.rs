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
        .stdout()?;

    let nsqlookupd_host = dag().get_env("NSQLOOKUPD_HOST")?;
    let nsqlookupd_port = dag().get_env("NSQLOOKUPD_PORT")?;

    if nsqlookupd_host.is_empty() {
        dag().set_envs(vec![("NSQLOOKUPD_HOST".into(), "127.0.0.1".into())])?;
    }

    if nsqlookupd_port.is_empty() {
        dag().set_envs(vec![("NSQLOOKUPD_PORT".into(), "4160".into())])?;
    }

    let stdout = dag()
        .flox()?
        .with_workdir(".fluentci")?
        .with_exec(vec![
            "flox",
            "install",
            "nsq",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec![
            "grep -q nslookupd: Procfile || echo -e 'nsqlookupd: nsqlookupd \\n' >> Procfile",
        ])?
        .with_exec(vec![
            "grep -q nsqd: Procfile || echo -e 'nsqd: nsqd --lookupd-tcp-address=$NSQLOOKUPD_HOST:$NSQLOOKUPD_PORT \\n' >> Procfile",
        ])?
        .with_exec(vec![
            "grep -q nsqadmin: Procfile || echo -e 'nsqadmin: nsqadmin --lookupd-http-address=127.0.0.1:4161 \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
