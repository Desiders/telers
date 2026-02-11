use crate::types::{Chat, UniqueGiftBackdrop, UniqueGiftModel, UniqueGiftSymbol};

use serde::{Deserialize, Serialize};

/// This object describes a unique gift that was upgraded from a regular gift
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegift>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UniqueGift {
    /// Identifier of the regular gift from which the gift was upgraded
    pub gift_id: Box<str>,
    /// Human-readable name of the regular gift from which this unique gift was upgraded
    pub base_name: Box<str>,
    /// Unique name of the gift. This name can be used in `https://t.me/nft/...` links and story areas
    pub name: Box<str>,
    /// Unique number of the upgraded gift among gifts upgraded from the same regular gift
    pub number: i64,
    /// Model of the gift
    pub model: UniqueGiftModel,
    /// Symbol of the gift
    pub symbol: UniqueGiftSymbol,
    /// Backdrop of the gift
    pub backdrop: UniqueGiftBackdrop,
    /// `true`, if the original regular gift was exclusively purchaseable by Telegram Premium subscribers
    pub is_premium: Option<bool>,
    /// `true`, if the gift was used to craft another gift and isn't available anymore
    pub is_burned: Option<bool>,
    /// `true`, if the gift is assigned from the TON blockchain and can't be resold or transferred in Telegram
    pub is_from_blockchain: Option<bool>,
    /// Information about the chat that published the gift
    pub publisher_chat: Option<Chat>,
}
