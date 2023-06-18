use std::fs;
use std::io;
use std::io::{ErrorKind, Write};
use json::JsonValue;
use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct Token {
    pub token_type: i8,
    pub identifier: String,
    password: Option<String>
}

impl Token {
    pub fn from_json(json_v: &JsonValue) -> Result<Token, String> {
        if !json_v.has_key("token_type") || !json_v.has_key("identifier") || !json_v.has_key("password") {
            return Err("invalid json for token: without 'token_type' or 'identifier' or 'password'".to_string());
        }
        if None == json_v["token_type"].as_i8() || None == json_v["identifier"].as_str()
            || (json_v["password"] != JsonValue::Null && json_v["password"].as_str() == None) {
            return Err("invalid json for token: invaid type of some keys".to_string());
        }
        let password;
        if json_v["password"] == JsonValue::Null {
            password = None;
        } else {
            password = Some(json_v["password"].as_str().unwrap().to_string());
        }
        Ok(Token {
            token_type: json_v["token_type"].as_i8().unwrap(),
            identifier: json_v["identifier"].as_str().unwrap().to_string(),
            password: password
        })
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_json() {
        let token = Token {
            token_type: 0,
            identifier: "hello".to_string(),
            password: None
        };
        let json_str = serde_json5::to_string(&token).unwrap();
        println!("{}", json_str)
    }
}