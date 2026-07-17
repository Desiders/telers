use serde::{Deserialize, Serialize};
/// At most one of the optional fields can be present in any given object.
/// Currently, it can be one of
/// - [`crate::types::PollMediaAnimation`]
/// - [`crate::types::PollMediaAudio`]
/// - [`crate::types::PollMediaDocument`]
/// - [`crate::types::PollMediaLink`]
/// - [`crate::types::PollMediaLivePhoto`]
/// - [`crate::types::PollMediaLocation`]
/// - [`crate::types::PollMediaPhoto`]
/// - [`crate::types::PollMediaSticker`]
/// - [`crate::types::PollMediaVenue`]
/// - [`crate::types::PollMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PollMedia {
    Animation(crate::types::PollMediaAnimation),
    LivePhoto(crate::types::PollMediaLivePhoto),
    Venue(crate::types::PollMediaVenue),
    Audio(crate::types::PollMediaAudio),
    Document(crate::types::PollMediaDocument),
    Link(crate::types::PollMediaLink),
    Location(crate::types::PollMediaLocation),
    Photo(crate::types::PollMediaPhoto),
    Sticker(crate::types::PollMediaSticker),
    Video(crate::types::PollMediaVideo),
}
impl PollMedia {
    /// Helper method for field `animation`.
    ///
    /// Media is an animation, information about the animation
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::Animation> {
        match self {
            Self::Animation(val) => Some(val.animation.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `audio`.
    ///
    /// Media is an audio file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::Audio(val) => Some(val.audio.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `document`.
    ///
    /// Media is a general file, information about the file; currently, can't be received in a poll option
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::Document> {
        match self {
            Self::Document(val) => Some(val.document.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `link`.
    ///
    /// The HTTP link attached to the poll option
    #[must_use]
    pub fn link(&self) -> Option<&crate::types::Link> {
        match self {
            Self::Link(val) => Some(&val.link),
            _ => None,
        }
    }

    /// Helper method for field `live_photo`.
    ///
    /// Media is a live photo, information about the live photo
    #[must_use]
    pub fn live_photo(&self) -> Option<&crate::types::LivePhoto> {
        match self {
            Self::LivePhoto(val) => Some(&val.live_photo),
            _ => None,
        }
    }

    /// Helper method for field `location`.
    ///
    /// Media is a shared location, information about the location
    #[must_use]
    pub fn location(&self) -> Option<&crate::types::Location> {
        match self {
            Self::Location(val) => Some(&val.location),
            _ => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// Media is a photo, available sizes of the photo
    #[must_use]
    pub fn photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Photo(val) => Some(val.photo.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `sticker`.
    ///
    /// Media is a sticker, information about the sticker; currently, for poll options only
    #[must_use]
    pub fn sticker(&self) -> Option<&crate::types::Sticker> {
        match self {
            Self::Sticker(val) => Some(val.sticker.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `venue`.
    ///
    /// Media is a venue, information about the venue
    #[must_use]
    pub fn venue(&self) -> Option<&crate::types::Venue> {
        match self {
            Self::Venue(val) => Some(val.venue.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `video`.
    ///
    /// Media is a video, information about the video
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::Video> {
        match self {
            Self::Video(val) => Some(val.video.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `address`.
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                Some(inner.address.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `cover`.
    #[must_use]
    pub fn cover(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.cover.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `custom_emoji_id`.
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::custom_emoji_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `duration`.
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.duration)
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.duration)
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.duration)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.duration)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `emoji`.
    #[must_use]
    pub fn emoji(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::emoji(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_id`.
    #[must_use]
    pub fn file_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.file_id.as_ref())
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::file_id(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_name`.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.file_name.as_deref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.file_name.as_deref()
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.file_name.as_deref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.file_name.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_size`.
    #[must_use]
    pub fn file_size(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.file_size
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.file_size
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.file_size
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                inner.file_size
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::file_size(inner)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.file_size
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_unique_id`.
    #[must_use]
    pub fn file_unique_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.file_unique_id.as_ref())
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::file_unique_id(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `foursquare_id`.
    #[must_use]
    pub fn foursquare_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.foursquare_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `foursquare_type`.
    #[must_use]
    pub fn foursquare_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.foursquare_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `google_place_id`.
    #[must_use]
    pub fn google_place_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.google_place_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `google_place_type`.
    #[must_use]
    pub fn google_place_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.google_place_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `heading`.
    #[must_use]
    pub fn heading(&self) -> Option<u16> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.heading
            }
            _ => None,
        }
    }

    /// Helper method for nested field `height`.
    #[must_use]
    pub fn height(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.height)
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.height)
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::height(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.height)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `horizontal_accuracy`.
    #[must_use]
    pub fn horizontal_accuracy(&self) -> Option<f64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.horizontal_accuracy
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_animated`.
    #[must_use]
    pub fn is_animated(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::is_animated(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_video`.
    #[must_use]
    pub fn is_video(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::is_video(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `latitude`.
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                Some(inner.latitude)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `live_period`.
    #[must_use]
    pub fn live_period(&self) -> Option<i64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.live_period
            }
            _ => None,
        }
    }

    /// Helper method for nested field `longitude`.
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                Some(inner.longitude)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `mask_position`.
    #[must_use]
    pub fn mask_position(&self) -> Option<&crate::types::MaskPosition> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::mask_position(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `mime_type`.
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.mime_type.as_deref()
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                inner.mime_type.as_deref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.mime_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `needs_repainting`.
    #[must_use]
    pub fn needs_repainting(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::needs_repainting(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `performer`.
    #[must_use]
    pub fn performer(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.performer.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `premium_animation`.
    #[must_use]
    pub fn premium_animation(&self) -> Option<&crate::types::File> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::premium_animation(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `proximity_alert_radius`.
    #[must_use]
    pub fn proximity_alert_radius(&self) -> Option<i64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.proximity_alert_radius
            }
            _ => None,
        }
    }

    /// Helper method for nested field `qualities`.
    #[must_use]
    pub fn qualities(&self) -> Option<&[crate::types::VideoQuality]> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.qualities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `set_name`.
    #[must_use]
    pub fn set_name(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::set_name(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `start_timestamp`.
    #[must_use]
    pub fn start_timestamp(&self) -> Option<i64> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.start_timestamp
            }
            _ => None,
        }
    }

    /// Helper method for nested field `thumbnail`.
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::PhotoSize> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::thumbnail(inner)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.thumbnail.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `title`.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.title.as_deref()
            }
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                Some(inner.title.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `url`.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Link(val) => {
                let inner = &val.link;
                Some(inner.url.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `width`.
    #[must_use]
    pub fn width(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.width)
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.width)
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::width(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.width)
            }
            _ => None,
        }
    }
}
impl From<crate::types::PollMediaAnimation> for PollMedia {
    fn from(val: crate::types::PollMediaAnimation) -> Self {
        Self::Animation(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::Animation(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaAnimation),
            ))
        }
    }
}
impl From<crate::types::PollMediaLivePhoto> for PollMedia {
    fn from(val: crate::types::PollMediaLivePhoto) -> Self {
        Self::LivePhoto(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaLivePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::LivePhoto(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaLivePhoto),
            ))
        }
    }
}
impl From<crate::types::PollMediaVenue> for PollMedia {
    fn from(val: crate::types::PollMediaVenue) -> Self {
        Self::Venue(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaVenue {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::Venue(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaVenue),
            ))
        }
    }
}
impl From<crate::types::PollMediaAudio> for PollMedia {
    fn from(val: crate::types::PollMediaAudio) -> Self {
        Self::Audio(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaAudio {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::Audio(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaAudio),
            ))
        }
    }
}
impl From<crate::types::PollMediaDocument> for PollMedia {
    fn from(val: crate::types::PollMediaDocument) -> Self {
        Self::Document(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::Document(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaDocument),
            ))
        }
    }
}
impl From<crate::types::PollMediaLink> for PollMedia {
    fn from(val: crate::types::PollMediaLink) -> Self {
        Self::Link(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaLink {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::Link(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaLink),
            ))
        }
    }
}
impl From<crate::types::PollMediaLocation> for PollMedia {
    fn from(val: crate::types::PollMediaLocation) -> Self {
        Self::Location(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaLocation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::Location(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaLocation),
            ))
        }
    }
}
impl From<crate::types::PollMediaPhoto> for PollMedia {
    fn from(val: crate::types::PollMediaPhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::Photo(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaPhoto),
            ))
        }
    }
}
impl From<crate::types::PollMediaSticker> for PollMedia {
    fn from(val: crate::types::PollMediaSticker) -> Self {
        Self::Sticker(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaSticker {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::Sticker(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaSticker),
            ))
        }
    }
}
impl From<crate::types::PollMediaVideo> for PollMedia {
    fn from(val: crate::types::PollMediaVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<PollMedia> for crate::types::PollMediaVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PollMedia) -> Result<Self, Self::Error> {
        if let PollMedia::Video(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PollMedia),
                stringify!(PollMediaVideo),
            ))
        }
    }
}
