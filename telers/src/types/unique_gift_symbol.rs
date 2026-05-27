use serde::{Deserialize, Serialize};
/// This object describes the symbol shown on the pattern of a unique gift.
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegiftsymbol>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniqueGiftSymbol {
    /// Name of the symbol
    pub name: Box<str>,
    /// The sticker that represents the unique gift
    pub sticker: Box<crate::types::Sticker>,
    /// The number of unique gifts that receive this model for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}
impl UniqueGiftSymbol {
    /// Creates a new `UniqueGiftSymbol`.
    ///
    /// # Arguments
    /// * `name` - Name of the symbol
    /// * `sticker` - The sticker that represents the unique gift
    /// * `rarity_per_mille` - The number of unique gifts that receive this model for every 1000 gifts upgraded
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::Sticker>, T2: Into<i64>>(
        name: T0,
        sticker: T1,
        rarity_per_mille: T2,
    ) -> Self {
        Self {
            name: name.into(),
            sticker: Box::new(sticker.into()),
            rarity_per_mille: rarity_per_mille.into(),
        }
    }

    /// Name of the symbol
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }

    /// The sticker that represents the unique gift
    #[must_use]
    pub fn sticker<T: Into<crate::types::Sticker>>(mut self, val: T) -> Self {
        self.sticker = Box::new(val.into());
        self
    }

    /// The number of unique gifts that receive this model for every 1000 gifts upgraded
    #[must_use]
    pub fn rarity_per_mille<T: Into<i64>>(mut self, val: T) -> Self {
        self.rarity_per_mille = val.into();
        self
    }
}
