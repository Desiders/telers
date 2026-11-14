/// A custom emoji.
/// # Documentation
/// <https://core.telegram.org/bots/api#emojitypecustomemoji>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct EmojiTypeCustomEmoji {
    /// Identifier of the custom emoji. Use [`telers_methods::methods::GetCustomEmojiStickers`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_custom_emoji_stickers/struct.GetCustomEmojiStickers.html) to get full information about the sticker.
    pub custom_emoji_id: Box<str>,
}
impl EmojiTypeCustomEmoji {
    /// Creates a new `EmojiTypeCustomEmoji`.
    ///
    /// # Arguments
    /// * `custom_emoji_id` - Identifier of the custom emoji. Use [`telers_methods::methods::GetCustomEmojiStickers`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_custom_emoji_stickers/struct.GetCustomEmojiStickers.html) to get full information about the sticker.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(custom_emoji_id: T0) -> Self {
        Self {
            custom_emoji_id: custom_emoji_id.into(),
        }
    }

    /// Identifier of the custom emoji. Use [`telers_methods::methods::GetCustomEmojiStickers`](https://docs.rs/telers-methods/latest/telers_methods/methods/get_custom_emoji_stickers/struct.GetCustomEmojiStickers.html) to get full information about the sticker.
    #[must_use]
    pub fn custom_emoji_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.custom_emoji_id = val.into();
        self
    }
}
