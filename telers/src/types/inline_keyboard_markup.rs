use serde::{Deserialize, Serialize};
/// This object represents an inline keyboard that appears right next to the message it belongs to.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinekeyboardmarkup>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [`InlineKeyboardButton`] objects
    pub inline_keyboard: Box<[Box<[crate::types::InlineKeyboardButton]>]>,
}
impl InlineKeyboardMarkup {
    /// Creates a new `InlineKeyboardMarkup`.
    ///
    /// # Arguments
    /// * `inline_keyboard` - Array of button rows, each represented by an Array of [`InlineKeyboardButton`] objects
    #[must_use]
    pub fn new<
        T0Item: Into<Box<[crate::types::InlineKeyboardButton]>>,
        T0: IntoIterator<Item = T0Item>,
    >(
        inline_keyboard: T0,
    ) -> Self {
        Self {
            inline_keyboard: inline_keyboard.into_iter().map(Into::into).collect(),
        }
    }

    /// Array of button rows, each represented by an Array of [`InlineKeyboardButton`] objects
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn inline_keyboards<T: Into<Box<[Box<[crate::types::InlineKeyboardButton]>]>>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.inline_keyboard = this
            .inline_keyboard
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// Array of button rows, each represented by an Array of [`InlineKeyboardButton`] objects
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn inline_keyboard<T: Into<Box<[crate::types::InlineKeyboardButton]>>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.inline_keyboard = this
            .inline_keyboard
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
