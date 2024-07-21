use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/zipkin"])?
        .stdout()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/zipkin")?
        .with_packages(vec![
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec!["[ -f zipkin.jar ] || pkgx curl -sSL https://zipkin.io/quickstart.sh | bash -s"])?
        .with_exec(vec![
            "grep -q zipkin: Procfile || echo -e 'zipkin: pkgx java -jar zipkin.jar \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
