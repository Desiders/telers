use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see `aiogram_rs.types.reply_keyboard_markup.ReplyKeyboardMarkup`).
/// # Documentation
/// <https://core.telegram.org/bots/api#replykeyboardremove>
#[skip_serializing_none]
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use *one_time_keyboard* in `aiogram_rs.types.reply_keyboard_markup.ReplyKeyboardMarkup`)
    pub remove_keyboard: bool,
    /// *Optional*. Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the *text* of the `aiogram_rs.types.message.Message` object; 2) if the bot's message is a reply (has *reply_to_message_id*), sender of the original message.
    pub selective: Option<bool>,
}

impl ReplyKeyboardRemove {
    #[must_use]
    pub fn new(remove_keyboard: bool) -> Self {
        Self {
            remove_keyboard,
            selective: None,
        }
    }

    #[must_use]
    pub fn remove_keyboard(mut self, val: bool) -> Self {
        self.remove_keyboard = val;
        self
    }

    #[must_use]
    pub fn selective(mut self, val: bool) -> Self {
        self.selective = Some(val);
        self
    }
}
