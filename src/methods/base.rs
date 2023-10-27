use crate::{
    client::Bot,
    types::{InputFile, InputFileKind, InputMedia, InputSticker, ResponseParameters},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;

/// This object represents a request to Telegram API
pub struct Request<'a, T>
where
    T: Serialize + ?Sized,
{
    /// Telegram API method name
    pub method_name: &'static str,
    /// Telegram API method data
    pub data: &'a T,
    /// Files to send
    pub files: Option<Box<[&'a InputFile<'a>]>>,
}

impl<'a, T> Request<'a, T>
where
    T: Serialize + ?Sized,
{
    #[must_use]
    pub fn new(
        method_name: &'static str,
        data: &'a T,
        files: Option<Box<[&'a InputFile<'a>]>>,
    ) -> Self {
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
/// - If `ok` equals `True`, the request was successful and the result of the query can be found in the `result` field.
/// - In case of an unsuccessful request, `ok` equals false and the error is explained in the `description`.
/// - An Integer `error_code` field is also returned, but its contents are subject to change in the future.
/// - Some errors may also have an optional field `parameters` of the type [`ResponseParameters`], which can help to automatically handle the error.
#[derive(Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>,
    pub error_code: Option<i16>,
    pub parameters: Option<ResponseParameters>,
}

impl<T: DeserializeOwned> Response<T> {
    #[must_use]
    pub fn new(
        ok: bool,
        result: Option<T>,
        description: Option<String>,
        error_code: Option<i16>,
        parameters: Option<ResponseParameters>,
    ) -> Self {
        Self {
            ok,
            result,
            description,
            error_code,
            parameters,
        }
    }
}

pub trait TelegramMethod {
    /// This type represents a method to Telegram API with data (params)
    type Method: Serialize;
    /// This type represents a response from Telegram API, which is returned by the method
    type Return: DeserializeOwned;

    /// This method is called when a request is sent to Telegram API.
    /// It's need for preparing a request to Telegram API.
    #[must_use]
    fn build_request<Client>(&self, bot: &Bot<Client>) -> Request<Self::Method>;

    /// This method is called when a response is received from Telegram API.
    /// It's need for parsing a response from Telegram API.
    /// # Errors
    /// - If the response cannot be parsed
    fn build_response(&self, content: &str) -> Result<Response<Self::Return>, serde_json::Error> {
        serde_json::from_str::<Response<Self::Return>>(content)
    }
}

pub(super) fn prepare_file<'a>(files: &mut Vec<&'a InputFile<'a>>, file: &'a InputFile<'a>) {
    match file.kind() {
        InputFileKind::FS(_) | InputFileKind::Buffered(_) => {
            files.push(file);
        }
        InputFileKind::Id(_) | InputFileKind::Url(_) => {
            // This file not require be in `multipart/form-data`
            // So we don't need to add it to files
        }
    }
}

pub(super) fn prepare_input_media<'a>(
    files: &mut Vec<&'a InputFile<'a>>,
    input_media: &'a InputMedia<'a>,
) {
    match input_media {
        InputMedia::Animation(inner) => {
            prepare_file(files, &inner.media);
        }
        InputMedia::Audio(inner) => {
            prepare_file(files, &inner.media);
        }
        InputMedia::Document(inner) => {
            prepare_file(files, &inner.media);
        }
        InputMedia::Photo(inner) => {
            prepare_file(files, &inner.media);
        }
        InputMedia::Video(inner) => {
            prepare_file(files, &inner.media);
        }
    }
}

pub(super) fn prepare_input_media_group<'a>(
    files: &mut Vec<&'a InputFile<'a>>,
    input_media_group: &'a [InputMedia<'a>],
) {
    for input_media in input_media_group {
        prepare_input_media(files, input_media);
    }
}

pub(super) fn prepare_input_sticker<'a>(
    files: &mut Vec<&'a InputFile<'a>>,
    input_sticker: &'a InputSticker<'a>,
) {
    prepare_file(files, &input_sticker.sticker);
}

pub(super) fn prepare_input_stickers<'a>(
    files: &mut Vec<&'a InputFile<'a>>,
    input_stickers: &'a [InputSticker<'a>],
) {
    for input_sticker in input_stickers {
        prepare_input_sticker(files, input_sticker);
    }
}
