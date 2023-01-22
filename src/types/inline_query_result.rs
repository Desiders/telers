use crate::types::{
    InlineQueryResultArticle, InlineQueryResultAudio, InlineQueryResultCachedAudio,
    InlineQueryResultCachedDocument, InlineQueryResultCachedGif, InlineQueryResultCachedMpeg4Gif,
    InlineQueryResultCachedPhoto, InlineQueryResultCachedSticker, InlineQueryResultCachedVideo,
    InlineQueryResultCachedVoice, InlineQueryResultContact, InlineQueryResultDocument,
    InlineQueryResultGame, InlineQueryResultGif, InlineQueryResultLocation,
    InlineQueryResultMpeg4Gif, InlineQueryResultPhoto, InlineQueryResultVenue,
    InlineQueryResultVideo, InlineQueryResultVoice,
};

use serde::{Deserialize, Serialize};

/// This object represents one result of an inline query. Telegram clients currently support results of the following 20 types:
/// - `aiogram_rs.types.inline_query_result_cached_audio.InlineQueryResultCachedAudio`
/// - `aiogram_rs.types.inline_query_result_cached_document.InlineQueryResultCachedDocument`
/// - `aiogram_rs.types.inline_query_result_cached_gif.InlineQueryResultCachedGif`
/// - `aiogram_rs.types.inline_query_result_cached_mpeg4_gif.InlineQueryResultCachedMpeg4Gif`
/// - `aiogram_rs.types.inline_query_result_cached_photo.InlineQueryResultCachedPhoto`
/// - `aiogram_rs.types.inline_query_result_cached_sticker.InlineQueryResultCachedSticker`
/// - `aiogram_rs.types.inline_query_result_cached_video.InlineQueryResultCachedVideo`
/// - `aiogram_rs.types.inline_query_result_cached_voice.InlineQueryResultCachedVoice`
/// - `aiogram_rs.types.inline_query_result_article.InlineQueryResultArticle`
/// - `aiogram_rs.types.inline_query_result_audio.InlineQueryResultAudio`
/// - `aiogram_rs.types.inline_query_result_contact.InlineQueryResultContact`
/// - `aiogram_rs.types.inline_query_result_game.InlineQueryResultGame`
/// - `aiogram_rs.types.inline_query_result_document.InlineQueryResultDocument`
/// - `aiogram_rs.types.inline_query_result_gif.InlineQueryResultGif`
/// - `aiogram_rs.types.inline_query_result_location.InlineQueryResultLocation`
/// - `aiogram_rs.types.inline_query_result_mpeg4_gif.InlineQueryResultMpeg4Gif`
/// - `aiogram_rs.types.inline_query_result_photo.InlineQueryResultPhoto`
/// - `aiogram_rs.types.inline_query_result_venue.InlineQueryResultVenue`
/// - `aiogram_rs.types.inline_query_result_video.InlineQueryResultVideo`
/// - `aiogram_rs.types.inline_query_result_voice.InlineQueryResultVoice`
/// # Notes
/// All URLs passed in inline query results will be available to end users and therefore must be assumed to be **public**.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InlineQueryResult {
    #[serde(rename = "audio")]
    CachedAudio(InlineQueryResultCachedAudio),
    #[serde(rename = "document")]
    CachedDocument(InlineQueryResultCachedDocument),
    #[serde(rename = "gif")]
    CachedGif(InlineQueryResultCachedGif),
    #[serde(rename = "mpeg4_gif")]
    CachedMpeg4Gif(InlineQueryResultCachedMpeg4Gif),
    #[serde(rename = "photo")]
    CachedPhoto(InlineQueryResultCachedPhoto),
    #[serde(rename = "sticker")]
    CachedSticker(InlineQueryResultCachedSticker),
    #[serde(rename = "video")]
    CachedVideo(InlineQueryResultCachedVideo),
    #[serde(rename = "voice")]
    CachedVoice(InlineQueryResultCachedVoice),
    Article(InlineQueryResultArticle),
    Audio(InlineQueryResultAudio),
    Contact(InlineQueryResultContact),
    Game(InlineQueryResultGame),
    Document(InlineQueryResultDocument),
    Gif(InlineQueryResultGif),
    Location(InlineQueryResultLocation),
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Photo(InlineQueryResultPhoto),
    Venue(InlineQueryResultVenue),
    Video(InlineQueryResultVideo),
    Voice(InlineQueryResultVoice),
}

impl From<InlineQueryResultCachedAudio> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedAudio) -> Self {
        InlineQueryResult::CachedAudio(result)
    }
}

impl From<InlineQueryResultCachedDocument> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedDocument) -> Self {
        InlineQueryResult::CachedDocument(result)
    }
}

impl From<InlineQueryResultCachedGif> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedGif) -> Self {
        InlineQueryResult::CachedGif(result)
    }
}

impl From<InlineQueryResultCachedMpeg4Gif> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedMpeg4Gif) -> Self {
        InlineQueryResult::CachedMpeg4Gif(result)
    }
}

impl From<InlineQueryResultCachedPhoto> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedPhoto) -> Self {
        InlineQueryResult::CachedPhoto(result)
    }
}

impl From<InlineQueryResultCachedSticker> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedSticker) -> Self {
        InlineQueryResult::CachedSticker(result)
    }
}

impl From<InlineQueryResultCachedVideo> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedVideo) -> Self {
        InlineQueryResult::CachedVideo(result)
    }
}

impl From<InlineQueryResultCachedVoice> for InlineQueryResult {
    fn from(result: InlineQueryResultCachedVoice) -> Self {
        InlineQueryResult::CachedVoice(result)
    }
}

impl From<InlineQueryResultArticle> for InlineQueryResult {
    fn from(result: InlineQueryResultArticle) -> Self {
        InlineQueryResult::Article(result)
    }
}

impl From<InlineQueryResultAudio> for InlineQueryResult {
    fn from(result: InlineQueryResultAudio) -> Self {
        InlineQueryResult::Audio(result)
    }
}

impl From<InlineQueryResultContact> for InlineQueryResult {
    fn from(result: InlineQueryResultContact) -> Self {
        InlineQueryResult::Contact(result)
    }
}

impl From<InlineQueryResultGame> for InlineQueryResult {
    fn from(result: InlineQueryResultGame) -> Self {
        InlineQueryResult::Game(result)
    }
}

impl From<InlineQueryResultDocument> for InlineQueryResult {
    fn from(result: InlineQueryResultDocument) -> Self {
        InlineQueryResult::Document(result)
    }
}

impl From<InlineQueryResultGif> for InlineQueryResult {
    fn from(result: InlineQueryResultGif) -> Self {
        InlineQueryResult::Gif(result)
    }
}

impl From<InlineQueryResultLocation> for InlineQueryResult {
    fn from(result: InlineQueryResultLocation) -> Self {
        InlineQueryResult::Location(result)
    }
}

impl From<InlineQueryResultMpeg4Gif> for InlineQueryResult {
    fn from(result: InlineQueryResultMpeg4Gif) -> Self {
        InlineQueryResult::Mpeg4Gif(result)
    }
}

impl From<InlineQueryResultPhoto> for InlineQueryResult {
    fn from(result: InlineQueryResultPhoto) -> Self {
        InlineQueryResult::Photo(result)
    }
}

impl From<InlineQueryResultVenue> for InlineQueryResult {
    fn from(result: InlineQueryResultVenue) -> Self {
        InlineQueryResult::Venue(result)
    }
}

impl From<InlineQueryResultVideo> for InlineQueryResult {
    fn from(result: InlineQueryResultVideo) -> Self {
        InlineQueryResult::Video(result)
    }
}

impl From<InlineQueryResultVoice> for InlineQueryResult {
    fn from(result: InlineQueryResultVoice) -> Self {
        InlineQueryResult::Voice(result)
    }
}
