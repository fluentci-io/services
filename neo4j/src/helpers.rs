use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/neo4j"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    dag().set_envs(vec![
        ("NEO4J_CONF".into(), format!("{}", pwd)),
        ("NEO4J_HOME".into(), format!("{}/neo4j", pwd)),
    ])?;

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/neo4j")?
        .with_exec(vec!["devbox", "add", "neo4j", "overmind", "tmux"])?
        .with_exec(vec![
            "cp -r `realpath .devbox/nix/profile/default/share/neo4j` ../..",
        ])?
        .with_exec(vec!["chmod u+rw -R ../../neo4j"])?
        .with_exec(vec!["[ -f ../../neo4j.conf ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/neo4j/neo4j.conf -O ../../neo4j.conf"])?
        .with_exec(vec![
           "grep -q neo4j: Procfile || echo -e 'neo4j: devbox run neo4j console $NEO4J_ARGS \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
