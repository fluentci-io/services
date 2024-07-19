use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    let path = dag().get_env("PATH")?;
    let home = dag().get_env("HOME")?;
    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

    let os = dag().get_os()?;
    let arch = dag().get_arch()?;

    let os = match os.as_str() {
        "linux" => "unknown-linux-gnu",
        "macos" => "apple-darwin",
        _ => &os,
    };

    dag().set_envs(vec![("OS".into(), os.into()), ("ARCH".into(), arch.into())])?;

    let version = dag().get_env("QUICKWIT_VERSION")?;
    if version.is_empty() {
        dag().set_envs(vec![("QUICKWIT_VERSION".into(), "v0.8.2".into())])?;
    }

    dag()
        .pkgx()?
        .with_exec(vec!["mkdir", "-p", ".fluentci/quickwit", "$HOME/.local/bin"])?
        .with_exec(vec!["type quickwit || pkgx wget https://github.com/quickwit-oss/quickwit/releases/download/$QUICKWIT_VERSION/quickwit-$QUICKWIT_VERSION-$ARCH-$OS.tar.gz"])?
        .with_exec(vec!["type quickwit || pkgx tar -xvzf quickwit-$QUICKWIT_VERSION-$ARCH-$OS.tar.gz"])?
        .with_exec(vec!["type quickwit || cp quickwit-$QUICKWIT_VERSION/quickwit $HOME/.local/bin"])?
        .with_exec(vec!["[ -d quickwit-$QUICKWIT_VERSION ] && rm -rf quickwit-$QUICKWIT_VERSION* || true"])?
        .stdout()?;

    let pwd = dag().get_env("PWD")?;
    let quick_port = dag().get_env("QUICKWIT_PORT")?;
    let data_dir = dag().get_env("QUICKWIT_DATA_DIR")?;
    let quickwit_config = dag().get_env("QUICKWIT_CONFIG")?;

    if quick_port.is_empty() {
        dag().set_envs(vec![("QUICKWIT_PORT".into(), "7280".into())])?;
    }

    if data_dir.is_empty() {
        dag().set_envs(vec![(
            "QUICKWIT_DATA_DIR".into(),
            format!("{}/qwdata", pwd),
        )])?;
    }

    if quickwit_config.is_empty() {
        dag().set_envs(vec![(
            "QUICKWIT_CONFIG".into(),
            format!("{}/config/quickwit.yaml", pwd),
        )])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/quickwit")?
        .with_packages(vec![
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec!["mkdir -p $QUICKWIT_DATA_DIR"])?
        .with_exec(vec!["mkdir -p ../../config"])?
        .with_exec(vec!["[ -f quickwit.yaml.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/quickwit/quickwit.yaml.template"])?
        .with_exec(vec!["[ -f $QUICKWIT_CONFIG ] || pkgx envsubst < quickwit.yaml.template > $QUICKWIT_CONFIG "])?
        .with_exec(vec![
            "grep -q quickwit: Procfile || echo -e 'quickwit: cd ../.. && quickwit run \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
