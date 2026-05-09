use crate::client::Bot;
use serde::Serialize;
/// Verifies a chat on behalf of the organization which is represented by the bot. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#verifychat>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct VerifyChat {
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username. Channel direct messages chats can't be verified.
    pub chat_id: crate::types::ChatIdKind,
    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<Box<str>>,
}
impl VerifyChat {
    /// Creates a new `VerifyChat`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username. Channel direct messages chats can't be verified.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
            custom_description: None,
        }
    }

    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username. Channel direct messages chats can't be verified.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[must_use]
    pub fn custom_description<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.custom_description = Some(val.into());
        this
    }

    /// Custom description for the verification; 0-70 characters. Must be empty if the organization isn't allowed to provide a custom verification description.
    #[must_use]
    pub fn custom_description_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.custom_description = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for VerifyChat {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("verifyChat", self, None)
    }
}
