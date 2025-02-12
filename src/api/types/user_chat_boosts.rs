use crate::api::types::chat_boost::ChatBoost;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#userchatboosts
/// This object represents a list of boosts added to a chat by a user.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct UserChatBoosts {
    pub boosts: Vec<ChatBoost>,
}
