use super::{ChatBoostSourceGiftCode, ChatBoostSourceGiveaway, ChatBoostSourcePremium};

use serde::Deserialize;

/// This object describes the source of a chat boost. It can be one of
/// - [`ChatBoostSourcePremium`]
/// - [`ChatBoostSourceGiftCode`]
/// - [`ChatBoostSourceGiveaway`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsource>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum ChatBoostSource {
    Premium(ChatBoostSourcePremium),
    GiftCode(ChatBoostSourceGiftCode),
    Giveaway(ChatBoostSourceGiveaway),
}

impl From<ChatBoostSourcePremium> for ChatBoostSource {
    #[must_use]
    fn from(source: ChatBoostSourcePremium) -> Self {
        Self::Premium(source)
    }
}

impl From<ChatBoostSourceGiftCode> for ChatBoostSource {
    #[must_use]
    fn from(source: ChatBoostSourceGiftCode) -> Self {
        Self::GiftCode(source)
    }
}

impl From<ChatBoostSourceGiveaway> for ChatBoostSource {
    #[must_use]
    fn from(source: ChatBoostSourceGiveaway) -> Self {
        Self::Giveaway(source)
    }
}
