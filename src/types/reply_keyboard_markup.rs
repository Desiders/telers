use super::KeyboardButton;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a `custom keyboard <https://core.telegram.org/bots#keyboards>` with reply options (see `Introduction to bots <https://core.telegram.org/bots#keyboards>` for details and examples).
/// <https://core.telegram.org/bots/api#replykeyboardmarkup>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    // Array of button rows, each represented by an Array of `aiogram_rs.types.keyboard_button.KeyboardButton` objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// *Optional*. Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to *false*, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    pub resize_keyboard: Option<bool>,
    /// *Optional*. Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to *false*.
    pub one_time_keyboard: Option<bool>,
    /// *Optional*. The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    pub input_field_placeholder: Option<String>,
    /// *Optional*. Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the *text* of the `aiogram_rs.types.message.Message` object; 2) if the bot's message is a reply (has *reply_to_message_id*), sender of the original message.
    pub selective: Option<bool>,
}

impl ReplyKeyboardMarkup {
    #[must_use]
    pub fn new(keyboard: Vec<Vec<KeyboardButton>>) -> Self {
        Self {
            keyboard,
            ..Default::default()
        }
    }

    #[must_use]
    pub fn keyboard(mut self, val: Vec<Vec<KeyboardButton>>) -> Self {
        self.keyboard = val;
        self
    }

    #[must_use]
    pub fn resize_keyboard(mut self, val: bool) -> Self {
        self.resize_keyboard = Some(val);
        self
    }

    #[must_use]
    pub fn one_time_keyboard(mut self, val: bool) -> Self {
        self.one_time_keyboard = Some(val);
        self
    }

    #[must_use]
    pub fn input_field_placeholder<T: Into<String>>(mut self, val: T) -> Self {
        self.input_field_placeholder = Some(val.into());
        self
    }

    #[must_use]
    pub fn selective(mut self, val: bool) -> Self {
        self.selective = Some(val);
        self
    }
}
