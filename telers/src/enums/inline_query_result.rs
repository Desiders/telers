use crate::types::InlineQueryResult;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
/// - [`crate::types::InlineQueryResultCachedAudio`]
/// - [`crate::types::InlineQueryResultCachedDocument`]
/// - [`crate::types::InlineQueryResultCachedGif`]
/// - [`crate::types::InlineQueryResultCachedMpeg4Gif`]
/// - [`crate::types::InlineQueryResultCachedPhoto`]
/// - [`crate::types::InlineQueryResultCachedSticker`]
/// - [`crate::types::InlineQueryResultCachedVideo`]
/// - [`crate::types::InlineQueryResultCachedVoice`]
/// - [`crate::types::InlineQueryResultArticle`]
/// - [`crate::types::InlineQueryResultAudio`]
/// - [`crate::types::InlineQueryResultContact`]
/// - [`crate::types::InlineQueryResultGame`]
/// - [`crate::types::InlineQueryResultDocument`]
/// - [`crate::types::InlineQueryResultGif`]
/// - [`crate::types::InlineQueryResultLocation`]
/// - [`crate::types::InlineQueryResultMpeg4Gif`]
/// - [`crate::types::InlineQueryResultPhoto`]
/// - [`crate::types::InlineQueryResultVenue`]
/// - [`crate::types::InlineQueryResultVideo`]
/// - [`crate::types::InlineQueryResultVoice`]
///
/// Note: All URLs passed in inline query results will be available to end users and therefore must be assumed to be public.
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
pub enum InlineQueryResultType {
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "gif")]
    Gif,
    #[strum(serialize = "mpeg4_gif")]
    Mpeg4Gif,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "voice")]
    Voice,
    #[strum(serialize = "article")]
    Article,
    #[strum(serialize = "contact")]
    Contact,
    #[strum(serialize = "game")]
    Game,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "venue")]
    Venue,
}
impl InlineQueryResultType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultType; 13usize] {
        [
            InlineQueryResultType::Audio,
            InlineQueryResultType::Document,
            InlineQueryResultType::Gif,
            InlineQueryResultType::Mpeg4Gif,
            InlineQueryResultType::Photo,
            InlineQueryResultType::Sticker,
            InlineQueryResultType::Video,
            InlineQueryResultType::Voice,
            InlineQueryResultType::Article,
            InlineQueryResultType::Contact,
            InlineQueryResultType::Game,
            InlineQueryResultType::Location,
            InlineQueryResultType::Venue,
        ]
    }
}
impl From<InlineQueryResultType> for Box<str> {
    fn from(val: InlineQueryResultType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<InlineQueryResultType> for String {
    fn from(val: InlineQueryResultType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for InlineQueryResultType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a InlineQueryResult> for InlineQueryResultType {
    fn from(val: &'a InlineQueryResult) -> Self {
        match val {
            InlineQueryResult::Audio(_) => InlineQueryResultType::Audio,
            InlineQueryResult::Document(_) => InlineQueryResultType::Document,
            InlineQueryResult::Gif(_) => InlineQueryResultType::Gif,
            InlineQueryResult::Mpeg4Gif(_) => InlineQueryResultType::Mpeg4Gif,
            InlineQueryResult::Photo(_) => InlineQueryResultType::Photo,
            InlineQueryResult::Sticker(_) => InlineQueryResultType::Sticker,
            InlineQueryResult::Video(_) => InlineQueryResultType::Video,
            InlineQueryResult::Voice(_) => InlineQueryResultType::Voice,
            InlineQueryResult::Article(_) => InlineQueryResultType::Article,
            InlineQueryResult::Contact(_) => InlineQueryResultType::Contact,
            InlineQueryResult::Game(_) => InlineQueryResultType::Game,
            InlineQueryResult::Location(_) => InlineQueryResultType::Location,
            InlineQueryResult::Venue(_) => InlineQueryResultType::Venue,
        }
    }
}
impl From<InlineQueryResult> for InlineQueryResultType {
    fn from(val: InlineQueryResult) -> Self {
        InlineQueryResultType::from(&val)
    }
}
