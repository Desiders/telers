use crate::client::Bot;
use serde::Serialize;
/// Use this method to stream a partial message to a user while the message is being generated. Note that the streamed draft is ephemeral and acts as a temporary 30-second preview - once the output is finalized, you must call sendMessage with the complete message to persist it in the user's chat. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendmessagedraft>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SendMessageDraft {
    /// Unique identifier for the target private chat
    pub chat_id: i64,
    /// Unique identifier for the target message thread
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Unique identifier of the message draft; must be non-zero. Changes of drafts with the same identifier are animated.
    pub draft_id: i64,
    /// Text of the message to be sent, 0-4096 characters after entities parsing. Pass an empty text to show a `Thinking...` placeholder.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<str>>,
    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Box<[crate::types::MessageEntity]>>,
}
impl SendMessageDraft {
    /// Creates a new `SendMessageDraft`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target private chat
    /// * `draft_id` - Unique identifier of the message draft; must be non-zero. Changes of drafts with the same identifier are animated.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>>(chat_id: T0, draft_id: T1) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            draft_id: draft_id.into(),
            text: None,
            parse_mode: None,
            entities: None,
        }
    }

    /// Unique identifier for the target private chat
    #[must_use]
    pub fn chat_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier for the target message thread
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_thread_id = Some(val.into());
        this
    }

    /// Unique identifier for the target message thread
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_thread_id = val.map(Into::into);
        this
    }

    /// Unique identifier of the message draft; must be non-zero. Changes of drafts with the same identifier are animated.
    #[must_use]
    pub fn draft_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.draft_id = val.into();
        this
    }

    /// Text of the message to be sent, 0-4096 characters after entities parsing. Pass an empty text to show a `Thinking...` placeholder.
    #[must_use]
    pub fn text<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.text = Some(val.into());
        this
    }

    /// Text of the message to be sent, 0-4096 characters after entities parsing. Pass an empty text to show a `Thinking...` placeholder.
    #[must_use]
    pub fn text_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.text = val.map(Into::into);
        this
    }

    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.parse_mode = Some(val.into());
        this
    }

    /// Mode for parsing entities in the message text. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.parse_mode = val.map(Into::into);
        this
    }

    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities<TItem: Into<crate::types::MessageEntity>, T: IntoIterator<Item = TItem>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.entities = Some(
            this.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.entities = Some(
            this.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// A JSON-serialized list of special entities that appear in message text, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.entities = val.map(|v| v.into_iter().map(Into::into).collect());
        this
    }
}
impl super::TelegramMethod for SendMessageDraft {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendMessageDraft", self, None)
    }
}
