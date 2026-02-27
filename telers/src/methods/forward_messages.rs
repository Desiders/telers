use crate::client::Bot;
use serde::Serialize;
/// Use this method to forward multiple messages of any kind. If some of the specified messages can't be found or forwarded, they are skipped. Service messages and messages with protected content can't be forwarded. Album grouping is kept for forwarded messages. On success, an array of [`MessageId`] of the sent messages is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#forwardmessages>
/// # Returns
/// - `Box<[crate::types::MessageId]>`
#[derive(Clone, Debug, Serialize)]
pub struct ForwardMessages {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the messages will be forwarded; required if the messages are forwarded to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// Unique identifier for the chat where the original messages were sent (or channel username in the format @channelusername)
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
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    /// * `from_chat_id` - Unique identifier for the chat where the original messages were sent (or channel username in the format @channelusername)
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

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_thread_id = Some(val.into());
        this
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_thread_id = val.map(Into::into);
        this
    }

    /// Identifier of the direct messages topic to which the messages will be forwarded; required if the messages are forwarded to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.direct_messages_topic_id = Some(val.into());
        this
    }

    /// Identifier of the direct messages topic to which the messages will be forwarded; required if the messages are forwarded to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.direct_messages_topic_id = val.map(Into::into);
        this
    }

    /// Unique identifier for the chat where the original messages were sent (or channel username in the format @channelusername)
    #[must_use]
    pub fn from_chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.from_chat_id = val.into();
        this
    }

    /// A JSON-serialized list of 1-100 identifiers of messages in the chat `from_chat_id` to forward. The identifiers must be specified in a strictly increasing order.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn message_ids<TItem: Into<u8>, T: IntoIterator<Item = TItem>>(self, val: T) -> Self {
        let mut this = self;
        this.message_ids = this
            .message_ids
            .into_vec()
            .into_iter()
            .chain(val.into_iter().map(Into::into))
            .collect();
        this
    }

    /// A JSON-serialized list of 1-100 identifiers of messages in the chat `from_chat_id` to forward. The identifiers must be specified in a strictly increasing order.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn message_id<T: Into<u8>>(self, val: T) -> Self {
        let mut this = self;
        this.message_ids = this
            .message_ids
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Sends the messages silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.disable_notification = Some(val.into());
        this
    }

    /// Sends the messages silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.disable_notification = val.map(Into::into);
        this
    }

    /// Protects the contents of the forwarded messages from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.protect_content = Some(val.into());
        this
    }

    /// Protects the contents of the forwarded messages from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.protect_content = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for ForwardMessages {
    type Method = Self;
    type Return = Box<[crate::types::MessageId]>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("forwardMessages", self, None)
    }
}
