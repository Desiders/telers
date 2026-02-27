use crate::client::Bot;
use serde::Serialize;
/// Use this method to create a new sticker set owned by a user. The bot will be able to edit the sticker set thus created. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#createnewstickerset>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct CreateNewStickerSet {
    /// User identifier of created sticker set owner
    pub user_id: i64,
    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `_by_<bot_username>`. <`bot_username`> is case insensitive. 1-64 characters.
    pub name: Box<str>,
    /// Sticker set title, 1-64 characters
    pub title: Box<str>,
    /// A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    pub stickers: Box<[crate::types::InputSticker]>,
    /// Type of stickers in the set, pass `regular`, `mask`, or `custom_emoji`. By default, a regular sticker set is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<Box<str>>,
    /// Pass `true` if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}
impl CreateNewStickerSet {
    /// Creates a new `CreateNewStickerSet`.
    ///
    /// # Arguments
    /// * `user_id` - User identifier of created sticker set owner
    /// * `name` - Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `_by_<bot_username>`. <`bot_username`> is case insensitive. 1-64 characters.
    /// * `title` - Sticker set title, 1-64 characters
    /// * `stickers` - A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3Item: Into<crate::types::InputSticker>,
        T3: IntoIterator<Item = T3Item>,
    >(
        user_id: T0,
        name: T1,
        title: T2,
        stickers: T3,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            name: name.into(),
            title: title.into(),
            stickers: stickers.into_iter().map(Into::into).collect(),
            sticker_type: None,
            needs_repainting: None,
        }
    }

    /// User identifier of created sticker set owner
    #[must_use]
    pub fn user_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.user_id = val.into();
        this
    }

    /// Short name of sticker set, to be used in t.me/addstickers/ URLs (e.g., animals). Can contain only English letters, digits and underscores. Must begin with a letter, can't contain consecutive underscores and must end in `_by_<bot_username>`. <`bot_username`> is case insensitive. 1-64 characters.
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = val.into();
        this
    }

    /// Sticker set title, 1-64 characters
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn stickers<TItem: Into<crate::types::InputSticker>, T: IntoIterator<Item = TItem>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.stickers = this
            .stickers
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        this
    }

    /// A JSON-serialized list of 1-50 initial stickers to be added to the sticker set
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn sticker<T: Into<crate::types::InputSticker>>(self, val: T) -> Self {
        let mut this = self;
        this.stickers = this
            .stickers
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Type of stickers in the set, pass `regular`, `mask`, or `custom_emoji`. By default, a regular sticker set is created.
    #[must_use]
    pub fn sticker_type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.sticker_type = Some(val.into());
        this
    }

    /// Type of stickers in the set, pass `regular`, `mask`, or `custom_emoji`. By default, a regular sticker set is created.
    #[must_use]
    pub fn sticker_type_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.sticker_type = val.map(Into::into);
        this
    }

    /// Pass `true` if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    #[must_use]
    pub fn needs_repainting<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.needs_repainting = Some(val.into());
        this
    }

    /// Pass `true` if stickers in the sticker set must be repainted to the color of text when used in messages, the accent color if used as emoji status, white on chat photos, or another appropriate color based on context; for custom emoji sticker sets only
    #[must_use]
    pub fn needs_repainting_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.needs_repainting = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for CreateNewStickerSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_input_stickers(&mut files, self.stickers.iter_mut().collect());
        super::Request::new("createNewStickerSet", self, Some(files))
    }
}
