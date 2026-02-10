use crate::utils::process_command;
use std::{io::Error, process::Command};

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
    let url = process_command(
        Command::new("git")
            .arg("config")
            .arg("--get")
            .arg("remote.origin.url"),
    )?;
    let url = url.trim();
    if url.starts_with("https://github.com/") {
        Ok(url.replace("https://github.com/", "").replace(".git", ""))
    } else if url.starts_with("git@github.com:") {
        Ok(url.replace("git@github.com:", "").replace(".git", ""))
    } else {
        Err(Error::other("Unsupported repo URL format."))
    }
}

pub fn get_default_branch() -> Result<String, Error> {
    let origin = process_command(Command::new("git").arg("remote").arg("show").arg("origin"))?;
    for line in origin.lines() {
        if line.contains("HEAD branch:") {
            let branch = line.split("HEAD branch: ").collect::<Vec<&str>>()[1];
            return Ok(branch.to_string());
        }
    }
    Err(Error::other("Could not find the default branch."))
}
