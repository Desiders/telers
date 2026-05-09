use crate::client::Bot;
use serde::Serialize;
/// Use this method to change search keywords assigned to a regular or custom emoji sticker. The sticker must belong to a sticker set created by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickerkeywords>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerKeywords {
    /// File identifier of the sticker
    pub sticker: Box<str>,
    /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Box<[Box<str>]>>,
}
impl SetStickerKeywords {
    /// Creates a new `SetStickerKeywords`.
    ///
    /// # Arguments
    /// * `sticker` - File identifier of the sticker
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(sticker: T0) -> Self {
        Self {
            sticker: sticker.into(),
            keywords: None,
        }
    }

    /// File identifier of the sticker
    #[must_use]
    pub fn sticker<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sticker = val.into();
        self
    }

    /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn keywords<TItem: Into<Box<str>>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: T,
    ) -> Self {
        self.keywords = Some(
            self.keywords
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn keyword<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.keywords = Some(
            self.keywords
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of 0-20 search keywords for the sticker with total length of up to 64 characters
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn keywords_option<TItem: Into<Box<str>>, T: IntoIterator<Item = TItem>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.keywords = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }
}
impl super::TelegramMethod for SetStickerKeywords {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setStickerKeywords", self, None)
    }
}
