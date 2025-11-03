use crate::types::Sticker;

use serde::{Deserialize, Serialize};

/// This object describes the model of a unique gift
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegiftmodel>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct UniqueGiftModel {
    /// Name of the model
    pub name: String,
    /// The sticker that represents the unique gift
    pub sticker: Box<Sticker>,
    /// The number of unique gifts that receive this model for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}
