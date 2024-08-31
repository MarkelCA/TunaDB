use crate::storage::Engine;
use anyhow::anyhow;
use std::{str::FromStr, sync::Arc};
use tokio::sync::Mutex;

#[derive(Debug)]
pub enum Command {
    /// Get the value for the specified key
    Get { key: String },
    /// Sets the value for the specified key
    Set { key: String, value: String },
    /// Deletes the specified key
    Del { key: String },
    /// Lists all keys in the database
    List,
}

impl FromStr for Command {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let command = tokens.next().ok_or(anyhow!("Command not found"))?;
        match command {
            "get" => Ok(Command::Get {
                key: tokens
                    .next()
                    .ok_or(anyhow!("Key not found in get command"))?
                    .to_string(),
            }),
            "set" => Ok(Command::Set {
                key: tokens
                    .next()
                    .ok_or(anyhow!("Key not found in set command"))?
                    .to_string(),
                value: tokens
                    .next()
                    .ok_or(anyhow!("Value not found in set command"))?
                    .to_string(),
            }),
            "del" => Ok(Command::Del {
                key: tokens
                    .next()
                    .ok_or(anyhow!("Key not found in del command"))?
                    .to_string(),
            }),
            "list" => Ok(Command::List),
            _ => Err(anyhow!("Unknown command")),
        }
    }
}

pub async fn run(engine: Arc<Mutex<Box<dyn Engine>>>, command: Command) -> anyhow::Result<String> {
    match command {
        Command::Get { key } => match engine.lock().await.get(&key).await {
            Ok(value) => match value {
                Some(v) => Ok(format!("{}\n", v)),
                None => Ok("(nil)\n".to_string()),
            },
            Err(e) => Ok(format!("error: {}\n", e)),
        },
        Command::Set { key, value } => match engine.lock().await.set(&key, &value).await {
            Ok(_) => Ok("ok\n".to_string()),
            Err(e) => Ok(format!("error: {}", e)),
        },
        Command::Del { key } => match engine.lock().await.delete(&key).await {
            Ok(_) => Ok("ok\n".to_string()),
            Err(e) => Ok(format!("error: {}", e)),
        },
        Command::List => {
            let mut result = String::new();
            match engine.lock().await.list().await {
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
    }
}
