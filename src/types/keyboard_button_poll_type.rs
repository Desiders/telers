use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonpolltype>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    /// *Optional*. If *quiz* is passed, the user will be allowed to create only polls in the quiz mode. If *regular* is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
    #[serde(rename = "type")]
    pub keyboard_type: Option<String>,
}

impl KeyboardButtonPollType {
    #[must_use]
    pub fn new<T: Into<String>>(keyboard_type: Option<T>) -> Self {
        Self {
            keyboard_type: keyboard_type.map(Into::into),
        }
    }
}
