use crate::api::types::chat::Chat;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#messageoriginchat
/// The message was originally sent on behalf of a chat to a group chat.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MessageOriginChat {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub kind: String,
    pub date: i64,
    pub sender_chat: Chat,
    pub author_signature: Option<String>,
}
