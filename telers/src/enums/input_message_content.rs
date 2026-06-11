use crate::types::InputMessageContent;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents the content of a message to be sent as a result of an inline query. Telegram clients currently support the following types:
/// - [`crate::types::InputTextMessageContent`]
/// - [`crate::types::InputRichMessageContent`]
/// - [`crate::types::InputLocationMessageContent`]
/// - [`crate::types::InputVenueMessageContent`]
/// - [`crate::types::InputContactMessageContent`]
/// - [`crate::types::InputInvoiceMessageContent`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmessagecontent>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum InputMessageContentType {
    #[strum(serialize = "input_invoice_message_content")]
    InputInvoiceMessageContent,
    #[strum(serialize = "input_venue_message_content")]
    InputVenueMessageContent,
    #[strum(serialize = "input_location_message_content")]
    InputLocationMessageContent,
    #[strum(serialize = "input_contact_message_content")]
    InputContactMessageContent,
    #[strum(serialize = "input_text_message_content")]
    InputTextMessageContent,
    #[strum(serialize = "input_rich_message_content")]
    InputRichMessageContent,
}
impl InputMessageContentType {
    #[must_use]
    pub const fn all() -> [InputMessageContentType; 6usize] {
        [
            InputMessageContentType::InputInvoiceMessageContent,
            InputMessageContentType::InputVenueMessageContent,
            InputMessageContentType::InputLocationMessageContent,
            InputMessageContentType::InputContactMessageContent,
            InputMessageContentType::InputTextMessageContent,
            InputMessageContentType::InputRichMessageContent,
        ]
    }
}
impl From<InputMessageContentType> for Box<str> {
    fn from(val: InputMessageContentType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InputMessageContentType> for String {
    fn from(val: InputMessageContentType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InputMessageContentType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InputMessageContent> for InputMessageContentType {
    fn from(val: &'a InputMessageContent) -> Self {
        match val {
            InputMessageContent::InputInvoiceMessageContent(_) => {
                InputMessageContentType::InputInvoiceMessageContent
            }
            InputMessageContent::InputVenueMessageContent(_) => {
                InputMessageContentType::InputVenueMessageContent
            }
            InputMessageContent::InputLocationMessageContent(_) => {
                InputMessageContentType::InputLocationMessageContent
            }
            InputMessageContent::InputContactMessageContent(_) => {
                InputMessageContentType::InputContactMessageContent
            }
            InputMessageContent::InputTextMessageContent(_) => {
                InputMessageContentType::InputTextMessageContent
            }
            InputMessageContent::InputRichMessageContent(_) => {
                InputMessageContentType::InputRichMessageContent
            }
        }
    }
}
impl From<InputMessageContent> for InputMessageContentType {
    fn from(val: InputMessageContent) -> Self {
        InputMessageContentType::from(&val)
    }
}
