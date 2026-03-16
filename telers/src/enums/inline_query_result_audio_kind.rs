use crate::types::InlineQueryResultAudioKind;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedAudio`] and [`crate::types::InlineQueryResultAudio`].
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum InlineQueryResultAudioKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultAudioKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultAudioKindType; 2usize] {
        [
            InlineQueryResultAudioKindType::Uncached,
            InlineQueryResultAudioKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultAudioKindType> for Box<str> {
    fn from(val: InlineQueryResultAudioKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultAudioKindType> for String {
    fn from(val: InlineQueryResultAudioKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultAudioKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultAudioKind> for InlineQueryResultAudioKindType {
    fn from(val: &'a InlineQueryResultAudioKind) -> Self {
        match val {
            InlineQueryResultAudioKind::Uncached(_) => InlineQueryResultAudioKindType::Uncached,
            InlineQueryResultAudioKind::Cached(_) => InlineQueryResultAudioKindType::Cached,
        }
    }
}
impl From<InlineQueryResultAudioKind> for InlineQueryResultAudioKindType {
    fn from(val: InlineQueryResultAudioKind) -> Self {
        InlineQueryResultAudioKindType::from(&val)
    }
}
