use serde::{Deserialize, Serialize};
/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
/// - [`InlineQueryResultCachedAudio`]
/// - [`InlineQueryResultCachedDocument`]
/// - [`InlineQueryResultCachedGif`]
/// - [`InlineQueryResultCachedMpeg4Gif`]
/// - [`InlineQueryResultCachedPhoto`]
/// - [`InlineQueryResultCachedSticker`]
/// - [`InlineQueryResultCachedVideo`]
/// - [`InlineQueryResultCachedVoice`]
/// - [`InlineQueryResultArticle`]
/// - [`InlineQueryResultAudio`]
/// - [`InlineQueryResultContact`]
/// - [`InlineQueryResultGame`]
/// - [`InlineQueryResultDocument`]
/// - [`InlineQueryResultGif`]
/// - [`InlineQueryResultLocation`]
/// - [`InlineQueryResultMpeg4Gif`]
/// - [`InlineQueryResultPhoto`]
/// - [`InlineQueryResultVenue`]
/// - [`InlineQueryResultVideo`]
/// - [`InlineQueryResultVoice`]
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
    /// # Variants
    /// - `InlineQueryResultVenue`. Address of the venue
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => Some(val.address.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `description`.
    ///
    /// # Variants
    /// - `InlineQueryResultArticle`. Short description of the result
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Article(val) => val.description.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `first_name`.
    ///
    /// # Variants
    /// - `InlineQueryResultContact`. Contact's first name
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => Some(val.first_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `foursquare_id`.
    ///
    /// # Variants
    /// - `InlineQueryResultVenue`. Foursquare identifier of the venue if known
    #[must_use]
    pub fn foursquare_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => val.foursquare_id.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `foursquare_type`.
    ///
    /// # Variants
    /// - `InlineQueryResultVenue`. Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[must_use]
    pub fn foursquare_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => val.foursquare_type.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `game_short_name`.
    ///
    /// # Variants
    /// - `InlineQueryResultGame`. Short name of the game
    #[must_use]
    pub fn game_short_name(&self) -> Option<&str> {
        match self {
            Self::Game(val) => Some(val.game_short_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `google_place_id`.
    ///
    /// # Variants
    /// - `InlineQueryResultVenue`. Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => val.google_place_id.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `google_place_type`.
    ///
    /// # Variants
    /// - `InlineQueryResultVenue`. Google Places type of the venue. (See supported types.)
    #[must_use]
    pub fn google_place_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => val.google_place_type.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `heading`.
    ///
    /// # Variants
    /// - `InlineQueryResultLocation`. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading(&self) -> Option<u16> {
        match self {
            Self::Location(val) => val.heading,
            _ => None,
        }
    }

    /// Helper method for field `horizontal_accuracy`.
    ///
    /// # Variants
    /// - `InlineQueryResultLocation`. The radius of uncertainty for the location, measured in meters; 0-1500
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
    /// - `InlineQueryResultCachedSticker`. Unique identifier for this result, 1-64 bytes
    /// - `InlineQueryResultArticle`. Unique identifier for this result, 1-64 Bytes
    /// - `InlineQueryResultContact`. Unique identifier for this result, 1-64 Bytes
    /// - `InlineQueryResultGame`. Unique identifier for this result, 1-64 bytes
    /// - `InlineQueryResultLocation`. Unique identifier for this result, 1-64 Bytes
    /// - `InlineQueryResultVenue`. Unique identifier for this result, 1-64 Bytes
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => Some(val.id.as_ref()),
            Self::Article(val) => Some(val.id.as_ref()),
            Self::Contact(val) => Some(val.id.as_ref()),
            Self::Game(val) => Some(val.id.as_ref()),
            Self::Location(val) => Some(val.id.as_ref()),
            Self::Venue(val) => Some(val.id.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `input_message_content`.
    ///
    /// # Variants
    /// - `InlineQueryResultCachedSticker`. Content of the message to be sent instead of the sticker
    /// - `InlineQueryResultArticle`. Content of the message to be sent
    /// - `InlineQueryResultContact`. Content of the message to be sent instead of the contact
    /// - `InlineQueryResultLocation`. Content of the message to be sent instead of the location
    /// - `InlineQueryResultVenue`. Content of the message to be sent instead of the venue
    #[must_use]
    pub fn input_message_content(&self) -> Option<&crate::types::InputMessageContent> {
        match self {
            Self::Sticker(val) => val.input_message_content.as_ref(),
            Self::Article(val) => Some(&val.input_message_content),
            Self::Contact(val) => val.input_message_content.as_ref(),
            Self::Location(val) => val.input_message_content.as_ref(),
            Self::Venue(val) => val.input_message_content.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `last_name`.
    ///
    /// # Variants
    /// - `InlineQueryResultContact`. Contact's last name
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
    /// # Variants
    /// - `InlineQueryResultLocation`. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
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

    /// Helper method for field `phone_number`.
    ///
    /// # Variants
    /// - `InlineQueryResultContact`. Contact's phone number
    #[must_use]
    pub fn phone_number(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => Some(val.phone_number.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `proximity_alert_radius`.
    ///
    /// # Variants
    /// - `InlineQueryResultLocation`. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius(&self) -> Option<u32> {
        match self {
            Self::Location(val) => val.proximity_alert_radius,
            _ => None,
        }
    }

    /// Helper method for field `reply_markup`.
    ///
    /// # Variants
    /// - `InlineQueryResultCachedSticker`. Inline keyboard attached to the message
    /// - `InlineQueryResultArticle`. Inline keyboard attached to the message
    /// - `InlineQueryResultContact`. Inline keyboard attached to the message
    /// - `InlineQueryResultGame`. Inline keyboard attached to the message
    /// - `InlineQueryResultLocation`. Inline keyboard attached to the message
    /// - `InlineQueryResultVenue`. Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup(&self) -> Option<&crate::types::InlineKeyboardMarkup> {
        match self {
            Self::Sticker(val) => val.reply_markup.as_ref(),
            Self::Article(val) => val.reply_markup.as_ref(),
            Self::Contact(val) => val.reply_markup.as_ref(),
            Self::Game(val) => val.reply_markup.as_ref(),
            Self::Location(val) => val.reply_markup.as_ref(),
            Self::Venue(val) => val.reply_markup.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `sticker_file_id`.
    ///
    /// # Variants
    /// - `InlineQueryResultCachedSticker`. A valid file identifier of the sticker
    #[must_use]
    pub fn sticker_file_id(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => Some(val.sticker_file_id.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `thumbnail_height`.
    ///
    /// # Variants
    /// - `InlineQueryResultArticle`. Thumbnail height
    /// - `InlineQueryResultContact`. Thumbnail height
    /// - `InlineQueryResultLocation`. Thumbnail height
    /// - `InlineQueryResultVenue`. Thumbnail height
    #[must_use]
    pub fn thumbnail_height(&self) -> Option<i64> {
        match self {
            Self::Article(val) => val.thumbnail_height,
            Self::Contact(val) => val.thumbnail_height,
            Self::Location(val) => val.thumbnail_height,
            Self::Venue(val) => val.thumbnail_height,
            _ => None,
        }
    }

    /// Helper method for field `thumbnail_url`.
    ///
    /// # Variants
    /// - `InlineQueryResultArticle`. Url of the thumbnail for the result
    /// - `InlineQueryResultContact`. Url of the thumbnail for the result
    /// - `InlineQueryResultLocation`. Url of the thumbnail for the result
    /// - `InlineQueryResultVenue`. Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url(&self) -> Option<&str> {
        match self {
            Self::Article(val) => val.thumbnail_url.as_deref(),
            Self::Contact(val) => val.thumbnail_url.as_deref(),
            Self::Location(val) => val.thumbnail_url.as_deref(),
            Self::Venue(val) => val.thumbnail_url.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `thumbnail_width`.
    ///
    /// # Variants
    /// - `InlineQueryResultArticle`. Thumbnail width
    /// - `InlineQueryResultContact`. Thumbnail width
    /// - `InlineQueryResultLocation`. Thumbnail width
    /// - `InlineQueryResultVenue`. Thumbnail width
    #[must_use]
    pub fn thumbnail_width(&self) -> Option<i64> {
        match self {
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
    /// - `InlineQueryResultArticle`. Title of the result
    /// - `InlineQueryResultLocation`. Location title
    /// - `InlineQueryResultVenue`. Title of the venue
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Article(val) => Some(val.title.as_ref()),
            Self::Location(val) => Some(val.title.as_ref()),
            Self::Venue(val) => Some(val.title.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `url`.
    ///
    /// # Variants
    /// - `InlineQueryResultArticle`. URL of the result
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Article(val) => val.url.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `vcard`.
    ///
    /// # Variants
    /// - `InlineQueryResultContact`. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[must_use]
    pub fn vcard(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => val.vcard.as_deref(),
            _ => None,
        }
    }

    /// Helper method for nested field `currency`.
    #[must_use]
    pub fn currency(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::currency),
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
            _ => None,
        }
    }

    /// Helper method for nested field `entities`.
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::entities),
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
            _ => None,
        }
    }

    /// Helper method for nested field `inline_keyboard`.
    #[must_use]
    pub fn inline_keyboard(&self) -> Option<&[Box<[crate::types::InlineKeyboardButton]>]> {
        match self {
            Self::Sticker(val) => val
                .reply_markup
                .as_ref()
                .map(|inner| inner.inline_keyboard.as_ref()),
            Self::Article(val) => val
                .reply_markup
                .as_ref()
                .map(|inner| inner.inline_keyboard.as_ref()),
            Self::Contact(val) => val
                .reply_markup
                .as_ref()
                .map(|inner| inner.inline_keyboard.as_ref()),
            Self::Game(val) => val
                .reply_markup
                .as_ref()
                .map(|inner| inner.inline_keyboard.as_ref()),
            Self::Location(val) => val
                .reply_markup
                .as_ref()
                .map(|inner| inner.inline_keyboard.as_ref()),
            Self::Venue(val) => val
                .reply_markup
                .as_ref()
                .map(|inner| inner.inline_keyboard.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `is_flexible`.
    #[must_use]
    pub fn is_flexible(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::is_flexible),
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
            _ => None,
        }
    }

    /// Helper method for nested field `link_preview_options`.
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::link_preview_options),
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
            _ => None,
        }
    }

    /// Helper method for nested field `max_tip_amount`.
    #[must_use]
    pub fn max_tip_amount(&self) -> Option<i64> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::max_tip_amount),
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
            _ => None,
        }
    }

    /// Helper method for nested field `message_text`.
    #[must_use]
    pub fn message_text(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::message_text),
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
            _ => None,
        }
    }

    /// Helper method for nested field `need_email`.
    #[must_use]
    pub fn need_email(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_email),
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
            _ => None,
        }
    }

    /// Helper method for nested field `need_name`.
    #[must_use]
    pub fn need_name(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_name),
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
            _ => None,
        }
    }

    /// Helper method for nested field `need_phone_number`.
    #[must_use]
    pub fn need_phone_number(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_phone_number),
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
            _ => None,
        }
    }

    /// Helper method for nested field `need_shipping_address`.
    #[must_use]
    pub fn need_shipping_address(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::need_shipping_address),
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
            _ => None,
        }
    }

    /// Helper method for nested field `parse_mode`.
    #[must_use]
    pub fn parse_mode(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::parse_mode),
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::parse_mode(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::parse_mode),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::parse_mode),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::parse_mode),
            _ => None,
        }
    }

    /// Helper method for nested field `payload`.
    #[must_use]
    pub fn payload(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::payload),
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
            _ => None,
        }
    }

    /// Helper method for nested field `photo_height`.
    #[must_use]
    pub fn photo_height(&self) -> Option<i64> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_height),
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::photo_height(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_height),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_height),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_height),
            _ => None,
        }
    }

    /// Helper method for nested field `photo_size`.
    #[must_use]
    pub fn photo_size(&self) -> Option<i64> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_size),
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
            _ => None,
        }
    }

    /// Helper method for nested field `photo_url`.
    #[must_use]
    pub fn photo_url(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_url),
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::photo_url(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_url),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_url),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_url),
            _ => None,
        }
    }

    /// Helper method for nested field `photo_width`.
    #[must_use]
    pub fn photo_width(&self) -> Option<i64> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_width),
            Self::Article(val) => {
                let inner = &val.input_message_content;
                crate::types::InputMessageContent::photo_width(inner)
            }
            Self::Contact(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_width),
            Self::Location(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_width),
            Self::Venue(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::photo_width),
            _ => None,
        }
    }

    /// Helper method for nested field `prices`.
    #[must_use]
    pub fn prices(&self) -> Option<&[crate::types::LabeledPrice]> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::prices),
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
            _ => None,
        }
    }

    /// Helper method for nested field `provider_data`.
    #[must_use]
    pub fn provider_data(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_data),
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
            _ => None,
        }
    }

    /// Helper method for nested field `provider_token`.
    #[must_use]
    pub fn provider_token(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::provider_token),
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
            _ => None,
        }
    }

    /// Helper method for nested field `send_email_to_provider`.
    #[must_use]
    pub fn send_email_to_provider(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_email_to_provider),
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
            _ => None,
        }
    }

    /// Helper method for nested field `send_phone_number_to_provider`.
    #[must_use]
    pub fn send_phone_number_to_provider(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::send_phone_number_to_provider),
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
            _ => None,
        }
    }

    /// Helper method for nested field `suggested_tip_amounts`.
    #[must_use]
    pub fn suggested_tip_amounts(&self) -> Option<&[i64]> {
        match self {
            Self::Sticker(val) => val
                .input_message_content
                .as_ref()
                .and_then(crate::types::InputMessageContent::suggested_tip_amounts),
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
            _ => None,
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
