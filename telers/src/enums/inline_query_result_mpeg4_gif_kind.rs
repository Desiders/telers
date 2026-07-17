use crate::types::InlineQueryResultMpeg4GifKind;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedMpeg4Gif`] and [`crate::types::InlineQueryResultMpeg4Gif`].
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
pub enum InlineQueryResultMpeg4GifKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultMpeg4GifKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultMpeg4GifKindType; 2usize] {
        [
            InlineQueryResultMpeg4GifKindType::Uncached,
            InlineQueryResultMpeg4GifKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultMpeg4GifKindType> for Box<str> {
    fn from(val: InlineQueryResultMpeg4GifKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultMpeg4GifKindType> for String {
    fn from(val: InlineQueryResultMpeg4GifKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultMpeg4GifKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultMpeg4GifKind> for InlineQueryResultMpeg4GifKindType {
    fn from(val: &'a InlineQueryResultMpeg4GifKind) -> Self {
        match val {
            InlineQueryResultMpeg4GifKind::Uncached(_) => {
                InlineQueryResultMpeg4GifKindType::Uncached
            }
            InlineQueryResultMpeg4GifKind::Cached(_) => InlineQueryResultMpeg4GifKindType::Cached,
        }
    }
}
impl From<InlineQueryResultMpeg4GifKind> for InlineQueryResultMpeg4GifKindType {
    fn from(val: InlineQueryResultMpeg4GifKind) -> Self {
        InlineQueryResultMpeg4GifKindType::from(&val)
    }
}
