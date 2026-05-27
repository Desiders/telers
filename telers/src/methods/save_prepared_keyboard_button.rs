use crate::client::Bot;
use serde::Serialize;
/// Stores a keyboard button that can be used by a user within a Mini App. Returns a [`crate::types::PreparedKeyboardButton`] object.
/// # Documentation
/// <https://core.telegram.org/bots/api#savepreparedkeyboardbutton>
/// # Returns
/// - `crate::types::PreparedKeyboardButton`
#[derive(Clone, Debug, Serialize)]
pub struct SavePreparedKeyboardButton {
    /// Unique identifier of the target user that can use the button
    pub user_id: i64,
    /// A JSON-serialized object describing the button to be saved. The button must be of the type `request_users`, `request_chat`, or `request_managed_bot`
    pub button: crate::types::KeyboardButton,
}
impl SavePreparedKeyboardButton {
    /// Creates a new `SavePreparedKeyboardButton`.
    ///
    /// # Arguments
    /// * `user_id` - Unique identifier of the target user that can use the button
    /// * `button` - A JSON-serialized object describing the button to be saved. The button must be of the type `request_users`, `request_chat`, or `request_managed_bot`
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::KeyboardButton>>(
        user_id: T0,
        button: T1,
    ) -> Self {
        Self {
            user_id: user_id.into(),
            button: button.into(),
        }
    }

    /// Unique identifier of the target user that can use the button
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }

    /// A JSON-serialized object describing the button to be saved. The button must be of the type `request_users`, `request_chat`, or `request_managed_bot`
    #[must_use]
    pub fn button<T: Into<crate::types::KeyboardButton>>(mut self, val: T) -> Self {
        self.button = val.into();
        self
    }
}
impl super::TelegramMethod for SavePreparedKeyboardButton {
    type Method = Self;
    type Return = crate::types::PreparedKeyboardButton;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("savePreparedKeyboardButton", self, None)
    }
}
