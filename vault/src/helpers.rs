use std::vec;

use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci"])?
        .stdout()?;

    let vault_port = dag().get_env("VAULT_PORT")?;
    let vault_addr = dag().get_env("VAULT_ADDR")?;
    let vault_data_dir = dag().get_env("VAULT_DATA_DIR")?;
    let vault_disable_mlock = dag().get_env("VAULT_DISABLE_MLOCK")?;
    let vault_disable_clustering = dag().get_env("VAULT_DISABLE_CLUSTERING")?;

    if vault_port.is_empty() {
        dag().set_envs(vec![("VAULT_PORT".into(), "8200".into())])?;
    }

    if vault_addr.is_empty() {
        dag().set_envs(vec![(
            "VAULT_ADDR".into(),
            format!("127.0.0.1:{}", vault_port),
        )])?;
    }

    if vault_data_dir.is_empty() {
        dag().set_envs(vec![("VAULT_DATA_DIR".into(), "../data".into())])?;
    }

    if vault_disable_mlock.is_empty() {
        dag().set_envs(vec![("VAULT_DISABLE_MLOCK".into(), "true".into())])?;
    }

    if vault_disable_clustering.is_empty() {
        dag().set_envs(vec![("VAULT_DISABLE_CLUSTERING".into(), "true".into())])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci")?
        .with_packages(vec![
            "vaultproject.io",
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec![
            "type curl > /dev/null 2>/dev/null || pkgx install curl",
        ])?
        .with_exec(vec![
            "type jq > /dev/null 2>/dev/null || pkgx install jq",
        ])?
        .with_exec(vec![
            "[ -d ../data ] || mkdir -p ../data",
        ])?
        .with_exec(vec!["[ -f config.hcl.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/vault/config.hcl.template"])?
        .with_exec(vec!["[ -f ../config.hcl ] || envsubst < config.hcl.template > ../config.hcl "])?
        .with_exec(vec![
            "grep -q vault Procfile || echo -e 'vault: vault server -config=../config.hcl \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
