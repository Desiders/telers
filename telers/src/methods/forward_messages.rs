use crate::client::Bot;
use serde::Serialize;
/// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [`crate::types::MessageId`] of the sent messages is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#forwardmessages>
/// # Returns
/// - `Box<[crate::types::MessageId]>`
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessages {
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the messages will be forwarded; required if the messages are forwarded to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Unique identifier for the chat where the original messages were sent (or username of the target bot, supergroup or channel in the format @username)
    pub from_chat_id: crate::types::ChatIdKind,
    /// A JSON-serialized list of 1-100 identifiers of messages in the chat `from_chat_id` to forward. The identifiers must be specified in a strictly increasing order.
    pub message_ids: Box<[u8]>,
    /// Sends the messages silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the forwarded messages from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}
impl ForwardMessages {
    /// Creates a new `ForwardMessages`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    /// * `from_chat_id` - Unique identifier for the chat where the original messages were sent (or username of the target bot, supergroup or channel in the format @username)
    /// * `message_ids` - A JSON-serialized list of 1-100 identifiers of messages in the chat `from_chat_id` to forward. The identifiers must be specified in a strictly increasing order.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::ChatIdKind>,
        T1: Into<crate::types::ChatIdKind>,
        T2Item: Into<u8>,
        T2: IntoIterator<Item = T2Item>,
    >(
        chat_id: T0,
        from_chat_id: T1,
        message_ids: T2,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            from_chat_id: from_chat_id.into(),
            message_ids: message_ids.into_iter().map(Into::into).collect(),
            disable_notification: None,
            protect_content: None,
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

    /// Identifier of the direct messages topic to which the messages will be forwarded; required if the messages are forwarded to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.direct_messages_topic_id = Some(val.into());
        self
    }

    /// Identifier of the direct messages topic to which the messages will be forwarded; required if the messages are forwarded to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.direct_messages_topic_id = val.map(Into::into);
        self
    }

    /// Unique identifier for the chat where the original messages were sent (or username of the target bot, supergroup or channel in the format @username)
    #[must_use]
    pub fn from_chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.from_chat_id = val.into();
        self
    }

    /// A JSON-serialized list of 1-100 identifiers of messages in the chat `from_chat_id` to forward. The identifiers must be specified in a strictly increasing order.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn message_ids<TItem: Into<u8>, T: IntoIterator<Item = TItem>>(mut self, val: T) -> Self {
        self.message_ids = self
            .message_ids
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        self
    }

    /// A JSON-serialized list of 1-100 identifiers of messages in the chat `from_chat_id` to forward. The identifiers must be specified in a strictly increasing order.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn message_id<T: Into<u8>>(mut self, val: T) -> Self {
        self.message_ids = self
            .message_ids
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Sends the messages silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }

    /// Sends the messages silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.disable_notification = val.map(Into::into);
        self
    }

    /// Protects the contents of the forwarded messages from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(mut self, val: T) -> Self {
        self.protect_content = Some(val.into());
        self
    }

    /// Protects the contents of the forwarded messages from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.protect_content = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for ForwardMessages {
    type Method = Self;
    type Return = Box<[crate::types::MessageId]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("forwardMessages", self, None)
    }
}
