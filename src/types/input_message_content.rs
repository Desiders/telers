use super::{
    InputContactMessageContent, InputInvoiceMessageContent, InputLocationMessageContent,
    InputTextMessageContent, InputVenueMessageContent,
};

use serde::{Deserialize, Serialize};

/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
/// - :class:`aiogram_rs.types.input_text_message_content.InputTextMessageContent`
/// - :class:`aiogram_rs.types.input_location_message_content.InputLocationMessageContent`
/// - :class:`aiogram_rs.types.input_venue_message_content.InputVenueMessageContent`
/// - :class:`aiogram_rs.types.input_contact_message_content.InputContactMessageContent`
/// - :class:`aiogram_rs.types.input_invoice_message_content.InputInvoiceMessageContent`
/// <https://core.telegram.org/bots/api#inputmessagecontent>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
    Location(InputLocationMessageContent),
    Venue(InputVenueMessageContent),
    Contact(InputContactMessageContent),
    Invoice(InputInvoiceMessageContent),
}
