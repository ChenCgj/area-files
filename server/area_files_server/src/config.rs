use std::{fs};
use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    server_ip: String,
    server_port: i16,
    file_save_path: String,
    mysql_ip: String,
    mysql_port: i16,
    redis_ip: String,
    redis_port: i16
}

impl Config {
    pub fn load_config(path: &str) -> Result<Config, String> {
        if let Ok(json_str) = fs::read_to_string(path) {
            match serde_json5::from_str(&json_str) {
                Ok(config) => Ok(config),
                Err(_) => Err(format!("couldn't parse configure from {}", path))
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

    pub fn get_file_save_path(&self) -> &str {
        &self.file_save_path
    }

    pub fn get_mysql_ip(&self) -> &str {
        &self.mysql_ip
    }

    pub fn get_mysql_port(&self) -> i16 {
        self.mysql_port
    }

    pub fn get_redis_ip(&self) -> &str {
        &self.redis_ip
    }

    pub fn get_redis_port(&self) -> i16 {
        self.redis_port
    }

}
