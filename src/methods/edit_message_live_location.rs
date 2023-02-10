use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InlineKeyboardMarkup, MessageOrTrue},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to edit live location messages. A location can be edited until its `live_period` expires or editing is explicitly disabled by a call to [stopMessageLiveLocation](crate::methods::StopMessageLiveLocation).
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagelivelocation>
/// # Returns
/// On success, if the edited message is not an inline message, the edited [`Message`] is returned,
/// otherwise `True` is returned
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EditMessageLiveLocation {
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: Option<ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message to edit
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    pub inline_message_id: Option<String>,
    /// Longitude of new location
    pub longitude: f64,
    /// Latitude of new location
    pub latitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    /// A JSON-serialized object for a new [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards).
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocation {
    #[must_use]
    pub fn new(longitude: f64, latitude: f64) -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            longitude,
            latitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = Some(val.into());
        self
    }

    #[must_use]
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = Some(val);
        self
    }

    #[must_use]
    pub fn inline_message_id<T: Into<String>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    #[must_use]
    pub fn longitude(mut self, val: f64) -> Self {
        self.longitude = val;
        self
    }

    #[must_use]
    pub fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    #[must_use]
    pub fn horizontal_accuracy(mut self, val: f64) -> Self {
        self.horizontal_accuracy = Some(val);
        self
    }

    #[must_use]
    pub fn heading(mut self, val: i64) -> Self {
        self.heading = Some(val);
        self
    }

    #[must_use]
    pub fn proximity_alert_radius(mut self, val: i64) -> Self {
        self.proximity_alert_radius = Some(val);
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl TelegramMethod for EditMessageLiveLocation {
    type Method = Self;
    type Return = MessageOrTrue;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("editMessageLiveLocation", self, None)
    }
}
