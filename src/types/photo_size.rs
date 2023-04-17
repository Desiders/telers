use serde::Deserialize;

/// This object represents one size of a photo or a file ([document](crate::types::Document), [sticker](crate::types::Sticker)) thumbnail.
/// # Documentation
/// <https://core.telegram.org/bots/api#photosize>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct PhotoSize {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: String,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: String,
    /// Photo width
    pub width: i64,
    /// Photo height
    pub height: i64,
    /// File size in bytes
    pub file_size: Option<i64>,
}
