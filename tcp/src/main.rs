use anyhow::anyhow;
use args::Args;
use clap::Parser;
use env_logger::Env;
use prost::Message;
use std::process::ExitCode;
use std::str::FromStr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use core::command::{self, Command};
use core::config;
use core::storage;
use log;

mod args;
mod tcp;

#[tokio::main]
async fn main() -> ExitCode {
    match init().await {
        Ok(_) => ExitCode::from(0),
        Err(e) => {
            log::error!("Error: {}", e);
            eprintln!("Error: {}", e);
            ExitCode::from(1)
        }
    }
}

async fn init() -> anyhow::Result<()> {
    let args = Args::parse();
    if !tcp::local_port_available(args.port) {
        return Err(anyhow!("Port {} is already in use", args.port));
    }
    env_logger::init_from_env(Env::default().default_filter_or(args.log_level.to_string()));

    log::info!("Starting server");
    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await?;
    let config = config::parse()?;
    let engine = Arc::new(Mutex::new(storage::new_engine(&config.file_path)?));

    loop {
        let (mut socket, _) = listener.accept().await?;
        let engine = engine.clone(); // Clone the engine pointer for each connection

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

                println!("Received a command!");
                // Write the data back
                let command = std::str::from_utf8(&buf[0..n]);
                println!("Received command: {:?}", command);
                let proto_command = core::proto::Command::decode(&buf[0..n]);

                if let Err(e) = proto_command {
                    if let Err(e) = socket.write_all(e.to_string().as_bytes()).await {
                        log::error!("failed to write to socket; err = {:?}", e);
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                    continue;
                }

                // log::info!("Received command: \"{}\"", command.unwrap().trim());
                // // We can use unwrap here because the error is handled above
                // let command = match Command::from_str(command.unwrap()) {
                //     Ok(cmd) => cmd,
                //     Err(error) => {
                //         log::error!("{}", error);
                //         continue;
                //     }
                // };
                // // let response = command::run(engine.clone(), command).await; // Pass a reference to the engine
                let response = command::run_proto(engine.clone(), proto_command.unwrap()).await; // Pass a reference to the engine

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
