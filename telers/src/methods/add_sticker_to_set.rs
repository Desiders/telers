use crate::client::Bot;
use serde::Serialize;
/// Use this method to add a new sticker to a set created by the bot. Emoji sticker sets can have up to 200 stickers. Other sticker sets can have up to 120 stickers. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#addstickertoset>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct AddStickerToSet {
    /// User identifier of sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: Box<str>,
    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    pub sticker: crate::types::InputSticker,
}
impl AddStickerToSet {
    /// Creates a new `AddStickerToSet`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier of sticker set owner
    /// * `name` - Sticker set name
    /// * `sticker` - A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>, T2: Into<crate::types::InputSticker>>(
        user_id: T0,
        name: T1,
        sticker: T2,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            name: name.into(),
            sticker: sticker.into(),
        }
    }

    /// User identifier of sticker set owner
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// Sticker set name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }

    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set isn't changed.
    #[must_use]
    pub fn sticker<T: Into<crate::types::InputSticker>>(mut self, val: T) -> Self {
        self.sticker = val.into();
        self
    }
}
impl super::TelegramMethod for AddStickerToSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_input_sticker(&mut files, &mut self.sticker);
        super::Request::new("addStickerToSet", self, Some(files))
    }
}
