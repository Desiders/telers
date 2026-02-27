use crate::types::PaidMedia;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes paid media. Currently, it can be one of
/// - [`PaidMediaPreview`]
/// - [`PaidMediaPhoto`]
/// - [`PaidMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmedia>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum PaidMediaType {
    #[strum(serialize = "preview")]
    Preview,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "video")]
    Video,
}
impl PaidMediaType {
    #[must_use]
    pub const fn all() -> [PaidMediaType; 3usize] {
        [
            PaidMediaType::Preview,
            PaidMediaType::Photo,
            PaidMediaType::Video,
        ]
    }
}
impl From<PaidMediaType> for Box<str> {
    fn from(val: PaidMediaType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<PaidMediaType> for String {
    fn from(val: PaidMediaType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for PaidMediaType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a PaidMedia> for PaidMediaType {
    fn from(val: &'a PaidMedia) -> Self {
        match val {
            PaidMedia::Preview(_) => PaidMediaType::Preview,
            PaidMedia::Photo(_) => PaidMediaType::Photo,
            PaidMedia::Video(_) => PaidMediaType::Video,
        }
    }
}
impl From<PaidMedia> for PaidMediaType {
    fn from(val: PaidMedia) -> Self {
        PaidMediaType::from(&val)
    }
}
