use crate::{
    client::Bot,
    types::{InputFile, InputFileKind, InputMedia, ResponseParameters},
};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json;
use std::{borrow::Cow, collections::HashMap};

/// This object represents a request to Telegram API
pub struct Request<'a, T>
where
    T: Serialize,
{
    /// Telegram API method name
    method_name: &'static str,
    /// Telegram API method data
    data: &'a T,
    /// Files to send
    files: Option<HashMap<Cow<'a, str>, &'a InputFile>>,
}

impl<'a, T> Request<'a, T>
where
    T: Serialize,
{
    #[must_use]
    pub fn new(
        method_name: &'static str,
        data: &'a T,
        files: Option<HashMap<Cow<'a, str>, &'a InputFile>>,
    ) -> Self {
        Self {
            method_name,
            data,
            files,
        }
    }

    #[must_use]
    pub fn method_name(&self) -> &str {
        self.method_name
    }

    #[must_use]
    pub fn data(&self) -> &T {
        self.data
    }

    #[must_use]
    pub fn files(&self) -> Option<&HashMap<Cow<'a, str>, &InputFile>> {
        self.files.as_ref()
    }
}

/// This object represents a response from Telegram API. It's returned by making requests to Telegram API, for more info check [Telegram API docs](https://core.telegram.org/bots/api#making-requests)
/// # Note
/// The response contains a JSON object, which always has a Boolean field `ok` and may have an optional String field `description` with a human-readable description of the result. \
/// If `ok` equals `True`, the request was successful and the result of the query can be found in the `result` field. \
/// In case of an unsuccessful request, `ok` equals false and the error is explained in the `description`. \
/// An Integer `error_code` field is also returned, but its contents are subject to change in the future. \
/// Some errors may also have an optional field `parameters` of the type [`ResponseParameters`](https://core.telegram.org/bots/api#responseparameters), which can help to automatically handle the error.
#[derive(Deserialize)]
pub struct Response<T> {
    ok: bool,
    pub(crate) result: Option<T>,
    description: Option<String>,
    error_code: Option<i16>,
    parameters: Option<ResponseParameters>,
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

    #[must_use]
    pub fn ok(&self) -> bool {
        self.ok
    }

    #[must_use]
    pub fn result(&self) -> Option<&T> {
        self.result.as_ref()
    }

    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    #[must_use]
    pub fn error_code(&self) -> Option<i16> {
        self.error_code
    }

    #[must_use]
    pub fn parameters(&self) -> Option<&ResponseParameters> {
        self.parameters.as_ref()
    }
}

pub trait TelegramMethod {
    /// This type represents a method to Telegram API with parameters
    type Method: Serialize;
    /// This type represents a response from Telegram API, which is returned by the method
    type Return: DeserializeOwned;

    /// This method is called when a request is sent to Telegram API.
    /// It's need for preparing a request to Telegram API.
    #[must_use]
    fn build_request(&self, bot: &Bot) -> Request<Self::Method>;

    /// This method is called when a response is received from Telegram API.
    /// It's need for parsing a response from Telegram API.
    /// # Errors
    /// - If the response cannot be parsed
    fn build_response(&self, content: &str) -> Result<Response<Self::Return>, serde_json::Error> {
        serde_json::from_str::<Response<Self::Return>>(content)
    }
}

pub(super) fn prepare_file_with_id<'a>(
    files: &mut HashMap<Cow<'a, str>, &'a InputFile>,
    file: &'a InputFile,
) {
    match file.kind() {
        InputFileKind::FS(inner) => {
            files.insert(inner.id().to_string().into(), file);
        }
        InputFileKind::Id(_) | InputFileKind::Url(_) => {
            // This file not require be in multipart/form-data
            // So we don't need to add it to files
        }
    }
}

pub(super) fn prepare_file_with_value<'a>(
    files: &mut HashMap<Cow<'a, str>, &'a InputFile>,
    file: &'a InputFile,
    value: impl Into<Cow<'a, str>>,
) {
    match file.kind() {
        InputFileKind::FS(inner) => {
            files.insert(value.into(), file);
        }
        InputFileKind::Id(_) | InputFileKind::Url(_) => {
            // This file not require be in multipart/form-data
            // So we don't need to add it to files
        }
    }
}

pub(super) fn prepare_input_media<'a>(
    files: &mut HashMap<Cow<'a, str>, &'a InputFile>,
    input_media: &'a InputMedia,
) {
    match input_media {
        InputMedia::Animation(inner) => {
            prepare_file_with_id(files, &inner.media);
        }
        InputMedia::Audio(inner) => {
            prepare_file_with_id(files, &inner.media);
        }
        InputMedia::Document(inner) => {
            prepare_file_with_id(files, &inner.media);
        }
        InputMedia::Photo(inner) => {
            prepare_file_with_id(files, &inner.media);
        }
        InputMedia::Video(inner) => {
            prepare_file_with_id(files, &inner.media);
        }
    }
}
