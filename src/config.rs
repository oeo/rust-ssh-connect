use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub ssh: SshConfig,
}

#[derive(Deserialize)]
pub struct SshConfig {
    pub certs: Vec<String>,
    pub ports: Vec<u16>,
    pub users: Vec<String>,
}

pub struct ConnectionInfo {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub private_key: String,
}

pub fn load_config() -> Config {
    let config_path = get_config_path();
    let config_content = fs::read_to_string(&config_path)
        .expect(&format!("Failed to read config file at {:?}", config_path));
    toml::from_str(&config_content).expect("Failed to parse config file")
}

fn get_config_path() -> PathBuf {
    let home = dirs::home_dir().expect("Unable to determine home directory");
    let config_dir = home.join(".config");
    let config_path = config_dir.join("connect_config.toml");

    if config_path.exists() {
        config_path
    } else {
        PathBuf::from("connect_config.toml")
    }
}

