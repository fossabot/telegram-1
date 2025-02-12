use crate::api::types::user::User;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageoriginuser
/// The message was originally sent by a known user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginUser {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub date: i64,
    pub sender_user: User,
}
