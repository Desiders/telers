use super::OwnedGift;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Contains the list of gifts received and owned by a user or a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgifts>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct OwnedGifts {
    /// The total number of gifts owned by the user or the chat
    pub total_count: i64,
    /// The list of gifts
    pub gifts: Box<[OwnedGift]>,
    /// Offset for the next request. If empty, then there are no more results
    pub next_offset: Option<Box<str>>,
}
