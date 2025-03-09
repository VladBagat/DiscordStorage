use directories::ProjectDirs;
use std::path::PathBuf;
use std::fs::{File, create_dir_all};
use std::io::{Error, Read, Write};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub token: String,
    pub category: u64,
    pub cache_channel: u64,
    pub storage_channel: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            token: String::new(),
            category: 0,
            cache_channel: 0,
            storage_channel: 0
        }
    }
}

pub fn write_config(config: &Config) -> Result<(), Error>{
    let config_path = get_config_path();
    create_dir_all(&config_path.parent().expect("Failed to get parent"))?;
    let mut file = File::create(config_path)?; 

    let toml_string = toml::to_string(config).expect("Failed to serialize");
    
    let buffer = toml_string.as_bytes();
    file.write(&buffer)?;
    Ok(())
}

pub fn read_config() -> Result<Config, Error> {
    let config_path = get_config_path();
    let mut file = File::open(config_path)?;
    let mut buffer = vec![0u8; 1024 * 10];

    let bytes_read = file.read(&mut buffer[..])?;
    buffer.truncate(bytes_read);
    let config: Config = toml::from_str(&String::from_utf8(buffer).expect("Failed to convert to string")).expect("Failed to deserialize");
    Ok(config)
}

fn get_config_path() -> PathBuf {
    let project_dirs = ProjectDirs::from("com", "DiscordStore", "DiscordStore").unwrap();
    let config_dir = project_dirs.config_dir();
    let config_file = config_dir.join("config.toml");
    config_file
}