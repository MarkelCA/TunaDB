use clap::Parser;
use core::config::{self, Config};
use core::storage::{Engine, EngineEnum};

#[derive(Parser, Debug)]
pub enum Command {
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
pub enum ConfigCommand {
    Get,
    Set {
        #[clap(subcommand)]
        command: SetConfigCommand,
    },
}

#[derive(Parser, Debug)]
pub enum SetConfigCommand {
    FilePath { value: String },
}
pub fn run(config: Config, engine: &mut EngineEnum, command: Command) -> anyhow::Result<()> {
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
