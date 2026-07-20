use serde::{Deserialize, Serialize};
/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
/// - [`crate::types::InlineQueryResultCachedAudio`]
/// - [`crate::types::InlineQueryResultCachedDocument`]
/// - [`crate::types::InlineQueryResultCachedGif`]
/// - [`crate::types::InlineQueryResultCachedMpeg4Gif`]
/// - [`crate::types::InlineQueryResultCachedPhoto`]
/// - [`crate::types::InlineQueryResultCachedSticker`]
/// - [`crate::types::InlineQueryResultCachedVideo`]
/// - [`crate::types::InlineQueryResultCachedVoice`]
/// - [`crate::types::InlineQueryResultArticle`]
/// - [`crate::types::InlineQueryResultAudio`]
/// - [`crate::types::InlineQueryResultContact`]
/// - [`crate::types::InlineQueryResultGame`]
/// - [`crate::types::InlineQueryResultDocument`]
/// - [`crate::types::InlineQueryResultGif`]
/// - [`crate::types::InlineQueryResultLocation`]
/// - [`crate::types::InlineQueryResultMpeg4Gif`]
/// - [`crate::types::InlineQueryResultPhoto`]
/// - [`crate::types::InlineQueryResultVenue`]
/// - [`crate::types::InlineQueryResultVideo`]
/// - [`crate::types::InlineQueryResultVoice`]
///
/// Note: All URLs passed in inline query results will be available to end users and therefore must be assumed to be public.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InlineQueryResult {
    Audio(crate::types::InlineQueryResultAudioKind),
    Document(crate::types::InlineQueryResultDocumentKind),
    Gif(crate::types::InlineQueryResultGifKind),
    Mpeg4Gif(crate::types::InlineQueryResultMpeg4GifKind),
    Photo(crate::types::InlineQueryResultPhotoKind),
    Sticker(crate::types::InlineQueryResultCachedSticker),
    Video(crate::types::InlineQueryResultVideoKind),
    Voice(crate::types::InlineQueryResultVoiceKind),
    Article(crate::types::InlineQueryResultArticle),
    Contact(crate::types::InlineQueryResultContact),
    Game(crate::types::InlineQueryResultGame),
    Location(crate::types::InlineQueryResultLocation),
    Venue(crate::types::InlineQueryResultVenue),
}
impl InlineQueryResult {
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

    /// Helper method for field `audio_duration`.
    ///
    /// Audio duration in seconds
    #[must_use]
    pub fn audio_duration(&self) -> Option<i64> {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::audio_duration(val),
            _ => None,
        }
    }

    /// Helper method for field `audio_file_id`.
    ///
    /// A valid file identifier for the audio file
    #[must_use]
    pub fn audio_file_id(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::audio_file_id(val),
            _ => None,
        }
    }

    /// Helper method for field `audio_url`.
    ///
    /// A valid URL for the audio file
    #[must_use]
    pub fn audio_url(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::audio_url(val),
            _ => None,
        }
    }

    /// Helper method for field `caption`.
    ///
    /// # Variants
    /// - `InlineQueryResultAudioKind`, `InlineQueryResultVoiceKind`. Caption, 0-1024 characters after entities parsing
    /// - `InlineQueryResultDocumentKind`. Caption of the document to be sent, 0-1024 characters after entities parsing
    /// - `InlineQueryResultGifKind`. Caption of the GIF file to be sent, 0-1024 characters after entities parsing
    /// - `InlineQueryResultMpeg4GifKind`. Caption of the MPEG-4 file to be sent, 0-1024 characters after entities parsing
    /// - `InlineQueryResultPhotoKind`. Caption of the photo to be sent, 0-1024 characters after entities parsing
    /// - `InlineQueryResultVideoKind`. Caption of the video to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::caption(val),
            Self::Document(val) => crate::types::InlineQueryResultDocumentKind::caption(val),
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::caption(val),
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::caption(val),
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::caption(val),
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::caption(val),
            Self::Voice(val) => crate::types::InlineQueryResultVoiceKind::caption(val),
            _ => None,
        }
    }

    /// Helper method for field `caption_entities`.
    ///
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::caption_entities(val),
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::caption_entities(val)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::caption_entities(val),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::caption_entities(val)
            }
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::caption_entities(val),
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::caption_entities(val),
            Self::Voice(val) => crate::types::InlineQueryResultVoiceKind::caption_entities(val),
            _ => None,
        }
    }

    /// Helper method for field `description`.
    ///
    /// Short description of the result
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Document(val) => crate::types::InlineQueryResultDocumentKind::description(val),
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::description(val),
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::description(val),
            Self::Article(val) => val.description.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `document_file_id`.
    ///
    /// A valid file identifier for the file
    #[must_use]
    pub fn document_file_id(&self) -> Option<&str> {
        match self {
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::document_file_id(val)
            }
            _ => None,
        }
    }

    /// Helper method for field `document_url`.
    ///
    /// A valid URL for the file
    #[must_use]
    pub fn document_url(&self) -> Option<&str> {
        match self {
            Self::Document(val) => crate::types::InlineQueryResultDocumentKind::document_url(val),
            _ => None,
        }
    }

    /// Helper method for field `first_name`.
    ///
    /// Contact's first name
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => Some(val.first_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `foursquare_id`.
    ///
    /// Foursquare identifier of the venue if known
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

    /// Helper method for field `game_short_name`.
    ///
    /// Short name of the game
    #[must_use]
    pub fn game_short_name(&self) -> Option<&str> {
        match self {
            Self::Game(val) => Some(val.game_short_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `gif_duration`.
    ///
    /// Duration of the GIF in seconds
    #[must_use]
    pub fn gif_duration(&self) -> Option<i64> {
        match self {
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::gif_duration(val),
            _ => None,
        }
    }

    /// Helper method for field `gif_file_id`.
    ///
    /// A valid file identifier for the GIF file
    #[must_use]
    pub fn gif_file_id(&self) -> Option<&str> {
        match self {
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::gif_file_id(val),
            _ => None,
        }
    }

    /// Helper method for field `gif_height`.
    ///
    /// Height of the GIF
    #[must_use]
    pub fn gif_height(&self) -> Option<i64> {
        match self {
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::gif_height(val),
            _ => None,
        }
    }

    /// Helper method for field `gif_url`.
    ///
    /// A valid URL for the GIF file
    #[must_use]
    pub fn gif_url(&self) -> Option<&str> {
        match self {
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::gif_url(val),
            _ => None,
        }
    }

    /// Helper method for field `gif_width`.
    ///
    /// Width of the GIF
    #[must_use]
    pub fn gif_width(&self) -> Option<i64> {
        match self {
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::gif_width(val),
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

    /// Helper method for field `heading`.
    ///
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading(&self) -> Option<u16> {
        match self {
            Self::Location(val) => val.heading,
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

    /// Helper method for field `id`.
    ///
    /// # Variants
    /// - `InlineQueryResultAudioKind`, `InlineQueryResultDocumentKind`, `InlineQueryResultGifKind`, `InlineQueryResultMpeg4GifKind`, `InlineQueryResultPhotoKind`, `InlineQueryResultCachedSticker`, `InlineQueryResultVideoKind`, `InlineQueryResultVoiceKind`, `InlineQueryResultGame`. Unique identifier for this result, 1-64 bytes
    /// - `InlineQueryResultArticle`, `InlineQueryResultContact`, `InlineQueryResultLocation`, `InlineQueryResultVenue`. Unique identifier for this result, 1-64 Bytes
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::id(val),
            Self::Document(val) => crate::types::InlineQueryResultDocumentKind::id(val),
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::id(val),
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::id(val),
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::id(val),
            Self::Sticker(val) => val.id.as_ref(),
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::id(val),
            Self::Voice(val) => crate::types::InlineQueryResultVoiceKind::id(val),
            Self::Article(val) => val.id.as_ref(),
            Self::Contact(val) => val.id.as_ref(),
            Self::Game(val) => val.id.as_ref(),
            Self::Location(val) => val.id.as_ref(),
            Self::Venue(val) => val.id.as_ref(),
        }
    }

    /// Helper method for field `input_message_content`.
    ///
    /// # Variants
    /// - `InlineQueryResultAudioKind`. Content of the message to be sent instead of the audio
    /// - `InlineQueryResultDocumentKind`. Content of the message to be sent instead of the file
    /// - `InlineQueryResultGifKind`. Content of the message to be sent instead of the GIF animation
    /// - `InlineQueryResultMpeg4GifKind`. Content of the message to be sent instead of the video animation
    /// - `InlineQueryResultPhotoKind`. Content of the message to be sent instead of the photo
    /// - `InlineQueryResultCachedSticker`. Content of the message to be sent instead of the sticker
    /// - `InlineQueryResultVideoKind`. Content of the message to be sent instead of the video. This field is required if [`crate::types::InlineQueryResultVideo`] is used to send an HTML-page as a result (e.g., a `YouTube` video).
    /// - `InlineQueryResultVoiceKind`. Content of the message to be sent instead of the voice recording
    /// - `InlineQueryResultArticle`. Content of the message to be sent
    /// - `InlineQueryResultContact`. Content of the message to be sent instead of the contact
    /// - `InlineQueryResultLocation`. Content of the message to be sent instead of the location
    /// - `InlineQueryResultVenue`. Content of the message to be sent instead of the venue
    #[must_use]
    pub fn input_message_content(&self) -> Option<&crate::types::InputMessageContent> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
            }
            Self::Sticker(val) => val.input_message_content.as_ref(),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
            }
            Self::Article(val) => Some(&val.input_message_content),
            Self::Contact(val) => val.input_message_content.as_ref(),
            Self::Location(val) => val.input_message_content.as_ref(),
            Self::Venue(val) => val.input_message_content.as_ref(),
            Self::Game(_) => None,
        }
    }

    /// Helper method for field `last_name`.
    ///
    /// Contact's last name
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => val.last_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `latitude`.
    ///
    /// # Variants
    /// - `InlineQueryResultLocation`. Location latitude in degrees
    /// - `InlineQueryResultVenue`. Latitude of the venue location in degrees
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => Some(val.latitude),
            Self::Venue(val) => Some(val.latitude),
            _ => None,
        }
    }

    /// Helper method for field `live_period`.
    ///
    /// Period in seconds during which the location can be updated, must be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely
    #[must_use]
    pub fn live_period(&self) -> Option<u32> {
        match self {
            Self::Location(val) => val.live_period,
            _ => None,
        }
    }

    /// Helper method for field `longitude`.
    ///
    /// # Variants
    /// - `InlineQueryResultLocation`. Location longitude in degrees
    /// - `InlineQueryResultVenue`. Longitude of the venue location in degrees
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => Some(val.longitude),
            Self::Venue(val) => Some(val.longitude),
            _ => None,
        }
    }

    /// Helper method for field `mime_type`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocumentKind`. MIME type of the content of the file, either `application/pdf` or `application/zip`
    /// - `InlineQueryResultVideoKind`. MIME type of the content of the video URL, `text/html` or `video/mp4`
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::Document(val) => crate::types::InlineQueryResultDocumentKind::mime_type(val),
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::mime_type(val),
            _ => None,
        }
    }

    /// Helper method for field `mpeg4_duration`.
    ///
    /// Video duration in seconds
    #[must_use]
    pub fn mpeg4_duration(&self) -> Option<i64> {
        match self {
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::mpeg4_duration(val),
            _ => None,
        }
    }

    /// Helper method for field `mpeg4_file_id`.
    ///
    /// A valid file identifier for the MPEG4 file
    #[must_use]
    pub fn mpeg4_file_id(&self) -> Option<&str> {
        match self {
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::mpeg4_file_id(val),
            _ => None,
        }
    }

    /// Helper method for field `mpeg4_height`.
    ///
    /// Video height
    #[must_use]
    pub fn mpeg4_height(&self) -> Option<i64> {
        match self {
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::mpeg4_height(val),
            _ => None,
        }
    }

    /// Helper method for field `mpeg4_url`.
    ///
    /// A valid URL for the MPEG4 file
    #[must_use]
    pub fn mpeg4_url(&self) -> Option<&str> {
        match self {
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::mpeg4_url(val),
            _ => None,
        }
    }

    /// Helper method for field `mpeg4_width`.
    ///
    /// Video width
    #[must_use]
    pub fn mpeg4_width(&self) -> Option<i64> {
        match self {
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::mpeg4_width(val),
            _ => None,
        }
    }

    /// Helper method for field `parse_mode`.
    ///
    /// # Variants
    /// - `InlineQueryResultAudioKind`. Mode for parsing entities in the audio caption. See formatting options for more details.
    /// - `InlineQueryResultDocumentKind`. Mode for parsing entities in the document caption. See formatting options for more details.
    /// - `InlineQueryResultGifKind`, `InlineQueryResultMpeg4GifKind`. Mode for parsing entities in the caption. See formatting options for more details.
    /// - `InlineQueryResultPhotoKind`. Mode for parsing entities in the photo caption. See formatting options for more details.
    /// - `InlineQueryResultVideoKind`. Mode for parsing entities in the video caption. See formatting options for more details.
    /// - `InlineQueryResultVoiceKind`. Mode for parsing entities in the voice message caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::parse_mode(val),
            Self::Document(val) => crate::types::InlineQueryResultDocumentKind::parse_mode(val),
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::parse_mode(val),
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::parse_mode(val),
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::parse_mode(val),
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::parse_mode(val),
            Self::Voice(val) => crate::types::InlineQueryResultVoiceKind::parse_mode(val),
            _ => None,
        }
    }

    /// Helper method for field `performer`.
    ///
    /// Performer
    #[must_use]
    pub fn performer(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::performer(val),
            _ => None,
        }
    }

    /// Helper method for field `phone_number`.
    ///
    /// Contact's phone number
    #[must_use]
    pub fn phone_number(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => Some(val.phone_number.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `photo_file_id`.
    ///
    /// A valid file identifier of the photo
    #[must_use]
    pub fn photo_file_id(&self) -> Option<&str> {
        match self {
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::photo_file_id(val),
            _ => None,
        }
    }

    /// Helper method for field `photo_height`.
    ///
    /// Height of the photo
    #[must_use]
    pub fn photo_height(&self) -> Option<i64> {
        match self {
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::photo_height(val),
            _ => None,
        }
    }

    /// Helper method for field `photo_url`.
    ///
    /// A valid URL of the photo. Photo must be in JPEG format. Photo size must not exceed 5MB.
    #[must_use]
    pub fn photo_url(&self) -> Option<&str> {
        match self {
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::photo_url(val),
            _ => None,
        }
    }

    /// Helper method for field `photo_width`.
    ///
    /// Width of the photo
    #[must_use]
    pub fn photo_width(&self) -> Option<i64> {
        match self {
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::photo_width(val),
            _ => None,
        }
    }

    /// Helper method for field `proximity_alert_radius`.
    ///
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius(&self) -> Option<u32> {
        match self {
            Self::Location(val) => val.proximity_alert_radius,
            _ => None,
        }
    }

    /// Helper method for field `reply_markup`.
    ///
    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup(&self) -> Option<&crate::types::InlineKeyboardMarkup> {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::reply_markup(val),
            Self::Document(val) => crate::types::InlineQueryResultDocumentKind::reply_markup(val),
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::reply_markup(val),
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::reply_markup(val),
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::reply_markup(val),
            Self::Sticker(val) => val.reply_markup.as_ref(),
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::reply_markup(val),
            Self::Voice(val) => crate::types::InlineQueryResultVoiceKind::reply_markup(val),
            Self::Article(val) => val.reply_markup.as_ref(),
            Self::Contact(val) => val.reply_markup.as_ref(),
            Self::Game(val) => val.reply_markup.as_ref(),
            Self::Location(val) => val.reply_markup.as_ref(),
            Self::Venue(val) => val.reply_markup.as_ref(),
        }
    }

    /// Helper method for field `show_caption_above_media`.
    ///
    /// Pass `true` if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        match self {
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::show_caption_above_media(val),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::show_caption_above_media(val)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::show_caption_above_media(val)
            }
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::show_caption_above_media(val)
            }
            _ => None,
        }
    }

    /// Helper method for field `sticker_file_id`.
    ///
    /// A valid file identifier of the sticker
    #[must_use]
    pub fn sticker_file_id(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => Some(val.sticker_file_id.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `thumbnail_height`.
    ///
    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height(&self) -> Option<i64> {
        match self {
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::thumbnail_height(val)
            }
            Self::Article(val) => val.thumbnail_height,
            Self::Contact(val) => val.thumbnail_height,
            Self::Location(val) => val.thumbnail_height,
            Self::Venue(val) => val.thumbnail_height,
            _ => None,
        }
    }

    /// Helper method for field `thumbnail_mime_type`.
    ///
    /// MIME type of the thumbnail, must be one of `image/jpeg`, `image/gif`, or `video/mp4`. Defaults to `image/jpeg`.
    #[must_use]
    pub fn thumbnail_mime_type(&self) -> Option<&str> {
        match self {
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::thumbnail_mime_type(val),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::thumbnail_mime_type(val)
            }
            _ => None,
        }
    }

    /// Helper method for field `thumbnail_url`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocumentKind`. URL of the thumbnail (JPEG only) for the file
    /// - `InlineQueryResultGifKind`, `InlineQueryResultMpeg4GifKind`. URL of the static (JPEG or GIF) or animated (MPEG4) thumbnail for the result
    /// - `InlineQueryResultPhotoKind`. URL of the thumbnail for the photo
    /// - `InlineQueryResultVideoKind`. URL of the thumbnail (JPEG only) for the video
    /// - `InlineQueryResultArticle`, `InlineQueryResultContact`, `InlineQueryResultLocation`, `InlineQueryResultVenue`. Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url(&self) -> Option<&str> {
        match self {
            Self::Document(val) => crate::types::InlineQueryResultDocumentKind::thumbnail_url(val),
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::thumbnail_url(val),
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::thumbnail_url(val),
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::thumbnail_url(val),
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::thumbnail_url(val),
            Self::Article(val) => val.thumbnail_url.as_deref(),
            Self::Contact(val) => val.thumbnail_url.as_deref(),
            Self::Location(val) => val.thumbnail_url.as_deref(),
            Self::Venue(val) => val.thumbnail_url.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `thumbnail_width`.
    ///
    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width(&self) -> Option<i64> {
        match self {
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::thumbnail_width(val)
            }
            Self::Article(val) => val.thumbnail_width,
            Self::Contact(val) => val.thumbnail_width,
            Self::Location(val) => val.thumbnail_width,
            Self::Venue(val) => val.thumbnail_width,
            _ => None,
        }
    }

    /// Helper method for field `title`.
    ///
    /// # Variants
    /// - `InlineQueryResultAudioKind`. Title
    /// - `InlineQueryResultDocumentKind`, `InlineQueryResultVideoKind`. Title for the result
    /// - `InlineQueryResultGifKind`, `InlineQueryResultMpeg4GifKind`, `InlineQueryResultPhotoKind`. Title for the result
    /// - `InlineQueryResultVoiceKind`. Recording title
    /// - `InlineQueryResultArticle`. Title of the result
    /// - `InlineQueryResultLocation`. Location title
    /// - `InlineQueryResultVenue`. Title of the venue
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => crate::types::InlineQueryResultAudioKind::title(val),
            Self::Document(val) => Some(crate::types::InlineQueryResultDocumentKind::title(val)),
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::title(val),
            Self::Mpeg4Gif(val) => crate::types::InlineQueryResultMpeg4GifKind::title(val),
            Self::Photo(val) => crate::types::InlineQueryResultPhotoKind::title(val),
            Self::Video(val) => Some(crate::types::InlineQueryResultVideoKind::title(val)),
            Self::Voice(val) => Some(crate::types::InlineQueryResultVoiceKind::title(val)),
            Self::Article(val) => Some(val.title.as_ref()),
            Self::Location(val) => Some(val.title.as_ref()),
            Self::Venue(val) => Some(val.title.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `url`.
    ///
    /// URL of the result
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Article(val) => val.url.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `vcard`.
    ///
    /// Additional data about the contact in the form of a `vCard`, 0-2048 bytes
    #[must_use]
    pub fn vcard(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => val.vcard.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `video_duration`.
    ///
    /// Video duration in seconds
    #[must_use]
    pub fn video_duration(&self) -> Option<i64> {
        match self {
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::video_duration(val),
            _ => None,
        }
    }

    /// Helper method for field `video_file_id`.
    ///
    /// A valid file identifier for the video file
    #[must_use]
    pub fn video_file_id(&self) -> Option<&str> {
        match self {
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::video_file_id(val),
            _ => None,
        }
    }

    /// Helper method for field `video_height`.
    ///
    /// Video height
    #[must_use]
    pub fn video_height(&self) -> Option<i64> {
        match self {
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::video_height(val),
            _ => None,
        }
    }

    /// Helper method for field `video_url`.
    ///
    /// A valid URL for the embedded video player or video file
    #[must_use]
    pub fn video_url(&self) -> Option<&str> {
        match self {
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::video_url(val),
            _ => None,
        }
    }

    /// Helper method for field `video_width`.
    ///
    /// Video width
    #[must_use]
    pub fn video_width(&self) -> Option<i64> {
        match self {
            Self::Video(val) => crate::types::InlineQueryResultVideoKind::video_width(val),
            _ => None,
        }
    }

    /// Helper method for field `voice_duration`.
    ///
    /// Recording duration in seconds
    #[must_use]
    pub fn voice_duration(&self) -> Option<i64> {
        match self {
            Self::Voice(val) => crate::types::InlineQueryResultVoiceKind::voice_duration(val),
            _ => None,
        }
    }

    /// Helper method for field `voice_file_id`.
    ///
    /// A valid file identifier for the voice message
    #[must_use]
    pub fn voice_file_id(&self) -> Option<&str> {
        match self {
            Self::Voice(val) => crate::types::InlineQueryResultVoiceKind::voice_file_id(val),
            _ => None,
        }
    }

    /// Helper method for field `voice_url`.
    ///
    /// A valid URL for the voice recording
    #[must_use]
    pub fn voice_url(&self) -> Option<&str> {
        match self {
            Self::Voice(val) => crate::types::InlineQueryResultVoiceKind::voice_url(val),
            _ => None,
        }
    }

    /// Helper method for nested field `currency`.
    #[must_use]
    pub fn currency(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::currency)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::currency)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::currency),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::currency)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::currency)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::currency),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::currency)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::currency)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::currency(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::currency),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::currency),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::currency),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `entities`.
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::entities)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::entities)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::entities),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::entities)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::entities)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::entities),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::entities)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::entities)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::entities(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::entities),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::entities),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::entities),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `inline_keyboard`.
    #[must_use]
    pub fn inline_keyboard(&self) -> Option<&[Box<[crate::types::InlineKeyboardButton]>]> {
        self.reply_markup()
            .map(|inner| inner.inline_keyboard.as_ref())
    }

    /// Helper method for nested field `is_flexible`.
    #[must_use]
    pub fn is_flexible(&self) -> Option<bool> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::is_flexible)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::is_flexible)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::is_flexible),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::is_flexible)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::is_flexible)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::is_flexible),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::is_flexible)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::is_flexible)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::is_flexible(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::is_flexible),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::is_flexible),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::is_flexible),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `link_preview_options`.
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::link_preview_options)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::link_preview_options)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::link_preview_options),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::link_preview_options)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::link_preview_options)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::link_preview_options),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::link_preview_options)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::link_preview_options)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::link_preview_options(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::link_preview_options),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::link_preview_options),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::link_preview_options),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `max_tip_amount`.
    #[must_use]
    pub fn max_tip_amount(&self) -> Option<i64> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::max_tip_amount)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::max_tip_amount)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::max_tip_amount),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::max_tip_amount)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::max_tip_amount)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::max_tip_amount),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::max_tip_amount)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::max_tip_amount)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::max_tip_amount(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::max_tip_amount),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::max_tip_amount),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::max_tip_amount),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `message_text`.
    #[must_use]
    pub fn message_text(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::message_text)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::message_text)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::message_text),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::message_text)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::message_text)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::message_text),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::message_text)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::message_text)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::message_text(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::message_text),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::message_text),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::message_text),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `need_email`.
    #[must_use]
    pub fn need_email(&self) -> Option<bool> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_email)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_email)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::need_email),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_email)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_email)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_email),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_email)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_email)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::need_email(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_email),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_email),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_email),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `need_name`.
    #[must_use]
    pub fn need_name(&self) -> Option<bool> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_name)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_name)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::need_name),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_name)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_name)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_name),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_name)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_name)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::need_name(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_name),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_name),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_name),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `need_phone_number`.
    #[must_use]
    pub fn need_phone_number(&self) -> Option<bool> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_phone_number)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_phone_number)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::need_phone_number),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_phone_number)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_phone_number)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_phone_number),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_phone_number)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_phone_number)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::need_phone_number(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_phone_number),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_phone_number),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_phone_number),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `need_shipping_address`.
    #[must_use]
    pub fn need_shipping_address(&self) -> Option<bool> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_shipping_address)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_shipping_address)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::need_shipping_address),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_shipping_address)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_shipping_address)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_shipping_address),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_shipping_address)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::need_shipping_address)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::need_shipping_address(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_shipping_address),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_shipping_address),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_shipping_address),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `payload`.
    #[must_use]
    pub fn payload(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::payload)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::payload)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::payload),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::payload)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::payload)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::payload),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::payload)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::payload)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::payload(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::payload),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::payload),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::payload),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `photo_size`.
    #[must_use]
    pub fn photo_size(&self) -> Option<i64> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::photo_size)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::photo_size)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::photo_size),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::photo_size)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::photo_size)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_size),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::photo_size)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::photo_size)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::photo_size(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_size),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_size),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_size),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `prices`.
    #[must_use]
    pub fn prices(&self) -> Option<&[crate::types::LabeledPrice]> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::prices)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::prices)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::prices),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::prices)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::prices)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::prices),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::prices)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::prices)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::prices(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::prices),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::prices),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::prices),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `provider_data`.
    #[must_use]
    pub fn provider_data(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_data)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_data)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::provider_data),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_data)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_data)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_data),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_data)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_data)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::provider_data(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_data),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_data),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_data),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `provider_token`.
    #[must_use]
    pub fn provider_token(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_token)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_token)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::provider_token),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_token)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_token)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_token),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_token)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::provider_token)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::provider_token(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_token),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_token),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_token),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `rich_message`.
    #[must_use]
    pub fn rich_message(&self) -> Option<&crate::types::InputRichMessage> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::rich_message)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::rich_message)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::rich_message),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::rich_message)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::rich_message)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::rich_message),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::rich_message)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::rich_message)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::rich_message(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::rich_message),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::rich_message),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::rich_message),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `send_email_to_provider`.
    #[must_use]
    pub fn send_email_to_provider(&self) -> Option<bool> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_email_to_provider)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_email_to_provider)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::send_email_to_provider),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_email_to_provider)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_email_to_provider)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_email_to_provider),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_email_to_provider)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_email_to_provider)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::send_email_to_provider(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_email_to_provider),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_email_to_provider),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_email_to_provider),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `send_phone_number_to_provider`.
    #[must_use]
    pub fn send_phone_number_to_provider(&self) -> Option<bool> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_phone_number_to_provider)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_phone_number_to_provider)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::send_phone_number_to_provider),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_phone_number_to_provider)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_phone_number_to_provider)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_phone_number_to_provider),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_phone_number_to_provider)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::send_phone_number_to_provider)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::send_phone_number_to_provider(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_phone_number_to_provider),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_phone_number_to_provider),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_phone_number_to_provider),
            Self::Game(_) => None,
        }
    }

    /// Helper method for nested field `suggested_tip_amounts`.
    #[must_use]
    pub fn suggested_tip_amounts(&self) -> Option<&[i64]> {
        match self {
            Self::Audio(val) => {
                crate::types::InlineQueryResultAudioKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::suggested_tip_amounts)
            }
            Self::Document(val) => {
                crate::types::InlineQueryResultDocumentKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::suggested_tip_amounts)
            }
            Self::Gif(val) => crate::types::InlineQueryResultGifKind::input_message_content(val)
                .and_then(crate::types::InputMessageContent::suggested_tip_amounts),
            Self::Mpeg4Gif(val) => {
                crate::types::InlineQueryResultMpeg4GifKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::suggested_tip_amounts)
            }
            Self::Photo(val) => {
                crate::types::InlineQueryResultPhotoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::suggested_tip_amounts)
            }
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::suggested_tip_amounts),
            Self::Video(val) => {
                crate::types::InlineQueryResultVideoKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::suggested_tip_amounts)
            }
            Self::Voice(val) => {
                crate::types::InlineQueryResultVoiceKind::input_message_content(val)
                    .and_then(crate::types::InputMessageContent::suggested_tip_amounts)
            }
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::suggested_tip_amounts(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::suggested_tip_amounts),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::suggested_tip_amounts),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::suggested_tip_amounts),
            Self::Game(_) => None,
        }
    }
}
impl From<crate::types::InlineQueryResultAudioKind> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultAudioKind) -> Self {
        Self::Audio(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultAudioKind {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Audio(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultAudioKind),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultDocumentKind> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultDocumentKind) -> Self {
        Self::Document(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultDocumentKind {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Document(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultDocumentKind),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultGifKind> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultGifKind) -> Self {
        Self::Gif(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultGifKind {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Gif(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultGifKind),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultMpeg4GifKind> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultMpeg4GifKind) -> Self {
        Self::Mpeg4Gif(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultMpeg4GifKind {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Mpeg4Gif(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultMpeg4GifKind),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultPhotoKind> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultPhotoKind) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultPhotoKind {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Photo(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultPhotoKind),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultCachedSticker> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultCachedSticker) -> Self {
        Self::Sticker(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultCachedSticker {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Sticker(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultCachedSticker),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultVideoKind> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultVideoKind) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultVideoKind {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Video(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultVideoKind),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultVoiceKind> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultVoiceKind) -> Self {
        Self::Voice(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultVoiceKind {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Voice(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultVoiceKind),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultArticle> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultArticle) -> Self {
        Self::Article(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultArticle {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Article(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultArticle),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultContact> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultContact) -> Self {
        Self::Contact(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultContact {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Contact(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultContact),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultGame> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultGame) -> Self {
        Self::Game(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultGame {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Game(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultGame),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultLocation> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultLocation) -> Self {
        Self::Location(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultLocation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Location(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultLocation),
            ))
        }
    }
}
impl From<crate::types::InlineQueryResultVenue> for InlineQueryResult {
    fn from(val: crate::types::InlineQueryResultVenue) -> Self {
        Self::Venue(val)
    }
}
impl TryFrom<InlineQueryResult> for crate::types::InlineQueryResultVenue {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResult) -> Result<Self, Self::Error> {
        if let InlineQueryResult::Venue(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InlineQueryResult),
                stringify!(InlineQueryResultVenue),
            ))
        }
    }
}
