use super::{Chat, UniqueGiftBackdrop, UniqueGiftModel, UniqueGiftSymbol};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object describes a unique gift that was upgraded from a regular gift.
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegift>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UniqueGiftInfo {
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
    /// Information about the chat that published the gift
    pub publisher_chat: Option<Chat>,
}
