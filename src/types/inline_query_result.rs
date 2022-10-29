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
/// **Note:** All URLs passed in inline query results will be available to end users and therefore must be assumed to be **public**.
/// <https://core.telegram.org/bots/api#inlinequeryresult>_
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
