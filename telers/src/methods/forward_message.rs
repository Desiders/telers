use crate::client::Bot;
use serde::Serialize;
/// Use this method to forward messages of any kind. Service messages and messages with protected content can't be forwarded. On success, the sent Message is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#forwardmessage>
/// # Returns
/// - `crate::types::Message`
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be forwarded; required if the message is forwarded to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Unique identifier for the chat where the original message was sent (or username of the target bot, supergroup or channel in the format @username)
    pub from_chat_id: crate::types::ChatIdKind,
    /// New start timestamp for the forwarded video in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_start_timestamp: Option<i64>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the forwarded message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; only available when forwarding to private chats
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<Box<str>>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<crate::types::SuggestedPostParameters>,
    /// Message identifier in the chat specified in `from_chat_id`
    pub message_id: i64,
}
impl ForwardMessage {
    /// Creates a new `ForwardMessage`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    /// * `from_chat_id` - Unique identifier for the chat where the original message was sent (or username of the target bot, supergroup or channel in the format @username)
    /// * `message_id` - Message identifier in the chat specified in `from_chat_id`
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::ChatIdKind>,
        T1: Into<crate::types::ChatIdKind>,
        T2: Into<i64>,
    >(
        chat_id: T0,
        from_chat_id: T1,
        message_id: T2,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            from_chat_id: from_chat_id.into(),
            video_start_timestamp: None,
            disable_notification: None,
            protect_content: None,
            message_effect_id: None,
            suggested_post_parameters: None,
            message_id: message_id.into(),
        }
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

    /// Identifier of the direct messages topic to which the message will be forwarded; required if the message is forwarded to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.direct_messages_topic_id = Some(val.into());
        self
    }

    /// Identifier of the direct messages topic to which the message will be forwarded; required if the message is forwarded to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.direct_messages_topic_id = val.map(Into::into);
        self
    }

    /// Unique identifier for the chat where the original message was sent (or username of the target bot, supergroup or channel in the format @username)
    #[must_use]
    pub fn from_chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.from_chat_id = val.into();
        self
    }

    /// New start timestamp for the forwarded video in the message
    #[must_use]
    pub fn video_start_timestamp<T: Into<i64>>(mut self, val: T) -> Self {
        self.video_start_timestamp = Some(val.into());
        self
    }

    /// New start timestamp for the forwarded video in the message
    #[must_use]
    pub fn video_start_timestamp_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.video_start_timestamp = val.map(Into::into);
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

    /// Protects the contents of the forwarded message from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(mut self, val: T) -> Self {
        self.protect_content = Some(val.into());
        self
    }

    /// Protects the contents of the forwarded message from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.protect_content = val.map(Into::into);
        self
    }

    /// Unique identifier of the message effect to be added to the message; only available when forwarding to private chats
    #[must_use]
    pub fn message_effect_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }

    /// Unique identifier of the message effect to be added to the message; only available when forwarding to private chats
    #[must_use]
    pub fn message_effect_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.message_effect_id = val.map(Into::into);
        self
    }

    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only
    #[must_use]
    pub fn suggested_post_parameters<T: Into<crate::types::SuggestedPostParameters>>(
        mut self,
        val: T,
    ) -> Self {
        self.suggested_post_parameters = Some(val.into());
        self
    }

    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only
    #[must_use]
    pub fn suggested_post_parameters_option<T: Into<crate::types::SuggestedPostParameters>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.suggested_post_parameters = val.map(Into::into);
        self
    }

    /// Message identifier in the chat specified in `from_chat_id`
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }
}
impl super::TelegramMethod for ForwardMessage {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("forwardMessage", self, None)
    }
}
