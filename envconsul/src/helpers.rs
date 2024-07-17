use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag().call(
        "https://pkg.fluentci.io/consul@v0.1.1?wasm=1",
        "start",
        vec![],
    )?;

    let prefix = dag().get_env("ENVCONSUL_PREFIX")?;
    if prefix.is_empty() {
        dag().set_envs(vec![("ENVCONSUL_PREFIX".into(), "my-app".into())])?;
    }
    let prefix = dag().get_env("ENVCONSUL_PREFIX")?;

    let workdir = format!(".fluentci/{}", prefix);

    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", &workdir])?
        .stdout()?;

    let app = dag().get_env("ENVCONSUL_APP")?;

    if app.is_empty() {
        dag().set_envs(vec![(
            "ENVCONSUL_APP".into(),
            "pkgx bunx serve -p $PORT".into(),
        )])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(&workdir)?
        .with_packages(vec![
            "consul.io",
            "hashicorp.com/envconsul",
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec!["consul", "kv", "put", "$ENVCONSUL_PREFIX/address", "1.2.3.4"])?
        .with_exec(vec!["consul", "kv", "put", "$ENVCONSUL_PREFIX/port", "4000"])?
        .with_exec(vec!["consul", "kv", "put", "$ENVCONSUL_PREFIX/max_conns", "5"])?
        .with_exec(vec![
            &format!("grep -q {}: Procfile || echo -e '{}: envconsul -upcase $ENVCONSUL_OPTIONS -prefix $ENVCONSUL_PREFIX $ENVCONSUL_APP \\n' >> Procfile", prefix, prefix),
        ])?
        .stdout()?;

    Ok(stdout)
}
