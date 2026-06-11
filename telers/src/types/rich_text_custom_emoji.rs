use serde::{Deserialize, Serialize};
/// A custom emoji.
/// # Documentation
/// <https://core.telegram.org/bots/api#richtextcustomemoji>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichTextCustomEmoji {
    /// Unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker.
    pub custom_emoji_id: Box<str>,
    /// Alternative emoji for the custom emoji
    pub alternative_text: Box<str>,
}
impl RichTextCustomEmoji {
    /// Creates a new `RichTextCustomEmoji`.
    ///
    /// # Arguments
    /// * `custom_emoji_id` - Unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker.
    /// * `alternative_text` - Alternative emoji for the custom emoji
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(
        custom_emoji_id: T0,
        alternative_text: T1,
    ) -> Self {
        Self {
            custom_emoji_id: custom_emoji_id.into(),
            alternative_text: alternative_text.into(),
        }
    }

    /// Unique identifier of the custom emoji. Use getCustomEmojiStickers to get full information about the sticker.
    #[must_use]
    pub fn custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.custom_emoji_id = val.into();
        self
    }

    /// Alternative emoji for the custom emoji
    #[must_use]
    pub fn alternative_text<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.alternative_text = val.into();
        self
    }
}
