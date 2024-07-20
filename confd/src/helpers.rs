use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_flox() -> Result<(), Error> {
    let os = dag().get_os()?;
    if os == "macos" {
        dag()
        .pipeline("setup-flox")?
        .with_exec(vec![r#"type brew > /dev/null 2> /dev/null || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#])?
        .with_exec(vec!["type flox > /dev/null 2> /dev/null || brew install flox"])?
        .stdout()?;
    }
    Ok(())
}

pub fn setup() -> Result<String, Error> {
    setup_flox()?;
    let confd_backend = dag().get_env("CONFD_BACKEND")?;

    let pwd = dag().get_env("PWD")?;
    let confdir = dag().get_env("CONFDIR")?;
    if confdir.is_empty() {
        dag().set_envs(vec![("CONFDIR".into(), pwd.clone())])?;
    }

    let prefix = dag().get_env("CONFD_APP_PREFIX")?;
    if prefix.is_empty() {
        dag().set_envs(vec![("CONFD_APP_PREFIX".into(), "myapp".into())])?;
    }
    let prefix = dag().get_env("CONFD_APP_PREFIX")?;

    if dag()
        .pkgx()?
        .with_exec(vec!["pkgx curl http://localhost:8500 || echo KO"])?
        .stdout()?
        .contains("KO")
        && confd_backend == "consul"
    {
        dag().call(
            "https://pkg.fluentci.io/consul@v0.1.1?wasm=1",
            "start",
            vec![],
        )?;

        dag()
            .pkgx()?
            .with_exec(vec![
                "consul",
                "kv",
                "put",
                "$CONFD_APP_PREFIX/database/url",
                "http://localhost:4000",
            ])?
            .stdout()?;
    }

    if dag()
        .pkgx()?
        .with_exec(vec![
            "pkgx deno run -A npm:wait-port -t 2000 2379 || echo KO",
        ])?
        .stdout()?
        .contains("KO")
        && confd_backend == "etcd"
    {
        dag().call(
            "https://pkg.fluentci.io/etcd@v0.1.1?wasm=1",
            "start",
            vec![],
        )?;
    }

    let confd_config = dag().get_env("CONFD_CONFIG")?;

    if confd_config.is_empty() {
        dag().set_envs(vec![(
            "CONFD_CONFIG".into(),
            format!("{}/config.toml", pwd),
        )])?;
    }

    let workdir = format!(".fluentci/{}", prefix);

    dag()
        .pipeline("setup")?
        .with_exec(vec![
            "mkdir",
            "-p",
            &workdir,
            "$CONFDIR/conf.d",
            "$CONFDIR/templates",
        ])?
        .stdout()?;

    let stdout = dag()
        .flox()?
        .with_workdir(&workdir)?
        .with_exec(vec![
            "flox",
            "install",
            "confd",
            "overmind",
            "tmux",
        ])?
        .with_exec(vec!["mkdir", "-p", "$CONFDIR/conf.d", "$CONFDIR/templates"])?
        .with_exec(vec!["[ -f confd.toml.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/confd/config.toml.template"])?
        .with_exec(vec!["[ -f $CONFD_CONFIG ] || pkgx envsubst < config.toml.template > $CONFD_CONFIG "])?
        .with_exec(vec!["[ -f $CONFDIR/templates/sample.conf.tmpl ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/confd/sample.conf.tmpl -O $CONFDIR/templates/sample.conf.tmpl"])?
        .with_exec(vec!["[ -f $CONFDIR/conf.d/sample.toml ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/confd/sample.toml -O $CONFDIR/conf.d/sample.toml"])?
        .with_exec(vec![
            &format!("grep -q {}: Procfile || echo -e '{}: confd -config-file $CONFD_CONFIG \\n' >> Procfile", prefix, prefix),
        ])?
        .stdout()?;

    Ok(stdout)
}
