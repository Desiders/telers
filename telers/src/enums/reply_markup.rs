use crate::types::ReplyMarkup;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents available reply markup variants.
/// Currently, it can be one of
/// - [`ForceReply`]
/// - [`InlineKeyboardMarkup`]
/// - [`ReplyKeyboardMarkup`]
/// - [`ReplyKeyboardRemove`]
/// # Documentation
/// <https://core.telegram.org/bots/api>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum ReplyMarkupType {
    #[strum(serialize = "reply_keyboard_markup")]
    ReplyKeyboardMarkup,
    #[strum(serialize = "force_reply")]
    ForceReply,
    #[strum(serialize = "reply_keyboard_remove")]
    ReplyKeyboardRemove,
    #[strum(serialize = "inline_keyboard_markup")]
    InlineKeyboardMarkup,
}
impl ReplyMarkupType {
    #[must_use]
    pub const fn all() -> [ReplyMarkupType; 4usize] {
        [
            ReplyMarkupType::ReplyKeyboardMarkup,
            ReplyMarkupType::ForceReply,
            ReplyMarkupType::ReplyKeyboardRemove,
            ReplyMarkupType::InlineKeyboardMarkup,
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
            ReplyMarkup::ReplyKeyboardRemove(_) => ReplyMarkupType::ReplyKeyboardRemove,
            ReplyMarkup::InlineKeyboardMarkup(_) => ReplyMarkupType::InlineKeyboardMarkup,
        }
    }
}
impl From<ReplyMarkup> for ReplyMarkupType {
    fn from(val: ReplyMarkup) -> Self {
        ReplyMarkupType::from(&val)
    }
}
