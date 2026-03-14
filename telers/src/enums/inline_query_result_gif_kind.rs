use crate::types::InlineQueryResultGifKind;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedGif`] and [`crate::types::InlineQueryResultGif`].
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
pub enum InlineQueryResultGifKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultGifKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultGifKindType; 2usize] {
        [
            InlineQueryResultGifKindType::Uncached,
            InlineQueryResultGifKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultGifKindType> for Box<str> {
    fn from(val: InlineQueryResultGifKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultGifKindType> for String {
    fn from(val: InlineQueryResultGifKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultGifKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultGifKind> for InlineQueryResultGifKindType {
    fn from(val: &'a InlineQueryResultGifKind) -> Self {
        match val {
            InlineQueryResultGifKind::Uncached(_) => InlineQueryResultGifKindType::Uncached,
            InlineQueryResultGifKind::Cached(_) => InlineQueryResultGifKindType::Cached,
        }
    }
}
impl From<InlineQueryResultGifKind> for InlineQueryResultGifKindType {
    fn from(val: InlineQueryResultGifKind) -> Self {
        InlineQueryResultGifKindType::from(&val)
    }
}
