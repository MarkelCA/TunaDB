use clap::Parser;
use core::{
    config::{self, Config},
    storage::Engine,
};

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

pub async fn run(
    config: Config,
    engine: &mut Box<dyn Engine>,
    command: Command,
) -> anyhow::Result<()> {
    match command {
        Command::Get { key } => match engine.get(&key).await? {
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
            engine.set(&key, &value).await?;
        }
        Command::Del { key } => {
            engine.delete(&key).await?;
        }
        Command::List => {
            for key in engine.list().await? {
                println!("{}", key);
            }
        }
    };
    Ok(())
}
