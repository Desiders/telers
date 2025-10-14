use super::{ChatBoostSourceGiftCode, ChatBoostSourceGiveaway, ChatBoostSourcePremium};

use serde::{Deserialize, Serialize};

/// This object describes the source of a chat boost. It can be one of
/// - [`ChatBoostSourcePremium`]
/// - [`ChatBoostSourceGiftCode`]
/// - [`ChatBoostSourceGiveaway`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsource>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}

impl From<ChatBoostSourcePremium> for ChatBoostSource {
    fn from(source: ChatBoostSourcePremium) -> Self {
        Self::Premium(source)
    }
}

impl From<ChatBoostSourceGiftCode> for ChatBoostSource {
    fn from(source: ChatBoostSourceGiftCode) -> Self {
        Self::GiftCode(source)
    }
}

impl From<ChatBoostSourceGiveaway> for ChatBoostSource {
    fn from(source: ChatBoostSourceGiveaway) -> Self {
        Self::Giveaway(source)
    }
}
