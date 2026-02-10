use std::{
    io::Error,
    process::{Command, Stdio},
};

pub fn process_command(command: &mut Command) -> Result<String, Error> {
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
        Err(Error::other(String::from_utf8(output.stderr).unwrap()))
    }
}
