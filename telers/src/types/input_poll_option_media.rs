use serde::{Deserialize, Serialize};
/// This object represents the content of a poll option to be sent. It should be one of
/// - [`crate::types::InputMediaAnimation`]
/// - [`crate::types::InputMediaLink`]
/// - [`crate::types::InputMediaLivePhoto`]
/// - [`crate::types::InputMediaLocation`]
/// - [`crate::types::InputMediaPhoto`]
/// - [`crate::types::InputMediaSticker`]
/// - [`crate::types::InputMediaVenue`]
/// - [`crate::types::InputMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpolloptionmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPollOptionMedia {
    Animation(crate::types::InputMediaAnimation),
    Link(crate::types::InputMediaLink),
    LivePhoto(crate::types::InputMediaLivePhoto),
    Location(crate::types::InputMediaLocation),
    Photo(crate::types::InputMediaPhoto),
    Sticker(crate::types::InputMediaSticker),
    Venue(crate::types::InputMediaVenue),
    Video(crate::types::InputMediaVideo),
}
impl InputPollOptionMedia {
    /// Helper method for field `address`.
    ///
    /// Address of the venue
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => Some(val.address.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `caption`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Caption of the animation to be sent, 0-1024 characters after entities parsing
    /// - `InputMediaLivePhoto`. Caption of the live photo to be sent, 0-1024 characters after entities parsing
    /// - `InputMediaPhoto`. Caption of the photo to be sent, 0-1024 characters after entities parsing
    /// - `InputMediaVideo`. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.caption.as_deref(),
            Self::LivePhoto(val) => val.caption.as_deref(),
            Self::Photo(val) => val.caption.as_deref(),
            Self::Video(val) => val.caption.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `caption_entities`.
    ///
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Animation(val) => val.caption_entities.as_deref(),
            Self::LivePhoto(val) => val.caption_entities.as_deref(),
            Self::Photo(val) => val.caption_entities.as_deref(),
            Self::Video(val) => val.caption_entities.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `cover`.
    ///
    /// Cover for the video in the message. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn cover(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Video(val) => val.cover.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `duration`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Animation duration in seconds
    /// - `InputMediaVideo`. Video duration in seconds
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.duration,
            Self::Video(val) => val.duration,
            _ => None,
        }
    }

    /// Helper method for field `emoji`.
    ///
    /// Emoji associated with the sticker; only for just uploaded stickers
    #[must_use]
    pub fn emoji(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => val.emoji.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `foursquare_id`.
    ///
    /// Foursquare identifier of the venue
    #[must_use]
    pub fn foursquare_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => val.foursquare_id.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `foursquare_type`.
    ///
    /// Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[must_use]
    pub fn foursquare_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => val.foursquare_type.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `google_place_id`.
    ///
    /// Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => val.google_place_id.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `google_place_type`.
    ///
    /// Google Places type of the venue. (See supported types.)
    #[must_use]
    pub fn google_place_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => val.google_place_type.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `has_spoiler`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Pass `true` if the animation needs to be covered with a spoiler animation
    /// - `InputMediaLivePhoto`. Pass `true` if the live photo needs to be covered with a spoiler animation
    /// - `InputMediaPhoto`. Pass `true` if the photo needs to be covered with a spoiler animation
    /// - `InputMediaVideo`. Pass `true` if the video needs to be covered with a spoiler animation
    #[must_use]
    pub fn has_spoiler(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.has_spoiler,
            Self::LivePhoto(val) => val.has_spoiler,
            Self::Photo(val) => val.has_spoiler,
            Self::Video(val) => val.has_spoiler,
            _ => None,
        }
    }

    /// Helper method for field `height`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Animation height
    /// - `InputMediaVideo`. Video height
    #[must_use]
    pub fn height(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.height,
            Self::Video(val) => val.height,
            _ => None,
        }
    }

    /// Helper method for field `horizontal_accuracy`.
    ///
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy(&self) -> Option<f64> {
        match self {
            Self::Location(val) => val.horizontal_accuracy,
            _ => None,
        }
    }

    /// Helper method for field `latitude`.
    ///
    /// Latitude of the location
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => Some(val.latitude),
            Self::Venue(val) => Some(val.latitude),
            _ => None,
        }
    }

    /// Helper method for field `longitude`.
    ///
    /// Longitude of the location
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => Some(val.longitude),
            Self::Venue(val) => Some(val.longitude),
            _ => None,
        }
    }

    /// Helper method for field `media`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`, `InputMediaPhoto`, `InputMediaVideo`. File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a file from the Internet, or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    /// - `InputMediaLivePhoto`. Video of the live photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    /// - `InputMediaSticker`. File to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended), pass an HTTP URL for Telegram to get a .WEBP sticker from the Internet, or pass `attach://<file_attach_name>` to upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn media(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Animation(val) => Some(&val.media),
            Self::LivePhoto(val) => Some(&val.media),
            Self::Photo(val) => Some(&val.media),
            Self::Sticker(val) => Some(&val.media),
            Self::Video(val) => Some(&val.media),
            _ => None,
        }
    }

    /// Helper method for field `parse_mode`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Mode for parsing entities in the animation caption. See formatting options for more details.
    /// - `InputMediaLivePhoto`. Mode for parsing entities in the live photo caption. See formatting options for more details.
    /// - `InputMediaPhoto`. Mode for parsing entities in the photo caption. See formatting options for more details.
    /// - `InputMediaVideo`. Mode for parsing entities in the video caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.parse_mode.as_deref(),
            Self::LivePhoto(val) => val.parse_mode.as_deref(),
            Self::Photo(val) => val.parse_mode.as_deref(),
            Self::Video(val) => val.parse_mode.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// The static photo to send. Pass a `file_id` to send a file that exists on the Telegram servers (recommended) or pass `attach://<file_attach_name>` to upload a new one using multipart/form-data under <`file_attach_name`> name. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Sending live photos by a URL is currently unsupported.
    #[must_use]
    pub fn photo(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::LivePhoto(val) => Some(&val.photo),
            _ => None,
        }
    }

    /// Helper method for field `show_caption_above_media`.
    ///
    /// Pass `true` if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.show_caption_above_media,
            Self::LivePhoto(val) => val.show_caption_above_media,
            Self::Photo(val) => val.show_caption_above_media,
            Self::Video(val) => val.show_caption_above_media,
            _ => None,
        }
    }

    /// Helper method for field `start_timestamp`.
    ///
    /// Start timestamp for the video in the message
    #[must_use]
    pub fn start_timestamp(&self) -> Option<i64> {
        match self {
            Self::Video(val) => val.start_timestamp,
            _ => None,
        }
    }

    /// Helper method for field `supports_streaming`.
    ///
    /// Pass `true` if the uploaded video is suitable for streaming
    #[must_use]
    pub fn supports_streaming(&self) -> Option<bool> {
        match self {
            Self::Video(val) => val.supports_streaming,
            _ => None,
        }
    }

    /// Helper method for field `thumbnail`.
    ///
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::InputFile> {
        match self {
            Self::Animation(val) => val.thumbnail.as_ref(),
            Self::Video(val) => val.thumbnail.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `title`.
    ///
    /// Name of the venue
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => Some(val.title.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `url`.
    ///
    /// HTTP URL of the link
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Link(val) => Some(val.url.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `width`.
    ///
    /// # Variants
    /// - `InputMediaAnimation`. Animation width
    /// - `InputMediaVideo`. Video width
    #[must_use]
    pub fn width(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.width,
            Self::Video(val) => val.width,
            _ => None,
        }
    }
}
impl From<crate::types::InputMediaAnimation> for InputPollOptionMedia {
    fn from(val: crate::types::InputMediaAnimation) -> Self {
        Self::Animation(val)
    }
}
impl TryFrom<InputPollOptionMedia> for crate::types::InputMediaAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPollOptionMedia) -> Result<Self, Self::Error> {
        if let InputPollOptionMedia::Animation(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputPollOptionMedia),
                stringify!(InputMediaAnimation),
            ))
        }
    }
}
impl From<crate::types::InputMediaLink> for InputPollOptionMedia {
    fn from(val: crate::types::InputMediaLink) -> Self {
        Self::Link(val)
    }
}
impl TryFrom<InputPollOptionMedia> for crate::types::InputMediaLink {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPollOptionMedia) -> Result<Self, Self::Error> {
        if let InputPollOptionMedia::Link(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputPollOptionMedia),
                stringify!(InputMediaLink),
            ))
        }
    }
}
impl From<crate::types::InputMediaLivePhoto> for InputPollOptionMedia {
    fn from(val: crate::types::InputMediaLivePhoto) -> Self {
        Self::LivePhoto(val)
    }
}
impl TryFrom<InputPollOptionMedia> for crate::types::InputMediaLivePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPollOptionMedia) -> Result<Self, Self::Error> {
        if let InputPollOptionMedia::LivePhoto(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputPollOptionMedia),
                stringify!(InputMediaLivePhoto),
            ))
        }
    }
}
impl From<crate::types::InputMediaLocation> for InputPollOptionMedia {
    fn from(val: crate::types::InputMediaLocation) -> Self {
        Self::Location(val)
    }
}
impl TryFrom<InputPollOptionMedia> for crate::types::InputMediaLocation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPollOptionMedia) -> Result<Self, Self::Error> {
        if let InputPollOptionMedia::Location(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputPollOptionMedia),
                stringify!(InputMediaLocation),
            ))
        }
    }
}
impl From<crate::types::InputMediaPhoto> for InputPollOptionMedia {
    fn from(val: crate::types::InputMediaPhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<InputPollOptionMedia> for crate::types::InputMediaPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPollOptionMedia) -> Result<Self, Self::Error> {
        if let InputPollOptionMedia::Photo(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputPollOptionMedia),
                stringify!(InputMediaPhoto),
            ))
        }
    }
}
impl From<crate::types::InputMediaSticker> for InputPollOptionMedia {
    fn from(val: crate::types::InputMediaSticker) -> Self {
        Self::Sticker(val)
    }
}
impl TryFrom<InputPollOptionMedia> for crate::types::InputMediaSticker {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPollOptionMedia) -> Result<Self, Self::Error> {
        if let InputPollOptionMedia::Sticker(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputPollOptionMedia),
                stringify!(InputMediaSticker),
            ))
        }
    }
}
impl From<crate::types::InputMediaVenue> for InputPollOptionMedia {
    fn from(val: crate::types::InputMediaVenue) -> Self {
        Self::Venue(val)
    }
}
impl TryFrom<InputPollOptionMedia> for crate::types::InputMediaVenue {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPollOptionMedia) -> Result<Self, Self::Error> {
        if let InputPollOptionMedia::Venue(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputPollOptionMedia),
                stringify!(InputMediaVenue),
            ))
        }
    }
}
impl From<crate::types::InputMediaVideo> for InputPollOptionMedia {
    fn from(val: crate::types::InputMediaVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<InputPollOptionMedia> for crate::types::InputMediaVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputPollOptionMedia) -> Result<Self, Self::Error> {
        if let InputPollOptionMedia::Video(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputPollOptionMedia),
                stringify!(InputMediaVideo),
            ))
        }
    }
}
