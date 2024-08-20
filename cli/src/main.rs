extern crate core;
use std::io::stdin;

use clap::Parser;
use command::Command;
use core::config;
use core::storage;
use std::process::ExitCode;

mod command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CliArgs {
    #[clap(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> ExitCode {
    let args = CliArgs::parse();

    let mut buffer = String::new();

    loop {
        let _ = stdin().read_line(&mut buffer);

        if buffer.trim() == "exit" {
            println!("bye");
            break;
        }
    }

    /////////////////////////

    let config = config::parse().expect("config couldn't be found");
    let mut engine = storage::new_engine(&config.file_path).expect("Couldn't create engine");

    match command::run(config, &mut engine, args.command).await {
        Err(err) => {
            println!("Error: {}", err.to_string());
            ExitCode::from(1)
        }
        Ok(_) => ExitCode::from(0),
    }
}
