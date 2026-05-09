use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the mask position of a mask sticker. The sticker must belong to a sticker set that was created by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setstickermaskposition>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetStickerMaskPosition {
    /// File identifier of the sticker
    pub sticker: Box<str>,
    /// A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<crate::types::MaskPosition>,
}
impl SetStickerMaskPosition {
    /// Creates a new `SetStickerMaskPosition`.
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
            mask_position: None,
        }
    }

    /// File identifier of the sticker
    #[must_use]
    pub fn sticker<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sticker = val.into();
        self
    }

    /// A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    #[must_use]
    pub fn mask_position<T: Into<crate::types::MaskPosition>>(mut self, val: T) -> Self {
        self.mask_position = Some(val.into());
        self
    }

    /// A JSON-serialized object with the position where the mask should be placed on faces. Omit the parameter to remove the mask position.
    #[must_use]
    pub fn mask_position_option<T: Into<crate::types::MaskPosition>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.mask_position = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SetStickerMaskPosition {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setStickerMaskPosition", self, None)
    }
}
