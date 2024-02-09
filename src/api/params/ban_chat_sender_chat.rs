use serde::Serialize;

/// https://core.telegram.org/bots/api#banchatsenderchat
/// Use this method to ban a channel chat in a supergroup or a channel. Until the chat is unbanned, the owner of the banned chat won't be able to send messages on behalf of any of their channels. The bot must be an administrator in the supergroup or channel for this to work and must have the appropriate administrator rights. Returns True on success.
#[derive(Debug, Serialize)]
pub struct BanChatSenderChat {
    chat_id: i64,
    sender_chat_id: i64,
}
