use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag().call(
        "https://pkg.fluentci.io/consul@v0.1.1?wasm=1",
        "start",
        vec![],
    )?;

    let consul_template_config = dag().get_env("CONSUL_TEMPLATE_CONFIG")?;
    let consul_app_config = dag().get_env("CONSUL_APP_CONFIG")?;

    if consul_template_config.is_empty() {
        dag().set_envs(vec![(
            "CONSUL_TEMPLATE_CONFIG".into(),
            "../../config.hcl".into(),
        )])?;
    }

    if consul_app_config.is_empty() {
        dag().set_envs(vec![("CONSUL_APP_CONFIG".into(), "../../out.txt".into())])?;
    }

    let prefix = dag().get_env("CONSUL_APP_PREFIX")?;
    if prefix.is_empty() {
        dag().set_envs(vec![("CONSUL_APP_PREFIX".into(), "my-app".into())])?;
    }
    let prefix = dag().get_env("CONSUL_APP_PREFIX")?;

    let workdir = format!(".fluentci/{}", prefix);

    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", &workdir])?
        .stdout()?;

    let app = dag().get_env("CONSUL_APP")?;

    if app.is_empty() {
        dag().set_envs(vec![("CONSUL_APP".into(), "cat $CONSUL_APP_CONFIG".into())])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(&workdir)?
        .with_packages(vec![
            "consul.io",
            "hashicorp.com/consul-template",
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec!["consul", "kv", "put", "$CONSUL_APP_PREFIX/address", "1.2.3.4"])?
        .with_exec(vec!["consul", "kv", "put", "$CONSUL_APP_PREFIX/port", "4000"])?
        .with_exec(vec!["consul", "kv", "put", "$CONSUL_APP_PREFIX/max_conns", "5"])?
        .with_exec(vec!["[ -f config.hcl.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/consul-template/config.hcl.template"])?
        .with_exec(vec!["[ -f $CONSUL_TEMPLATE_CONFIG ] || pkgx envsubst < config.hcl.template > $CONSUL_TEMPLATE_CONFIG "])?
        .with_exec(vec![
            &format!("grep -q {}: Procfile || echo -e '{}: consul-template -config $CONSUL_TEMPLATE_CONFIG \\n' >> Procfile", prefix, prefix),
        ])?
        .stdout()?;

    Ok(stdout)
}
