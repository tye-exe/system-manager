pub mod args;

use app_dirs2::AppInfo;
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

    #[error("Failed to execute command. Error: {error}")]
    CommandExecutionFail { error: std::io::Error },
    #[error("Command failed")]
    CommandFailed { command: String },
}

/// The persistent configuration data for this program.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The identity of this system.
    pub identity: String,
    /// The path to the nix configuration.
    pub nix_path: Option<Box<Path>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            identity: "undefined".to_owned(),
            nix_path: Default::default(),
        }
    }
}
