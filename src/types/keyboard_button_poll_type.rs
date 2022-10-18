use serde::{Deserialize, Serialize};

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
/// <https://core.telegram.org/bots/api#keyboardbuttonpolltype>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    /// *Optional*. If *quiz* is passed, the user will be allowed to create only polls in the quiz mode. If *regular* is passed, only regular polls will be allowed. Otherwise, the user will be allowed to create a poll of any type.
    #[serde(rename = "type")]
    pub keyboard_type: Option<String>,
}

impl Default for KeyboardButtonPollType {
    fn default() -> Self {
        Self {
            keyboard_type: None,
        }
    }
}
