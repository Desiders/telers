use serde::{Deserialize, Serialize};
/// This object describes the backdrop of a unique gift.
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegiftbackdrop>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniqueGiftBackdrop {
    /// Name of the backdrop
    pub name: Box<str>,
    /// Colors of the backdrop
    pub colors: crate::types::UniqueGiftBackdropColors,
    /// The number of unique gifts that receive this backdrop for every 1000 gifts upgraded
    pub rarity_per_mille: i64,
}
impl UniqueGiftBackdrop {
    /// Creates a new `UniqueGiftBackdrop`.
    ///
    /// # Arguments
    /// * `name` - Name of the backdrop
    /// * `colors` - Colors of the backdrop
    /// * `rarity_per_mille` - The number of unique gifts that receive this backdrop for every 1000 gifts upgraded
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::UniqueGiftBackdropColors>,
        T2: Into<i64>,
    >(
        name: T0,
        colors: T1,
        rarity_per_mille: T2,
    ) -> Self {
        Self {
            name: name.into(),
            colors: colors.into(),
            rarity_per_mille: rarity_per_mille.into(),
        }
    }

    /// Name of the backdrop
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = val.into();
        this
    }

    /// Colors of the backdrop
    #[must_use]
    pub fn colors<T: Into<crate::types::UniqueGiftBackdropColors>>(self, val: T) -> Self {
        let mut this = self;
        this.colors = val.into();
        this
    }

    /// The number of unique gifts that receive this backdrop for every 1000 gifts upgraded
    #[must_use]
    pub fn rarity_per_mille<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.rarity_per_mille = val.into();
        this
    }
}
