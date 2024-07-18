use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup(name: &str, command: &str) -> Result<String, Error> {
    let workdir = format!(".fluentci/{}", name);

    dag()
        .pipeline("setup")?
        .with_exec(vec!["mkdir", "-p", &workdir])?
        .stdout()?;

    let stdout = dag()
        .pkgx()?
        .with_workdir(&workdir)?
        .with_packages(vec!["github.com/darthsim/overmind", "github.com/tmux/tmux"])?
        .with_exec(vec![&format!(
            "grep -q {}: Procfile || echo -e '{}: {} \\n' >> Procfile",
            name, name, command
        )])?
        .stdout()?;

    Ok(stdout)
}
