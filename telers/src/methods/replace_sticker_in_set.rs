use crate::client::Bot;
use serde::Serialize;
/// Use this method to replace an existing sticker in a sticker set with a new one. The method is equivalent to calling deleteStickerFromSet, then addStickerToSet, then setStickerPositionInSet. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#replacestickerinset>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct ReplaceStickerInSet {
    /// User identifier of the sticker set owner
    pub user_id: i64,
    /// Sticker set name
    pub name: Box<str>,
    /// File identifier of the replaced sticker
    pub old_sticker: Box<str>,
    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.
    pub sticker: crate::types::InputSticker,
}
impl ReplaceStickerInSet {
    /// Creates a new `ReplaceStickerInSet`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier of the sticker set owner
    /// * `name` - Sticker set name
    /// * `old_sticker` - File identifier of the replaced sticker
    /// * `sticker` - A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<crate::types::InputSticker>,
    >(
        user_id: T0,
        name: T1,
        old_sticker: T2,
        sticker: T3,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            name: name.into(),
            old_sticker: old_sticker.into(),
            sticker: sticker.into(),
        }
    }

    /// User identifier of the sticker set owner
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// Sticker set name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = val.into();
        this
    }

    /// File identifier of the replaced sticker
    #[must_use]
    pub fn old_sticker<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.old_sticker = val.into();
        this
    }

    /// A JSON-serialized object with information about the added sticker. If exactly the same sticker had already been added to the set, then the set remains unchanged.
    #[must_use]
    pub fn sticker<T: Into<crate::types::InputSticker>>(self, val: T) -> Self {
        let mut this = self;
        this.sticker = val.into();
        this
    }
}
impl super::TelegramMethod for ReplaceStickerInSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_input_sticker(&mut files, &mut self.sticker);
        super::Request::new("replaceStickerInSet", self, Some(files))
    }
}
