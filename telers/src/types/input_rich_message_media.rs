use serde::{Deserialize, Serialize};
/// Describes a media element embedded in an outgoing rich message.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichmessagemedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichMessageMedia {
    /// Unique identifier of the media used in a `tg://photo?id=`, `tg://video?id=`, or `tg://audio?id=` link. 1-64 characters, only A-Z, a-z, 0-9, `_` and - are allowed.
    pub id: Box<str>,
    /// The media to be sent. Everything except the media itself and its properties is ignored.
    pub media: crate::types::InputRichMessageMediaContent,
}
impl InputRichMessageMedia {
    /// Creates a new `InputRichMessageMedia`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the media used in a `tg://photo?id=`, `tg://video?id=`, or `tg://audio?id=` link. 1-64 characters, only A-Z, a-z, 0-9, `_` and - are allowed.
    /// * `media` - The media to be sent. Everything except the media itself and its properties is ignored.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::InputRichMessageMediaContent>>(
        id: T0,
        media: T1,
    ) -> Self {
        Self {
            id: id.into(),
            media: media.into(),
        }
    }

    /// Unique identifier of the media used in a `tg://photo?id=`, `tg://video?id=`, or `tg://audio?id=` link. 1-64 characters, only A-Z, a-z, 0-9, `_` and - are allowed.
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// The media to be sent. Everything except the media itself and its properties is ignored.
    #[must_use]
    pub fn media<T: Into<crate::types::InputRichMessageMediaContent>>(mut self, val: T) -> Self {
        self.media = val.into();
        self
    }
}
