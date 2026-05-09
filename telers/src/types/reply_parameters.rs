use serde::{Deserialize, Serialize};
/// Describes reply parameters for the message that is being sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#replyparameters>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplyParameters {
    /// Identifier of the message that will be replied to in the current chat, or in the chat `chat_id` if it is specified
    pub message_id: i64,
    /// If the message to be replied to is from a different chat, unique identifier for the chat or username of the bot, supergroup or channel in the format @username. Not supported for messages sent on behalf of a business account and messages from channel direct messages chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<crate::types::ChatIdKind>,
    /// Pass `true` if the message should be sent even if the specified message to be replied to is not found. Always `false` for replies in another chat or forum topic. Always `true` for messages sent on behalf of a business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    /// Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities. The message will fail to send if the quote isn't found in the original message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<str>>,
    /// Mode for parsing entities in the quote. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the quote. It can be specified instead of `quote_parse_mode`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Position of the quote in the original message in UTF-16 code units
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i64>,
    /// Identifier of the specific checklist task to be replied to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checklist_task_id: Option<i64>,
    /// Persistent identifier of the specific poll option to be replied to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_option_id: Option<Box<str>>,
}
impl ReplyParameters {
    /// Creates a new `ReplyParameters`.
    ///
    /// # Arguments
    /// * `message_id` - Identifier of the message that will be replied to in the current chat, or in the chat `chat_id` if it is specified
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(message_id: T0) -> Self {
        Self {
            message_id: message_id.into(),
            chat_id: None,
            allow_sending_without_reply: None,
            quote: None,
            quote_parse_mode: None,
            quote_entities: None,
            quote_position: None,
            checklist_task_id: None,
            poll_option_id: None,
        }
    }

    /// Identifier of the message that will be replied to in the current chat, or in the chat `chat_id` if it is specified
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = val.into();
        this
    }

    /// If the message to be replied to is from a different chat, unique identifier for the chat or username of the bot, supergroup or channel in the format @username. Not supported for messages sent on behalf of a business account and messages from channel direct messages chats.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = Some(val.into());
        this
    }

    /// If the message to be replied to is from a different chat, unique identifier for the chat or username of the bot, supergroup or channel in the format @username. Not supported for messages sent on behalf of a business account and messages from channel direct messages chats.
    #[must_use]
    pub fn chat_id_option<T: Into<crate::types::ChatIdKind>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.chat_id = val.map(Into::into);
        this
    }

    /// Pass `true` if the message should be sent even if the specified message to be replied to is not found. Always `false` for replies in another chat or forum topic. Always `true` for messages sent on behalf of a business account.
    #[must_use]
    pub fn allow_sending_without_reply<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.allow_sending_without_reply = Some(val.into());
        this
    }

    /// Pass `true` if the message should be sent even if the specified message to be replied to is not found. Always `false` for replies in another chat or forum topic. Always `true` for messages sent on behalf of a business account.
    #[must_use]
    pub fn allow_sending_without_reply_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.allow_sending_without_reply = val.map(Into::into);
        this
    }

    /// Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities. The message will fail to send if the quote isn't found in the original message.
    #[must_use]
    pub fn quote<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.quote = Some(val.into());
        this
    }

    /// Quoted part of the message to be replied to; 0-1024 characters after entities parsing. The quote must be an exact substring of the message to be replied to, including bold, italic, underline, strikethrough, spoiler, `custom_emoji`, and `date_time` entities. The message will fail to send if the quote isn't found in the original message.
    #[must_use]
    pub fn quote_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.quote = val.map(Into::into);
        this
    }

    /// Mode for parsing entities in the quote. See formatting options for more details.
    #[must_use]
    pub fn quote_parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.quote_parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the quote. See formatting options for more details.
    #[must_use]
    pub fn quote_parse_mode_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.quote_parse_mode = val.map(Into::into);
        this
    }

    /// A JSON-serialized list of special entities that appear in the quote. It can be specified instead of `quote_parse_mode`.
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn quote_entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.quote_entities = Some(
            this.quote_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the quote. It can be specified instead of `quote_parse_mode`.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn quote_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.quote_entities = Some(
            this.quote_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in the quote. It can be specified instead of `quote_parse_mode`.
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn quote_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.quote_entities = val.map(Into::into);
        this
    }

    /// Position of the quote in the original message in UTF-16 code units
    #[must_use]
    pub fn quote_position<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.quote_position = Some(val.into());
        this
    }

    /// Position of the quote in the original message in UTF-16 code units
    #[must_use]
    pub fn quote_position_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.quote_position = val.map(Into::into);
        this
    }

    /// Identifier of the specific checklist task to be replied to
    #[must_use]
    pub fn checklist_task_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.checklist_task_id = Some(val.into());
        this
    }

    /// Identifier of the specific checklist task to be replied to
    #[must_use]
    pub fn checklist_task_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.checklist_task_id = val.map(Into::into);
        this
    }

    /// Persistent identifier of the specific poll option to be replied to
    #[must_use]
    pub fn poll_option_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.poll_option_id = Some(val.into());
        this
    }

    /// Persistent identifier of the specific poll option to be replied to
    #[must_use]
    pub fn poll_option_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.poll_option_id = val.map(Into::into);
        this
    }
}
