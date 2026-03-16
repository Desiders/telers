use crate::types::ChatBoostSource;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the source of a chat boost. It can be one of
/// - [`crate::types::ChatBoostSourcePremium`]
/// - [`crate::types::ChatBoostSourceGiftCode`]
/// - [`crate::types::ChatBoostSourceGiveaway`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsource>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum ChatBoostSourceType {
    #[strum(serialize = "premium")]
    Premium,
    #[strum(serialize = "gift_code")]
    GiftCode,
    #[strum(serialize = "giveaway")]
    Giveaway,
}
impl ChatBoostSourceType {
    #[must_use]
    pub const fn all() -> [ChatBoostSourceType; 3usize] {
        [
            ChatBoostSourceType::Premium,
            ChatBoostSourceType::GiftCode,
            ChatBoostSourceType::Giveaway,
        ]
    }
}
impl From<ChatBoostSourceType> for Box<str> {
    fn from(val: ChatBoostSourceType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<ChatBoostSourceType> for String {
    fn from(val: ChatBoostSourceType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for ChatBoostSourceType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a ChatBoostSource> for ChatBoostSourceType {
    fn from(val: &'a ChatBoostSource) -> Self {
        match val {
            ChatBoostSource::Premium(_) => ChatBoostSourceType::Premium,
            ChatBoostSource::GiftCode(_) => ChatBoostSourceType::GiftCode,
            ChatBoostSource::Giveaway(_) => ChatBoostSourceType::Giveaway,
        }
    }
}
impl From<ChatBoostSource> for ChatBoostSourceType {
    fn from(val: ChatBoostSource) -> Self {
        ChatBoostSourceType::from(&val)
    }
}
