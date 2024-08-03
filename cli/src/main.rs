extern crate core;

use clap::Parser;
use core::config::{self, Config};
use core::storage::{self, Engine};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    /// Get the value for the specified key
    Get { key: String },
    /// Sets the value for the specified key
    Set { key: String, value: String },
    /// Deletes the specified key
    Del { key: String },
    /// Lists all keys in the database
    List,
    /// Manages the database configuration
    Config {
        #[clap(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(Parser, Debug)]
enum ConfigCommand {
    Get,
    Set {
        #[clap(subcommand)]
        command: SetConfigCommand,
    },
}

#[derive(Parser, Debug)]
enum SetConfigCommand {
    FilePath { value: String },
}

fn main() -> ExitCode {
    let args = CliArgs::parse();

    let config = config::parse().expect("config couldn't be found");
    let mut engine = storage::new_engine(&config.file_path).expect("Couldn't create engine");

    match run_command(config, &mut engine, args.command) {
        Err(err) => {
            println!("Error: {}", err.to_string());
            ExitCode::from(1)
        }
        Ok(_) => ExitCode::from(0),
    }
}

fn run_command(
    config: Config,
    engine: &mut Box<dyn Engine>,
    command: Command,
) -> anyhow::Result<()> {
    match command {
        Command::Get { key } => match engine.get(&key)? {
            Some(value) => println!("{}", value),
            None => println!("(nil)"),
        },
        Command::Config { command } => match command {
            ConfigCommand::Get => println!("{:#?}", config),
            ConfigCommand::Set { command } => match command {
                SetConfigCommand::FilePath { value } => {
                    config::set_file_path(value)?;
                }
            },
        },
        Command::Set { key, value } => {
            engine.set(&key, &value)?;
        }
        Command::Del { key } => {
            engine.delete(&key)?;
        }
        Command::List => {
            for key in engine.list()? {
                println!("{}", key);
            }
        }
    };
    Ok(())
}
