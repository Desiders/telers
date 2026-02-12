use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, Message, ReplyMarkup, ReplyParameters, SuggestedPostParameters},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send point on the map.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendlocation>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SendLocation {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    pub business_connection_id: Option<String>,
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    pub direct_messages_topic_id: Option<i64>,
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
    /// Pass `true` to allow up to 1000 messages per second, ignoring [broadcasting limits](https://core.telegram.org/bots/faq#how-can-i-message-all-of-my-bot-39s-subscribers-at-once) for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    pub message_effect_id: Option<String>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendLocation {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, longitude: f64, latitude: f64) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            longitude,
            latitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            disable_notification: None,
            protect_content: None,
            allow_paid_broadcast: None,
            message_effect_id: None,
            suggested_post_parameters: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn business_connection_id(self, val: String) -> Self {
        Self {
            business_connection_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id(self, val: i64) -> Self {
        Self {
            message_thread_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn direct_messages_topic_id(self, val: i64) -> Self {
        Self {
            direct_messages_topic_id: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn longitude(self, val: f64) -> Self {
        Self {
            longitude: val,
            ..self
        }
    }

    #[must_use]
    pub fn latitude(self, val: f64) -> Self {
        Self {
            latitude: val,
            ..self
        }
    }

    #[must_use]
    pub fn horizontal_accuracy(self, val: f64) -> Self {
        Self {
            horizontal_accuracy: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn live_period(self, val: i64) -> Self {
        Self {
            live_period: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn heading(self, val: i64) -> Self {
        Self {
            heading: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn proximity_alert_radius(self, val: i64) -> Self {
        Self {
            proximity_alert_radius: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn disable_notification(self, val: bool) -> Self {
        Self {
            disable_notification: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn protect_content(self, val: bool) -> Self {
        Self {
            protect_content: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn allow_paid_broadcast(self, val: bool) -> Self {
        Self {
            allow_paid_broadcast: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn message_effect_id(self, val: impl Into<String>) -> Self {
        Self {
            message_effect_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn suggested_post_parameters(self, val: SuggestedPostParameters) -> Self {
        Self {
            suggested_post_parameters: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn reply_parameters(self, val: ReplyParameters) -> Self {
        Self {
            reply_parameters: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<ReplyMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl SendLocation {
    #[must_use]
    pub fn business_connection_id_option(self, val: Option<String>) -> Self {
        Self {
            business_connection_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_thread_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn direct_messages_topic_id_option(self, val: Option<i64>) -> Self {
        Self {
            direct_messages_topic_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn horizontal_accuracy_option(self, val: Option<f64>) -> Self {
        Self {
            horizontal_accuracy: val,
            ..self
        }
    }

    #[must_use]
    pub fn live_period_option(self, val: Option<i64>) -> Self {
        Self {
            live_period: val,
            ..self
        }
    }

    #[must_use]
    pub fn heading_option(self, val: Option<i64>) -> Self {
        Self {
            heading: val,
            ..self
        }
    }

    #[must_use]
    pub fn proximity_alert_radius_option(self, val: Option<i64>) -> Self {
        Self {
            proximity_alert_radius: val,
            ..self
        }
    }

    #[must_use]
    pub fn disable_notification_option(self, val: Option<bool>) -> Self {
        Self {
            disable_notification: val,
            ..self
        }
    }

    #[must_use]
    pub fn protect_content_option(self, val: Option<bool>) -> Self {
        Self {
            protect_content: val,
            ..self
        }
    }

    #[must_use]
    pub fn allow_paid_broadcast_option(self, val: Option<bool>) -> Self {
        Self {
            allow_paid_broadcast: val,
            ..self
        }
    }

    #[must_use]
    pub fn message_effect_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            message_effect_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn suggested_post_parameters_option(self, val: Option<SuggestedPostParameters>) -> Self {
        Self {
            suggested_post_parameters: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_parameters_option(self, val: Option<ReplyParameters>) -> Self {
        Self {
            reply_parameters: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<ReplyMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SendLocation {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<'_, Self::Method> {
        Request::new("sendLocation", self, None)
    }
}

impl AsRef<SendLocation> for SendLocation {
    fn as_ref(&self) -> &Self {
        self
    }
}
