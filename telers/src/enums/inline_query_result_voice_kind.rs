use crate::types::InlineQueryResultVoiceKind;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// # Notes
/// This object represents an inline query result kind as combine of [`crate::types::InlineQueryResultCachedVoice`] and [`crate::types::InlineQueryResultVoice`].
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
pub enum InlineQueryResultVoiceKindType {
    #[strum(serialize = "uncached")]
    Uncached,
    #[strum(serialize = "cached")]
    Cached,
}
impl InlineQueryResultVoiceKindType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultVoiceKindType; 2usize] {
        [
            InlineQueryResultVoiceKindType::Uncached,
            InlineQueryResultVoiceKindType::Cached,
        ]
    }
}
impl From<InlineQueryResultVoiceKindType> for Box<str> {
    fn from(val: InlineQueryResultVoiceKindType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultVoiceKindType> for String {
    fn from(val: InlineQueryResultVoiceKindType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultVoiceKindType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResultVoiceKind> for InlineQueryResultVoiceKindType {
    fn from(val: &'a InlineQueryResultVoiceKind) -> Self {
        match val {
            InlineQueryResultVoiceKind::Uncached(_) => InlineQueryResultVoiceKindType::Uncached,
            InlineQueryResultVoiceKind::Cached(_) => InlineQueryResultVoiceKindType::Cached,
        }
    }
}
impl From<InlineQueryResultVoiceKind> for InlineQueryResultVoiceKindType {
    fn from(val: InlineQueryResultVoiceKind) -> Self {
        InlineQueryResultVoiceKindType::from(&val)
    }
}
