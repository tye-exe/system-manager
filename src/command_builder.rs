use std::{
    io::{Stdout, Write},
    process::{Command, Stdio},
};

/// Used to execute commands.
///
/// It allows for shared output and configurations between commands.
pub struct Executer<Out: Write> {
    /// Whether the commands should be executed or displayed.
    display: bool,
    /// Where to write the commands to.
    out: Out,
}

/// The possible errors when executing commands.
#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("Failed to execute command: '{command}' Error: {err}")]
    ExecutionError {
        err: std::io::Error,
        command: Box<str>,
    },
    #[error("Command failed. Command: {command}")]
    Failed { command: Box<str> },
    #[error("Cannot write output to writer.")]
    PipeOutput,
}

impl<Out: Write> Executer<Out> {
    /// Creates a new [`Executer<Out>`].
    pub fn new(display: bool, out: Out) -> Self {
        Self { display, out }
    }

    /// Returns a pre-configured command modify then execute.
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
    /// Executes the given command.
    fn execute(&mut self, command: &str) -> Result<(), CommandError>;
}

impl<Out: std::io::Write> Execute for Executer<Out> {
    default fn execute(&mut self, command: &str) -> Result<(), CommandError> {
        let output = self
            .generate_command()
            .arg(command)
            .stdin(Stdio::piped())
            .output()
            .map_err(|err| CommandError::ExecutionError {
                err,
                command: command.into(),
            })?;

        self.out
            .write_all(&output.stdout)
            .map_err(|_| CommandError::PipeOutput)?;

        // If the run command failed that's an error.
        if !output.status.success() {
            Err(CommandError::Failed {
                command: command.into(),
            })?;
        }

        Ok(())
    }
}

impl Execute for Executer<Stdout> {
    fn execute(&mut self, command: &str) -> Result<(), CommandError> {
        let success = self
            .generate_command()
            .arg(command)
            .status()
            .map_err(|err| CommandError::ExecutionError {
                err,
                command: command.into(),
            })?
            .success();

        // If the run command failed that's an error.
        if !success {
            Err(CommandError::Failed {
                command: command.into(),
            })?;
        }

        Ok(())
    }
}
