use super::{InlineKeyboardMarkup, InputMessageContent};

use serde::{Deserialize, Serialize};

/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the contact.
/// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// <https://core.telegram.org/bots/api#inlinequeryresultcontact>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    /// Type of the result, must be *contact*
    #[serde(rename = "type", default = "contact")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// *Optional*. Contact's last name
    pub last_name: Option<String>,
    /// *Optional*. Additional data about the contact in the form of a `vCard <https://en.wikipedia.org/wiki/VCard>`_, 0-2048 bytes
    pub vcard: Option<String>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating>`_ attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// *Optional*. Content of the message to be sent instead of the contact
    pub input_message_content: Option<InputMessageContent>,
    /// *Optional*. Url of the thumbnail for the result
    pub thumb_url: Option<String>,
    /// *Optional*. Thumbnail width
    pub thumb_width: Option<i64>,
    /// *Optional*. Thumbnail height
    pub thumb_height: Option<i64>,
}

impl Default for InlineQueryResultContact {
    fn default() -> Self {
        Self {
            result_type: contact(),
            id: String::default(),
            phone_number: String::default(),
            first_name: String::default(),
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }
}

fn contact() -> String {
    "contact".to_string()
}
