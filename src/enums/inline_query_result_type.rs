use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the inline query result
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum InlineQueryResultType {
    #[strum(serialize = "article")]
    Article,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "contact")]
    Contact,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "game")]
    Game,
    #[strum(serialize = "gif")]
    Gif,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "mpeg4_gif")]
    Mpeg4Gif,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "video_note")]
    VideoNote,
    #[strum(serialize = "voice")]
    Voice,
}

impl InlineQueryResultType {
    #[must_use]
    pub const fn all() -> [InlineQueryResultType; 14] {
        [
            InlineQueryResultType::Article,
            InlineQueryResultType::Audio,
            InlineQueryResultType::Contact,
            InlineQueryResultType::Document,
            InlineQueryResultType::Game,
            InlineQueryResultType::Gif,
            InlineQueryResultType::Location,
            InlineQueryResultType::Mpeg4Gif,
            InlineQueryResultType::Photo,
            InlineQueryResultType::Sticker,
            InlineQueryResultType::Venue,
            InlineQueryResultType::Video,
            InlineQueryResultType::VideoNote,
            InlineQueryResultType::Voice,
        ]
    }
}

impl From<InlineQueryResultType> for Box<str> {
    fn from(inline_query_result_type: InlineQueryResultType) -> Self {
        Into::<&'static str>::into(inline_query_result_type).into()
    }
}

impl From<InlineQueryResultType> for String {
    fn from(inline_query_result_type: InlineQueryResultType) -> Self {
        inline_query_result_type.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for InlineQueryResultType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
