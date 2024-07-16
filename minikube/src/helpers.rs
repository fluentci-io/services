use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_packages(vec![
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec!["type minikube > /dev/null 2> /dev/null || pkgx install minikube"])?
        .with_exec(vec![
            "grep -q minikube Procfile || echo -e 'minikube: minikube start --force $MINIKUBE_ARGS && minikube logs -f \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
