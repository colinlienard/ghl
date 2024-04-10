use crate::utils::process_command;
use std::{
    fs,
    io::{Error, ErrorKind},
    process::Command,
};

pub fn create_branch(branch: &str) -> Result<String, Error> {
    process_command(Command::new("git").arg("switch").arg("-c").arg(branch))
}

pub fn create_commit(msg: &str) -> Result<String, Error> {
    process_command(
        Command::new("git")
            .arg("commit")
            .arg("--allow-empty")
            .arg("-m")
            .arg(msg),
    )
}

pub fn push(branch: &str) -> Result<String, Error> {
    process_command(
        Command::new("git")
            .arg("push")
            .arg("-u")
            .arg("origin")
            .arg(branch),
    )
}

pub fn get_current_repo() -> Result<String, Error> {
    let git_config = fs::read_to_string(".git/config")?;
    for line in git_config.lines() {
        if line.contains("url = ") {
            let url = line.split("url = ").collect::<Vec<&str>>()[1];
            let repo = if url.starts_with("https://github.com/") {
                url.replace("https://github.com/", "").replace(".git", "")
            } else if url.starts_with("git@github.com:") {
                url.replace("git@github.com:", "").replace(".git", "")
            } else {
                return Err(Error::new(
                    ErrorKind::Other,
                    "Unsupported repo URL format.".to_string(),
                ));
            };

            return Ok(repo);
        }
    }
    Err(Error::new(
        ErrorKind::Other,
        "Could not find the repository.".to_string(),
    ))
}

pub fn get_default_branch() -> Result<String, Error> {
    let origin = process_command(Command::new("git").arg("remote").arg("show").arg("origin"))?;
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
