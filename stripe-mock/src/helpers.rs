use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    let path = dag().get_env("PATH")?;
    dag().set_envs(vec![(
        "PATH".into(),
        format!("/home/linuxbrew/.linuxbrew/bin:{}", path),
    )])?;

    let stripe_http_port = dag().get_env("STRIPE_HTTP_PORT")?;
    let stripe_https_port = dag().get_env("STRIPE_HTTPS_PORT")?;

    if stripe_http_port.is_empty() {
        dag().set_envs(vec![("STRIPE_HTTP_PORT".into(), "12111".into())])?;
    }

    if stripe_https_port.is_empty() {
        dag().set_envs(vec![("STRIPE_HTTPS_PORT".into(), "12112".into())])?;
    }

    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .with_exec(vec![r#"type brew > /dev/null 2> /dev/null || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#])?
        .with_exec(vec!["type stripe-mock > /dev/null 2> /dev/null || brew install stripe/stripe-mock/stripe-mock"])?
        .stdout()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_packages(vec!["github.com/darthsim/overmind", "github.com/tmux/tmux"])?
        .with_exec(vec![
            "grep -q stripe-mock Procfile || echo -e 'stripe-mock: stripe-mock -http-port $STRIPE_HTTP_PORT -https-port $STRIPE_HTTPS_PORT \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
