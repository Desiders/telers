use serde::{Deserialize, Serialize};
/// Describes a service message about the chat owner leaving the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatownerleft>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatOwnerLeft {
    /// The user which will be the new owner of the chat if the previous owner does not return to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_owner: Option<Box<crate::types::User>>,
}
impl ChatOwnerLeft {
    /// Creates a new `ChatOwnerLeft`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            new_owner: None,
        }
    }

    /// The user which will be the new owner of the chat if the previous owner does not return to the chat
    #[must_use]
    pub fn new_owner<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.new_owner = Some(Box::new(val.into()));
        this
    }

    /// The user which will be the new owner of the chat if the previous owner does not return to the chat
    #[must_use]
    pub fn new_owner_option<T: Into<crate::types::User>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.new_owner = val.map(|val| Box::new(val.into()));
        this
    }
}
impl Default for ChatOwnerLeft {
    fn default() -> Self {
        Self::new()
    }
}
