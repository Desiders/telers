use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This object represents type of a poll, which is allowed to be created and sent when the corresponding button is pressed.
/// # Documentation
/// <https://core.telegram.org/bots/api#keyboardbuttonpolltype>
#[derive(
    Debug,
    Display,
    Clone,
    Hash,
    PartialEq,
    Eq,
    Deserialize,
    Serialize,
    EnumString,
    AsRefStr,
    IntoStaticStr,
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum KeyboardButtonPollType {
    /// If `Quiz` is passed, the user will be allowed to create only polls in
    /// the quiz mode.
    #[strum(serialize = "quiz")]
    Quiz,
    /// If `Regular` is passed, only regular polls will be allowed.
    #[strum(serialize = "regular")]
    Regular,
    /// If `Any` is passed, the user will be allowed to create a poll of any
    /// type.
    #[serde(rename = "")]
    #[strum(serialize = "any")]
    Any,
}

impl Default for KeyboardButtonPollType {
    fn default() -> Self {
        Self::Any
    }
}
