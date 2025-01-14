use crate::api::types::chat_boost_source_gift_code::ChatBoostSourceGiftCode;
use crate::api::types::chat_boost_source_giveaway::ChatBoostSourceGiveaway;
use crate::api::types::chat_boost_source_premium::ChatBoostSourcePremium;
use serde::{Deserialize, Serialize};

/// https://core.telegram.org/bots/api#chatboostsource
/// This object describes the source of a chat boost. It can be one of
/// ChatBoostSourcePremium
/// ChatBoostSourceGiftCode
/// ChatBoostSourceGiveaway
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ChatBoostSource {
    ChatBoostSourcePremium(ChatBoostSourcePremium),
    ChatBoostSourceGiftCode(ChatBoostSourceGiftCode),
    ChatBoostSourceGiveaway(ChatBoostSourceGiveaway),
}
