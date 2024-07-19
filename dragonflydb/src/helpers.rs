use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    let os = dag().get_os()?;
    let arch = dag().get_arch()?;

    let path = dag().get_env("PATH")?;
    let home = dag().get_env("HOME")?;
    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

    if os == "linux" {
        let version = dag().get_env("DRAGONFLY_VERSION")?;
        if version.is_empty() {
            dag().set_envs(vec![("DRAGONFLY_VERSION".into(), "v1.20.1".into())])?;
        }
        dag().set_envs(vec![("ARCH".into(), arch.into())])?;

        dag()
        .pkgx()?
        .with_exec(vec!["mkdir", "-p", "$HOME/.local/bin"])?
        .with_exec(vec!["type dragonfly || pkgx wget https://github.com/dragonflydb/dragonfly/releases/download/$DRAGONFLY_VERSION/dragonfly-$ARCH.tar.gz"])?
        .with_exec(vec!["type dragonfly > /dev/null 2> /dev/null || pkgx tar -xvzf dragonfly-$ARCH.tar.gz"])?
        .with_exec(vec!["type dragonfly > /dev/null 2> /dev/null || mv dragonfly-$ARCH $HOME/.local/bin/dragonfly"])?
        .with_exec(vec!["[ -f dragonfly-$ARCH.tar.gz ] && rm -rf dragonfly-$ARCH.tar.gz LICENSE.md || true"])?
        .stdout()?;
    }

    let command = match os.as_str() {
        "linux" => "dragonfly --port $DRAGONFLY_PORT $DRAGONFLY_ARGS",
        _ => "pkgx docker run -p $DRAGONFLY_PORT:6379 --ulimit memlock=-1 docker.dragonflydb.io/dragonflydb/dragonfly"
    };

    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/dragonflydb"])?
        .stdout()?;

    let port = dag().get_env("DRAGONFLY_PORT")?;
    if port.is_empty() {
        dag().set_envs(vec![("DRAGONFLY_PORT".into(), "6379".into())])?;
    }

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/dragonflydb")?
        .with_packages(vec!["github.com/darthsim/overmind", "github.com/tmux/tmux"])?
        .with_exec(vec![&format!(
            "grep -q dragonflydb: Procfile || echo -e 'dragonflydb: {} \\n' >> Procfile",
            command
        )])?
        .stdout()?;

    Ok(stdout)
}
