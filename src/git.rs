use std::{
    io::{Error, ErrorKind},
    process::{Command, Stdio},
};

pub struct Git {}

impl Git {
    pub fn create_branch(branch: &str) -> Result<(), Error> {
        Git::process_command(Command::new("git").arg("switch").arg("-c").arg(branch))
    }

    pub fn create_commit(msg: &str) -> Result<(), Error> {
        Git::process_command(
            Command::new("git")
                .arg("commit")
                .arg("--allow-empty")
                .arg("-m")
                .arg(msg),
        )
    }

    pub fn push(branch: &str) -> Result<(), Error> {
        Git::process_command(Command::new("git").arg("push").arg("origin").arg(branch))
    }

    fn process_command(command: &mut Command) -> Result<(), Error> {
        let output = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()?;

        if output.status.success() {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                String::from_utf8(output.stderr).unwrap(),
            ))
        }
    }
}

// pub fn on_default_branch() -> Result<bool, Error> {
//     let head = fs::read_to_string(".git/HEAD")?;
//     println!("{}", head);
//     Ok(true)
// }
