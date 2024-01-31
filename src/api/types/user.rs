use serde::Deserialize;

/// https://core.telegram.org/bots/api#user
/// This object represents a Telegram user or bot.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Deserialize)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
    is_premium: Option<bool>,
    added_to_attachment_menu: Option<bool>,
    can_join_groups: Option<bool>,
    can_read_all_group_messages: Option<bool>,
    supports_inline_queries: Option<bool>,
}
