use super::KeyboardButton;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a [`custom keyboard`](https://core.telegram.org/bots#keyboards) with reply options (see [`Introduction to bots`](https://core.telegram.org/bots#keyboards) for details and examples).
/// # Documentation
/// <https://core.telegram.org/bots/api#replykeyboardmarkup>
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    // Array of button rows, each represented by an Array of [`KeyboardButton`] objects
    pub keyboard: Vec<Vec<KeyboardButton>>,
    /// Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to false, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    pub is_persistent: Option<bool>,
    /// Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to *false*, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    pub resize_keyboard: Option<bool>,
    /// Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to *false*.
    pub one_time_keyboard: Option<bool>,
    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    pub input_field_placeholder: Option<String>,
    /// Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the *text* of the [`Message`](crate::types::Message) object; 2) if the bot's message is a reply (has *reply_to_message_id*), sender of the original message.
    pub selective: Option<bool>,
}

impl ReplyKeyboardMarkup {
    #[must_use]
    pub fn new<T, I>(keyboard: I) -> Self
    where
        T: IntoIterator<Item = KeyboardButton>,
        I: IntoIterator<Item = T>,
    {
        Self {
            keyboard: keyboard
                .into_iter()
                .map(|val| val.into_iter().collect())
                .collect(),
            is_persistent: None,
            resize_keyboard: None,
            one_time_keyboard: None,
            input_field_placeholder: None,
            selective: None,
        }
    }

    #[must_use]
    pub fn is_persistent(self, val: bool) -> Self {
        Self {
            is_persistent: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn keyboard<T, I>(self, val: I) -> Self
    where
        T: IntoIterator<Item = KeyboardButton>,
        I: IntoIterator<Item = T>,
    {
        Self {
            keyboard: val
                .into_iter()
                .map(|val| val.into_iter().collect())
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn resize_keyboard(self, val: bool) -> Self {
        Self {
            resize_keyboard: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn one_time_keyboard(self, val: bool) -> Self {
        Self {
            one_time_keyboard: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn input_field_placeholder(self, val: impl Into<String>) -> Self {
        Self {
            input_field_placeholder: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn selective(self, val: bool) -> Self {
        Self {
            selective: Some(val),
            ..self
        }
    }
}
