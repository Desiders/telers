use serde::{Deserialize, Serialize};
/// # Notes
/// This object represents an inline query result kind as combine of [`InlineQueryResultCachedDocument`] and [`InlineQueryResultDocument`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineQueryResultDocumentKind {
    Uncached(crate::types::InlineQueryResultDocument),
    Cached(crate::types::InlineQueryResultCachedDocument),
}
impl InlineQueryResultDocumentKind {
    /// Helper method for field `caption`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. Caption of the document to be sent, 0-1024 characters after entities parsing
    /// - `InlineQueryResultCachedDocument`. Caption of the document to be sent, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        match self {
            Self::Uncached(val) => val.caption.as_deref(),
            Self::Cached(val) => val.caption.as_deref(),
        }
    }

    /// Helper method for field `caption_entities`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    /// - `InlineQueryResultCachedDocument`. List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Uncached(val) => val.caption_entities.as_deref(),
            Self::Cached(val) => val.caption_entities.as_deref(),
        }
    }

    /// Helper method for field `description`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. Short description of the result
    /// - `InlineQueryResultCachedDocument`. Short description of the result
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Uncached(val) => val.description.as_deref(),
            Self::Cached(val) => val.description.as_deref(),
        }
    }

    /// Helper method for field `document_file_id`.
    ///
    /// # Variants
    /// - `InlineQueryResultCachedDocument`. A valid file identifier for the file
    #[must_use]
    pub fn document_file_id(&self) -> Option<&str> {
        match self {
            Self::Cached(val) => Some(val.document_file_id.as_ref()),
            Self::Uncached(_) => None,
        }
    }

    /// Helper method for field `document_url`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. A valid URL for the file
    #[must_use]
    pub fn document_url(&self) -> Option<&str> {
        match self {
            Self::Uncached(val) => Some(val.document_url.as_ref()),
            Self::Cached(_) => None,
        }
    }

    /// Helper method for field `id`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. Unique identifier for this result, 1-64 bytes
    /// - `InlineQueryResultCachedDocument`. Unique identifier for this result, 1-64 bytes
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Uncached(val) => val.id.as_ref(),
            Self::Cached(val) => val.id.as_ref(),
        }
    }

    /// Helper method for field `input_message_content`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. Content of the message to be sent instead of the file
    /// - `InlineQueryResultCachedDocument`. Content of the message to be sent instead of the file
    #[must_use]
    pub fn input_message_content(&self) -> Option<&crate::types::InputMessageContent> {
        match self {
            Self::Uncached(val) => val.input_message_content.as_ref(),
            Self::Cached(val) => val.input_message_content.as_ref(),
        }
    }

    /// Helper method for field `mime_type`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. MIME type of the content of the file, either `application/pdf` or `application/zip`
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::Uncached(val) => Some(val.mime_type.as_ref()),
            Self::Cached(_) => None,
        }
    }

    /// Helper method for field `parse_mode`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. Mode for parsing entities in the document caption. See formatting options for more details.
    /// - `InlineQueryResultCachedDocument`. Mode for parsing entities in the document caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode(&self) -> Option<&str> {
        match self {
            Self::Uncached(val) => val.parse_mode.as_deref(),
            Self::Cached(val) => val.parse_mode.as_deref(),
        }
    }

    /// Helper method for field `reply_markup`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. Inline keyboard attached to the message
    /// - `InlineQueryResultCachedDocument`. Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup(&self) -> Option<&crate::types::InlineKeyboardMarkup> {
        match self {
            Self::Uncached(val) => val.reply_markup.as_ref(),
            Self::Cached(val) => val.reply_markup.as_ref(),
        }
    }

    /// Helper method for field `thumbnail_height`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. Thumbnail height
    #[must_use]
    pub fn thumbnail_height(&self) -> Option<i64> {
        match self {
            Self::Uncached(val) => val.thumbnail_height,
            Self::Cached(_) => None,
        }
    }

    /// Helper method for field `thumbnail_url`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. URL of the thumbnail (JPEG only) for the file
    #[must_use]
    pub fn thumbnail_url(&self) -> Option<&str> {
        match self {
            Self::Uncached(val) => val.thumbnail_url.as_deref(),
            Self::Cached(_) => None,
        }
    }

    /// Helper method for field `thumbnail_width`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. Thumbnail width
    #[must_use]
    pub fn thumbnail_width(&self) -> Option<i64> {
        match self {
            Self::Uncached(val) => val.thumbnail_width,
            Self::Cached(_) => None,
        }
    }

    /// Helper method for field `title`.
    ///
    /// # Variants
    /// - `InlineQueryResultDocument`. Title for the result
    /// - `InlineQueryResultCachedDocument`. Title for the result
    #[must_use]
    pub fn title(&self) -> &str {
        match self {
            Self::Uncached(val) => val.title.as_ref(),
            Self::Cached(val) => val.title.as_ref(),
        }
    }

    /// Helper method for nested field `address`.
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::address)
    }

    /// Helper method for nested field `currency`.
    #[must_use]
    pub fn currency(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::currency)
    }

    /// Helper method for nested field `entities`.
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::entities)
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::first_name)
    }

    /// Helper method for nested field `foursquare_id`.
    #[must_use]
    pub fn foursquare_id(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::foursquare_id)
    }

    /// Helper method for nested field `foursquare_type`.
    #[must_use]
    pub fn foursquare_type(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::foursquare_type)
    }

    /// Helper method for nested field `google_place_id`.
    #[must_use]
    pub fn google_place_id(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::google_place_id)
    }

    /// Helper method for nested field `google_place_type`.
    #[must_use]
    pub fn google_place_type(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::google_place_type)
    }

    /// Helper method for nested field `heading`.
    #[must_use]
    pub fn heading(&self) -> Option<u16> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::heading)
    }

    /// Helper method for nested field `horizontal_accuracy`.
    #[must_use]
    pub fn horizontal_accuracy(&self) -> Option<f64> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::horizontal_accuracy)
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
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::is_flexible)
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::last_name)
    }

    /// Helper method for nested field `latitude`.
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::latitude)
    }

    /// Helper method for nested field `link_preview_options`.
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::link_preview_options)
    }

    /// Helper method for nested field `live_period`.
    #[must_use]
    pub fn live_period(&self) -> Option<u32> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::live_period)
    }

    /// Helper method for nested field `longitude`.
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::longitude)
    }

    /// Helper method for nested field `max_tip_amount`.
    #[must_use]
    pub fn max_tip_amount(&self) -> Option<i64> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::max_tip_amount)
    }

    /// Helper method for nested field `message_text`.
    #[must_use]
    pub fn message_text(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::message_text)
    }

    /// Helper method for nested field `need_email`.
    #[must_use]
    pub fn need_email(&self) -> Option<bool> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::need_email)
    }

    /// Helper method for nested field `need_name`.
    #[must_use]
    pub fn need_name(&self) -> Option<bool> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::need_name)
    }

    /// Helper method for nested field `need_phone_number`.
    #[must_use]
    pub fn need_phone_number(&self) -> Option<bool> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::need_phone_number)
    }

    /// Helper method for nested field `need_shipping_address`.
    #[must_use]
    pub fn need_shipping_address(&self) -> Option<bool> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::need_shipping_address)
    }

    /// Helper method for nested field `payload`.
    #[must_use]
    pub fn payload(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::payload)
    }

    /// Helper method for nested field `phone_number`.
    #[must_use]
    pub fn phone_number(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::phone_number)
    }

    /// Helper method for nested field `photo_height`.
    #[must_use]
    pub fn photo_height(&self) -> Option<i64> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::photo_height)
    }

    /// Helper method for nested field `photo_size`.
    #[must_use]
    pub fn photo_size(&self) -> Option<i64> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::photo_size)
    }

    /// Helper method for nested field `photo_url`.
    #[must_use]
    pub fn photo_url(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::photo_url)
    }

    /// Helper method for nested field `photo_width`.
    #[must_use]
    pub fn photo_width(&self) -> Option<i64> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::photo_width)
    }

    /// Helper method for nested field `prices`.
    #[must_use]
    pub fn prices(&self) -> Option<&[crate::types::LabeledPrice]> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::prices)
    }

    /// Helper method for nested field `provider_data`.
    #[must_use]
    pub fn provider_data(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::provider_data)
    }

    /// Helper method for nested field `provider_token`.
    #[must_use]
    pub fn provider_token(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::provider_token)
    }

    /// Helper method for nested field `proximity_alert_radius`.
    #[must_use]
    pub fn proximity_alert_radius(&self) -> Option<u32> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::proximity_alert_radius)
    }

    /// Helper method for nested field `send_email_to_provider`.
    #[must_use]
    pub fn send_email_to_provider(&self) -> Option<bool> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::send_email_to_provider)
    }

    /// Helper method for nested field `send_phone_number_to_provider`.
    #[must_use]
    pub fn send_phone_number_to_provider(&self) -> Option<bool> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::send_phone_number_to_provider)
    }

    /// Helper method for nested field `suggested_tip_amounts`.
    #[must_use]
    pub fn suggested_tip_amounts(&self) -> Option<&[i64]> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::suggested_tip_amounts)
    }

    /// Helper method for nested field `vcard`.
    #[must_use]
    pub fn vcard(&self) -> Option<&str> {
        self.input_message_content()
            .and_then(crate::types::InputMessageContent::vcard)
    }
}
impl From<crate::types::InlineQueryResultDocument> for InlineQueryResultDocumentKind {
    fn from(val: crate::types::InlineQueryResultDocument) -> Self {
        Self::Uncached(val)
    }
}
impl TryFrom<InlineQueryResultDocumentKind> for crate::types::InlineQueryResultDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResultDocumentKind) -> Result<Self, Self::Error> {
        match val {
            InlineQueryResultDocumentKind::Uncached(inner) => Ok(inner),
            InlineQueryResultDocumentKind::Cached(_) => Err(Self::Error::new(
                stringify!(InlineQueryResultDocumentKind),
                stringify!(InlineQueryResultDocument),
            )),
        }
    }
}
impl From<crate::types::InlineQueryResultCachedDocument> for InlineQueryResultDocumentKind {
    fn from(val: crate::types::InlineQueryResultCachedDocument) -> Self {
        Self::Cached(val)
    }
}
impl TryFrom<InlineQueryResultDocumentKind> for crate::types::InlineQueryResultCachedDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InlineQueryResultDocumentKind) -> Result<Self, Self::Error> {
        match val {
            InlineQueryResultDocumentKind::Cached(inner) => Ok(inner),
            InlineQueryResultDocumentKind::Uncached(_) => Err(Self::Error::new(
                stringify!(InlineQueryResultDocumentKind),
                stringify!(InlineQueryResultCachedDocument),
            )),
        }
    }
}
