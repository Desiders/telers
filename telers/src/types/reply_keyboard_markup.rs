use serde::{Deserialize, Serialize};
/// This object represents a custom keyboard with reply options (see Introduction to bots for details and examples). Not supported in channels and for messages sent on behalf of a business account.
/// # Documentation
/// <https://core.telegram.org/bots/api#replykeyboardmarkup>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [`crate::types::KeyboardButton`] objects
    pub keyboard: Box<[Box<[crate::types::KeyboardButton]>]>,
    /// Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to `false`, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,
    /// Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to `false`, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,
    /// Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,
    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<Box<str>>,
    /// Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language. Other users in the group don't see the keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}
impl ReplyKeyboardMarkup {
    /// Creates a new `ReplyKeyboardMarkup`.
    ///
    /// # Arguments
    /// * `keyboard` - Array of button rows, each represented by an Array of [`crate::types::KeyboardButton`] objects
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0Item: Into<Box<[crate::types::KeyboardButton]>>,
        T0: IntoIterator<Item = T0Item>,
    >(
        keyboard: T0,
    ) -> Self {
        Self {
            keyboard: keyboard.into_iter().map(Into::into).collect(),
            is_persistent: None,
            resize_keyboard: None,
            one_time_keyboard: None,
            input_field_placeholder: None,
            selective: None,
        }
    }

    /// Array of button rows, each represented by an Array of [`crate::types::KeyboardButton`] objects
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn keyboards<T: Into<Box<[Box<[crate::types::KeyboardButton]>]>>>(
        mut self,
        val: T,
    ) -> Self {
        self.keyboard = self
            .keyboard
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Array of button rows, each represented by an Array of [`crate::types::KeyboardButton`] objects
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn keyboard<T: Into<Box<[crate::types::KeyboardButton]>>>(mut self, val: T) -> Self {
        self.keyboard = self
            .keyboard
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to `false`, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    #[must_use]
    pub fn is_persistent<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_persistent = Some(val.into());
        self
    }

    /// Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to `false`, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    #[must_use]
    pub fn is_persistent_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_persistent = val.map(Into::into);
        self
    }

    /// Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to `false`, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[must_use]
    pub fn resize_keyboard<T: Into<bool>>(mut self, val: T) -> Self {
        self.resize_keyboard = Some(val.into());
        self
    }

    /// Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to `false`, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[must_use]
    pub fn resize_keyboard_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.resize_keyboard = val.map(Into::into);
        self
    }

    /// Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to `false`.
    #[must_use]
    pub fn one_time_keyboard<T: Into<bool>>(mut self, val: T) -> Self {
        self.one_time_keyboard = Some(val.into());
        self
    }

    /// Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to `false`.
    #[must_use]
    pub fn one_time_keyboard_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.one_time_keyboard = val.map(Into::into);
        self
    }

    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    #[must_use]
    pub fn input_field_placeholder<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.input_field_placeholder = Some(val.into());
        self
    }

    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    #[must_use]
    pub fn input_field_placeholder_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.input_field_placeholder = val.map(Into::into);
        self
    }

    /// Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language. Other users in the group don't see the keyboard.
    #[must_use]
    pub fn selective<T: Into<bool>>(mut self, val: T) -> Self {
        self.selective = Some(val.into());
        self
    }

    /// Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language. Other users in the group don't see the keyboard.
    #[must_use]
    pub fn selective_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.selective = val.map(Into::into);
        self
    }
}
