use std::{fs};
use std::net::Ipv4Addr;
use serde_derive::Deserialize;
use area_files_lib::util::get_local_ip;

#[derive(Deserialize, Debug)]
pub struct Config {
    download_path: String,
    shared_path: String,
    token_path: String,

    server_ip: String,
    server_port: u16,

    host_ip: String,
    host_port: u16,

    broadcast_ip: String,
    // broadcast_port: u16,

    client_listen_ip: String,
    client_listen_port: u16,

    localhost_name: String
}

impl Config {
    fn check(&mut self) -> Result<(), String> {
        if let Ok(hostip) = self.host_ip.parse::<Ipv4Addr>() {
            if hostip == Ipv4Addr::new(0, 0, 0, 0) {
                let ips = get_local_ip();
                if ips.is_empty() {
                    return Err("no available ip".to_string())
                }
                self.host_ip = ips[0].to_string();
            }
        } else {
            return Err("incorrect host ip".to_string())
        }
        if let Err(_) = self.server_ip.parse::<Ipv4Addr>() {
            return Err("invalid server ip".to_string())
        }
        if let Err(_) = self.client_listen_ip.parse::<Ipv4Addr>() {
            return Err("invalid client listen ip".to_string())
        }
        if let Err(_) = self.broadcast_ip.parse::<Ipv4Addr>() {
            return Err("invalid broadcast ip".to_string())
        }
        Ok(())
    }
    pub fn load_config(path: &str) -> Result<Config, String> {
        if let Ok(json_str) = fs::read_to_string(path) {
            match serde_json5::from_str::<Config>(&json_str) {
                Ok(mut config) => {
                    // check the config
                    match config.check() {
                        Ok(_) => Ok(config),
                        Err(e) => Err(e)
                    }
                },
                Err(e) => Err(format!("couldn't parse configure from {}: {}", path, e))
            }
        } else {
            Err(format!("couldn't read configure file: {}", path))
        }
    }

    pub fn get_server_ip(&self) -> &str {
        &self.server_ip
    }

    pub fn get_server_port(&self) -> u16 {
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

    pub fn get_host_port(&self) -> u16 {
        self.host_port
    }

    // pub fn get_broadcast_port(&self) -> i16 {
    //     self.broadcast_port
    // }

    pub fn get_broadcast_ip(&self) -> &str {
        &self.broadcast_ip
    }

    pub fn get_client_listen_ip(&self) -> &str {
        &self.client_listen_ip
    }

    pub fn get_client_listen_port(&self) -> u16 {
        self.client_listen_port
    }

    pub fn get_localhost_name(&self) -> &str {
        &self.localhost_name
    }
}
