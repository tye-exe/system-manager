use std::process::ExitCode;

use app_dirs2::AppDataType;
use camino::Utf8PathBuf;
use system_manager::{
    APP_INFO, Config, Errors, LOGO,
    command_builder::Executer,
    options::{self, ConfigPath, Identity, Operation, Task},
};

fn main() -> ExitCode {
    let operation = match options::parse() {
        Task::Completion { shell } => {
            options::completions(shell);
            return ExitCode::SUCCESS;
        }
        Task::Command { option } => option,
    };

    if let Err(err) = execute(operation) {
        eprintln!("Error: {err}");
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

fn execute(operation: Operation) -> Result<(), Errors> {
    let config_path = {
        let mut path = app_dirs2::app_root(AppDataType::UserConfig, &APP_INFO)?;
        path.push("config.json");
        path.into_boxed_path()
    };

    let config = {
        let config_exists = std::fs::exists(&config_path).map_err(|_| Errors::ConfigFileRead {
            path: config_path.clone(),
        })?;

        if config_exists {
            Config::parse(config_path.as_ref())?
        } else {
            let config = Config::default();
            config.write(&config_path.as_ref())?;
            println!(
                "Set '{}' as path to 'flake.nix' file.\nTo change see 'identity' sub command",
                config.nix_path
            );
            config
        }
    };

    match operation {
        Operation::Switch { switch } => {
            let executor = Executer::new(switch.display_command, std::io::stdout());
            system_manager::switch(&config, &switch.targets, switch.update, executor)?;
        }
        Operation::Identity { operation } => match operation {
            Identity::Get { raw } => {
                if raw {
                    let identity = format!("{}", config.identity);
                    println!("{identity}")
                } else {
                    println!("Identity: {}", config.identity)
                }
            }
            Identity::Set { identity } => {
                println!("Old identity: {}", config.identity);

                let mut config = config.clone();
                config.identity = identity.trim().into();
                config.write(&config_path)?;

                println!("New identity: {}", config.identity)
            }
        },
        Operation::Path { operation } => match operation {
            ConfigPath::Set { path } => {
                let true_path = path
                    .canonicalize()
                    .map_err(|err| Errors::InvalidPath { error: err })?;

                let true_path = Utf8PathBuf::from_path_buf(true_path)
                    .map_err(|_| Errors::NotUTFPath)?
                    .into_boxed_path();

                let mut config = config.clone();
                config.nix_path = true_path;
                config.write(&config_path)?;
            }
            ConfigPath::Get { raw } => {
                if raw {
                    println!("{}", config.nix_path)
                } else {
                    println!("Nix Path: {}", config.nix_path)
                }
            }
        },
        Operation::Logo => println!("{LOGO}"),
    }

    Ok(())
}
