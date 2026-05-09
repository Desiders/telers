use serde::{Deserialize, Serialize};
/// At most one of the optional fields can be present in any given object.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMedia {
    /// Media is an animation, information about the animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Box<crate::types::Animation>>,
    /// Media is an audio file, information about the file; currently, can't be received in a poll option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Box<crate::types::Audio>>,
    /// Media is a general file, information about the file; currently, can't be received in a poll option
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Box<crate::types::Document>>,
    /// Media is a live photo, information about the live photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_photo: Option<crate::types::LivePhoto>,
    /// Media is a shared location, information about the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::types::Location>,
    /// Media is a photo, available sizes of the photo
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Box<[crate::types::PhotoSize]>>,
    /// Media is a sticker, information about the sticker; currently, for poll options only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Box<crate::types::Sticker>>,
    /// Media is a venue, information about the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Box<crate::types::Venue>>,
    /// Media is a video, information about the video
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<crate::types::Video>>,
}
impl PollMedia {
    /// Creates a new `PollMedia`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            animation: None,
            audio: None,
            document: None,
            live_photo: None,
            location: None,
            photo: None,
            sticker: None,
            venue: None,
            video: None,
        }
    }

    /// Media is an animation, information about the animation
    #[must_use]
    pub fn animation<T: Into<crate::types::Animation>>(mut self, val: T) -> Self {
        self.animation = Some(Box::new(val.into()));
        self
    }

    /// Media is an animation, information about the animation
    #[must_use]
    pub fn animation_option<T: Into<crate::types::Animation>>(mut self, val: Option<T>) -> Self {
        self.animation = val.map(|val| Box::new(val.into()));
        self
    }

    /// Media is an audio file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn audio<T: Into<crate::types::Audio>>(mut self, val: T) -> Self {
        self.audio = Some(Box::new(val.into()));
        self
    }

    /// Media is an audio file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn audio_option<T: Into<crate::types::Audio>>(mut self, val: Option<T>) -> Self {
        self.audio = val.map(|val| Box::new(val.into()));
        self
    }

    /// Media is a general file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn document<T: Into<crate::types::Document>>(mut self, val: T) -> Self {
        self.document = Some(Box::new(val.into()));
        self
    }

    /// Media is a general file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn document_option<T: Into<crate::types::Document>>(mut self, val: Option<T>) -> Self {
        self.document = val.map(|val| Box::new(val.into()));
        self
    }

    /// Media is a live photo, information about the live photo
    #[must_use]
    pub fn live_photo<T: Into<crate::types::LivePhoto>>(mut self, val: T) -> Self {
        self.live_photo = Some(val.into());
        self
    }

    /// Media is a live photo, information about the live photo
    #[must_use]
    pub fn live_photo_option<T: Into<crate::types::LivePhoto>>(mut self, val: Option<T>) -> Self {
        self.live_photo = val.map(Into::into);
        self
    }

    /// Media is a shared location, information about the location
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(mut self, val: T) -> Self {
        self.location = Some(val.into());
        self
    }

    /// Media is a shared location, information about the location
    #[must_use]
    pub fn location_option<T: Into<crate::types::Location>>(mut self, val: Option<T>) -> Self {
        self.location = val.map(Into::into);
        self
    }

    /// Media is a photo, available sizes of the photo
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn photos<T: Into<Box<[crate::types::PhotoSize]>>>(mut self, val: T) -> Self {
        self.photo = Some(
            self.photo
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Media is a photo, available sizes of the photo
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo<T: Into<crate::types::PhotoSize>>(mut self, val: T) -> Self {
        self.photo = Some(
            self.photo
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Media is a photo, available sizes of the photo
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo_option<T: Into<Box<[crate::types::PhotoSize]>>>(mut self, val: Option<T>) -> Self {
        self.photo = val.map(Into::into);
        self
    }

    /// Media is a sticker, information about the sticker; currently, for poll options only
    #[must_use]
    pub fn sticker<T: Into<crate::types::Sticker>>(mut self, val: T) -> Self {
        self.sticker = Some(Box::new(val.into()));
        self
    }

    /// Media is a sticker, information about the sticker; currently, for poll options only
    #[must_use]
    pub fn sticker_option<T: Into<crate::types::Sticker>>(mut self, val: Option<T>) -> Self {
        self.sticker = val.map(|val| Box::new(val.into()));
        self
    }

    /// Media is a venue, information about the venue
    #[must_use]
    pub fn venue<T: Into<crate::types::Venue>>(mut self, val: T) -> Self {
        self.venue = Some(Box::new(val.into()));
        self
    }

    /// Media is a venue, information about the venue
    #[must_use]
    pub fn venue_option<T: Into<crate::types::Venue>>(mut self, val: Option<T>) -> Self {
        self.venue = val.map(|val| Box::new(val.into()));
        self
    }

    /// Media is a video, information about the video
    #[must_use]
    pub fn video<T: Into<crate::types::Video>>(mut self, val: T) -> Self {
        self.video = Some(Box::new(val.into()));
        self
    }

    /// Media is a video, information about the video
    #[must_use]
    pub fn video_option<T: Into<crate::types::Video>>(mut self, val: Option<T>) -> Self {
        self.video = val.map(|val| Box::new(val.into()));
        self
    }
}
impl Default for PollMedia {
    fn default() -> Self {
        Self::new()
    }
}
