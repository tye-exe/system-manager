use clap::{Parser, Subcommand};
use clap_complete::Shell;
use std::path::Path;

/// The options passed to the program by the user.
#[derive(Parser)]
#[command(version, propagate_version = true)]
#[command(about, long_about = None)]
#[command(disable_help_subcommand = true)]
pub(crate) enum CLIArgs {
    /// Rebuild and switch the system with the current identity.
    Switch {
        #[command(flatten)]
        args: SwitchArgs,
    },
    /// The identity of the nix configuration to use.
    ///
    /// This determines which flake "#___" will be used when rebuilding the system.
    /// Different machines should use different configuration.
    Identity {
        #[command(subcommand)]
        operation: IdentityOptions,
    },
    /// The path to the nix configuration.
    ///
    ///  Relative paths are converted into absolute paths.
    Path {
        #[command(subcommand)]
        operation: PathOption,
    },
    /// Displays "tye-nix" in ASCII; Ignore the vanity.
    Logo,
    /// Writes the shell completions for the given shell to stdout.
    Completions { shell: Shell },
}

#[derive(Clone, Debug, Subcommand)]
pub(crate) enum IdentityOptions {
    /// Set the identity of the configuration.
    ///
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
pub(crate) enum PathOption {
    /// Sets the path to the nix configuration.
    Set { path: Box<Path> },
    /// Gets the absolute path of the nix configuration.
    Get {
        /// Display the raw config value.
        #[arg(long)]
        raw: bool,
    },
}

#[derive(Clone, Debug, clap::Args)]
pub(crate) struct SwitchArgs {
    #[command(subcommand)]
    pub(crate) target: SwitchTarget,

    /// Display the switch commands instead of executing them.
    #[arg(long = "display", global = true)]
    pub(crate) display_command: bool,

    /// Update the 'flake.lock' file as well as rebuilding the system.
    #[arg(long, global = true)]
    pub(crate) update: bool,
}

#[derive(Clone, Debug, Subcommand)]
pub(crate) enum SwitchTarget {
    /// Perform a home-manager switch.
    Home,
    /// Perform a system switch.
    System {
        /// Switch system without downloading any more data.
        #[arg(long, global = true)]
        offline: bool,
    },
    /// Switches the system and then home-manager.
    Both,
}
