use serde::{Deserialize, Serialize};
/// This object describes the model of a unique gift.
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegiftmodel>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniqueGiftModel {
    /// Name of the model
    pub name: Box<str>,
    /// The sticker that represents the unique gift
    pub sticker: Box<crate::types::Sticker>,
    /// The number of unique gifts that receive this model for every 1000 gift upgrades. Always 0 for crafted gifts.
    pub rarity_per_mille: i64,
    /// Rarity of the model if it is a crafted model. Currently, can be `uncommon`, `rare`, `epic`, or `legendary`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<Box<str>>,
}
impl UniqueGiftModel {
    /// Creates a new `UniqueGiftModel`.
    ///
    /// # Arguments
    /// * `name` - Name of the model
    /// * `sticker` - The sticker that represents the unique gift
    /// * `rarity_per_mille` - The number of unique gifts that receive this model for every 1000 gift upgrades. Always 0 for crafted gifts.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
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
            rarity: None,
        }
    }

    /// Name of the model
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

    /// The number of unique gifts that receive this model for every 1000 gift upgrades. Always 0 for crafted gifts.
    #[must_use]
    pub fn rarity_per_mille<T: Into<i64>>(mut self, val: T) -> Self {
        self.rarity_per_mille = val.into();
        self
    }

    /// Rarity of the model if it is a crafted model. Currently, can be `uncommon`, `rare`, `epic`, or `legendary`.
    #[must_use]
    pub fn rarity<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.rarity = Some(val.into());
        self
    }

    /// Rarity of the model if it is a crafted model. Currently, can be `uncommon`, `rare`, `epic`, or `legendary`.
    #[must_use]
    pub fn rarity_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.rarity = val.map(Into::into);
        self
    }
}
