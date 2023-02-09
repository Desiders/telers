use super::{InlineKeyboardMarkup, InputMessageContent};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the contact.
/// # Notes
/// This will only work in Telegram versions released after 9 April, 2016. Older clients will ignore them.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultcontact>
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    /// Type of the result, must be *contact*
    #[serde(rename = "type")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// *Optional*. Contact's last name
    pub last_name: Option<String>,
    /// *Optional*. Additional data about the contact in the form of a `vCard <https://en.wikipedia.org/wiki/VCard>`, 0-2048 bytes
    pub vcard: Option<String>,
    /// *Optional*. `Inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` attached to the message
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

impl InlineQueryResultContact {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, phone_number: T, first_name: T) -> Self {
        Self {
            id: id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id<T: Into<String>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    #[must_use]
    pub fn phone_number<T: Into<String>>(mut self, val: T) -> Self {
        self.phone_number = val.into();
        self
    }

    #[must_use]
    pub fn first_name<T: Into<String>>(mut self, val: T) -> Self {
        self.first_name = val.into();
        self
    }

    #[must_use]
    pub fn last_name<T: Into<String>>(mut self, val: T) -> Self {
        self.last_name = Some(val.into());
        self
    }

    #[must_use]
    pub fn vcard<T: Into<String>>(mut self, val: T) -> Self {
        self.vcard = Some(val.into());
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }

    #[must_use]
    pub fn thumb_url<T: Into<String>>(mut self, val: T) -> Self {
        self.thumb_url = Some(val.into());
        self
    }

    #[must_use]
    pub fn thumb_width(mut self, val: i64) -> Self {
        self.thumb_width = Some(val);
        self
    }

    #[must_use]
    pub fn thumb_height(mut self, val: i64) -> Self {
        self.thumb_height = Some(val);
        self
    }
}

impl Default for InlineQueryResultContact {
    #[must_use]
    fn default() -> Self {
        Self {
            result_type: "contact".to_string(),
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
