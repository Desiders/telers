use super::{
    InputContactMessageContent, InputInvoiceMessageContent, InputLocationMessageContent,
    InputTextMessageContent, InputVenueMessageContent,
};

use serde::{Deserialize, Serialize};

/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
/// - `aiogram_rs.types.input_text_message_content.InputTextMessageContent`
/// - `aiogram_rs.types.input_location_message_content.InputLocationMessageContent`
/// - `aiogram_rs.types.input_venue_message_content.InputVenueMessageContent`
/// - `aiogram_rs.types.input_contact_message_content.InputContactMessageContent`
/// - `aiogram_rs.types.input_invoice_message_content.InputInvoiceMessageContent`
/// <https://core.telegram.org/bots/api#inputmessagecontent>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}

impl From<InputTextMessageContent> for InputMessageContent {
    fn from(val: InputTextMessageContent) -> Self {
        Self::Text(val)
    }
}

impl From<InputLocationMessageContent> for InputMessageContent {
    fn from(val: InputLocationMessageContent) -> Self {
        Self::Location(val)
    }
}

impl From<InputVenueMessageContent> for InputMessageContent {
    fn from(val: InputVenueMessageContent) -> Self {
        Self::Venue(val)
    }
}

impl From<InputContactMessageContent> for InputMessageContent {
    fn from(val: InputContactMessageContent) -> Self {
        Self::Contact(val)
    }
}

impl From<InputInvoiceMessageContent> for InputMessageContent {
    fn from(val: InputInvoiceMessageContent) -> Self {
        Self::Invoice(val)
    }
}
