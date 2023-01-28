use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::File};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to get basic info about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where `<file_path>` is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling [`GetFile`](crate::methods::GetFile) again.
/// # Documentation
/// <https://core.telegram.org/bots/api#getfile>
/// # Notes
/// This function may not preserve the original file name and MIME type. You should save the file's MIME type and name (if available) when the File object is received.
/// # Returns
/// On success, a [`File`] object is returned.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct GetFile {
    /// File identifier to get info about
    pub file_id: String,
}

impl GetFile {
    #[must_use]
    pub fn new<T: Into<String>>(file_id: T) -> Self {
        Self {
            file_id: file_id.into(),
        }
    }

    #[must_use]
    pub fn file_id<T: Into<String>>(mut self, file_id: T) -> Self {
        self.file_id = file_id.into();
        self
    }
}

impl TelegramMethod for GetFile {
    type Method = Self;
    type Return = File;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("getFile", self, None)
    }
}