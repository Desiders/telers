use crate::client::Bot;
use serde::Serialize;
/// Use this method to get information about custom emoji stickers by their identifiers. Returns an Array of Sticker objects.
/// # Documentation
/// <https://core.telegram.org/bots/api#getcustomemojistickers>
/// # Returns
/// - `Box<[crate::types::Sticker]>`
#[derive(Clone, Debug, Serialize)]
pub struct GetCustomEmojiStickers {
    /// A JSON-serialized list of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    pub custom_emoji_ids: Box<[Box<str>]>,
}
impl GetCustomEmojiStickers {
    /// Creates a new `GetCustomEmojiStickers`.
    ///
    /// # Arguments
    /// * `custom_emoji_ids` - A JSON-serialized list of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    #[must_use]
    pub fn new<T0Item: Into<Box<str>>, T0: IntoIterator<Item = T0Item>>(
        custom_emoji_ids: T0,
    ) -> Self {
        Self {
            custom_emoji_ids: custom_emoji_ids.into_iter().map(Into::into).collect(),
        }
    }

    /// A JSON-serialized list of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn custom_emoji_ids<TItem: Into<Box<str>>, T: IntoIterator<Item = TItem>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.custom_emoji_ids = this
            .custom_emoji_ids
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        this
    }

    /// A JSON-serialized list of custom emoji identifiers. At most 200 custom emoji identifiers can be specified.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn custom_emoji_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.custom_emoji_ids = this
            .custom_emoji_ids
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
impl super::TelegramMethod for GetCustomEmojiStickers {
    type Method = Self;
    type Return = Box<[crate::types::Sticker]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getCustomEmojiStickers", self, None)
    }
}
