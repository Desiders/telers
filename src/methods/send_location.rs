use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, Message, ReplyMarkup},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send point on the map.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendlocation>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct SendLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds for which the location will be updated (see [Live Locations](https://telegram.org/blog/live-locations), should be between 60 and 86400.
    pub live_period: Option<i64>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Pass `True`, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendLocation {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T, longitude: f64, latitude: f64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            longitude,
            latitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
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
    pub fn live_period(mut self, val: i64) -> Self {
        self.live_period = Some(val);
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
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }

    #[must_use]
    pub fn reply_to_message_id(mut self, val: i64) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, val: bool) -> Self {
        self.allow_sending_without_reply = Some(val);
        self
    }

    #[must_use]
    pub fn reply_markup<T: Into<ReplyMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}

impl SendLocation {
    #[must_use]
    pub fn message_thread_id_some(mut self, val: Option<i64>) -> Self {
        self.message_thread_id = val;
        self
    }

    #[must_use]
    pub fn horizontal_accuracy_some(mut self, val: Option<f64>) -> Self {
        self.horizontal_accuracy = val;
        self
    }

    #[must_use]
    pub fn live_period_some(mut self, val: Option<i64>) -> Self {
        self.live_period = val;
        self
    }

    #[must_use]
    pub fn heading_some(mut self, val: Option<i64>) -> Self {
        self.heading = val;
        self
    }

    #[must_use]
    pub fn proximity_alert_radius_some(mut self, val: Option<i64>) -> Self {
        self.proximity_alert_radius = val;
        self
    }

    #[must_use]
    pub fn disable_notification_some(mut self, val: Option<bool>) -> Self {
        self.disable_notification = val;
        self
    }

    #[must_use]
    pub fn protect_content_some(mut self, val: Option<bool>) -> Self {
        self.protect_content = val;
        self
    }

    #[must_use]
    pub fn reply_to_message_id_some(mut self, val: Option<i64>) -> Self {
        self.reply_to_message_id = val;
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply_some(mut self, val: Option<bool>) -> Self {
        self.allow_sending_without_reply = val;
        self
    }

    #[must_use]
    pub fn reply_markup_some<T: Into<ReplyMarkup>>(mut self, val: Option<T>) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}

impl TelegramMethod for SendLocation {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("sendLocation", self, None)
    }
}
