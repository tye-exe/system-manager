use crate::options::parsed::{CLIArgs, IdentityOptions, PathOption, SwitchArgs, SwitchTarget};
use clap::{CommandFactory as _, Parser};
use clap_complete::Shell;
use std::path::Path;

mod parsed;

pub fn parse() -> Task {
    CLIArgs::parse().into()
}

pub fn completions(shell: Shell) {
    clap_complete::generate(
        shell,
        &mut CLIArgs::command(),
        env!("CARGO_PKG_NAME"),
        &mut std::io::stdout(),
    );
}

pub enum Task {
    Completion { shell: Shell },
    Command { option: Operation },
}

pub enum Operation {
    /// Rebuild and switch the system with the current identity.
    Switch { switch: Switch },
    /// The identity of the nix configuration to use.
    Identity { operation: Identity },
    /// The path to the nix configuration.
    Path { operation: ConfigPath },
    /// Displays "tye-nix" in ASCII; Ignore the vanity.
    Logo,
}

/// Switch configuration.
pub struct Switch {
    pub targets: Box<[ToSwitch]>,

    /// Display the switch commands instead of executing them.
    pub display_command: bool,

    /// Update the 'flake.lock' file as well as rebuilding the system.
    pub update: bool,
}

/// Target to switch.
pub enum ToSwitch {
    /// Perform a home-manager switch.
    Home,
    /// Perform a system switch.
    System {
        /// Switch system without downloading any more data.
        offline: bool,
    },
}

/// Which identity operation to perform.
pub enum Identity {
    /// Set the identity of the configuration.
    Set { identity: Box<str> },
    /// Get the identity of the configuration.
    Get {
        /// Display the raw config value.
        raw: bool,
    },
}

/// Which config path operation to perform.
pub enum ConfigPath {
    /// Sets the path to the nix configuration.
    Set { path: Box<Path> },
    /// Gets the absolute path of the nix configuration.
    Get {
        /// Display the raw config value.
        raw: bool,
    },
}

impl From<CLIArgs> for Task {
    fn from(value: CLIArgs) -> Self {
        match value {
            CLIArgs::Switch { args } => Task::Command {
                option: Operation::Switch {
                    switch: args.into(),
                },
            },
            CLIArgs::Identity { operation } => Task::Command {
                option: Operation::Identity {
                    operation: operation.into(),
                },
            },
            CLIArgs::Path { operation } => Task::Command {
                option: Operation::Path {
                    operation: operation.into(),
                },
            },
            CLIArgs::Logo => Task::Command {
                option: Operation::Logo,
            },
            CLIArgs::Completions { shell } => Task::Completion { shell },
        }
    }
}

impl From<SwitchArgs> for Switch {
    fn from(value: SwitchArgs) -> Self {
        let mut targets = Vec::new();

        match value.target {
            SwitchTarget::Home => targets.push(ToSwitch::Home),
            SwitchTarget::System { offline } => targets.push(ToSwitch::System { offline }),
            SwitchTarget::Both => {
                targets.push(ToSwitch::System { offline: true });
                targets.push(ToSwitch::Home);
            }
        };

        Self {
            targets: targets.into_boxed_slice(),
            display_command: value.display_command,
            update: value.update,
        }
    }
}

impl From<IdentityOptions> for Identity {
    fn from(value: IdentityOptions) -> Self {
        match value {
            IdentityOptions::Set { identity } => Self::Set {
                identity: identity.into(),
            },
            IdentityOptions::Get { raw } => Self::Get { raw },
        }
    }
}

impl From<PathOption> for ConfigPath {
    fn from(value: PathOption) -> Self {
        match value {
            PathOption::Set { path } => ConfigPath::Set { path },
            PathOption::Get { raw } => ConfigPath::Get { raw },
        }
    }
}
