use serde::{Deserialize, Serialize};
/// Represents a result of an inline query that was chosen by the user and sent to their chat partner.
/// Note: It is necessary to enable inline feedback via @`BotFather` in order to receive these objects in updates.
/// # Documentation
/// <https://core.telegram.org/bots/api#choseninlineresult>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: Box<str>,
    /// The user that chose the result
    pub from: Box<crate::types::User>,
    /// Sender location, only for bots that require user location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::types::Location>,
    /// Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message. Will be also received in callback queries and can be used to edit the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Box<str>>,
    /// The query that was used to obtain the result
    pub query: Box<str>,
}
impl ChosenInlineResult {
    /// Creates a new `ChosenInlineResult`.
    ///
    /// # Arguments
    /// * `result_id` - The unique identifier for the result that was chosen
    /// * `from` - The user that chose the result
    /// * `query` - The query that was used to obtain the result
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::User>, T2: Into<Box<str>>>(
        result_id: T0,
        from: T1,
        query: T2,
    ) -> Self {
        Self {
            result_id: result_id.into(),
            from: Box::new(from.into()),
            location: None,
            inline_message_id: None,
            query: query.into(),
        }
    }

    /// The unique identifier for the result that was chosen
    #[must_use]
    pub fn result_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.result_id = val.into();
        this
    }

    /// The user that chose the result
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.from = Box::new(val.into());
        this
    }

    /// Sender location, only for bots that require user location
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(self, val: T) -> Self {
        let mut this = self;
        this.location = Some(val.into());
        this
    }

    /// Sender location, only for bots that require user location
    #[must_use]
    pub fn location_option<T: Into<crate::types::Location>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.location = val.map(Into::into);
        this
    }

    /// Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message. Will be also received in callback queries and can be used to edit the message.
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.inline_message_id = Some(val.into());
        this
    }

    /// Identifier of the sent inline message. Available only if there is an inline keyboard attached to the message. Will be also received in callback queries and can be used to edit the message.
    #[must_use]
    pub fn inline_message_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.inline_message_id = val.map(Into::into);
        this
    }

    /// The query that was used to obtain the result
    #[must_use]
    pub fn query<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.query = val.into();
        this
    }
}
