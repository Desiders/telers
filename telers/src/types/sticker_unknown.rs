use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a Sticker unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#sticker>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StickerUnknown {
    /// Raw `type` value of the variant unknown to this version of the library
    pub r#type: Box<str>,
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// Sticker width
    pub width: i64,
    /// Sticker height
    pub height: i64,
    /// `true`, if the sticker is animated
    pub is_animated: bool,
    /// `true`, if the sticker is a video sticker
    pub is_video: bool,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl StickerUnknown {
    /// Creates a new `StickerUnknown`.
    ///
    /// # Arguments
    /// * `type` - Raw `type` value of the variant unknown to this version of the library
    /// * `file_id` - Identifier for this file, which can be used to download or reuse the file
    /// * `file_unique_id` - Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    /// * `width` - Sticker width
    /// * `height` - Sticker height
    /// * `is_animated` - `true`, if the sticker is animated
    /// * `is_video` - `true`, if the sticker is a video sticker
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<i64>,
        T4: Into<i64>,
        T5: Into<bool>,
        T6: Into<bool>,
    >(
        r#type: T0,
        file_id: T1,
        file_unique_id: T2,
        width: T3,
        height: T4,
        is_animated: T5,
        is_video: T6,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            width: width.into(),
            height: height.into(),
            is_animated: is_animated.into(),
            is_video: is_video.into(),
            extra: BTreeMap::new(),
        }
    }

    /// Raw `type` value of the variant unknown to this version of the library
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Identifier for this file, which can be used to download or reuse the file
    #[must_use]
    pub fn file_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_id = val.into();
        self
    }

    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[must_use]
    pub fn file_unique_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_unique_id = val.into();
        self
    }

    /// Sticker width
    #[must_use]
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = val.into();
        self
    }

    /// Sticker height
    #[must_use]
    pub fn height<T: Into<i64>>(mut self, val: T) -> Self {
        self.height = val.into();
        self
    }

    /// `true`, if the sticker is animated
    #[must_use]
    pub fn is_animated<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_animated = val.into();
        self
    }

    /// `true`, if the sticker is a video sticker
    #[must_use]
    pub fn is_video<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_video = val.into();
        self
    }
}
