use serde::{Deserialize, Serialize};
/// This object represents a/an custom emoji message entity.
/// # Notes
/// This object represents a message entity from original field `custom_emoji`.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageentity>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageEntityCustomEmoji {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: i64,
    /// Length of the entity in UTF-16 code units
    pub length: i64,
    /// For `custom_emoji` only, unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker
    pub custom_emoji_id: Box<str>,
}
impl MessageEntityCustomEmoji {
    /// Creates a new `MessageEntityCustomEmoji`.
    ///
    /// # Arguments
    /// * `offset` - Offset in UTF-16 code units to the start of the entity
    /// * `length` - Length of the entity in UTF-16 code units
    /// * `custom_emoji_id` - For `custom_emoji` only, unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<Box<str>>>(
        offset: T0,
        length: T1,
        custom_emoji_id: T2,
    ) -> Self {
        Self {
            offset: offset.into(),
            length: length.into(),
            custom_emoji_id: custom_emoji_id.into(),
        }
    }

    /// Offset in UTF-16 code units to the start of the entity
    #[must_use]
    pub fn offset<T: Into<i64>>(mut self, val: T) -> Self {
        self.offset = val.into();
        self
    }

    /// Length of the entity in UTF-16 code units
    #[must_use]
    pub fn length<T: Into<i64>>(mut self, val: T) -> Self {
        self.length = val.into();
        self
    }

    /// For `custom_emoji` only, unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker
    #[must_use]
    pub fn custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.custom_emoji_id = val.into();
        self
    }
}
