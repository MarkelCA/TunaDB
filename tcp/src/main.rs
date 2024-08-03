use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use core::config;
use core::storage::{self, Engine, EngineEnum};

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
                let command = std::str::from_utf8(&buf[0..n]).unwrap();
                let command = command.split(' ').nth(0).unwrap();
                println!("Received command: {}", command);
                let response = run_command(&mut engine, &command); // Pass a reference to the engine

                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

fn run_command(engine: &mut EngineEnum, command: &str) -> String {
    match command {
        "get" => match engine.get("key") {
            Ok(value) => value.unwrap_or("(nil)\n".to_string()),
            Err(e) => format!("error: {}\n", e),
        },
        "set" => match engine.set("key", "value") {
            Ok(_) => "ok\n".to_string(),
            Err(e) => format!("error: {}", e),
        },
        "list" => {
            let mut result = String::new();
            match engine.list() {
                Ok(keys) => {
                    for key in keys {
                        result.push_str(&format!("{}\n", key));
                    }
                    result.push_str("\n");
                }
                Err(e) => {
                    result.push_str(&format!("error: {}\n", e));
                }
            }
            result
        }
        _ => "unknown command\n".to_string(),
    }
}
