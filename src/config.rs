use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::io::Write;
use std::env;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub ssh: SshConfig,
}

#[derive(Deserialize, Serialize)]
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
    
    if !config_path.exists() {
        create_default_config(&config_path);
        println!("A default configuration file has been created at: {:?}", config_path);
        println!("Please edit this file with your desired settings before running the tool again.");
        std::process::exit(0);
    }

    let config_content = fs::read_to_string(&config_path)
        .expect(&format!("Failed to read config file at {:?}", config_path));
    toml::from_str(&config_content).expect("Failed to parse config file")
}

fn get_config_path() -> PathBuf {
    if let Ok(path) = env::var("RUST_SSH_CONNECT_CONFIG") {
        PathBuf::from(path)
    } else {
        let home = dirs::home_dir().expect("Unable to determine home directory");
        home.join(".config").join("rust-ssh-connect.toml")
    }
}

fn create_default_config(path: &PathBuf) {
    let ssh_keys = find_ssh_keys();
    
    let default_config = Config {
        ssh: SshConfig {
            certs: ssh_keys,
            ports: vec![22],
            users: env::var("RUST_SSH_CONNECT_USERS")
                .unwrap_or_else(|_| "ubuntu,ec2-user,admin".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
        },
    };

    let toml_string = toml::to_string_pretty(&default_config).expect("Failed to serialize config");
    
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("Failed to create config directory");
    }

    let mut file = fs::File::create(path).expect("Failed to create config file");
    file.write_all(toml_string.as_bytes()).expect("Failed to write to config file");
}

pub fn find_ssh_keys() -> Vec<String> {
    let home = dirs::home_dir().expect("Unable to determine home directory");
    let ssh_dir = home.join(".ssh");
    
    if !ssh_dir.exists() {
        return vec![];
    }

    let mut keys = Vec::new();
    
    if let Ok(entries) = fs::read_dir(ssh_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() && !path.to_str().unwrap().ends_with(".pub") {
                keys.push(path.to_str().unwrap().to_string());
            }
        }
    }

    keys
}

