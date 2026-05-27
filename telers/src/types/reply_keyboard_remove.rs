use serde::{Deserialize, Serialize};
/// Upon receiving a message with this object, Telegram clients will remove the current custom keyboard and display the default letter-keyboard. By default, custom keyboards are displayed until a new keyboard is sent by a bot. An exception is made for one-time keyboards that are hidden immediately after the user presses a button (see [`crate::types::ReplyKeyboardMarkup`]). Not supported in channels and for messages sent on behalf of a business account.
/// # Documentation
/// <https://core.telegram.org/bots/api#replykeyboardremove>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    /// Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use `one_time_keyboard` in [`crate::types::ReplyKeyboardMarkup`])
    pub remove_keyboard: bool,
    /// Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}
impl ReplyKeyboardRemove {
    /// Creates a new `ReplyKeyboardRemove`.
    ///
    /// # Arguments
    /// * `remove_keyboard` - Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use `one_time_keyboard` in [`crate::types::ReplyKeyboardMarkup`])
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<bool>>(remove_keyboard: T0) -> Self {
        Self {
            remove_keyboard: remove_keyboard.into(),
            selective: None,
        }
    }

    /// Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use `one_time_keyboard` in [`crate::types::ReplyKeyboardMarkup`])
    #[must_use]
    pub fn remove_keyboard<T: Into<bool>>(mut self, val: T) -> Self {
        self.remove_keyboard = val.into();
        self
    }

    /// Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.
    #[must_use]
    pub fn selective<T: Into<bool>>(mut self, val: T) -> Self {
        self.selective = Some(val.into());
        self
    }

    /// Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.
    #[must_use]
    pub fn selective_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.selective = val.map(Into::into);
        self
    }
}
