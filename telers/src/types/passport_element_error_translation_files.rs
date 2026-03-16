use serde::{Deserialize, Serialize};
/// Represents an issue with the translated version of a document. The error is considered resolved when a file with the document translation change.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrortranslationfiles>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFiles {
    /// Type of element of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    pub r#type: Box<str>,
    /// List of base64-encoded file hashes
    pub file_hashes: Box<[Box<str>]>,
    /// Error message
    pub message: Box<str>,
}
impl PassportElementErrorTranslationFiles {
    /// Creates a new `PassportElementErrorTranslationFiles`.
    ///
    /// # Arguments
    /// * `type` - Type of element of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    /// * `file_hashes` - List of base64-encoded file hashes
    /// * `message` - Error message
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1Item: Into<Box<str>>,
        T1: IntoIterator<Item = T1Item>,
        T2: Into<Box<str>>,
    >(
        r#type: T0,
        file_hashes: T1,
        message: T2,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            file_hashes: file_hashes.into_iter().map(Into::into).collect(),
            message: message.into(),
        }
    }

    /// Type of element of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.r#type = val.into();
        this
    }

    /// List of base64-encoded file hashes
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn file_hashes<T: Into<Box<[Box<str>]>>>(self, val: T) -> Self {
        let mut this = self;
        this.file_hashes = this
            .file_hashes
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// List of base64-encoded file hashes
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn file_hash<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.file_hashes = this
            .file_hashes
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Error message
    #[must_use]
    pub fn message<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.message = val.into();
        this
    }
}
