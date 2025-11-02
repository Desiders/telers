use super::{OwnedGiftRegular, OwnedGiftUnique};

use serde::{Deserialize, Serialize};

/// This object describes a gift received and owned by a user or a chat. Currently, it can be one of
/// - [`OwnedGiftRegular`]
/// - [`OwnedGiftUnique`]
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgift>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OwnedGift {
    Regular(OwnedGiftRegular),
    Unique(OwnedGiftUnique),
}

impl From<OwnedGiftRegular> for OwnedGift {
    fn from(gift: OwnedGiftRegular) -> Self {
        Self::Regular(gift)
    }
}

impl From<OwnedGiftUnique> for OwnedGift {
    fn from(gift: OwnedGiftUnique) -> Self {
        Self::Unique(gift)
    }
}
