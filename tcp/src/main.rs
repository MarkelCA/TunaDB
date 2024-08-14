use anyhow::anyhow;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use core::config;
use core::storage::{self, Engine, EngineEnum};
use std::error::Error;
use std::str::SplitWhitespace;

#[path = "./storage.rs"]
mod tcp_storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                let command = std::str::from_utf8(&buf[0..n]);

                if let Err(e) = command {
                    if let Err(e) = socket.write_all(e.to_string().as_bytes()).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                    continue;
                }
                // We can use unwrap here because the error is handled above
                let tokens = command.unwrap().split_whitespace();
                let response = run_command(&mut engine, tokens); // Pass a reference to the engine

                match response {
                    Ok(response) => {
                        if let Err(e) = socket.write_all(response.as_bytes()).await {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                    Err(e) => {
                        if let Err(e) = socket
                            .write_all(format!("{}\n", e.to_string()).as_bytes())
                            .await
                        {
                            eprintln!("failed to write to socket; err = {:?}", e);
                            return;
                        }
                    }
                }
            }
        });
    }
}

fn run_command(engine: &mut EngineEnum, mut tokens: SplitWhitespace) -> anyhow::Result<String> {
    let command = tokens.nth(0).ok_or(anyhow!("Command not found"))?;
    match command {
        "get" => match engine.get(
            tokens
                .nth(0)
                .ok_or(anyhow!("Key not found in get command"))?,
        ) {
            Ok(value) => match value {
                Some(v) => Ok(format!("{}\n", v)),
                None => Ok("(nil)\n".to_string()),
            },
            Err(e) => Ok(format!("error: {}\n", e)),
        },
        "set" => match engine.set(
            tokens
                .nth(0)
                .ok_or(anyhow!("Key not found in set command"))?,
            tokens
                .nth(0)
                .ok_or(anyhow!("Value not found in set command"))?,
        ) {
            Ok(_) => Ok("ok\n".to_string()),
            Err(e) => Ok(format!("error: {}", e)),
        },
        "list" => {
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
        "help" => Ok("Commands:\nget <key>\nset <key> <value>\nlist\n".to_string()),
        _ => Ok("unknown command\n".to_string()),
    }
}
