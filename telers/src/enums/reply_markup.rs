use crate::types::ReplyMarkup;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents available reply markup variants.
/// Currently, it can be one of
/// - [`crate::types::ForceReply`]
/// - [`crate::types::InlineKeyboardMarkup`]
/// - [`crate::types::ReplyKeyboardMarkup`]
/// - [`crate::types::ReplyKeyboardRemove`]
/// # Documentation
/// <https://core.telegram.org/bots/api>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum ReplyMarkupType {
    #[strum(serialize = "reply_keyboard_markup")]
    ReplyKeyboardMarkup,
    #[strum(serialize = "force_reply")]
    ForceReply,
    #[strum(serialize = "inline_keyboard_markup")]
    InlineKeyboardMarkup,
    #[strum(serialize = "reply_keyboard_remove")]
    ReplyKeyboardRemove,
}
impl ReplyMarkupType {
    #[must_use]
    pub const fn all() -> [ReplyMarkupType; 4usize] {
        [
            ReplyMarkupType::ReplyKeyboardMarkup,
            ReplyMarkupType::ForceReply,
            ReplyMarkupType::InlineKeyboardMarkup,
            ReplyMarkupType::ReplyKeyboardRemove,
        ]
    }
}
impl From<ReplyMarkupType> for Box<str> {
    fn from(val: ReplyMarkupType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ReplyMarkupType> for String {
    fn from(val: ReplyMarkupType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ReplyMarkupType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ReplyMarkup> for ReplyMarkupType {
    fn from(val: &'a ReplyMarkup) -> Self {
        match val {
            ReplyMarkup::ReplyKeyboardMarkup(_) => ReplyMarkupType::ReplyKeyboardMarkup,
            ReplyMarkup::ForceReply(_) => ReplyMarkupType::ForceReply,
            ReplyMarkup::InlineKeyboardMarkup(_) => ReplyMarkupType::InlineKeyboardMarkup,
            ReplyMarkup::ReplyKeyboardRemove(_) => ReplyMarkupType::ReplyKeyboardRemove,
        }
    }
}
impl From<ReplyMarkup> for ReplyMarkupType {
    fn from(val: ReplyMarkup) -> Self {
        ReplyMarkupType::from(&val)
    }
}
