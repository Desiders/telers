use serde::{Deserialize, Serialize};
/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonpolltype>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    /// If quiz is passed, the user will be allowed to create only polls in the quiz mode. If regular is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<str>>,
}
impl KeyboardButtonPollType {
    /// Creates a new `KeyboardButtonPollType`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            r#type: None,
        }
    }

    /// If quiz is passed, the user will be allowed to create only polls in the quiz mode. If regular is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.r#type = Some(val.into());
        this
    }

    /// If quiz is passed, the user will be allowed to create only polls in the quiz mode. If regular is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
    #[must_use]
    pub fn type_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.r#type = val.map(Into::into);
        this
    }
}
impl Default for KeyboardButtonPollType {
    fn default() -> Self {
        Self::new()
    }
}
