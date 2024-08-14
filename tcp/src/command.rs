use anyhow::anyhow;
use std::str::FromStr;

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
    /// Prints the help message
    Help,
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
            "help" => Ok(Command::Help),
            _ => Err(anyhow!("Unknown command")),
        }
    }
}
