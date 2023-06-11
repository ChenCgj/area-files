use serde_derive::Deserialize;
use serde_derive::Serialize;

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

