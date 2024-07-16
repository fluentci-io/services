use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/etcd"])?
        .stdout()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/etcd")?
        .with_packages(vec![
            "etcd.io",
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec![
            "grep -q etcd Procfile || echo -e 'etcd: etcd $ETCD_OPTS \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
