use app_dirs2::AppDataType;
use args::{IdentityOptions, Operation, Options, SwitchTarget};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use std::{path::Path, process::Command};
use system_manager::*;

fn main() -> Result<(), Errors> {
    // The program is designed to manage nix configs, on linux.
    if cfg!(not(target_os = "linux")) {
        Err(Errors::NotLinux)?;
    }

    let operation = match Options::parse() {
        Options::Operation(operation) => operation,
        Options::Completions { shell } => {
            generate(
                shell,
                &mut Options::command(),
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
            return Ok(());
        }
    };

    let config_path = {
        let mut path = app_dirs2::app_root(AppDataType::UserConfig, &APP_INFO)?;
        path.push("config.json");
        path.into_boxed_path()
    };

    let config = get_or_create_config(&config_path)?;

    match operation {
        Operation::Switch {
            target,
            display_command,
            no_update,
        } => {
            // Separated due to ownership limitations.
            let path = config.nix_path.clone().ok_or(Errors::PathNotSet)?;
            let path = path.to_str().ok_or(Errors::NotUTFPath)?;

            let identity = format!("{:?}", config.identity);

            // Get sudo perms firms, as flake update can take awhile.
            if let SwitchTarget::System = target {
                execute_args(
                    display_command,
                    "echo 'Sudo perms required for system rebuild.'".to_owned(),
                )?;
                execute_args(
                    display_command,
                    "sudo echo 'Sudo perms given for system rebuild.'".to_owned(),
                )?;
            }

            // Update flake lock file
            if !no_update {
                execute_args(display_command, format!("nix flake update --flake {path}"))?;
            }

            // Perform switch
            execute_args(
                display_command,
                match target {
                    SwitchTarget::Home => {
                        format!("home-manager switch --flake {path}#{identity}")
                    }
                    SwitchTarget::System => {
                        format!(
                            "sudo nixos-rebuild --option experimental-features 'nix-command flakes pipe-operators' switch --flake {path}#{identity}"
                        )
                    }
                },
            )?;
        }
        Operation::Identity { operation } => match operation {
            IdentityOptions::Get { raw } => {
                if raw {
                    let identity = format!("{:?}", config.identity);
                    println!("{identity}")
                } else {
                    println!("Identity: {:?}", config.identity)
                }
            }
            IdentityOptions::Set { identity } => {
                println!("Old identity: {:?}", config.identity);

                let mut config = config.clone();
                config.identity = identity.trim().into();
                write_config(&config, &config_path)?;

                println!("New identity: {:?}", config.identity)
            }
        },
        Operation::Path { operation } => match operation {
            args::PathOption::Set { path } => {
                let true_path = path
                    .canonicalize()
                    .map_err(|err| Errors::InvalidPath { error: err })?
                    .into_boxed_path();

                let mut config = config.clone();
                config.nix_path = Some(true_path);
                write_config(&config, &config_path)?;
            }
            args::PathOption::Get { raw } => {
                if raw {
                    let output = match config.nix_path {
                        Some(path) => path.to_str().ok_or(Errors::NotUTFPath)?.to_owned(),
                        None => "None".to_owned(),
                    };
                    println!("{output}")
                } else {
                    println!("Nix Path: {:?}", config.nix_path)
                }
            }
        },
        Operation::Logo => println!("{}", LOGO),
    }

    Ok(())
}
