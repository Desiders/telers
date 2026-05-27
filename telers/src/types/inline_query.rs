use serde::{Deserialize, Serialize};
/// This object represents an incoming inline query. When the user sends an empty query, your bot could return some default or trending results.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequery>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query
    pub id: Box<str>,
    /// Sender
    pub from: Box<crate::types::User>,
    /// Text of the query (up to 256 characters)
    pub query: Box<str>,
    /// Offset of the results to be returned, can be controlled by the bot
    pub offset: Box<str>,
    /// Type of the chat from which the inline query was sent. Can be either `sender` for a private chat with the inline query sender, `private`, `group`, `supergroup`, or `channel`. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<Box<str>>,
    /// Sender location, only for bots that request user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::types::Location>,
}
impl InlineQuery {
    /// Creates a new `InlineQuery`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this query
    /// * `from` - Sender
    /// * `query` - Text of the query (up to 256 characters)
    /// * `offset` - Offset of the results to be returned, can be controlled by the bot
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::User>,
        T2: Into<Box<str>>,
        T3: Into<Box<str>>,
    >(
        id: T0,
        from: T1,
        query: T2,
        offset: T3,
    ) -> Self {
        Self {
            id: id.into(),
            from: Box::new(from.into()),
            query: query.into(),
            offset: offset.into(),
            chat_type: None,
            location: None,
        }
    }

    /// Unique identifier for this query
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Sender
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.from = Box::new(val.into());
        self
    }

    /// Text of the query (up to 256 characters)
    #[must_use]
    pub fn query<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.query = val.into();
        self
    }

    /// Offset of the results to be returned, can be controlled by the bot
    #[must_use]
    pub fn offset<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.offset = val.into();
        self
    }

    /// Type of the chat from which the inline query was sent. Can be either `sender` for a private chat with the inline query sender, `private`, `group`, `supergroup`, or `channel`. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    #[must_use]
    pub fn chat_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.chat_type = Some(val.into());
        self
    }

    /// Type of the chat from which the inline query was sent. Can be either `sender` for a private chat with the inline query sender, `private`, `group`, `supergroup`, or `channel`. The chat type should be always known for requests sent from official clients and most third-party clients, unless the request was sent from a secret chat
    #[must_use]
    pub fn chat_type_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.chat_type = val.map(Into::into);
        self
    }

    /// Sender location, only for bots that request user location
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(mut self, val: T) -> Self {
        self.location = Some(val.into());
        self
    }

    /// Sender location, only for bots that request user location
    #[must_use]
    pub fn location_option<T: Into<crate::types::Location>>(mut self, val: Option<T>) -> Self {
        self.location = val.map(Into::into);
        self
    }
}
