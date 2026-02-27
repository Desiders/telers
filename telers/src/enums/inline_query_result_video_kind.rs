use crate::types::InlineQueryResultVideoKind;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// # Notes
/// This object represents an inline query result kind as combine of [`InlineQueryResultCachedVideo`] and [`InlineQueryResultVideo`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum InlineQueryResultVideoKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultVideoKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultVideoKindType; 2usize] {
        [
            InlineQueryResultVideoKindType::Uncached,
            InlineQueryResultVideoKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultVideoKindType> for Box<str> {
    fn from(val: InlineQueryResultVideoKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultVideoKindType> for String {
    fn from(val: InlineQueryResultVideoKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultVideoKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultVideoKind> for InlineQueryResultVideoKindType {
    fn from(val: &'a InlineQueryResultVideoKind) -> Self {
        match val {
            InlineQueryResultVideoKind::Uncached(_) => InlineQueryResultVideoKindType::Uncached,
            InlineQueryResultVideoKind::Cached(_) => InlineQueryResultVideoKindType::Cached,
        }
    }
}
impl From<InlineQueryResultVideoKind> for InlineQueryResultVideoKindType {
    fn from(val: InlineQueryResultVideoKind) -> Self {
        InlineQueryResultVideoKindType::from(&val)
    }
}
