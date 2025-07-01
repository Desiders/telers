use crate::types::UniqueGiftBackdropColors;

use serde::{Deserialize, Serialize};

/// This object describes the backdrop of a unique gift
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegiftbackdrop>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct UniqueGiftBackdrop {
    /// Name of the backdrop
    pub name: Box<str>,
    /// Colors of the backdrop
    pub colors: UniqueGiftBackdropColors,
    /// The number of unique gifts that receive this backdrop for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}
