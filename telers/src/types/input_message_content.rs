use super::{
    InputContactMessageContent, InputInvoiceMessageContent, InputLocationMessageContent,
    InputTextMessageContent, InputVenueMessageContent,
};

use serde::{Deserialize, Serialize};

/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following 5 types:
/// - [`InputTextMessageContent`]
/// - [`InputLocationMessageContent`]
/// - [`InputVenueMessageContent`]
/// - [`InputContactMessageContent`]
/// - [`InputInvoiceMessageContent`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmessagecontent>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
