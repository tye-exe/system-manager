#![feature(min_specialization)]

pub mod command_builder;
pub mod options;
#[cfg(test)]
mod test;

use crate::options::ToSwitch;
use app_dirs2::AppInfo;
use camino::{Utf8Path, Utf8PathBuf};
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
    pub nix_path: Box<Utf8Path>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            identity: "undefined".into(),
            nix_path: std::env::current_dir()
                .ok()
                .and_then(|var| Utf8PathBuf::from_path_buf(var).ok())
                .map(|var| var.into_boxed_path())
                .unwrap_or_else(|| Utf8Path::new("").into()),
        }
    }
}

impl Config {
    /// Parses the [`Config`] from the given filepath.
    pub fn parse(filepath: &Path) -> Result<Self, Errors> {
        use std::fs::read_to_string;
        Ok(serde_json::from_str(&read_to_string(filepath).map_err(
            |_| Errors::ConfigFileRead {
                path: filepath.into(),
            },
        )?)?)
    }

    /// Writes the given config to the given file.
    pub fn write(&self, config_path: &Path) -> Result<(), Errors> {
        let text = serde_json::to_string(self)?;
        std::fs::write(config_path, text).map_err(|_| Errors::ConfigWrite {
            path: config_path.into(),
        })?;
        Ok(())
    }
}

/// Executes shell commands to perform a nix switch.
pub fn switch<T: std::io::Write>(
    config: &Config,
    targets: &[ToSwitch],
    update: bool,
    mut executer: Executer<T>,
) -> Result<(), Errors> {
    let path = config.nix_path.clone();

    let switches_system = targets
        .iter()
        .any(|target| matches!(target, ToSwitch::System { .. }));

    if switches_system {
        executer.execute("echo 'Sudo perms required for system rebuild.'")?;
        executer.execute("sudo echo 'Sudo perms given for system rebuild.'")?;
    }

    if update {
        executer.execute(&format!("nix flake update --flake {path}"))?;
    }

    for target in targets {
        executer.execute(&
        match target {
            ToSwitch::Home => {
                format!("home-manager switch --flake {path}#{}", config.identity)
            }
            ToSwitch::System {offline} => {
                let offline_arg = if *offline {" --offline"} else {""};
                format!(
                    "sudo nixos-rebuild --option experimental-features 'nix-command flakes pipe-operators' switch --flake {path}#{}{offline_arg}", config.identity
                )
            }
        },
        )?;
    }

    Ok(())
}
