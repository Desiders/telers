use serde::{Deserialize, Serialize};
/// Represents a contact with a phone number. By default, this contact will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the contact.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultcontact>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: Box<str>,
    /// Contact's phone number
    pub phone_number: Box<str>,
    /// Contact's first name
    pub first_name: Box<str>,
    /// Contact's last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<Box<str>>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<Box<str>>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the contact
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<crate::types::InputMessageContent>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<Box<str>>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}
impl InlineQueryResultContact {
    /// Creates a new `InlineQueryResultContact`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 Bytes
    /// * `phone_number` - Contact's phone number
    /// * `first_name` - Contact's first name
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        id: T0,
        phone_number: T1,
        first_name: T2,
    ) -> Self {
        Self {
            id: id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    /// Unique identifier for this result, 1-64 Bytes
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Contact's phone number
    #[must_use]
    pub fn phone_number<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.phone_number = val.into();
        self
    }

    /// Contact's first name
    #[must_use]
    pub fn first_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.first_name = val.into();
        self
    }

    /// Contact's last name
    #[must_use]
    pub fn last_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.last_name = Some(val.into());
        self
    }

    /// Contact's last name
    #[must_use]
    pub fn last_name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.last_name = val.map(Into::into);
        self
    }

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[must_use]
    pub fn vcard<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.vcard = Some(val.into());
        self
    }

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    #[must_use]
    pub fn vcard_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.vcard = val.map(Into::into);
        self
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }

    /// Content of the message to be sent instead of the contact
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: T,
    ) -> Self {
        self.input_message_content = Some(val.into());
        self
    }

    /// Content of the message to be sent instead of the contact
    #[must_use]
    pub fn input_message_content_option<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.input_message_content = val.map(Into::into);
        self
    }

    /// Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.thumbnail_url = Some(val.into());
        self
    }

    /// Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.thumbnail_url = val.map(Into::into);
        self
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width<T: Into<i64>>(mut self, val: T) -> Self {
        self.thumbnail_width = Some(val.into());
        self
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.thumbnail_width = val.map(Into::into);
        self
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height<T: Into<i64>>(mut self, val: T) -> Self {
        self.thumbnail_height = Some(val.into());
        self
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.thumbnail_height = val.map(Into::into);
        self
    }
}
