use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub file_path: String,
}

#[derive(Debug)]
struct Thing;

pub fn parse() -> Result<Config, anyhow::Error> {
    let home_dir = home::home_dir();
    let config_dir;
    let db_dir;

    if let Some(result) = home_dir {
        config_dir = result.join(".config").join("sallydb");
        db_dir = result.join(".local").join("state").join("sallydb");
    } else {
        return Err(anyhow!("Home dir couldn't be found"));
    }

    let db_file = db_dir.join("sally.db");

    let config_file: PathBuf;

    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }

    config_file = config_dir.join("config.toml");

    if !config_file.exists() {
        let default_config = Config {
            file_path: db_file
                .to_str()
                .ok_or(anyhow!("db file path coulnd't be stringifyed"))?
                .to_string(),
        };

        let toml = toml::to_string(&default_config)?;
        fs::write(config_file.to_str().unwrap(), toml)?;
    }

    if !db_dir.exists() {
        let _ = fs::create_dir_all(&db_dir);
    }

    let contents = fs::read_to_string(config_file.to_str().unwrap()).unwrap();

    toml::from_str(&contents).map_err(|err| anyhow!(err.to_string()))
}

pub fn set_file_path(file_path: String) {
    let config_dir = home::home_dir().unwrap().join(".config").join("sallydb");
    let config_file = config_dir.join("config.toml");

    let config = Config { file_path };

    let toml = toml::to_string(&config).unwrap();
    fs::write(config_file.to_str().unwrap(), toml).unwrap();
}
