use std::{fs};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    download_path: String,
    shared_path: String,
    token_path: String,

    server_ip: String,
    server_port: i16,

    host_ip: String,
    host_port: i16,

    broadcast_ip: String,
    // broadcast_port: i16,

    client_listen_ip: String,
    client_listen_port: i16,

    localhost_name: String
}

impl Config {
    pub fn load_config(path: &str) -> Result<Config, String> {
        if let Ok(json_str) = fs::read_to_string(path) {
            match serde_json5::from_str(&json_str) {
                Ok(config) => Ok(config),
                Err(e) => Err(format!("couldn't parse configure from {}: {}", path, e))
            }
        } else {
            Err(format!("couldn't read configure file: {}", path))
        }
    }

    pub fn get_server_ip(&self) -> &str {
        &self.server_ip
    }

    pub fn get_server_port(&self) -> i16 {
        self.server_port
    }

    pub fn get_token_path(&self) -> &str {
        &self.token_path
    }

    pub fn get_download_path(&self) -> &str {
        &self.download_path
    }

    pub fn get_shared_path(&self) -> &str {
        &self.shared_path
    }

    pub fn get_host_ip(&self) -> &str {
        &self.host_ip
    }

    pub fn get_host_port(&self) -> i16 {
        self.host_port
    }

    // pub fn get_broadcast_port(&self) -> i16 {
    //     self.broadcast_port
    // }

    pub fn get_client_listen_ip(&self) -> &str {
        &self.client_listen_ip
    }

    pub fn get_client_listen_port(&self) -> i16 {
        self.client_listen_port
    }

    pub fn get_localhost_name(&self) -> &str {
        &self.localhost_name
    }
}
