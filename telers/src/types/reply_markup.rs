use serde::{Deserialize, Serialize};
/// This object represents available reply markup variants.
/// Currently, it can be one of
/// - [`ForceReply`]
/// - [`InlineKeyboardMarkup`]
/// - [`ReplyKeyboardMarkup`]
/// - [`ReplyKeyboardRemove`]
/// # Documentation
/// <https://core.telegram.org/bots/api>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    ReplyKeyboardMarkup(crate::types::ReplyKeyboardMarkup),
    ForceReply(crate::types::ForceReply),
    ReplyKeyboardRemove(crate::types::ReplyKeyboardRemove),
    InlineKeyboardMarkup(crate::types::InlineKeyboardMarkup),
}
impl ReplyMarkup {
    /// Helper method for field `force_reply`.
    ///
    /// Shows reply interface to the user, as if they manually selected the bot's message and tapped 'Reply'
    #[must_use]
    pub fn force_reply(&self) -> Option<bool> {
        match self {
            Self::ForceReply(val) => Some(val.force_reply),
            _ => None,
        }
    }

    /// Helper method for field `inline_keyboard`.
    ///
    /// Array of button rows, each represented by an Array of [`InlineKeyboardButton`] objects
    #[must_use]
    pub fn inline_keyboard(&self) -> Option<&[Box<[crate::types::InlineKeyboardButton]>]> {
        match self {
            Self::InlineKeyboardMarkup(val) => Some(val.inline_keyboard.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `input_field_placeholder`.
    ///
    /// # Variants
    /// - `ReplyKeyboardMarkup`. The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    /// - `ForceReply`. The placeholder to be shown in the input field when the reply is active; 1-64 characters
    #[must_use]
    pub fn input_field_placeholder(&self) -> Option<&str> {
        match self {
            Self::ReplyKeyboardMarkup(val) => val.input_field_placeholder.as_deref(),
            Self::ForceReply(val) => val.input_field_placeholder.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `is_persistent`.
    ///
    /// Requests clients to always show the keyboard when the regular keyboard is hidden. Defaults to false, in which case the custom keyboard can be hidden and opened with a keyboard icon.
    #[must_use]
    pub fn is_persistent(&self) -> Option<bool> {
        match self {
            Self::ReplyKeyboardMarkup(val) => val.is_persistent,
            _ => None,
        }
    }

    /// Helper method for field `keyboard`.
    ///
    /// Array of button rows, each represented by an Array of [`KeyboardButton`] objects
    #[must_use]
    pub fn keyboard(&self) -> Option<&[Box<[crate::types::KeyboardButton]>]> {
        match self {
            Self::ReplyKeyboardMarkup(val) => Some(val.keyboard.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `one_time_keyboard`.
    ///
    /// Requests clients to hide the keyboard as soon as it's been used. The keyboard will still be available, but clients will automatically display the usual letter-keyboard in the chat - the user can press a special button in the input field to see the custom keyboard again. Defaults to false.
    #[must_use]
    pub fn one_time_keyboard(&self) -> Option<bool> {
        match self {
            Self::ReplyKeyboardMarkup(val) => val.one_time_keyboard,
            _ => None,
        }
    }

    /// Helper method for field `remove_keyboard`.
    ///
    /// Requests clients to remove the custom keyboard (user will not be able to summon this keyboard; if you want to hide the keyboard from sight but keep it accessible, use `one_time_keyboard` in [`ReplyKeyboardMarkup`])
    #[must_use]
    pub fn remove_keyboard(&self) -> Option<bool> {
        match self {
            Self::ReplyKeyboardRemove(val) => Some(val.remove_keyboard),
            _ => None,
        }
    }

    /// Helper method for field `resize_keyboard`.
    ///
    /// Requests clients to resize the keyboard vertically for optimal fit (e.g., make the keyboard smaller if there are just two rows of buttons). Defaults to false, in which case the custom keyboard is always of the same height as the app's standard keyboard.
    #[must_use]
    pub fn resize_keyboard(&self) -> Option<bool> {
        match self {
            Self::ReplyKeyboardMarkup(val) => val.resize_keyboard,
            _ => None,
        }
    }

    /// Helper method for field `selective`.
    ///
    /// # Variants
    /// - `ReplyKeyboardMarkup`. Use this parameter if you want to show the keyboard to specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user requests to change the bot's language, bot replies to the request with a keyboard to select the new language. Other users in the group don't see the keyboard.
    /// - `ForceReply`. Use this parameter if you want to force reply from specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message.
    /// - `ReplyKeyboardRemove`. Use this parameter if you want to remove the keyboard for specific users only. Targets: 1) users that are @mentioned in the text of the Message object; 2) if the bot's message is a reply to a message in the same chat and forum topic, sender of the original message. Example: A user votes in a poll, bot returns confirmation message in reply to the vote and removes the keyboard for that user, while still showing the keyboard with poll options to users who haven't voted yet.
    #[must_use]
    pub fn selective(&self) -> Option<bool> {
        match self {
            Self::ReplyKeyboardMarkup(val) => val.selective,
            Self::ForceReply(val) => val.selective,
            Self::ReplyKeyboardRemove(val) => val.selective,
            Self::InlineKeyboardMarkup(_) => None,
        }
    }
}
impl From<crate::types::ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(val: crate::types::ReplyKeyboardMarkup) -> Self {
        Self::ReplyKeyboardMarkup(val)
    }
}
impl TryFrom<ReplyMarkup> for crate::types::ReplyKeyboardMarkup {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ReplyMarkup) -> Result<Self, Self::Error> {
        if let ReplyMarkup::ReplyKeyboardMarkup(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ReplyMarkup),
                stringify!(ReplyKeyboardMarkup),
            ))
        }
    }
}
impl From<crate::types::ForceReply> for ReplyMarkup {
    fn from(val: crate::types::ForceReply) -> Self {
        Self::ForceReply(val)
    }
}
impl TryFrom<ReplyMarkup> for crate::types::ForceReply {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ReplyMarkup) -> Result<Self, Self::Error> {
        if let ReplyMarkup::ForceReply(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ReplyMarkup),
                stringify!(ForceReply),
            ))
        }
    }
}
impl From<crate::types::ReplyKeyboardRemove> for ReplyMarkup {
    fn from(val: crate::types::ReplyKeyboardRemove) -> Self {
        Self::ReplyKeyboardRemove(val)
    }
}
impl TryFrom<ReplyMarkup> for crate::types::ReplyKeyboardRemove {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ReplyMarkup) -> Result<Self, Self::Error> {
        if let ReplyMarkup::ReplyKeyboardRemove(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ReplyMarkup),
                stringify!(ReplyKeyboardRemove),
            ))
        }
    }
}
impl From<crate::types::InlineKeyboardMarkup> for ReplyMarkup {
    fn from(val: crate::types::InlineKeyboardMarkup) -> Self {
        Self::InlineKeyboardMarkup(val)
    }
}
impl TryFrom<ReplyMarkup> for crate::types::InlineKeyboardMarkup {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: ReplyMarkup) -> Result<Self, Self::Error> {
        if let ReplyMarkup::InlineKeyboardMarkup(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(ReplyMarkup),
                stringify!(InlineKeyboardMarkup),
            ))
        }
    }
}
