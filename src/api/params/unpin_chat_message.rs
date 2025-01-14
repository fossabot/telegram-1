use crate::api::enums::chat_uid::ChatUId;
use serde::Serialize;

/// https://core.telegram.org/bots/api#unpinchatmessage
/// Use this method to remove a message from the list of pinned messages in a chat. If the chat is not a private chat, the bot must be an administrator in the chat for this to work and must have the 'can_pin_messages' administrator right in a supergroup or 'can_edit_messages' administrator right in a channel. Returns True on success.
#[derive(Debug, Serialize)]
pub struct UnpinChatMessage {
    pub chat_id: ChatUId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
}
