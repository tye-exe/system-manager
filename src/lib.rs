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

/// Executes the given args as a shell command.
pub fn execute_args(display_command: bool, arg: String) -> Result<(), Errors> {
    use std::process::Command;

    // Display/Execute command.
    let mut command;
    if display_command {
        command = Command::new("echo");
    } else {
        command = Command::new("sh");
        command.arg("-c");
    };

    // Run command.
    let success = command
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
