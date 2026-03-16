use crate::types::OwnedGift;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes a gift received and owned by a user or a chat. Currently, it can be one of
/// - [`crate::types::OwnedGiftRegular`]
/// - [`crate::types::OwnedGiftUnique`]
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgift>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum OwnedGiftType {
    #[strum(serialize = "regular")]
    Regular,
    #[strum(serialize = "unique")]
    Unique,
}
impl OwnedGiftType {
    #[must_use]
    pub const fn all() -> [OwnedGiftType; 2usize] {
        [OwnedGiftType::Regular, OwnedGiftType::Unique]
    }
}
impl From<OwnedGiftType> for Box<str> {
    fn from(val: OwnedGiftType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<OwnedGiftType> for String {
    fn from(val: OwnedGiftType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for OwnedGiftType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a OwnedGift> for OwnedGiftType {
    fn from(val: &'a OwnedGift) -> Self {
        match val {
            OwnedGift::Regular(_) => OwnedGiftType::Regular,
            OwnedGift::Unique(_) => OwnedGiftType::Unique,
        }
    }
}
impl From<OwnedGift> for OwnedGiftType {
    fn from(val: OwnedGift) -> Self {
        OwnedGiftType::from(&val)
    }
}
