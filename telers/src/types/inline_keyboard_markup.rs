use serde::{Deserialize, Serialize};
/// This object represents an inline keyboard that appears right next to the message it belongs to.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinekeyboardmarkup>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [`crate::types::InlineKeyboardButton`] objects
    pub inline_keyboard: Box<[Box<[crate::types::InlineKeyboardButton]>]>,
    /// Pass `true` if the reply interface must be shown to the user, as if they had manually selected the bot's message and tapped 'Reply'. The value of the field can't be changed when the inline keyboard is edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_reply: Option<bool>,
}
impl InlineKeyboardMarkup {
    /// Creates a new `InlineKeyboardMarkup`.
    ///
    /// # Arguments
    /// * `inline_keyboard` - Array of button rows, each represented by an Array of [`crate::types::InlineKeyboardButton`] objects
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0Item: Into<Box<[crate::types::InlineKeyboardButton]>>,
        T0: IntoIterator<Item = T0Item>,
    >(
        inline_keyboard: T0,
    ) -> Self {
        Self {
            inline_keyboard: inline_keyboard.into_iter().map(Into::into).collect(),
            force_reply: None,
        }
    }

    /// Array of button rows, each represented by an Array of [`crate::types::InlineKeyboardButton`] objects
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn inline_keyboards<T: Into<Box<[Box<[crate::types::InlineKeyboardButton]>]>>>(
        mut self,
        val: T,
    ) -> Self {
        self.inline_keyboard = self
            .inline_keyboard
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Array of button rows, each represented by an Array of [`crate::types::InlineKeyboardButton`] objects
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn inline_keyboard<T: Into<Box<[crate::types::InlineKeyboardButton]>>>(
        mut self,
        val: T,
    ) -> Self {
        self.inline_keyboard = self
            .inline_keyboard
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Pass `true` if the reply interface must be shown to the user, as if they had manually selected the bot's message and tapped 'Reply'. The value of the field can't be changed when the inline keyboard is edited.
    #[must_use]
    pub fn force_reply<T: Into<bool>>(mut self, val: T) -> Self {
        self.force_reply = Some(val.into());
        self
    }

    /// Pass `true` if the reply interface must be shown to the user, as if they had manually selected the bot's message and tapped 'Reply'. The value of the field can't be changed when the inline keyboard is edited.
    #[must_use]
    pub fn force_reply_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.force_reply = val.map(Into::into);
        self
    }
}
