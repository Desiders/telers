use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the list of emoji assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickeremojilist>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerEmojiList {
    /// File identifier of the sticker
    pub sticker: Box<str>,
    /// A JSON-serialized list of 1-20 emoji associated with the sticker
    pub emoji_list: Box<[Box<str>]>,
}
impl SetStickerEmojiList {
    /// Creates a new `SetStickerEmojiList`.
    ///
    /// # Arguments
    /// * `sticker` - File identifier of the sticker
    /// * `emoji_list` - A JSON-serialized list of 1-20 emoji associated with the sticker
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1Item: Into<Box<str>>, T1: IntoIterator<Item = T1Item>>(
        sticker: T0,
        emoji_list: T1,
    ) -> Self {
        Self {
            sticker: sticker.into(),
            emoji_list: emoji_list.into_iter().map(Into::into).collect(),
        }
    }

    /// File identifier of the sticker
    #[must_use]
    pub fn sticker<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sticker = val.into();
        self
    }

    /// A JSON-serialized list of 1-20 emoji associated with the sticker
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn emoji_lists<TItem: Into<Box<str>>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.emoji_list = self
            .emoji_list
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        self
    }

    /// A JSON-serialized list of 1-20 emoji associated with the sticker
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn emoji_list<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.emoji_list = self
            .emoji_list
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
impl super::TelegramMethod for SetStickerEmojiList {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setStickerEmojiList", self, None)
    }
}
