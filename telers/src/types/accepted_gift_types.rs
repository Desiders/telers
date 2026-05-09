use serde::{Deserialize, Serialize};
/// This object describes the types of gifts that can be gifted to a user or a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#acceptedgifttypes>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AcceptedGiftTypes {
    /// `true`, if unlimited regular gifts are accepted
    pub unlimited_gifts: bool,
    /// `true`, if limited regular gifts are accepted
    pub limited_gifts: bool,
    /// `true`, if unique gifts or gifts that can be upgraded to unique for free are accepted
    pub unique_gifts: bool,
    /// `true`, if a Telegram Premium subscription is accepted
    pub premium_subscription: bool,
    /// `true`, if transfers of unique gifts from channels are accepted
    pub gifts_from_channels: bool,
}
impl AcceptedGiftTypes {
    /// Creates a new `AcceptedGiftTypes`.
    ///
    /// # Arguments
    /// * `unlimited_gifts` - `true`, if unlimited regular gifts are accepted
    /// * `limited_gifts` - `true`, if limited regular gifts are accepted
    /// * `unique_gifts` - `true`, if unique gifts or gifts that can be upgraded to unique for free are accepted
    /// * `premium_subscription` - `true`, if a Telegram Premium subscription is accepted
    /// * `gifts_from_channels` - `true`, if transfers of unique gifts from channels are accepted
    #[must_use]
    pub fn new<T0: Into<bool>, T1: Into<bool>, T2: Into<bool>, T3: Into<bool>, T4: Into<bool>>(
        unlimited_gifts: T0,
        limited_gifts: T1,
        unique_gifts: T2,
        premium_subscription: T3,
        gifts_from_channels: T4,
    ) -> Self {
        Self {
            unlimited_gifts: unlimited_gifts.into(),
            limited_gifts: limited_gifts.into(),
            unique_gifts: unique_gifts.into(),
            premium_subscription: premium_subscription.into(),
            gifts_from_channels: gifts_from_channels.into(),
        }
    }

    /// `true`, if unlimited regular gifts are accepted
    #[must_use]
    pub fn unlimited_gifts<T: Into<bool>>(mut self, val: T) -> Self {
        self.unlimited_gifts = val.into();
        self
    }

    /// `true`, if limited regular gifts are accepted
    #[must_use]
    pub fn limited_gifts<T: Into<bool>>(mut self, val: T) -> Self {
        self.limited_gifts = val.into();
        self
    }

    /// `true`, if unique gifts or gifts that can be upgraded to unique for free are accepted
    #[must_use]
    pub fn unique_gifts<T: Into<bool>>(mut self, val: T) -> Self {
        self.unique_gifts = val.into();
        self
    }

    /// `true`, if a Telegram Premium subscription is accepted
    #[must_use]
    pub fn premium_subscription<T: Into<bool>>(mut self, val: T) -> Self {
        self.premium_subscription = val.into();
        self
    }

    /// `true`, if transfers of unique gifts from channels are accepted
    #[must_use]
    pub fn gifts_from_channels<T: Into<bool>>(mut self, val: T) -> Self {
        self.gifts_from_channels = val.into();
        self
    }
}
