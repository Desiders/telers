//! This module contains the [`TelegramMethod`] trait, which every method in [`crate::methods`] implements,
//! and the helpers that collect the files of a request into the `multipart/form-data` part before it is sent.

use crate::{
    client::Bot,
    serialization::{self, Deserialize, DeserializeOwned, Serialize},
    types::{
        InlineQueryResult, InlineQueryResultAudioKind, InlineQueryResultDocumentKind,
        InlineQueryResultGifKind, InlineQueryResultMpeg4GifKind, InlineQueryResultPhotoKind,
        InlineQueryResultVideoKind, InlineQueryResultVoiceKind, InputFile, InputMedia,
        InputMessageContent, InputPaidMedia, InputPollMedia, InputPollOption, InputPollOptionMedia,
        InputProfilePhoto, InputRichBlock, InputRichMessage, InputRichMessageMediaContent,
        InputSticker, InputStoryContent, ResponseParameters,
    },
    utils::format_error_report,
};

use tracing::{event, instrument, Level};

/// This object represents a request to Telegram API
pub struct Request<T>
where
    T: Serialize,
{
    /// Telegram API method name
    pub method_name: &'static str,
    /// Telegram API method data
    pub data: T,
    /// Files to send
    pub files: Option<Vec<InputFile>>,
}

impl<T> Request<T>
where
    T: Serialize,
{
    #[must_use]
    pub fn new(method_name: &'static str, data: T, files: Option<Vec<InputFile>>) -> Self {
        Self {
            method_name,
            data,
            files,
        }
    }
}

/// This object represents a response from Telegram API. It's returned by making requests to Telegram API, for more info check [Telegram API docs](https://core.telegram.org/bots/api#making-requests)
/// # Notes
/// - The response contains a JSON object, which always has a Boolean field `ok` and may have an optional String field `description` with a human-readable description of the result.
/// - If `ok` equals `true`, the request was successful and the result of the query can be found in the `result` field.
/// - In case of an unsuccessful request, `ok` equals false and the error is explained in the `description`.
/// - An Integer `error_code` field is also returned, but its contents are subject to change in the future.
/// - Some errors may also have an optional field `parameters` of the type [`ResponseParameters`], which can help to automatically handle the error.
#[derive(Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<Box<str>>,
    pub error_code: Option<i16>,
    pub parameters: Option<ResponseParameters>,
}

/// A Telegram Bot API method: its parameters and how to build a request and parse the response.
///
/// Every generated method in [`crate::methods`] implements this trait,
/// so any of them can be passed to [`Bot::send`](crate::Bot::send).
pub trait TelegramMethod {
    /// This type represents a method to Telegram API with data (params)
    type Method: Serialize;
    /// This type represents a response from Telegram API, which is returned by the method
    type Return: DeserializeOwned;

    /// This method is called when a request is sent to Telegram API.
    /// It's need for preparing a request to Telegram API.
    #[must_use]
    fn build_request<Client>(self, bot: &Bot<Client>) -> Request<Self::Method>;

    /// This method is called when a response is received from Telegram API.
    /// It's need for parsing a response from Telegram API.
    /// # Errors
    /// - If the response cannot be parsed
    #[instrument(name = "build", skip_all)]
    fn build_response(content: &str) -> Result<Response<Self::Return>, serialization::Error> {
        event!(Level::TRACE, %content, "Parsing");
        #[cfg(not(feature = "deser"))]
        let res = {
            let mut deserializer = serde_json::Deserializer::from_str(content);
            deserializer.disable_recursion_limit();
            let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
            Response::<Self::Return>::deserialize(deserializer)
        };
        #[cfg(feature = "deser")]
        let res = deser_json::from_str::<Response<Self::Return>>(content);
        let res = res.inspect_err(|err| {
            event!(
                Level::ERROR,
                error = %format_error_report(&err),
                %content,
                "Cannot parse content",
            );
        });
        event!(Level::TRACE, "Parsed");
        res
    }
}

/// Moves the file into the list of parts sent as `multipart/form-data`.
///
/// A file that is only referenced, by its `file_id` or by a URL, is left where it is, because Telegram
/// gets its contents itself. A file from the file system, from memory or from a stream is taken out of
/// `file`, which is left holding nothing but the `attach://` reference to the part it was moved into.
/// # Arguments
/// * `files` - Collected files of the request
/// * `file` - File to collect
pub fn prepare_file(files: &mut Vec<InputFile>, file: &mut InputFile) {
    match file {
        InputFile::FS(_) | InputFile::Buffered(_) | InputFile::Stream(_) => {
            files.push(file.take());
        }
        InputFile::Id(_) | InputFile::Url(_) => {
            // This file not require be in `multipart/form-data`
            // So we don't need to add it to files
        }
    }
}

/// Collects the file only when there is one, so an absent file stays absent.
/// # Arguments
/// * `files` - Collected files of the request
/// * `file` - File to collect, if it is set
pub fn prepare_optional_file(files: &mut Vec<InputFile>, file: &mut Option<InputFile>) {
    if let Some(file) = file {
        prepare_file(files, file);
    }
}

/// Collects the media of an [`InputMedia`] item together with the files its variant carries: the thumbnail
/// of an animation, an audio or a document, the extra photo of a live photo, the thumbnail and the cover
/// of a video.
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_media` - Item to collect the files of
pub fn prepare_input_media(files: &mut Vec<InputFile>, input_media: &mut InputMedia) {
    match input_media {
        InputMedia::Animation(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputMedia::Audio(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputMedia::Document(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputMedia::LivePhoto(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_file(files, &mut inner.photo);
        }
        InputMedia::Photo(inner) => {
            prepare_file(files, &mut inner.media);
        }
        InputMedia::Video(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
            prepare_optional_file(files, &mut inner.cover);
        }
    }
}

/// Collects the files of every item of an [`InputMedia`] group, in the order of the group.
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_media_group` - Items of the group
pub fn prepare_input_media_group(
    files: &mut Vec<InputFile>,
    input_media_group: Vec<&mut InputMedia>,
) {
    for input_media in input_media_group {
        prepare_input_media(files, input_media);
    }
}

/// Collects the sticker of an [`InputSticker`].
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_sticker` - Sticker to collect the file of
pub fn prepare_input_sticker(files: &mut Vec<InputFile>, input_sticker: &mut InputSticker) {
    prepare_file(files, &mut input_sticker.sticker);
}

/// Collects the sticker of every item of an [`InputSticker`] set, in the order of the set.
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_stickers` - Items of the set
pub fn prepare_input_stickers(files: &mut Vec<InputFile>, input_stickers: Vec<&mut InputSticker>) {
    for input_sticker in input_stickers {
        prepare_input_sticker(files, input_sticker);
    }
}

/// Collects the media of an [`InputPaidMedia`] item together with the files its variant carries: the extra
/// photo of a live photo, the thumbnail and the cover of a video.
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_paid_media` - Item to collect the files of
pub fn prepare_input_paid_media(files: &mut Vec<InputFile>, input_paid_media: &mut InputPaidMedia) {
    match input_paid_media {
        InputPaidMedia::LivePhoto(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_file(files, &mut inner.photo);
        }
        InputPaidMedia::Photo(inner) => {
            prepare_file(files, &mut inner.media);
        }
        InputPaidMedia::Video(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
            prepare_optional_file(files, &mut inner.cover);
        }
    }
}

/// Collects the media of an [`InputPollMedia`] item together with the files its variant carries. A location
/// or a venue holds no file and is left as it is.
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_poll_media` - Item to collect the files of
pub fn prepare_input_poll_media(files: &mut Vec<InputFile>, input_poll_media: &mut InputPollMedia) {
    match input_poll_media {
        InputPollMedia::Animation(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputPollMedia::Audio(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputPollMedia::Document(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputPollMedia::LivePhoto(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_file(files, &mut inner.photo);
        }
        InputPollMedia::Photo(inner) => {
            prepare_file(files, &mut inner.media);
        }
        InputPollMedia::Video(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
            prepare_optional_file(files, &mut inner.cover);
        }
        InputPollMedia::Location(_) | InputPollMedia::Venue(_) => {}
    }
}

/// Collects the media of an [`InputPollOptionMedia`] item together with the files its variant carries. A link,
/// a location or a venue holds no file and is left as it is.
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_poll_option_media` - Item to collect the files of
pub fn prepare_input_poll_option_media(
    files: &mut Vec<InputFile>,
    input_poll_option_media: &mut InputPollOptionMedia,
) {
    match input_poll_option_media {
        InputPollOptionMedia::Animation(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputPollOptionMedia::LivePhoto(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_file(files, &mut inner.photo);
        }
        InputPollOptionMedia::Photo(inner) => {
            prepare_file(files, &mut inner.media);
        }
        InputPollOptionMedia::Sticker(inner) => {
            prepare_file(files, &mut inner.media);
        }
        InputPollOptionMedia::Video(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
            prepare_optional_file(files, &mut inner.cover);
        }
        InputPollOptionMedia::Link(_)
        | InputPollOptionMedia::Location(_)
        | InputPollOptionMedia::Venue(_) => {}
    }
}

/// Collects the files of the media of every option of a poll, in the order of the options. An option without
/// media is skipped.
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_poll_options` - Options of the poll
pub fn prepare_input_poll_options(
    files: &mut Vec<InputFile>,
    input_poll_options: Vec<&mut InputPollOption>,
) {
    for input_poll_option in input_poll_options {
        if let Some(media) = &mut input_poll_option.media {
            prepare_input_poll_option_media(files, media);
        }
    }
}

/// Collects the media of an [`InputRichMessageMediaContent`] item together with the files its variant carries:
/// the thumbnail of an animation, an audio or a document, the thumbnail and the cover of a video.
/// # Arguments
/// * `files` - Collected files of the request
/// * `content` - Item to collect the files of
pub fn prepare_input_rich_message_media_content(
    files: &mut Vec<InputFile>,
    content: &mut InputRichMessageMediaContent,
) {
    match content {
        InputRichMessageMediaContent::Animation(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputRichMessageMediaContent::Audio(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputRichMessageMediaContent::Document(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
        }
        InputRichMessageMediaContent::Photo(inner) => {
            prepare_file(files, &mut inner.media);
        }
        InputRichMessageMediaContent::Video(inner) => {
            prepare_file(files, &mut inner.media);
            prepare_optional_file(files, &mut inner.thumbnail);
            prepare_optional_file(files, &mut inner.cover);
        }
        InputRichMessageMediaContent::VoiceNote(inner) => {
            prepare_file(files, &mut inner.media);
        }
    }
}

/// Collects the files of an [`InputRichBlock`] and, when the block holds other blocks, of all of them, as a
/// collage, a slideshow, a details block or a list does.
/// # Arguments
/// * `files` - Collected files of the request
/// * `block` - Block to collect the files of
pub fn prepare_input_rich_block(files: &mut Vec<InputFile>, block: &mut InputRichBlock) {
    match block {
        InputRichBlock::Animation(inner) => {
            prepare_file(files, &mut inner.animation.media);
            prepare_optional_file(files, &mut inner.animation.thumbnail);
        }
        InputRichBlock::Audio(inner) => {
            prepare_file(files, &mut inner.audio.media);
            prepare_optional_file(files, &mut inner.audio.thumbnail);
        }
        InputRichBlock::Document(inner) => {
            prepare_file(files, &mut inner.document.media);
            prepare_optional_file(files, &mut inner.document.thumbnail);
        }
        InputRichBlock::Photo(inner) => {
            prepare_file(files, &mut inner.photo.media);
        }
        InputRichBlock::Video(inner) => {
            prepare_file(files, &mut inner.video.media);
            prepare_optional_file(files, &mut inner.video.thumbnail);
            prepare_optional_file(files, &mut inner.video.cover);
        }
        InputRichBlock::VoiceNote(inner) => {
            prepare_file(files, &mut inner.voice_note.media);
        }
        InputRichBlock::Collage(inner) => {
            for block in &mut inner.blocks {
                prepare_input_rich_block(files, block);
            }
        }
        InputRichBlock::Slideshow(inner) => {
            for block in &mut inner.blocks {
                prepare_input_rich_block(files, block);
            }
        }
        InputRichBlock::Details(inner) => {
            for block in &mut inner.blocks {
                prepare_input_rich_block(files, block);
            }
        }
        InputRichBlock::List(inner) => {
            for item in &mut inner.items {
                for block in &mut item.blocks {
                    prepare_input_rich_block(files, block);
                }
            }
        }
        // No media in these blocks. Listed explicitly so a new block kind is a compile
        // error here instead of silently skipping its files.
        InputRichBlock::Paragraph(_)
        | InputRichBlock::Heading(_)
        | InputRichBlock::Pre(_)
        | InputRichBlock::Footer(_)
        | InputRichBlock::Divider(_)
        | InputRichBlock::MathematicalExpression(_)
        | InputRichBlock::Anchor(_)
        | InputRichBlock::Blockquote(_)
        | InputRichBlock::ExpandableBlockquote(_)
        | InputRichBlock::Pullquote(_)
        | InputRichBlock::Table(_)
        | InputRichBlock::Map(_)
        | InputRichBlock::Buttons(_)
        | InputRichBlock::Thinking(_) => {}
    }
}

/// Collects the files of the media and of the blocks of an [`InputRichMessage`], in the order they are given.
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_rich_message` - Message to collect the files of
pub fn prepare_input_rich_message(
    files: &mut Vec<InputFile>,
    input_rich_message: &mut InputRichMessage,
) {
    if let Some(media) = &mut input_rich_message.media {
        for media in &mut *media {
            prepare_input_rich_message_media_content(files, &mut media.media);
        }
    }
    if let Some(blocks) = &mut input_rich_message.blocks {
        for block in &mut *blocks {
            prepare_input_rich_block(files, block);
        }
    }
}

/// Collects the files of an [`InputMessageContent`]. Only rich message content carries files, every other
/// variant is left as it is.
/// # Arguments
/// * `files` - Collected files of the request
/// * `content` - Content to collect the files of
pub fn prepare_input_message_content(
    files: &mut Vec<InputFile>,
    content: &mut InputMessageContent,
) {
    if let InputMessageContent::InputRichMessageContent(inner) = content {
        prepare_input_rich_message(files, &mut inner.rich_message);
    }
}

/// Collects the files of the content only when there is one, so an absent content stays absent.
/// # Arguments
/// * `files` - Collected files of the request
/// * `content` - Content to collect the files of, if it is set
pub fn prepare_optional_input_message_content(
    files: &mut Vec<InputFile>,
    content: &mut Option<InputMessageContent>,
) {
    if let Some(content) = content {
        prepare_input_message_content(files, content);
    }
}

/// Collects the files of the content each answer of an inline query would send, in the order of the results.
/// An answer that carries no content, a game, is left as it is.
/// # Arguments
/// * `files` - Collected files of the request
/// * `results` - Results of the query
pub fn prepare_inline_query_results(
    files: &mut Vec<InputFile>,
    results: Vec<&mut InlineQueryResult>,
) {
    for result in results {
        match result {
            InlineQueryResult::Audio(kind) => match kind {
                InlineQueryResultAudioKind::Cached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
                InlineQueryResultAudioKind::Uncached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
            },
            InlineQueryResult::Document(kind) => match kind {
                InlineQueryResultDocumentKind::Cached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
                InlineQueryResultDocumentKind::Uncached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
            },
            InlineQueryResult::Gif(kind) => match kind {
                InlineQueryResultGifKind::Cached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
                InlineQueryResultGifKind::Uncached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
            },
            InlineQueryResult::Mpeg4Gif(kind) => match kind {
                InlineQueryResultMpeg4GifKind::Cached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
                InlineQueryResultMpeg4GifKind::Uncached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
            },
            InlineQueryResult::Photo(kind) => match kind {
                InlineQueryResultPhotoKind::Cached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
                InlineQueryResultPhotoKind::Uncached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
            },
            InlineQueryResult::Video(kind) => match kind {
                InlineQueryResultVideoKind::Cached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
                InlineQueryResultVideoKind::Uncached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
            },
            InlineQueryResult::Voice(kind) => match kind {
                InlineQueryResultVoiceKind::Cached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
                InlineQueryResultVoiceKind::Uncached(inner) => {
                    prepare_optional_input_message_content(files, &mut inner.input_message_content);
                }
            },
            InlineQueryResult::Sticker(inner) => {
                prepare_optional_input_message_content(files, &mut inner.input_message_content);
            }
            InlineQueryResult::Article(inner) => {
                prepare_input_message_content(files, &mut inner.input_message_content);
            }
            InlineQueryResult::Contact(inner) => {
                prepare_optional_input_message_content(files, &mut inner.input_message_content);
            }
            InlineQueryResult::Location(inner) => {
                prepare_optional_input_message_content(files, &mut inner.input_message_content);
            }
            InlineQueryResult::Venue(inner) => {
                prepare_optional_input_message_content(files, &mut inner.input_message_content);
            }
            InlineQueryResult::Game(_) => {}
        }
    }
}

/// Collects the files of every item of an [`InputPaidMedia`] group, in the order of the group.
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_paid_media_group` - Items of the group
pub fn prepare_input_paid_media_group(
    files: &mut Vec<InputFile>,
    input_paid_media_group: Vec<&mut InputPaidMedia>,
) {
    for input_paid_media in input_paid_media_group {
        prepare_input_paid_media(files, input_paid_media);
    }
}

/// Collects the photo or the video of an [`InputStoryContent`].
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_story_content` - Content to collect the file of
pub fn prepare_input_story_content(
    files: &mut Vec<InputFile>,
    input_story_content: &mut InputStoryContent,
) {
    match input_story_content {
        InputStoryContent::Photo(inner) => {
            prepare_file(files, &mut inner.photo);
        }
        InputStoryContent::Video(inner) => {
            prepare_file(files, &mut inner.video);
        }
    }
}

/// Collects the static photo or the animation of an [`InputProfilePhoto`].
/// # Arguments
/// * `files` - Collected files of the request
/// * `input_profile_photo` - Profile photo to collect the file of
pub fn prepare_input_profile_photo(
    files: &mut Vec<InputFile>,
    input_profile_photo: &mut InputProfilePhoto,
) {
    match input_profile_photo {
        InputProfilePhoto::Static(inner) => {
            prepare_file(files, &mut inner.photo);
        }
        InputProfilePhoto::Animated(inner) => {
            prepare_file(files, &mut inner.animation);
        }
    }
}
