use std::process::ExitCode;

use app_dirs2::AppDataType;
use args::{IdentityOptions, Operation, Options};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use system_manager::{command_builder::Executer, *};

fn main() -> ExitCode {
    let operation = match Options::parse() {
        Options::Operation(operation) => operation,
        Options::Completions { shell } => {
            generate(
                shell,
                &mut Options::command(),
                env!("CARGO_PKG_NAME"),
                &mut std::io::stdout(),
            );
            return ExitCode::SUCCESS;
        }
    };

    if let Err(err) = execute(operation) {
        eprintln!("Error: {err}");
        return ExitCode::FAILURE;
    };

    ExitCode::SUCCESS
}

fn execute(operation: Operation) -> Result<(), Errors> {
    // The program is designed to manage nix configs, on linux.
    if cfg!(not(target_os = "linux")) {
        Err(Errors::NotLinux)?;
    }

    let config_path = {
        let mut path = app_dirs2::app_root(AppDataType::UserConfig, &APP_INFO)?;
        path.push("config.json");
        path.into_boxed_path()
    };

    let config = get_or_create_config(&config_path)?;

    match operation {
        Operation::Switch { args } => {
            let executor = Executer::new(args.display_command, std::io::stdout());
            switch(&config, args, executor)?;
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
