use std::path::Path;

use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from `Cargo.toml`
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub operation: Option<Operations>,

    /// Whether to output debug information.
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// Writes the shell completions for the given shell to stdout.
    #[arg(long)]
    pub generate: Option<Shell>,
}

#[derive(Clone, Debug, Subcommand)]
pub enum Operations {
    /// Switch the current system to the one specified in the nix configuration.
    Switch {
        #[command(subcommand)]
        target: SwitchTarget,

        /// Display the switch commands instead of executing them.
        #[arg(long = "display", global = true)]
        display_command: bool,

        /// Don't update the inputs (`flake.lock` file), only rebuild the system.
        #[arg(long = "no_update", global = true)]
        no_update: bool,
    },
    /// The identity of the nix configuration used. Different machines have different configuration.
    Identity {
        #[command(subcommand)]
        operation: IdentityOptions,
    },
    /// The path to the nix configuration; Relative paths are converted into absolute paths.
    Path {
        #[command(subcommand)]
        operation: PathOption,
    },
    /// Displays "tye-nix" in ASCII; Ignore the vanity.
    Logo,
}

#[derive(Clone, Debug, Subcommand)]
pub enum SwitchTarget {
    /// Perform a home-manager switch.
    Home,
    /// Perform a system switch.
    System,
}

#[derive(Clone, Debug, Subcommand)]
pub enum IdentityOptions {
    /// Set the identity of the configuration.
    /// The valid identities are the flake parameters (listed in "flake.nix").
    Set { identity: String },
    /// Get the identity of the configuration.
    Get {
        /// Display the raw config value.
        #[arg(long)]
        raw: bool,
    },
}

#[derive(Clone, Debug, Subcommand)]
pub enum PathOption {
    /// Sets the path to the nix configuration.
    Set { path: Box<Path> },
    /// Gets the absolute path of the nix configuration.
    Get {
        /// Display the raw config value.
        #[arg(long)]
        raw: bool,
    },
}
