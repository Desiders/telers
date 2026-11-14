/// A regular emoji.
/// # Documentation
/// <https://core.telegram.org/bots/api#emojitypeemoji>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct EmojiTypeEmoji {
    /// The emoji
    pub emoji: Box<str>,
}
impl EmojiTypeEmoji {
    /// Creates a new `EmojiTypeEmoji`.
    ///
    /// # Arguments
    /// * `emoji` - The emoji
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(emoji: T0) -> Self {
        Self {
            emoji: emoji.into(),
        }
    }

    /// The emoji
    #[must_use]
    pub fn emoji<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.emoji = val.into();
        self
    }
}
