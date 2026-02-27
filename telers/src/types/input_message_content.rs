use serde::{Deserialize, Serialize};
/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
/// - [`InputTextMessageContent`]
/// - [`InputLocationMessageContent`]
/// - [`InputVenueMessageContent`]
/// - [`InputContactMessageContent`]
/// - [`InputInvoiceMessageContent`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmessagecontent>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    InputInvoiceMessageContent(crate::types::InputInvoiceMessageContent),
    InputVenueMessageContent(crate::types::InputVenueMessageContent),
    InputLocationMessageContent(crate::types::InputLocationMessageContent),
    InputContactMessageContent(crate::types::InputContactMessageContent),
    InputTextMessageContent(crate::types::InputTextMessageContent),
}
impl InputMessageContent {
    /// Helper method for field `address`.
    ///
    /// # Variants
    /// - `InputVenueMessageContent`. Address of the venue
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        match self {
            Self::InputVenueMessageContent(val) => Some(val.address.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `currency`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Three-letter ISO 4217 currency code, see more on currencies. Pass `XTR` for payments in Telegram Stars.
    #[must_use]
    pub fn currency(&self) -> Option<&str> {
        match self {
            Self::InputInvoiceMessageContent(val) => Some(val.currency.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `description`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Product description, 1-255 characters
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::InputInvoiceMessageContent(val) => Some(val.description.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `entities`.
    ///
    /// # Variants
    /// - `InputTextMessageContent`. List of special entities that appear in message text, which can be specified instead of `parse_mode`
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::InputTextMessageContent(val) => val.entities.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `first_name`.
    ///
    /// # Variants
    /// - `InputContactMessageContent`. Contact's first name
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::InputContactMessageContent(val) => Some(val.first_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `foursquare_id`.
    ///
    /// # Variants
    /// - `InputVenueMessageContent`. Foursquare identifier of the venue, if known
    #[must_use]
    pub fn foursquare_id(&self) -> Option<&str> {
        match self {
            Self::InputVenueMessageContent(val) => val.foursquare_id.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `foursquare_type`.
    ///
    /// # Variants
    /// - `InputVenueMessageContent`. Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[must_use]
    pub fn foursquare_type(&self) -> Option<&str> {
        match self {
            Self::InputVenueMessageContent(val) => val.foursquare_type.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `google_place_id`.
    ///
    /// # Variants
    /// - `InputVenueMessageContent`. Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id(&self) -> Option<&str> {
        match self {
            Self::InputVenueMessageContent(val) => val.google_place_id.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `google_place_type`.
    ///
    /// # Variants
    /// - `InputVenueMessageContent`. Google Places type of the venue. (See supported types.)
    #[must_use]
    pub fn google_place_type(&self) -> Option<&str> {
        match self {
            Self::InputVenueMessageContent(val) => val.google_place_type.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `heading`.
    ///
    /// # Variants
    /// - `InputLocationMessageContent`. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading(&self) -> Option<u16> {
        match self {
            Self::InputLocationMessageContent(val) => val.heading,
            _ => None,
        }
    }

    /// Helper method for field `horizontal_accuracy`.
    ///
    /// # Variants
    /// - `InputLocationMessageContent`. The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy(&self) -> Option<f64> {
        match self {
            Self::InputLocationMessageContent(val) => val.horizontal_accuracy,
            _ => None,
        }
    }

    /// Helper method for field `is_flexible`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Pass `true` if the final price depends on the shipping method. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn is_flexible(&self) -> Option<bool> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.is_flexible,
            _ => None,
        }
    }

    /// Helper method for field `last_name`.
    ///
    /// # Variants
    /// - `InputContactMessageContent`. Contact's last name
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::InputContactMessageContent(val) => val.last_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `latitude`.
    ///
    /// # Variants
    /// - `InputVenueMessageContent`. Latitude of the venue in degrees
    /// - `InputLocationMessageContent`. Latitude of the location in degrees
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        match self {
            Self::InputVenueMessageContent(val) => Some(val.latitude),
            Self::InputLocationMessageContent(val) => Some(val.latitude),
            _ => None,
        }
    }

    /// Helper method for field `link_preview_options`.
    ///
    /// # Variants
    /// - `InputTextMessageContent`. Link preview generation options for the message
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        match self {
            Self::InputTextMessageContent(val) => val.link_preview_options.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `live_period`.
    ///
    /// # Variants
    /// - `InputLocationMessageContent`. Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[must_use]
    pub fn live_period(&self) -> Option<u32> {
        match self {
            Self::InputLocationMessageContent(val) => val.live_period,
            _ => None,
        }
    }

    /// Helper method for field `longitude`.
    ///
    /// # Variants
    /// - `InputVenueMessageContent`. Longitude of the venue in degrees
    /// - `InputLocationMessageContent`. Longitude of the location in degrees
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        match self {
            Self::InputVenueMessageContent(val) => Some(val.longitude),
            Self::InputLocationMessageContent(val) => Some(val.longitude),
            _ => None,
        }
    }

    /// Helper method for field `max_tip_amount`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. The maximum accepted amount for tips in the smallest units of the currency (integer, not float/double). For example, for a maximum tip of US$ 1.45 pass `max_tip_amount` = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies). Defaults to 0. Not supported for payments in Telegram Stars.
    #[must_use]
    pub fn max_tip_amount(&self) -> Option<i64> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.max_tip_amount,
            _ => None,
        }
    }

    /// Helper method for field `message_text`.
    ///
    /// # Variants
    /// - `InputTextMessageContent`. Text of the message to be sent, 1-4096 characters
    #[must_use]
    pub fn message_text(&self) -> Option<&str> {
        match self {
            Self::InputTextMessageContent(val) => Some(val.message_text.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `need_email`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Pass `true` if you require the user's email address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_email(&self) -> Option<bool> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.need_email,
            _ => None,
        }
    }

    /// Helper method for field `need_name`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Pass `true` if you require the user's full name to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_name(&self) -> Option<bool> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.need_name,
            _ => None,
        }
    }

    /// Helper method for field `need_phone_number`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Pass `true` if you require the user's phone number to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_phone_number(&self) -> Option<bool> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.need_phone_number,
            _ => None,
        }
    }

    /// Helper method for field `need_shipping_address`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Pass `true` if you require the user's shipping address to complete the order. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn need_shipping_address(&self) -> Option<bool> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.need_shipping_address,
            _ => None,
        }
    }

    /// Helper method for field `parse_mode`.
    ///
    /// # Variants
    /// - `InputTextMessageContent`. Mode for parsing entities in the message text. See formatting options for more details.
    #[must_use]
    pub fn parse_mode(&self) -> Option<&str> {
        match self {
            Self::InputTextMessageContent(val) => val.parse_mode.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `payload`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Bot-defined invoice payload, 1-128 bytes. This will not be displayed to the user, use it for your internal processes.
    #[must_use]
    pub fn payload(&self) -> Option<&str> {
        match self {
            Self::InputInvoiceMessageContent(val) => Some(val.payload.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `phone_number`.
    ///
    /// # Variants
    /// - `InputContactMessageContent`. Contact's phone number
    #[must_use]
    pub fn phone_number(&self) -> Option<&str> {
        match self {
            Self::InputContactMessageContent(val) => Some(val.phone_number.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `photo_height`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Photo height
    #[must_use]
    pub fn photo_height(&self) -> Option<i64> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.photo_height,
            _ => None,
        }
    }

    /// Helper method for field `photo_size`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Photo size in bytes
    #[must_use]
    pub fn photo_size(&self) -> Option<i64> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.photo_size,
            _ => None,
        }
    }

    /// Helper method for field `photo_url`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. URL of the product photo for the invoice. Can be a photo of the goods or a marketing image for a service.
    #[must_use]
    pub fn photo_url(&self) -> Option<&str> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.photo_url.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `photo_width`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Photo width
    #[must_use]
    pub fn photo_width(&self) -> Option<i64> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.photo_width,
            _ => None,
        }
    }

    /// Helper method for field `prices`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Price breakdown, a JSON-serialized list of components (e.g. product price, tax, discount, delivery cost, delivery tax, bonus, etc.). Must contain exactly one item for payments in Telegram Stars.
    #[must_use]
    pub fn prices(&self) -> Option<&[crate::types::LabeledPrice]> {
        match self {
            Self::InputInvoiceMessageContent(val) => Some(val.prices.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `provider_data`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. A JSON-serialized object for data about the invoice, which will be shared with the payment provider. A detailed description of the required fields should be provided by the payment provider.
    #[must_use]
    pub fn provider_data(&self) -> Option<&str> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.provider_data.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `provider_token`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Payment provider token, obtained via @[`BotFather`]. Pass an empty string for payments in Telegram Stars.
    #[must_use]
    pub fn provider_token(&self) -> Option<&str> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.provider_token.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `proximity_alert_radius`.
    ///
    /// # Variants
    /// - `InputLocationMessageContent`. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius(&self) -> Option<u32> {
        match self {
            Self::InputLocationMessageContent(val) => val.proximity_alert_radius,
            _ => None,
        }
    }

    /// Helper method for field `send_email_to_provider`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Pass `true` if the user's email address should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_email_to_provider(&self) -> Option<bool> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.send_email_to_provider,
            _ => None,
        }
    }

    /// Helper method for field `send_phone_number_to_provider`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Pass `true` if the user's phone number should be sent to the provider. Ignored for payments in Telegram Stars.
    #[must_use]
    pub fn send_phone_number_to_provider(&self) -> Option<bool> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.send_phone_number_to_provider,
            _ => None,
        }
    }

    /// Helper method for field `suggested_tip_amounts`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. A JSON-serialized array of suggested amounts of tip in the smallest units of the currency (integer, not float/double). At most 4 suggested tip amounts can be specified. The suggested tip amounts must be positive, passed in a strictly increased order and must not exceed `max_tip_amount`.
    #[must_use]
    pub fn suggested_tip_amounts(&self) -> Option<&[i64]> {
        match self {
            Self::InputInvoiceMessageContent(val) => val.suggested_tip_amounts.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `title`.
    ///
    /// # Variants
    /// - `InputInvoiceMessageContent`. Product name, 1-32 characters
    /// - `InputVenueMessageContent`. Name of the venue
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::InputInvoiceMessageContent(val) => Some(val.title.as_ref()),
            Self::InputVenueMessageContent(val) => Some(val.title.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `vcard`.
    ///
    /// # Variants
    /// - `InputContactMessageContent`. Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[must_use]
    pub fn vcard(&self) -> Option<&str> {
        match self {
            Self::InputContactMessageContent(val) => val.vcard.as_deref(),
            _ => None,
        }
    }

    /// Helper method for nested field `is_disabled`.
    #[must_use]
    pub fn is_disabled(&self) -> Option<bool> {
        match self {
            Self::InputTextMessageContent(val) => val
                .link_preview_options
                .as_ref()
                .and_then(|inner| inner.is_disabled),
            _ => None,
        }
    }

    /// Helper method for nested field `prefer_large_media`.
    #[must_use]
    pub fn prefer_large_media(&self) -> Option<bool> {
        match self {
            Self::InputTextMessageContent(val) => val
                .link_preview_options
                .as_ref()
                .and_then(|inner| inner.prefer_large_media),
            _ => None,
        }
    }

    /// Helper method for nested field `prefer_small_media`.
    #[must_use]
    pub fn prefer_small_media(&self) -> Option<bool> {
        match self {
            Self::InputTextMessageContent(val) => val
                .link_preview_options
                .as_ref()
                .and_then(|inner| inner.prefer_small_media),
            _ => None,
        }
    }

    /// Helper method for nested field `show_above_text`.
    #[must_use]
    pub fn show_above_text(&self) -> Option<bool> {
        match self {
            Self::InputTextMessageContent(val) => val
                .link_preview_options
                .as_ref()
                .and_then(|inner| inner.show_above_text),
            _ => None,
        }
    }

    /// Helper method for nested field `url`.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::InputTextMessageContent(val) => val
                .link_preview_options
                .as_ref()
                .and_then(|inner| inner.url.as_deref()),
            _ => None,
        }
    }
}
impl From<crate::types::InputInvoiceMessageContent> for InputMessageContent {
    fn from(val: crate::types::InputInvoiceMessageContent) -> Self {
        Self::InputInvoiceMessageContent(val)
    }
}
impl TryFrom<InputMessageContent> for crate::types::InputInvoiceMessageContent {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMessageContent) -> Result<Self, Self::Error> {
        if let InputMessageContent::InputInvoiceMessageContent(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMessageContent),
                stringify!(InputInvoiceMessageContent),
            ))
        }
    }
}
impl From<crate::types::InputVenueMessageContent> for InputMessageContent {
    fn from(val: crate::types::InputVenueMessageContent) -> Self {
        Self::InputVenueMessageContent(val)
    }
}
impl TryFrom<InputMessageContent> for crate::types::InputVenueMessageContent {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMessageContent) -> Result<Self, Self::Error> {
        if let InputMessageContent::InputVenueMessageContent(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMessageContent),
                stringify!(InputVenueMessageContent),
            ))
        }
    }
}
impl From<crate::types::InputLocationMessageContent> for InputMessageContent {
    fn from(val: crate::types::InputLocationMessageContent) -> Self {
        Self::InputLocationMessageContent(val)
    }
}
impl TryFrom<InputMessageContent> for crate::types::InputLocationMessageContent {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMessageContent) -> Result<Self, Self::Error> {
        if let InputMessageContent::InputLocationMessageContent(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMessageContent),
                stringify!(InputLocationMessageContent),
            ))
        }
    }
}
impl From<crate::types::InputContactMessageContent> for InputMessageContent {
    fn from(val: crate::types::InputContactMessageContent) -> Self {
        Self::InputContactMessageContent(val)
    }
}
impl TryFrom<InputMessageContent> for crate::types::InputContactMessageContent {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMessageContent) -> Result<Self, Self::Error> {
        if let InputMessageContent::InputContactMessageContent(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMessageContent),
                stringify!(InputContactMessageContent),
            ))
        }
    }
}
impl From<crate::types::InputTextMessageContent> for InputMessageContent {
    fn from(val: crate::types::InputTextMessageContent) -> Self {
        Self::InputTextMessageContent(val)
    }
}
impl TryFrom<InputMessageContent> for crate::types::InputTextMessageContent {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: InputMessageContent) -> Result<Self, Self::Error> {
        if let InputMessageContent::InputTextMessageContent(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(InputMessageContent),
                stringify!(InputTextMessageContent),
            ))
        }
    }
}
