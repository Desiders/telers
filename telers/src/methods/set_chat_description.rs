use crate::client::Bot;
use serde::Serialize;
/// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatdescription>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    pub chat_id: crate::types::ChatIdKind,
    /// New chat description, 0-255 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Box<str>>,
}
impl SetChatDescription {
    /// Creates a new `SetChatDescription`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
            description: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// New chat description, 0-255 characters
    #[must_use]
    pub fn description<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.description = Some(val.into());
        this
    }

    /// New chat description, 0-255 characters
    #[must_use]
    pub fn description_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.description = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for SetChatDescription {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setChatDescription", self, None)
    }
}
