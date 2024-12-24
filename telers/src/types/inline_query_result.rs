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
/// - [`InlineQueryResultCachedAudio`]
/// - [`InlineQueryResultCachedDocument`]
/// - [`InlineQueryResultCachedGif`]
/// - [`InlineQueryResultCachedMpeg4Gif`]
/// - [`InlineQueryResultCachedPhoto`]
/// - [`InlineQueryResultCachedSticker`]
/// - [`InlineQueryResultCachedVideo`]
/// - [`InlineQueryResultCachedVoice`]
/// - [`InlineQueryResultArticle`]
/// - [`InlineQueryResultPhoto`]
/// - [`InlineQueryResultGif`]
/// - [`InlineQueryResultMpeg4Gif`]
/// - [`InlineQueryResultVideo`]
/// - [`InlineQueryResultAudio`]
/// - [`InlineQueryResultVoice`]
/// - [`InlineQueryResultDocument`]
/// - [`InlineQueryResultLocation`]
/// - [`InlineQueryResultVenue`]
/// - [`InlineQueryResultContact`]
/// - [`InlineQueryResultGame`]
///
/// All URLs passed in inline query results will be available to end users and therefore must be assumed to be **public**.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresult>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(
    tag = "type",
    rename_all = "snake_case",
    from = "raw::InlineQueryResult",
    into = "raw::InlineQueryResult"
)]
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
    Venue(InlineQueryResultVenue),
    Location(InlineQueryResultLocation),
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif(InlineQueryResultMpeg4Gif),
    Photo(InlineQueryResultPhoto),
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

mod raw {
    use super::{
        Deserialize, InlineQueryResultArticle, InlineQueryResultAudio,
        InlineQueryResultCachedAudio, InlineQueryResultCachedDocument, InlineQueryResultCachedGif,
        InlineQueryResultCachedMpeg4Gif, InlineQueryResultCachedPhoto,
        InlineQueryResultCachedSticker, InlineQueryResultCachedVideo, InlineQueryResultCachedVoice,
        InlineQueryResultContact, InlineQueryResultDocument, InlineQueryResultGame,
        InlineQueryResultGif, InlineQueryResultLocation, InlineQueryResultMpeg4Gif,
        InlineQueryResultPhoto, InlineQueryResultVenue, InlineQueryResultVideo,
        InlineQueryResultVoice, Serialize,
    };

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum AudioKind {
        Cached(InlineQueryResultCachedAudio),
        NonCached(InlineQueryResultAudio),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum DocumentKind {
        Cached(InlineQueryResultCachedDocument),
        NonCached(InlineQueryResultDocument),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum GifKind {
        Cached(InlineQueryResultCachedGif),
        NonCached(InlineQueryResultGif),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum Mpeg4GifKind {
        Cached(InlineQueryResultCachedMpeg4Gif),
        NonCached(InlineQueryResultMpeg4Gif),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum PhotoKind {
        Cached(InlineQueryResultCachedPhoto),
        NonCached(InlineQueryResultPhoto),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum VideoKind {
        Cached(InlineQueryResultCachedVideo),
        NonCached(InlineQueryResultVideo),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(untagged)]
    pub(super) enum VoiceKind {
        Cached(InlineQueryResultCachedVoice),
        NonCached(InlineQueryResultVoice),
    }

    #[derive(Serialize, Deserialize)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub(super) enum InlineQueryResult {
        // Types which have a cached and non-cached variant must be listed here
        Audio(AudioKind),
        Document(DocumentKind),
        Gif(GifKind),
        #[serde(rename = "mpeg4_gif")]
        Mpeg4Gif(Mpeg4GifKind),
        Photo(PhotoKind),
        Video(VideoKind),
        Voice(VoiceKind),

        // Types which have only a cached variant must be listed here
        #[serde(rename = "sticker")]
        CachedSticker(InlineQueryResultCachedSticker),

        // Types which have only a non-cached variant must be listed here
        Article(InlineQueryResultArticle),
        Contact(InlineQueryResultContact),
        Game(InlineQueryResultGame),
        Location(InlineQueryResultLocation),
        Venue(InlineQueryResultVenue),
    }

    impl From<InlineQueryResult> for super::InlineQueryResult {
        fn from(raw: InlineQueryResult) -> Self {
            match raw {
                InlineQueryResult::Audio(AudioKind::Cached(audio)) => {
                    super::InlineQueryResult::CachedAudio(audio)
                }
                InlineQueryResult::Audio(AudioKind::NonCached(audio)) => {
                    super::InlineQueryResult::Audio(audio)
                }
                InlineQueryResult::Document(DocumentKind::Cached(document)) => {
                    super::InlineQueryResult::CachedDocument(document)
                }
                InlineQueryResult::Document(DocumentKind::NonCached(document)) => {
                    super::InlineQueryResult::Document(document)
                }
                InlineQueryResult::Gif(GifKind::Cached(gif)) => {
                    super::InlineQueryResult::CachedGif(gif)
                }
                InlineQueryResult::Gif(GifKind::NonCached(gif)) => {
                    super::InlineQueryResult::Gif(gif)
                }
                InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::Cached(gif)) => {
                    super::InlineQueryResult::CachedMpeg4Gif(gif)
                }
                InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::NonCached(gif)) => {
                    super::InlineQueryResult::Mpeg4Gif(gif)
                }
                InlineQueryResult::Photo(PhotoKind::Cached(photo)) => {
                    super::InlineQueryResult::CachedPhoto(photo)
                }
                InlineQueryResult::Photo(PhotoKind::NonCached(photo)) => {
                    super::InlineQueryResult::Photo(photo)
                }
                InlineQueryResult::Video(VideoKind::Cached(video)) => {
                    super::InlineQueryResult::CachedVideo(video)
                }
                InlineQueryResult::Video(VideoKind::NonCached(video)) => {
                    super::InlineQueryResult::Video(video)
                }
                InlineQueryResult::Voice(VoiceKind::Cached(voice)) => {
                    super::InlineQueryResult::CachedVoice(voice)
                }
                InlineQueryResult::Voice(VoiceKind::NonCached(voice)) => {
                    super::InlineQueryResult::Voice(voice)
                }

                InlineQueryResult::CachedSticker(sticker) => {
                    super::InlineQueryResult::CachedSticker(sticker)
                }

                InlineQueryResult::Article(article) => super::InlineQueryResult::Article(article),
                InlineQueryResult::Contact(contact) => super::InlineQueryResult::Contact(contact),
                InlineQueryResult::Game(game) => super::InlineQueryResult::Game(game),
                InlineQueryResult::Location(location) => {
                    super::InlineQueryResult::Location(location)
                }
                InlineQueryResult::Venue(venue) => super::InlineQueryResult::Venue(venue),
            }
        }
    }

    impl From<super::InlineQueryResult> for InlineQueryResult {
        fn from(raw: super::InlineQueryResult) -> Self {
            match raw {
                super::InlineQueryResult::CachedAudio(audio) => {
                    InlineQueryResult::Audio(AudioKind::Cached(audio))
                }
                super::InlineQueryResult::Audio(audio) => {
                    InlineQueryResult::Audio(AudioKind::NonCached(audio))
                }
                super::InlineQueryResult::CachedDocument(document) => {
                    InlineQueryResult::Document(DocumentKind::Cached(document))
                }
                super::InlineQueryResult::Document(document) => {
                    InlineQueryResult::Document(DocumentKind::NonCached(document))
                }
                super::InlineQueryResult::CachedGif(gif) => {
                    InlineQueryResult::Gif(GifKind::Cached(gif))
                }
                super::InlineQueryResult::Gif(gif) => {
                    InlineQueryResult::Gif(GifKind::NonCached(gif))
                }
                super::InlineQueryResult::CachedMpeg4Gif(gif) => {
                    InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::Cached(gif))
                }
                super::InlineQueryResult::Mpeg4Gif(gif) => {
                    InlineQueryResult::Mpeg4Gif(Mpeg4GifKind::NonCached(gif))
                }
                super::InlineQueryResult::CachedPhoto(photo) => {
                    InlineQueryResult::Photo(PhotoKind::Cached(photo))
                }
                super::InlineQueryResult::Photo(photo) => {
                    InlineQueryResult::Photo(PhotoKind::NonCached(photo))
                }
                super::InlineQueryResult::CachedVideo(video) => {
                    InlineQueryResult::Video(VideoKind::Cached(video))
                }
                super::InlineQueryResult::Video(video) => {
                    InlineQueryResult::Video(VideoKind::NonCached(video))
                }
                super::InlineQueryResult::CachedVoice(voice) => {
                    InlineQueryResult::Voice(VoiceKind::Cached(voice))
                }
                super::InlineQueryResult::Voice(voice) => {
                    InlineQueryResult::Voice(VoiceKind::NonCached(voice))
                }

                super::InlineQueryResult::CachedSticker(sticker) => {
                    InlineQueryResult::CachedSticker(sticker)
                }

                super::InlineQueryResult::Article(article) => InlineQueryResult::Article(article),
                super::InlineQueryResult::Contact(contact) => InlineQueryResult::Contact(contact),
                super::InlineQueryResult::Game(game) => InlineQueryResult::Game(game),
                super::InlineQueryResult::Location(location) => {
                    InlineQueryResult::Location(location)
                }
                super::InlineQueryResult::Venue(venue) => InlineQueryResult::Venue(venue),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_inline_query_result_cached_audio() {
        let data =
            InlineQueryResult::CachedAudio(InlineQueryResultCachedAudio::new("test", "test"));

        let json = serde_json::to_string(&data).unwrap();

        assert_eq!(
            json,
            r#"{"type":"audio","id":"test","audio_file_id":"test"}"#
        );
    }

    #[test]
    fn serialize_inline_query_result_audio() {
        let data = InlineQueryResult::Audio(InlineQueryResultAudio::new("test", "test", "test"));

        let json = serde_json::to_string(&data).unwrap();

        assert_eq!(
            json,
            r#"{"type":"audio","id":"test","audio_url":"test","title":"test"}"#
        );
    }
}
