use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[clap(subcommand)]
    command: Command,
}

mod config;
mod storage;

#[derive(Parser, Debug)]
enum Command {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Config {
        #[clap(subcommand)]
        command: ConfigCommand,
    },
}

#[derive(Parser, Debug)]
enum ConfigCommand {
    Get,
    Set { key: String, value: String },
}

fn main() {
    let args = CliArgs::parse();
    let config = config::parse();

    let mut engine = storage::new_engine(&config.file_path);

    match args.command {
        Command::Get { key } => match engine.get(&key) {
            Some(value) => println!("{}", value),
            None => println!("{} not found", key),
        },
        Command::Config { command } => match command {
            ConfigCommand::Get => println!("{:#?}", config),
            ConfigCommand::Set { key, value } => {
                if key == "file_path" {
                    config::set_file_path(value);
                } else {
                    println!("Invalid key");
                }
            }
        },
        Command::Set { key, value } => {
            engine.set(&key, &value);
        }
    }
}
