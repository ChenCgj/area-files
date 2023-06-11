use std::fs;
use std::io;
use std::io::{ErrorKind, Write};
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub token_type: i8,
    pub identifier: String,
    password: Option<String>
}

pub fn load_token(path: &str) -> Result<Token, String> {
    match fs::read_to_string(path) {
        Ok(jsonstr) => {
            match serde_json5::from_str::<Token>(&jsonstr) {
                Ok(token) => Ok(token),
                Err(e) => Err(format!("parse json file fail: {}", e))
            }
        }
        Err(e) => Err(format!("read json file error: {}", e))
    }
}

pub fn is_token_existed(path: &str, identifier: &str) -> io::Result<bool> {
    let full_path = format!("{}/{}.json", path, identifier);
    match fs::File::open(full_path) {
        Ok(_) => Ok(false),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(true),
        Err(e) => Err(e)
    }
}

pub fn save_token(path: &str, token: &Token) -> Result<(), String> {
    match serde_json5::to_string(token) {
        Ok(jsonstr) => {
            match fs::File::create(format!("{}/{}.json", path, token.identifier)) {
                Ok(mut file) => {
                    match file.write_all(jsonstr.as_bytes()) {
                        Ok(_) => Ok(()),
                        Err(e) => Err(e.to_string())
                    }
                }
                Err(e) => Err(e.to_string())
            }
        }
        Err(e) => Err(e.to_string())
    }
}