/// Represents an issue with the selfie with a document. The error is considered resolved when the file with the selfie changes.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerrorselfie>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct PassportElementErrorSelfie {
    /// The section of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`
    pub r#type: Box<str>,
    /// Base64-encoded hash of the file with the selfie
    pub file_hash: Box<str>,
    /// Error message
    pub message: Box<str>,
}
impl PassportElementErrorSelfie {
    /// Creates a new `PassportElementErrorSelfie`.
    ///
    /// # Arguments
    /// * `type` - The section of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`
    /// * `file_hash` - Base64-encoded hash of the file with the selfie
    /// * `message` - Error message
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        r#type: T0,
        file_hash: T1,
        message: T2,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            file_hash: file_hash.into(),
            message: message.into(),
        }
    }

    /// The section of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Base64-encoded hash of the file with the selfie
    #[must_use]
    pub fn file_hash<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.file_hash = val.into();
        self
    }

    /// Error message
    #[must_use]
    pub fn message<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message = val.into();
        self
    }
}
#[cfg(not(feature = "deser"))]
impl<'de> serde::Deserialize<'de> for PassportElementErrorSelfie {
    #[allow(clippy::too_many_lines)]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let meta = crate::deserialization::Meta {
            name: "PassportElementErrorSelfie",
            fields: &["type", "file_hash", "message"],
            specific: &["file_hash"],
            sequence: "struct PassportElementErrorSelfie with 3 elements",
        };
        let (common, (value_file_hash,)) = crate::deserialization::passport_element_error::read_1::<
            D,
            Box<str>,
        >(deserializer, meta)?;
        Ok(Self {
            r#type: common.r#type,
            file_hash: value_file_hash,
            message: common.message,
        })
    }
}
