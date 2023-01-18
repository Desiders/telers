use super::{ForceReply, InlineKeyboardMarkup, ReplyKeyboardMarkup, ReplyKeyboardRemove};

use serde::{Deserialize, Serialize};

/// This object represents all possible types of reply markup
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl ReplyMarkup {
    #[must_use]
    pub fn inline(inline_keyboard_markup: InlineKeyboardMarkup) -> Self {
        Self::InlineKeyboard(inline_keyboard_markup)
    }

    #[must_use]
    pub fn reply(reply_keyboard_markup: ReplyKeyboardMarkup) -> Self {
        Self::ReplyKeyboard(reply_keyboard_markup)
    }

    #[must_use]
    pub fn remove(reply_keyboard_remove: ReplyKeyboardRemove) -> Self {
        Self::ReplyKeyboardRemove(reply_keyboard_remove)
    }

    #[must_use]
    pub fn force(force_reply: ForceReply) -> Self {
        Self::ForceReply(force_reply)
    }
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(inline_keyboard_markup: InlineKeyboardMarkup) -> Self {
        Self::InlineKeyboard(inline_keyboard_markup)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(reply_keyboard_markup: ReplyKeyboardMarkup) -> Self {
        Self::ReplyKeyboard(reply_keyboard_markup)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(reply_keyboard_remove: ReplyKeyboardRemove) -> Self {
        Self::ReplyKeyboardRemove(reply_keyboard_remove)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(force_reply: ForceReply) -> Self {
        Self::ForceReply(force_reply)
    }
}
