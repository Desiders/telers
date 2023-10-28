use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};

/// This enum represents all possible types of the inline query result
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InlineQueryResultType {
    Article,
    Audio,
    Contact,
    Document,
    Game,
    Gif,
    Location,
    Mpeg4Gif,
    Photo,
    Sticker,
    Venue,
    Video,
    VideoNote,
    Voice,
}

impl InlineQueryResultType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            InlineQueryResultType::Article => "article",
            InlineQueryResultType::Audio => "audio",
            InlineQueryResultType::Contact => "contact",
            InlineQueryResultType::Document => "document",
            InlineQueryResultType::Game => "game",
            InlineQueryResultType::Gif => "gif",
            InlineQueryResultType::Location => "location",
            InlineQueryResultType::Mpeg4Gif => "mpeg4_gif",
            InlineQueryResultType::Photo => "photo",
            InlineQueryResultType::Sticker => "sticker",
            InlineQueryResultType::Venue => "venue",
            InlineQueryResultType::Video => "video",
            InlineQueryResultType::VideoNote => "video_note",
            InlineQueryResultType::Voice => "voice",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [InlineQueryResultType; 14] {
        &[
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

impl Deref for InlineQueryResultType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for InlineQueryResultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<InlineQueryResultType> for Box<str> {
    fn from(inline_query_result_type: InlineQueryResultType) -> Self {
        inline_query_result_type.into()
    }
}

impl From<InlineQueryResultType> for String {
    fn from(inline_query_result_type: InlineQueryResultType) -> Self {
        inline_query_result_type.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for InlineQueryResultType {
    fn eq(&self, other: &&'a str) -> bool {
        self == other
    }
}
