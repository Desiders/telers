use serde::{Deserialize, Serialize};
/// This object represents a sticker set.
/// # Documentation
/// <https://core.telegram.org/bots/api#stickerset>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StickerSet {
    /// Sticker set name
    pub name: Box<str>,
    /// Sticker set title
    pub title: Box<str>,
    /// Type of stickers in the set, currently one of `regular`, `mask`, `custom_emoji`
    pub sticker_type: Box<str>,
    /// List of all set stickers
    pub stickers: Box<[crate::types::Sticker]>,
    /// Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::PhotoSize>,
}
impl StickerSet {
    /// Creates a new `StickerSet`.
    ///
    /// # Arguments
    /// * `name` - Sticker set name
    /// * `title` - Sticker set title
    /// * `sticker_type` - Type of stickers in the set, currently one of `regular`, `mask`, `custom_emoji`
    /// * `stickers` - List of all set stickers
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3Item: Into<crate::types::Sticker>,
        T3: IntoIterator<Item = T3Item>,
    >(
        name: T0,
        title: T1,
        sticker_type: T2,
        stickers: T3,
    ) -> Self {
        Self {
            name: name.into(),
            title: title.into(),
            sticker_type: sticker_type.into(),
            stickers: stickers.into_iter().map(Into::into).collect(),
            thumbnail: None,
        }
    }

    /// Sticker set name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = val.into();
        this
    }

    /// Sticker set title
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// Type of stickers in the set, currently one of `regular`, `mask`, `custom_emoji`
    #[must_use]
    pub fn sticker_type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.sticker_type = val.into();
        this
    }

    /// List of all set stickers
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn stickers<T: Into<Box<[crate::types::Sticker]>>>(self, val: T) -> Self {
        let mut this = self;
        this.stickers = this
            .stickers
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// List of all set stickers
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn sticker<T: Into<crate::types::Sticker>>(self, val: T) -> Self {
        let mut this = self;
        this.stickers = this
            .stickers
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::PhotoSize>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail = Some(val.into());
        this
    }

    /// Sticker set thumbnail in the .WEBP, .TGS, or .WEBM format
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::PhotoSize>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail = val.map(Into::into);
        this
    }
}
