use serde::{Deserialize, Serialize};
/// This object represents a file uploaded to Telegram Passport. Currently all Telegram Passport files are in JPEG format when decrypted and don't exceed 10MB.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportfile>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassportFile {
    /// Identifier for this file, which can be used to download or reuse the file
    pub file_id: Box<str>,
    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    pub file_unique_id: Box<str>,
    /// File size in bytes
    pub file_size: i64,
    /// Unix time when the file was uploaded
    pub file_date: i64,
}
impl PassportFile {
    /// Creates a new `PassportFile`.
    ///
    /// # Arguments
    /// * `file_id` - Identifier for this file, which can be used to download or reuse the file
    /// * `file_unique_id` - Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    /// * `file_size` - File size in bytes
    /// * `file_date` - Unix time when the file was uploaded
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<i64>, T3: Into<i64>>(
        file_id: T0,
        file_unique_id: T1,
        file_size: T2,
        file_date: T3,
    ) -> Self {
        Self {
            file_id: file_id.into(),
            file_unique_id: file_unique_id.into(),
            file_size: file_size.into(),
            file_date: file_date.into(),
        }
    }

    /// Identifier for this file, which can be used to download or reuse the file
    #[must_use]
    pub fn file_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_id = val.into();
        self
    }

    /// Unique identifier for this file, which is supposed to be the same over time and for different bots. Can't be used to download or reuse the file.
    #[must_use]
    pub fn file_unique_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_unique_id = val.into();
        self
    }

    /// File size in bytes
    #[must_use]
    pub fn file_size<T: Into<i64>>(mut self, val: T) -> Self {
        self.file_size = val.into();
        self
    }

    /// Unix time when the file was uploaded
    #[must_use]
    pub fn file_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.file_date = val.into();
        self
    }
}
