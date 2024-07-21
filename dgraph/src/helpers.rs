use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/dgraph"])?
        .stdout()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/dgraph")?
        .with_packages(vec![
            "dgraph.io",
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec![
            "grep -q dgraph-zero: Procfile || echo -e 'dgraph-zero: dgraph zero \\n' >> Procfile",
        ])?
        .with_exec(vec![
            "grep -q dgraph: Procfile || echo -e 'dgraph: dgraph alpha --zero localhost:5080 \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
