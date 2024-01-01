use std::{
    fs,
    io::{Error, ErrorKind},
    process::{Command, Stdio},
};

pub struct Git {}

impl Git {
    pub fn create_branch(branch: &str) -> Result<String, Error> {
        Git::process_command(Command::new("git").arg("switch").arg("-c").arg(branch))
    }

    pub fn create_commit(msg: &str) -> Result<String, Error> {
        Git::process_command(
            Command::new("git")
                .arg("commit")
                .arg("--allow-empty")
                .arg("-m")
                .arg(msg),
        )
    }

    pub fn push(branch: &str) -> Result<String, Error> {
        Git::process_command(Command::new("git").arg("push").arg("origin").arg(branch))
    }

    pub fn get_current_repo() -> Result<String, Error> {
        let git_config = fs::read_to_string(".git/config")?;
        for line in git_config.lines() {
            if line.contains("url = ") {
                let url = line.split("url = ").collect::<Vec<&str>>()[1];
                let repo = url.replace("https://github.com/", "").replace(".git", "");
                return Ok(repo);
            }
        }
        Err(Error::new(
            ErrorKind::Other,
            "Could not find the repository.".to_string(),
        ))
    }

    pub fn get_default_branch() -> Result<String, Error> {
        let origin =
            Git::process_command(Command::new("git").arg("remote").arg("show").arg("origin"))?;
        for line in origin.lines() {
            if line.contains("HEAD branch:") {
                let branch = line.split("HEAD branch: ").collect::<Vec<&str>>()[1];
                return Ok(branch.to_string());
            }
        }
        Err(Error::new(
            ErrorKind::Other,
            "Could not find the default branch.".to_string(),
        ))
    }

    fn process_command(command: &mut Command) -> Result<String, Error> {
        let output = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()?;

        if output.status.success() {
            Ok(String::from_utf8(output.stdout).unwrap())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
    }
}
