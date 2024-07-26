use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub file_path: String,
}

pub fn parse() -> Config {
    let config_dir = home::home_dir().unwrap().join(".config").join("sallydb");
    let db_dir = home::home_dir()
        .unwrap()
        .join(".local")
        .join("share")
        .join("sallydb");

    let db_file = db_dir.join("sally.db");

    let config_file: PathBuf;

    if !config_dir.exists() {
        let _ = fs::create_dir_all(&config_dir);
    }

    config_file = config_dir.join("config.toml");

    if !config_file.exists() {
        let default_config = Config {
            file_path: db_file.to_str().unwrap().to_string(),
        };

        let toml = toml::to_string(&default_config).unwrap();
        fs::write(config_file.to_str().unwrap(), toml).unwrap();
    }

    if !db_dir.exists() {
        let _ = fs::create_dir_all(&db_dir);
    }

    let contents = fs::read_to_string(config_file.to_str().unwrap()).unwrap();
    toml::from_str(&contents).unwrap()
}
