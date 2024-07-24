use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup() -> Result<String, Error> {
    let os = dag().get_os()?;
    dag()
        .pipeline("setup")?
        .with_exec(vec![
            "mkdir",
            "-p",
            ".fluentci/arangodb",
            "arangodb3-apps",
            "arangodb3-data",
        ])?
        .stdout()?;

    let arangodb_port = dag().get_env("ARANGODB_PORT")?;
    let arangodb_datadir = dag().get_env("ARANGODB_DATADIR")?;
    let arangodb_version = dag().get_env("ARANGODB_VERSION")?;
    let is_root = dag()
        .pkgx()?
        .with_exec(vec!["whoami"])?
        .stdout()?
        .contains("root");

    if arangodb_port.is_empty() {
        dag().set_envs(vec![("ARANGODB_PORT".into(), "8529".into())])?;
    }

    if is_root {
        dag().set_envs(vec![("USER".into(), "fluentci".into())])?;
    }

    if arangodb_datadir.is_empty() {
        dag().set_envs(vec![(
            "ARANGODB_DATADIR".into(),
            "../../arangodb3-data".into(),
        )])?;
    }

    if arangodb_version.is_empty() {
        dag().set_envs(vec![("ARANGODB_VERSION".into(), "3.10.5.2".into())])?;
    }

    dag().set_envs(vec![
        (
            "ICU_DATA".into(),
            ".devbox/nix/profile/default/share/arangodb3/".into(),
        ),
        (
            "TZ_DATA".into(),
            ".devbox/nix/profile/default/share/arangodb3/tzdata/".into(),
        ),
    ])?;

    if os == "linux" {
        dag()
            .devbox()?
            .with_workdir("/tmp")?
            .with_exec(vec![match is_root {
                true => "chown -R fluentci /nix",
                false => "true",
            }])?
            .with_exec(vec!["rm", "`which devbox`"])?
            .stdout()?;
    }

    let command = match os.as_str() {
        "linux" => match is_root {
            true => "grep -q arangodb: Procfile || echo -e 'arangodb: sudo -H -E -u fluentci PATH=$PATH bash -c \"devbox run arangod --configuration ../../arangod.conf\" \\n' >> Procfile",
            false => "grep -q arangodb: Procfile || echo -e 'arangodb: devbox run arangod --configuration ../../arangod.conf \\n' >> Procfile"
        },
        _ => "grep -q arangodb: Procfile || echo -e 'arangodb: pkgx docker run -p $ARANGODB_PORT:8529 arangodb:$ARANGODB_VERSION \\n' >> Procfile"
    };

    let stdout = dag()
        .devbox()?
        .with_workdir(".fluentci/arangodb")?
        .with_exec(
            match os.as_str() {
                "linux" => vec!["devbox", "add", "arangodb@$ARANGODB_VERSION", "overmind", "tmux"],
                _ => vec!["devbox", "add", "overmind", "tmux"]
            }
        )?
        .with_exec(vec!["[ -f arangod.conf.template ] || pkgx wget https://raw.githubusercontent.com/fluentci-io/services/main/arangodb/arangod.conf.template"])?
        .with_exec(vec!["[ -f ../../arangod.conf ] || pkgx envsubst < arangod.conf.template > ../../arangod.conf "])?
        .with_exec(vec![
            match is_root && os == "linux" {
                true => "chown -R fluentci ../.. /nix",
                false => "true"
            }
        ])?
        .with_exec(vec![ command ])?
        .stdout()?;

    Ok(stdout)
}
