use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json5;
use crate::file_mgr::FileInfo;
use crate::user::User;

#[derive(Deserialize, Serialize)]
pub struct MsgQueryArea {
    pub msg_type: String,
    pub stamp: i32
}

impl MsgQueryArea {
    pub fn new(stamp: i32) -> MsgQueryArea {
        MsgQueryArea {
            msg_type: "query_area".to_string(),
            stamp
        }
    }
    pub fn generate_json(stamp: i32) -> Result<String, serde_json5::Error> {
        let ask = MsgQueryArea::new(stamp);
        serde_json5::to_string(&ask)
    }
}

#[derive(Deserialize, Serialize)]
pub struct MsgReplyArea {
    pub msg_type: String,
    pub stamp: i32,
    pub rest_msg: i32,
    pub user: User,
    pub info: Vec<FileInfo>
}

impl MsgReplyArea {
    pub fn new(stamp: i32, rest_msg: i32, info: &Vec<FileInfo>, user: &User) -> MsgReplyArea {
        MsgReplyArea {
            msg_type: "reply_area".to_string(),
            stamp,
            rest_msg: rest_msg,
            user: user.clone(),
            info: info.to_vec()
        }
    }

    pub fn generate_json(stamp: i32, rest_msg: i32, container: &Vec<FileInfo>, user: &User) -> Result<String, serde_json5::Error> {
        let reply = MsgReplyArea::new(stamp, rest_msg, container, user);
        serde_json5::to_string(&reply)
    }
}

#[cfg(test)]
mod test {
    use crate::User::UserLAN;
    use super::*;
    use super::super::file_mgr;
    #[test]
    fn test_generate_reply_get_info() {
        let container = file_mgr::get_all_info("../../client-core/area_files_client_core/shared_files").unwrap();
        let reply = MsgReplyArea::generate_json(0, 0, &container, &UserLAN { host_name: "localhost".to_string(), ip: "127.0.0.1".to_string() });
        println!("{}", reply.unwrap());
    }
}
