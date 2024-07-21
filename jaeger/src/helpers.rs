use anyhow::Error;
use fluentci_pdk::dag;

pub fn install_jaeger() -> Result<(), Error> {
    let os = dag().get_os()?;
    let arch = dag().get_arch()?;

    let os = match os.as_str() {
        "linux" => "linux",
        "macos" => "darwin",
        _ => &os,
    };
    let arch = match arch.as_str() {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        _ => &arch,
    };

    dag().set_envs(vec![("OS".into(), os.into()), ("ARCH".into(), arch.into())])?;

    let version = dag().get_env("JAEGER_VERSION")?;
    if version.is_empty() {
        dag().set_envs(vec![("JAEGER_VERSION".into(), "1.59.0".into())])?;
    }

    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;
    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

    dag()
        .pkgx()?
        .with_exec(vec!["mkdir", "-p", "$HOME/.local/bin"])?
        .with_exec(vec!["type jaeger-all-in-one > /dev/null 2> /dev/null || pkgx wget https://github.com/jaegertracing/jaeger/releases/download/v${JAEGER_VERSION}/jaeger-${JAEGER_VERSION}-${OS}-${ARCH}.tar.gz"])?
        .with_exec(vec!["type jaeger-all-in-one > /dev/null 2> /dev/null || pkgx tar -xvzf jaeger-${JAEGER_VERSION}-${OS}-${ARCH}.tar.gz"])?
        .with_exec(vec!["type jaeger-all-in-one > /dev/null 2> /dev/null || cp jaeger-${JAEGER_VERSION}-${OS}-${ARCH}/jaeger-* $HOME/.local/bin"])?
        .with_exec(vec![
            "[ -d jaeger-${JAEGER_VERSION}-${OS}-${ARCH} ] && ",
            "rm",
            "-rf",
            "jaeger-${JAEGER_VERSION}-${OS}-${ARCH}*",
            " || true",
        ])?
        .stdout()?;
    Ok(())
}

pub fn setup() -> Result<String, Error> {
    install_jaeger()?;

    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", ".fluentci/jaeger"])?
        .stdout()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(".fluentci/jaeger")?
        .with_packages(vec![
            "github.com/darthsim/overmind",
            "github.com/tmux/tmux",
        ])?
        .with_exec(vec![
            "grep -q jaeger: Procfile || echo -e 'jaeger: jaeger-all-in-one $JAEGER_ARGS \\n' >> Procfile",
        ])?
        .stdout()?;

    Ok(stdout)
}
