use crate::types::InlineQueryResultPhotoKind;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedPhoto`] and [`crate::types::InlineQueryResultPhoto`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
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
pub enum InlineQueryResultPhotoKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultPhotoKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultPhotoKindType; 2usize] {
        [
            InlineQueryResultPhotoKindType::Uncached,
            InlineQueryResultPhotoKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultPhotoKindType> for Box<str> {
    fn from(val: InlineQueryResultPhotoKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultPhotoKindType> for String {
    fn from(val: InlineQueryResultPhotoKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultPhotoKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultPhotoKind> for InlineQueryResultPhotoKindType {
    fn from(val: &'a InlineQueryResultPhotoKind) -> Self {
        match val {
            InlineQueryResultPhotoKind::Uncached(_) => InlineQueryResultPhotoKindType::Uncached,
            InlineQueryResultPhotoKind::Cached(_) => InlineQueryResultPhotoKindType::Cached,
        }
    }
}
impl From<InlineQueryResultPhotoKind> for InlineQueryResultPhotoKindType {
    fn from(val: InlineQueryResultPhotoKind) -> Self {
        InlineQueryResultPhotoKindType::from(&val)
    }
}
