use tcp_storage::SendableEngine;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use core::config;
use core::storage::{self, Engine};

#[path = "./storage.rs"]
mod tcp_storage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let config = config::parse()?;
    let engine = storage::new_engine(&config.file_path);

    loop {
        let (mut socket, _) = listener.accept().await?;

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
                // let command = run_command(engine, &command);
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}

// fn run_command(mut engine: Box<dyn SendableEngine>, command: &str) -> String {
//     match command {
//         "get" => engine.get("key")?,
//         "set" => {
//             engine.set("key", "value");
//             "ok".to_string()
//         }
//         "list" => {
//             let mut result = String::new();
//             for key in engine.list() {
//                 result.push_str(&format!("{}\n", key));
//             }
//             result
//         }
//         _ => "unknown command".to_string(),
//     }
// }
