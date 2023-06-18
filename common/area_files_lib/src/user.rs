use json::JsonValue;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::User::{UserLAN, UserWAN};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum User {
    UserLAN {
        host_name: String,
        ip: String
    },
    UserWAN {
        uuid: String,
        name: String
    }
}

impl User {
    pub fn from_json(json_v: &JsonValue) -> Result<User, String> {
        if !json_v.has_key("UserLAN") && !json_v.has_key("UserWAN") {
            return Err("invalid json for User: without key UserLAN or UserWAN".to_string());
        }
        let user;
        if json_v.has_key("UserLAN") {
            let inner = &json_v["UserLAN"];
            if !inner.has_key("host_name") || !inner.has_key("ip") {
                return Err("invalid json for User: without key 'host_name' or 'ip'".to_string());
            }
            if None == inner["host_name"].as_str() || None == inner["ip"].as_str() {
                return Err("invalid json for User: host_name or ip is not a string".to_string());
            }
            user = UserLAN {
                host_name: inner["host_name"].as_str().unwrap().to_string(),
                ip: inner["ip"].as_str().unwrap().to_string()
            }
        } else {
            let inner = &json_v["UserWAN"];
            if !inner.has_key("uuid") || !inner.has_key("name") {
                return Err("invalid json for User: without key 'uuid' or 'name'".to_string());
            }
            if None == inner["uuid"].as_str() || None == inner["name"].as_str() {
                return Err("invalid json for User: uuid or name is not a string".to_string());
            }
            user = UserWAN {
                uuid: inner["uuid"].as_str().unwrap().to_string(),
                name: inner["name"].as_str().unwrap().to_string()
            }
        }
        Ok(user)
    }
}

