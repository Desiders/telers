use serde::{Deserialize, Serialize};

/// This object represents a file ready to be downloaded. The file can be downloaded via the link :code:`https://api.telegram.org/file/bot<token>/<file_path>`. It is guaranteed that the link will be valid for at least 1 hour. When the link expires, a new one can be requested by calling `aiogram_rs.methods.get_file.GetFile`.
/// The maximum file size to download is 20 MB
/// <https://core.telegram.org/bots/api#file>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct File {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// *Optional*. File size in bytes. It can be bigger than 2^31 and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this value.
    pub file_size: Option<i64>,
    /// *Optional*. File path. Use :code:`https://api.telegram.org/file/bot<token>/<file_path>` to get the file.
    pub file_path: Option<String>,
}
