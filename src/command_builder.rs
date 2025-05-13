use std::{
    io::{Stdout, Write},
    process::{Command, Stdio},
};

use crate::Errors;

pub struct Executer<Out: Write> {
    display: bool,
    out: Out,
}

impl<Out: Write> Executer<Out> {
    pub fn new(display: bool, out: Out) -> Self {
        Self { display, out }
    }

    fn generate_command(&self) -> Command {
        if self.display {
            Command::new("echo")
        } else {
            let mut command = Command::new("sh");
            command.arg("-c");
            command
        }
    }
}

pub trait Execute {
    fn execute(&mut self, arg: String) -> Result<(), Errors>;
}

impl<Out: std::io::Write> Execute for Executer<Out> {
    default fn execute(&mut self, arg: String) -> Result<(), Errors> {
        let output = self
            .generate_command()
            .arg(&arg)
            .stdin(Stdio::piped())
            .output()
            .map_err(|err| Errors::CommandExecutionFail { error: err })?;

        self.out
            .write_all(&output.stdout)
            .map_err(|_| Errors::WriteOut)?;

        // If the run command failed that's an error.
        if !output.status.success() {
            Err(Errors::CommandFailed { command: arg })?;
        }

        Ok(())
    }
}

impl Execute for Executer<Stdout> {
    fn execute(&mut self, arg: String) -> Result<(), Errors> {
        let success = self
            .generate_command()
            .arg(&arg)
            .status()
            .map_err(|err| Errors::CommandExecutionFail { error: err })?
            .success();

        // If the run command failed that's an error.
        if !success {
            Err(Errors::CommandFailed { command: arg })?;
        }

        Ok(())
    }
}
