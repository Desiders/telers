use super::InlineKeyboardButton;

use serde::{Deserialize, Serialize};

/// This object represents an `inline keyboard <https://core.telegram.org/bots/features#inline-keyboards>` that appears right next to the message it belongs to.
/// # Notes
/// This will only work in Telegram versions released after 9 April, 2016. Older clients will display *unsupported message*.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinekeyboardmarkup>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of `aiogram_rs.types.inline_keyboard_button.InlineKeyboardButton` objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    #[must_use]
    pub fn new(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self { inline_keyboard }
    }

    #[must_use]
    pub fn inline_keyboard(mut self, val: Vec<Vec<InlineKeyboardButton>>) -> Self {
        self.inline_keyboard = val;
        self
    }
}
