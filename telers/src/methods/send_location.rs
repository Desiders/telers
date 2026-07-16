use crate::client::Bot;
use serde::Serialize;
/// Use this method to send point on the map. On success, the sent Message is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendlocation>
/// # Returns
/// - `crate::types::Message`
#[derive(Clone, Debug, Serialize)]
pub struct SendLocation {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// For outgoing ephemeral messages, unique identifier of the user who will receive the message; for group and supergroup chats only. It is not guaranteed that the user will receive the message, especially if they are offline. See ephemeral message sending for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<i64>,
    /// For outgoing ephemeral messages, identifier of the callback query which triggered the message if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query_id: Option<Box<str>>,
    /// Latitude of the location
    pub latitude: f64,
    /// Longitude of the location
    pub longitude: f64,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location will be updated (see Live Locations), must be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely. Must be 0 for ephemeral messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<Box<str>>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<crate::types::SuggestedPostParameters>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<crate::types::ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::ReplyMarkup>,
}
impl SendLocation {
    /// Creates a new `SendLocation`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    /// * `latitude` - Latitude of the location
    /// * `longitude` - Longitude of the location
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<f64>, T2: Into<f64>>(
        chat_id: T0,
        latitude: T1,
        longitude: T2,
    ) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            receiver_user_id: None,
            callback_query_id: None,
            latitude: latitude.into(),
            longitude: longitude.into(),
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

    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.business_connection_id = val.map(Into::into);
        self
    }

    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_thread_id = Some(val.into());
        self
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_thread_id = val.map(Into::into);
        self
    }

    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.direct_messages_topic_id = Some(val.into());
        self
    }

    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.direct_messages_topic_id = val.map(Into::into);
        self
    }

    /// For outgoing ephemeral messages, unique identifier of the user who will receive the message; for group and supergroup chats only. It is not guaranteed that the user will receive the message, especially if they are offline. See ephemeral message sending for more details.
    #[must_use]
    pub fn receiver_user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.receiver_user_id = Some(val.into());
        self
    }

    /// For outgoing ephemeral messages, unique identifier of the user who will receive the message; for group and supergroup chats only. It is not guaranteed that the user will receive the message, especially if they are offline. See ephemeral message sending for more details.
    #[must_use]
    pub fn receiver_user_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.receiver_user_id = val.map(Into::into);
        self
    }

    /// For outgoing ephemeral messages, identifier of the callback query which triggered the message if any
    #[must_use]
    pub fn callback_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.callback_query_id = Some(val.into());
        self
    }

    /// For outgoing ephemeral messages, identifier of the callback query which triggered the message if any
    #[must_use]
    pub fn callback_query_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.callback_query_id = val.map(Into::into);
        self
    }

    /// Latitude of the location
    #[must_use]
    pub fn latitude<T: Into<f64>>(mut self, val: T) -> Self {
        self.latitude = val.into();
        self
    }

    /// Longitude of the location
    #[must_use]
    pub fn longitude<T: Into<f64>>(mut self, val: T) -> Self {
        self.longitude = val.into();
        self
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy<T: Into<f64>>(mut self, val: T) -> Self {
        self.horizontal_accuracy = Some(val.into());
        self
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy_option<T: Into<f64>>(mut self, val: Option<T>) -> Self {
        self.horizontal_accuracy = val.map(Into::into);
        self
    }

    /// Period in seconds during which the location will be updated (see Live Locations), must be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely. Must be 0 for ephemeral messages.
    #[must_use]
    pub fn live_period<T: Into<u32>>(mut self, val: T) -> Self {
        self.live_period = Some(val.into());
        self
    }

    /// Period in seconds during which the location will be updated (see Live Locations), must be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely. Must be 0 for ephemeral messages.
    #[must_use]
    pub fn live_period_option<T: Into<u32>>(mut self, val: Option<T>) -> Self {
        self.live_period = val.map(Into::into);
        self
    }

    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading<T: Into<u16>>(mut self, val: T) -> Self {
        self.heading = Some(val.into());
        self
    }

    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading_option<T: Into<u16>>(mut self, val: Option<T>) -> Self {
        self.heading = val.map(Into::into);
        self
    }

    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius<T: Into<u32>>(mut self, val: T) -> Self {
        self.proximity_alert_radius = Some(val.into());
        self
    }

    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius_option<T: Into<u32>>(mut self, val: Option<T>) -> Self {
        self.proximity_alert_radius = val.map(Into::into);
        self
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.disable_notification = val.map(Into::into);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(mut self, val: T) -> Self {
        self.protect_content = Some(val.into());
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.protect_content = val.map(Into::into);
        self
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[must_use]
    pub fn allow_paid_broadcast<T: Into<bool>>(mut self, val: T) -> Self {
        self.allow_paid_broadcast = Some(val.into());
        self
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[must_use]
    pub fn allow_paid_broadcast_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allow_paid_broadcast = val.map(Into::into);
        self
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.message_effect_id = val.map(Into::into);
        self
    }

    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[must_use]
    pub fn suggested_post_parameters<T: Into<crate::types::SuggestedPostParameters>>(
        mut self,
        val: T,
    ) -> Self {
        self.suggested_post_parameters = Some(val.into());
        self
    }

    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[must_use]
    pub fn suggested_post_parameters_option<T: Into<crate::types::SuggestedPostParameters>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.suggested_post_parameters = val.map(Into::into);
        self
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters<T: Into<crate::types::ReplyParameters>>(mut self, val: T) -> Self {
        self.reply_parameters = Some(val.into());
        self
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters_option<T: Into<crate::types::ReplyParameters>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_parameters = val.map(Into::into);
        self
    }

    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user.
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::ReplyMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user.
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::ReplyMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SendLocation {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendLocation", self, None)
    }
}
