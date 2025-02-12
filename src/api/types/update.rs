use crate::api::types::callback_query::CallbackQuery;
use crate::api::types::chat_boost_removed::ChatBoostRemoved;
use crate::api::types::chat_boost_updated::ChatBoostUpdated;
use crate::api::types::chat_join_request::ChatJoinRequest;
use crate::api::types::chat_member_updated::ChatMemberUpdated;
use crate::api::types::chosen_inline_result::ChosenInlineResult;
use crate::api::types::inline_query::InlineQuery;
use crate::api::types::message::Message;
use crate::api::types::message_reaction_count_update::MessageReactionCountUpdated;
use crate::api::types::message_reaction_updated::MessageReactionUpdated;
use crate::api::types::poll_answer::PollAnswer;
use crate::api::types::pre_checkout_query::PreCheckoutQuery;
use crate::api::types::shipping_query::ShippingQuery;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#update
/// This object represents an incoming update.
/// At most one of the optional parameters can be present in any given update.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Update {
    pub update_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<MessageReactionUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<MessageReactionCountUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<PreCheckoutQuery>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<ChatBoostUpdated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<ChatBoostRemoved>,
}
