/// The theme based on a unique gift owned by the user or the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatsharethemeuniquegift>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct ChatShareThemeUniqueGift {
    /// The gift
    pub gift: Box<crate::types::UniqueGift>,
}
impl ChatShareThemeUniqueGift {
    /// Creates a new `ChatShareThemeUniqueGift`.
    ///
    /// # Arguments
    /// * `gift` - The gift
    #[must_use]
    pub fn new<T0: Into<crate::types::UniqueGift>>(gift: T0) -> Self {
        Self {
            gift: Box::new(gift.into()),
        }
    }

    /// The gift
    #[must_use]
    pub fn gift<T: Into<crate::types::UniqueGift>>(mut self, val: T) -> Self {
        self.gift = Box::new(val.into());
        self
    }
}
