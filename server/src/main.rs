use anyhow::anyhow;
use args::Args;
use clap::Parser;
use core::serializer::{CommandSerializer, ProtoCommandSerializer, ResponseSerializer};
use env_logger::Env;
use prost::Message;
use std::process::ExitCode;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use core::command::{self};
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

// TODO: refactor
fn new_command_serializer() -> Box<dyn CommandSerializer> {
    Box::new(ProtoCommandSerializer)
}

// TODO: refactor
fn new_response_serializer() -> Box<dyn ResponseSerializer> {
    Box::new(core::serializer::ProtoResponseSerializer)
}

async fn init() -> anyhow::Result<()> {
    let args = Args::parse();
    if !tcp::local_port_available(args.port) {
        return Err(anyhow!("Port {} is already in use", args.port));
    }
    env_logger::init_from_env(Env::default().default_filter_or(args.log_level.to_string()));

    log::info!("Starting server in port {}...", args.port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await?;
    let config = config::parse()?;
    let engine = Arc::new(Mutex::new(storage::new_engine(&config.file_path)?));
    let command_serializer = Arc::new(new_command_serializer());
    let response_serializer = Arc::new(new_response_serializer());

    log::info!("Server started");
    loop {
        let command_serializer = command_serializer.clone();
        let response_serializer = response_serializer.clone();
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

                // TODO: refactor
                match command_serializer.decode(&buf[0..n]) {
                    Ok(cmd) => {
                        log::info!("Received command: {:?}", cmd);
                        let response = command::run_proto(engine.clone(), cmd).await; // Pass a reference to the engine
                        let mut buf = Vec::new();
                        buf.reserve(response_serializer.encoded_len(&response));

                        match response_serializer.encode(&response, &mut buf) {
                            Ok(_) => {
                                let _ = socket.write_all(&buf).await;
                            }
                            Err(e) => {
                                if let Err(e) = socket.write_all(e.to_string().as_bytes()).await {
                                    log::error!("failed to encode response; err = {:?}", e);
                                    eprintln!("failed to encode response; err = {:?}", e);
                                    return;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        if let Err(e) = socket.write_all(e.to_string().as_bytes()).await {
                            log::error!("failed to write to socket; err = {:?}", e);
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                        continue;
                    }
                }
            }
        });
    }
}
