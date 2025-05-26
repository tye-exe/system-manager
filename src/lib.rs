#![feature(min_specialization)]

pub mod args;
pub mod command_builder;
#[cfg(test)]
mod test;

use app_dirs2::AppInfo;
use args::{SwitchArgs, SwitchTarget};
use command_builder::{CommandError, Execute as _, Executer};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Holds data for [app_dirs2].
pub const APP_INFO: AppInfo = AppInfo {
    name: "SystemManager",
    author: "tye",
};

/// ASCII art of "tye-nix".
pub const LOGO: &str = r#"
------------------------------------------------------------------
 ______   __  __     ______           __   __     __     __  __
/\__  _\ /\ \_\ \   /\  ___\         /\ "-.\ \   /\ \   /\_\_\_\
\/_/\ \/ \ \____ \  \ \  __\         \ \ \-.  \  \ \ \  \/_/\_\/_
   \ \_\  \/\_____\  \ \_____\        \ \_\\"\_\  \ \_\   /\_\/\_\
    \/_/   \/_____/   \/_____/         \/_/ \/_/   \/_/   \/_/\/_/

------------------------------------------------------------------
"#;

/// The possible errors this program can encounter.
#[derive(thiserror::Error, Debug)]
pub enum Errors {
    #[error("This program only supports Linux, because it only makes sense to run on Linux.")]
    NotLinux,
    #[error("No command was given.")]
    NoCommand,

    #[error("{0}")]
    ConfigPath(#[from] app_dirs2::AppDirsError),
    #[error("Unable to read config at path: {path}")]
    ConfigFileRead { path: Box<Path> },
    #[error("{0}")]
    ConfigParse(#[from] serde_json::Error),
    #[error("Unable to write config to path: {path}")]
    ConfigWrite { path: Box<Path> },

    #[error("{error}")]
    InvalidPath { error: std::io::Error },
    #[error("The path to the nix config has not been set. See option \"path\".")]
    PathNotSet,
    #[error(
        "The set path is not a valid UTF-8 string. Please set the path to a valid UTF-8 string."
    )]
    NotUTFPath,

    #[error(transparent)]
    CommandError(#[from] CommandError),
}

/// The persistent configuration data for this program.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    /// The identity of this system.
    pub identity: Box<str>,
    /// The path to the nix configuration.
    pub nix_path: Option<Box<Path>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            identity: "undefined".into(),
            nix_path: Default::default(),
        }
    }
}

/// Returns the current configuration.
///
/// If there is no configuration then a default config is written and returned.
pub fn get_or_create_config(filepath: &Path) -> Result<Config, Errors> {
    let config_exists = std::fs::exists(&filepath).map_err(|_| Errors::ConfigFileRead {
        path: filepath.into(),
    })?;

    if config_exists {
        use std::fs::read_to_string;
        Ok(serde_json::from_str(&read_to_string(filepath).map_err(
            |_| Errors::ConfigFileRead {
                path: filepath.into(),
            },
        )?)?)
    } else {
        let config = Config::default();
        write_config(&config, filepath)?;
        Ok(config)
    }
}

/// Writes the given config to the given file.
pub fn write_config(config: &Config, config_path: &Path) -> Result<(), Errors> {
    let text = serde_json::to_string(config)?;
    std::fs::write(config_path, text).map_err(|_| Errors::ConfigWrite {
        path: config_path.into(),
    })?;
    Ok(())
}

/// Performs a system rebuild and switch.
pub fn switch<T: std::io::Write>(
    config: &Config,
    args: SwitchArgs,
    mut executer: Executer<T>,
) -> Result<(), Errors> {
    let SwitchArgs {
        target,
        display_command: _,
        no_update,
    } = args;

    let path = config.nix_path.clone().ok_or(Errors::PathNotSet)?;
    let path = path.to_str().ok_or(Errors::NotUTFPath)?;

    if let SwitchTarget::System = target {
        executer.execute("echo 'Sudo perms required for system rebuild.'")?;
        executer.execute("sudo echo 'Sudo perms given for system rebuild.'")?;
    }

    if !no_update {
        executer.execute(&format!("nix flake update --flake {path}"))?;
    }

    executer.execute(&
        match target {
            SwitchTarget::Home => {
                format!("home-manager switch --flake {path}#{}", config.identity)
            }
            SwitchTarget::System => {
                format!(
                    "sudo nixos-rebuild --option experimental-features 'nix-command flakes pipe-operators' switch --flake {path}#{}", config.identity
                )
            }
        },
    )?;
    Ok(())
}
