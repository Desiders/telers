use super::InlineKeyboardButton;

use serde::{Deserialize, Serialize};

/// This object represents an [`inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) that appears right next to the message it belongs to.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinekeyboardmarkup>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct InlineKeyboardMarkup {
    /// Array of button rows, each represented by an Array of [`InlineKeyboardButton`] objects
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    #[must_use]
    pub fn new<T, I>(inline_keyboard: I) -> Self
    where
        T: IntoIterator<Item = InlineKeyboardButton>,
        I: IntoIterator<Item = T>,
    {
        Self {
            inline_keyboard: inline_keyboard
                .into_iter()
                .map(|val| val.into_iter().collect())
                .collect(),
        }
    }

    #[must_use]
    pub fn empty() -> Self {
        Self::new([[]])
    }

    #[must_use]
    pub fn inline_keyboard<T, I>(self, val: I) -> Self
    where
        T: IntoIterator<Item = InlineKeyboardButton>,
        I: IntoIterator<Item = T>,
    {
        Self {
            inline_keyboard: self
                .inline_keyboard
                .into_iter()
                .chain(val.into_iter().map(|val| val.into_iter().collect()))
                .collect(),
        }
    }
}

impl From<Vec<Vec<InlineKeyboardButton>>> for InlineKeyboardMarkup {
    fn from(val: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self {
            inline_keyboard: val,
        }
    }
}

impl From<Vec<InlineKeyboardButton>> for InlineKeyboardMarkup {
    fn from(val: Vec<InlineKeyboardButton>) -> Self {
        Self {
            inline_keyboard: vec![val],
        }
    }
}
