use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/consul"])?
        .stdout()?;

    let consul_config = dag().get_env("CONSUL_CONFIG")?;
    let consul_port = dag().get_env("CONSUL_HTTP_PORT")?;
    let consul_data_dir = dag().get_env("CONSUL_DATA_DIR")?;
    let consul_bind_addr = dag().get_env("CONSUL_BIND_ADDR")?;

    if consul_port.is_empty() {
        dag().set_envs(vec![("CONSUL_HTTP_PORT".into(), "8500".into())])?;
    }

    if consul_data_dir.is_empty() {
        dag().set_envs(vec![("CONSUL_DATA_DIR".into(), "../../consul-data".into())])?;
    }

    if consul_bind_addr.is_empty() {
        dag().set_envs(vec![("CONSUL_BIND_ADDR".into(), "127.0.0.1".into())])?;
    }

    if consul_config.is_empty() {
        dag().set_envs(vec![(
            "CONSUL_CONFIG".into(),
            "../../consul-config.json".into(),
        )])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/consul")?
        .with_packages(vec![
           "consul.io",
            "hashicorp.com/envconsul",
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec!["[ -f consul-config.json.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/consul/consul-config.json.template"])?
        .with_exec(vec!["[ -f $CONSUL_CONFIG ] || pkgx envsubst < consul-config.json.template > $CONSUL_CONFIG "])?
        .with_exec(vec![
            "grep -q consul: Procfile || echo -e 'consul: consul agent -config-file=$CONSUL_CONFIG \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
