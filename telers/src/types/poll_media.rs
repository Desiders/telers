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
    pub fn animation<T: Into<crate::types::Animation>>(self, val: T) -> Self {
        let mut this = self;
        this.animation = Some(Box::new(val.into()));
        this
    }

    /// Media is an animation, information about the animation
    #[must_use]
    pub fn animation_option<T: Into<crate::types::Animation>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.animation = val.map(|val| Box::new(val.into()));
        this
    }

    /// Media is an audio file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn audio<T: Into<crate::types::Audio>>(self, val: T) -> Self {
        let mut this = self;
        this.audio = Some(Box::new(val.into()));
        this
    }

    /// Media is an audio file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn audio_option<T: Into<crate::types::Audio>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.audio = val.map(|val| Box::new(val.into()));
        this
    }

    /// Media is a general file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn document<T: Into<crate::types::Document>>(self, val: T) -> Self {
        let mut this = self;
        this.document = Some(Box::new(val.into()));
        this
    }

    /// Media is a general file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn document_option<T: Into<crate::types::Document>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.document = val.map(|val| Box::new(val.into()));
        this
    }

    /// Media is a live photo, information about the live photo
    #[must_use]
    pub fn live_photo<T: Into<crate::types::LivePhoto>>(self, val: T) -> Self {
        let mut this = self;
        this.live_photo = Some(val.into());
        this
    }

    /// Media is a live photo, information about the live photo
    #[must_use]
    pub fn live_photo_option<T: Into<crate::types::LivePhoto>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.live_photo = val.map(Into::into);
        this
    }

    /// Media is a shared location, information about the location
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(self, val: T) -> Self {
        let mut this = self;
        this.location = Some(val.into());
        this
    }

    /// Media is a shared location, information about the location
    #[must_use]
    pub fn location_option<T: Into<crate::types::Location>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.location = val.map(Into::into);
        this
    }

    /// Media is a photo, available sizes of the photo
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn photos<T: Into<Box<[crate::types::PhotoSize]>>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = Some(
            this.photo
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// Media is a photo, available sizes of the photo
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo<T: Into<crate::types::PhotoSize>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = Some(
            this.photo
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// Media is a photo, available sizes of the photo
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo_option<T: Into<Box<[crate::types::PhotoSize]>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.photo = val.map(Into::into);
        this
    }

    /// Media is a sticker, information about the sticker; currently, for poll options only
    #[must_use]
    pub fn sticker<T: Into<crate::types::Sticker>>(self, val: T) -> Self {
        let mut this = self;
        this.sticker = Some(Box::new(val.into()));
        this
    }

    /// Media is a sticker, information about the sticker; currently, for poll options only
    #[must_use]
    pub fn sticker_option<T: Into<crate::types::Sticker>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.sticker = val.map(|val| Box::new(val.into()));
        this
    }

    /// Media is a venue, information about the venue
    #[must_use]
    pub fn venue<T: Into<crate::types::Venue>>(self, val: T) -> Self {
        let mut this = self;
        this.venue = Some(Box::new(val.into()));
        this
    }

    /// Media is a venue, information about the venue
    #[must_use]
    pub fn venue_option<T: Into<crate::types::Venue>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.venue = val.map(|val| Box::new(val.into()));
        this
    }

    /// Media is a video, information about the video
    #[must_use]
    pub fn video<T: Into<crate::types::Video>>(self, val: T) -> Self {
        let mut this = self;
        this.video = Some(Box::new(val.into()));
        this
    }

    /// Media is a video, information about the video
    #[must_use]
    pub fn video_option<T: Into<crate::types::Video>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.video = val.map(|val| Box::new(val.into()));
        this
    }
}
impl Default for PollMedia {
    fn default() -> Self {
        Self::new()
    }
}
