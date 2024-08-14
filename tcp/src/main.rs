use args::Args;
use clap::Parser;
use command::Command;
use env_logger::Env;
use std::str::FromStr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use core::config;
use core::storage::{self, Engine, EngineEnum};
use log;

mod args;
mod command;
#[path = "./storage.rs"]
mod tcp_storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    env_logger::init_from_env(Env::default().default_filter_or(args.log_level.to_string()));

    log::info!("Starting server");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let config = config::parse()?;
    let engine = storage::new_engine(&config.file_path)?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let mut engine = engine.clone(); // Clone the engine for each connection

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        log::error!("failed to read from socket; err = {:?}", e);
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                let command = std::str::from_utf8(&buf[0..n]);

                if let Err(e) = command {
                    if let Err(e) = socket.write_all(e.to_string().as_bytes()).await {
                        log::error!("failed to write to socket; err = {:?}", e);
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                    continue;
                }
                log::info!("Received command: \"{}\"", command.unwrap().trim());
                // We can use unwrap here because the error is handled above
                let response = run_command(&mut engine, command.unwrap()); // Pass a reference to the engine

                match response {
                    Ok(response) => {
                        if let Err(e) = socket.write_all(response.as_bytes()).await {
                            log::error!("failed to write to socket; err = {:?}", e);
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        if let Err(e) = socket
                            .write_all(format!("{}\n", e.to_string()).as_bytes())
                            .await
                        {
                            log::error!("failed to write to socket; err = {:?}", e);
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                }
            }
        });
    }
}

fn run_command(engine: &mut EngineEnum, command: &str) -> anyhow::Result<String> {
    let command = Command::from_str(command)?;
    match command {
        Command::Get { key } => match engine.get(&key) {
            Ok(value) => match value {
                Some(v) => Ok(format!("{}\n", v)),
                None => Ok("(nil)\n".to_string()),
            },
            Err(e) => Ok(format!("error: {}\n", e)),
        },
        Command::Set { key, value } => match engine.set(&key, &value) {
            Ok(_) => Ok("ok\n".to_string()),
            Err(e) => Ok(format!("error: {}", e)),
        },
        Command::Del { key } => match engine.delete(&key) {
            Ok(_) => Ok("ok\n".to_string()),
            Err(e) => Ok(format!("error: {}", e)),
        },
        Command::List => {
            let mut result = String::new();
            match engine.list() {
                Ok(keys) => {
                    for key in keys {
                        result.push_str(&format!("- {}\n", key));
                    }
                    result.push_str("\n");
                }
                Err(e) => {
                    result.push_str(&format!("error: {}\n", e));
                }
            }
            Ok(result)
        }
        Command::Help => Ok("Commands:\n\
            get <key> - Get the value for the specified key\n\
            set <key> <value> - Sets the value for the specified key\n\
            del <key> - Deletes the specified key\n\
            list - Lists all keys in the database\n\
            help - Prints the help message\n"
            .to_string()),
    }
}
