use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn start(_args: String) -> FnResult<String> {
    helpers::setup()?;

    let port = dag().get_env("VAULT_PORT")?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/vault")?
        .with_exec(vec!["overmind", "--version"])?
        .with_exec(vec!["vault", "--version"])?
        .with_exec(vec!["type", "overmind"])?
        .with_exec(vec!["type", "vault"])?
        .with_exec(vec![
            "overmind start -f Procfile --daemonize || overmind restart vault",
        ])?
        .wait_on(port.parse()?, None)?
        .with_exec(vec!["overmind", "status"])?
        .with_exec(vec![
            "response=$(curl -s http://localhost:$VAULT_PORT/v1/sys/init | jq '.initialized' || true) ;\
            [ $response == \"true\" ] || response=$(curl -s --request POST --data '{\"secret_shares\": 1, \"secret_threshold\": 1}' http://localhost:$VAULT_PORT/v1/sys/init) ;\
            root_token=$(echo \"$response\" | jq -r '.root_token') ;\
            first_key_base64=$(echo \"$response\" | jq -r '.keys_base64[0]') ;\
            export VAULT_TOKEN=\"$root_token\"; \
            export UNSEAL_KEY=\"$first_key_base64\";\
            echo \"Vault Unseal key is $UNSEAL_KEY\";\
            echo \"Vault Root token is $VAULT_TOKEN\";\
            curl -s --request POST --data \"{\\\"key\\\": \\\"$UNSEAL_KEY\\\"}\" http://localhost:$VAULT_PORT/v1/sys/unseal | jq"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn stop(args: String) -> FnResult<String> {
    helpers::setup()?;

    let args = if args.is_empty() {
        "vault".to_string()
    } else {
        args
    };

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/vault")?
        .with_exec(vec!["overmind", "stop", &args])?
        .stdout()?;
    Ok(stdout)
}
