use super::{InlineKeyboardMarkup, InputMessageContent};

use crate::enums::InlineQueryResultType;

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
    #[serde(rename = "type", default = "contact")]
    pub result_type: String,
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Contact's phone number
    pub phone_number: String,
    /// Contact's first name
    pub first_name: String,
    /// Contact's last name
    pub last_name: Option<String>,
    /// Additional data about the contact in the form of a [`vCard`](https://en.wikipedia.org/wiki/VCard), 0-2048 bytes
    pub vcard: Option<String>,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the contact
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumbnail_url: Option<String>,
    /// Thumbnail width
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    pub thumbnail_height: Option<i64>,
}

impl InlineQueryResultContact {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        phone_number: impl Into<String>,
        first_name: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn id(self, val: impl Into<String>) -> Self {
        Self {
            id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn phone_number(self, val: impl Into<String>) -> Self {
        Self {
            phone_number: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn first_name(self, val: impl Into<String>) -> Self {
        Self {
            first_name: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn last_name(self, val: impl Into<String>) -> Self {
        Self {
            last_name: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn vcard(self, val: impl Into<String>) -> Self {
        Self {
            vcard: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn input_message_content(self, val: impl Into<InputMessageContent>) -> Self {
        Self {
            input_message_content: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_url(self, val: impl Into<String>) -> Self {
        Self {
            thumbnail_url: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_width(self, val: i64) -> Self {
        Self {
            thumbnail_width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_height(self, val: i64) -> Self {
        Self {
            thumbnail_height: Some(val),
            ..self
        }
    }
}

impl Default for InlineQueryResultContact {
    #[must_use]
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
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }
}

fn contact() -> String {
    InlineQueryResultType::Contact.into()
}
