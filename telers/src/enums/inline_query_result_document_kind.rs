use crate::types::InlineQueryResultDocumentKind;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// # Notes
/// This object represents an inline query result kind as combine of [`InlineQueryResultCachedDocument`] and [`InlineQueryResultDocument`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum InlineQueryResultDocumentKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultDocumentKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultDocumentKindType; 2usize] {
        [
            InlineQueryResultDocumentKindType::Uncached,
            InlineQueryResultDocumentKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultDocumentKindType> for Box<str> {
    fn from(val: InlineQueryResultDocumentKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultDocumentKindType> for String {
    fn from(val: InlineQueryResultDocumentKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultDocumentKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultDocumentKind> for InlineQueryResultDocumentKindType {
    fn from(val: &'a InlineQueryResultDocumentKind) -> Self {
        match val {
            InlineQueryResultDocumentKind::Uncached(_) => {
                InlineQueryResultDocumentKindType::Uncached
            }
            InlineQueryResultDocumentKind::Cached(_) => InlineQueryResultDocumentKindType::Cached,
        }
    }
}
impl From<InlineQueryResultDocumentKind> for InlineQueryResultDocumentKindType {
    fn from(val: InlineQueryResultDocumentKind) -> Self {
        InlineQueryResultDocumentKindType::from(&val)
    }
}
