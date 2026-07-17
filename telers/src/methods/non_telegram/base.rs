use crate::{
    client::Bot,
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

use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
    fn build_response(content: &str) -> Result<Response<Self::Return>, serde_json::Error> {
        event!(Level::TRACE, %content, "Parsing");
        let mut deserializer = serde_json::Deserializer::from_str(content);
        deserializer.disable_recursion_limit();
        let deserializer = serde_stacker::Deserializer::new(&mut deserializer);
        let res = Response::<Self::Return>::deserialize(deserializer).inspect_err(|err| {
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

pub fn prepare_optional_file(files: &mut Vec<InputFile>, file: &mut Option<InputFile>) {
    if let Some(file) = file {
        prepare_file(files, file);
    }
}

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

pub fn prepare_input_media_group(
    files: &mut Vec<InputFile>,
    input_media_group: Vec<&mut InputMedia>,
) {
    for input_media in input_media_group {
        prepare_input_media(files, input_media);
    }
}

pub fn prepare_input_sticker(files: &mut Vec<InputFile>, input_sticker: &mut InputSticker) {
    prepare_file(files, &mut input_sticker.sticker);
}

pub fn prepare_input_stickers(files: &mut Vec<InputFile>, input_stickers: Vec<&mut InputSticker>) {
    for input_sticker in input_stickers {
        prepare_input_sticker(files, input_sticker);
    }
}

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
        | InputRichBlock::Pullquote(_)
        | InputRichBlock::Table(_)
        | InputRichBlock::Map(_)
        | InputRichBlock::Thinking(_) => {}
    }
}

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

pub fn prepare_input_message_content(
    files: &mut Vec<InputFile>,
    content: &mut InputMessageContent,
) {
    if let InputMessageContent::InputRichMessageContent(inner) = content {
        prepare_input_rich_message(files, &mut inner.rich_message);
    }
}

pub fn prepare_optional_input_message_content(
    files: &mut Vec<InputFile>,
    content: &mut Option<InputMessageContent>,
) {
    if let Some(content) = content {
        prepare_input_message_content(files, content);
    }
}

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

pub fn prepare_input_paid_media_group(
    files: &mut Vec<InputFile>,
    input_paid_media_group: Vec<&mut InputPaidMedia>,
) {
    for input_paid_media in input_paid_media_group {
        prepare_input_paid_media(files, input_paid_media);
    }
}

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
