use super::InlineKeyboardButton;

use serde::{Deserialize, Serialize};

/// This object represents an `inline keyboard <https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating>` that appears right next to the message it belongs to.
/// **Note:** This will only work in Telegram versions released after 9 April, 2016. Older clients will display *unsupported message*.
/// <https://core.telegram.org/bots/api#inlinekeyboardmarkup>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of `aiogram_rs.types.inline_keyboard_button.InlineKeyboardButton` objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}
