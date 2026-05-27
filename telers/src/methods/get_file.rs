use crate::client::Bot;
use serde::Serialize;
/// Use this method to get basic information about a file and prepare it for downloading. For the moment, bots can download files of up to 20MB in size. On success, a File object is returned. The file can then be downloaded via the link `https://api.telegram.org/file/bot<token>/<file_path>`, where <`file_path`> is taken from the response. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling getFile again.
/// Note: This function may not preserve the original file name and MIME type. You should save the file's MIME type and name (if available) when the File object is received.
/// # Documentation
/// <https://core.telegram.org/bots/api#getfile>
/// # Returns
/// - `crate::types::File`
#[derive(Clone, Debug, Serialize)]
pub struct GetFile {
    /// File identifier to get information about
    pub file_id: Box<str>,
}
impl GetFile {
    /// Creates a new `GetFile`.
    ///
    /// # Arguments
    /// * `file_id` - File identifier to get information about
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(file_id: T0) -> Self {
        Self {
            file_id: file_id.into(),
        }
    }

    /// File identifier to get information about
    #[must_use]
    pub fn file_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_id = val.into();
        self
    }
}
impl super::TelegramMethod for GetFile {
    type Method = Self;
    type Return = crate::types::File;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("getFile", self, None)
    }
}
