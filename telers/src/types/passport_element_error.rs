use serde::{Deserialize, Serialize};
/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
/// - [`PassportElementErrorDataField`]
/// - [`PassportElementErrorFrontSide`]
/// - [`PassportElementErrorReverseSide`]
/// - [`PassportElementErrorSelfie`]
/// - [`PassportElementErrorFile`]
/// - [`PassportElementErrorFiles`]
/// - [`PassportElementErrorTranslationFile`]
/// - [`PassportElementErrorTranslationFiles`]
/// - [`PassportElementErrorUnspecified`]
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerror>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum PassportElementError {
    Data(crate::types::PassportElementErrorDataField),
    FrontSide(crate::types::PassportElementErrorFrontSide),
    ReverseSide(crate::types::PassportElementErrorReverseSide),
    Selfie(crate::types::PassportElementErrorSelfie),
    File(crate::types::PassportElementErrorFile),
    Files(crate::types::PassportElementErrorFiles),
    TranslationFile(crate::types::PassportElementErrorTranslationFile),
    TranslationFiles(crate::types::PassportElementErrorTranslationFiles),
    Unspecified(crate::types::PassportElementErrorUnspecified),
}
impl PassportElementError {
    /// Helper method for field `data_hash`.
    ///
    /// Base64-encoded data hash
    #[must_use]
    pub fn data_hash(&self) -> Option<&str> {
        match self {
            Self::Data(val) => Some(val.data_hash.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `element_hash`.
    ///
    /// Base64-encoded element hash
    #[must_use]
    pub fn element_hash(&self) -> Option<&str> {
        match self {
            Self::Unspecified(val) => Some(val.element_hash.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `field_name`.
    ///
    /// Name of the data field which has the error
    #[must_use]
    pub fn field_name(&self) -> Option<&str> {
        match self {
            Self::Data(val) => Some(val.field_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `file_hash`.
    ///
    /// # Variants
    /// - `PassportElementErrorFrontSide`. Base64-encoded hash of the file with the front side of the document
    /// - `PassportElementErrorReverseSide`. Base64-encoded hash of the file with the reverse side of the document
    /// - `PassportElementErrorSelfie`. Base64-encoded hash of the file with the selfie
    /// - `PassportElementErrorFile`, `PassportElementErrorTranslationFile`. Base64-encoded file hash
    #[must_use]
    pub fn file_hash(&self) -> Option<&str> {
        match self {
            Self::FrontSide(val) => Some(val.file_hash.as_ref()),
            Self::ReverseSide(val) => Some(val.file_hash.as_ref()),
            Self::Selfie(val) => Some(val.file_hash.as_ref()),
            Self::File(val) => Some(val.file_hash.as_ref()),
            Self::TranslationFile(val) => Some(val.file_hash.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `file_hashes`.
    ///
    /// List of base64-encoded file hashes
    #[must_use]
    pub fn file_hashes(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Files(val) => Some(val.file_hashes.as_ref()),
            Self::TranslationFiles(val) => Some(val.file_hashes.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `message`.
    ///
    /// Error message
    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::Data(val) => val.message.as_ref(),
            Self::FrontSide(val) => val.message.as_ref(),
            Self::ReverseSide(val) => val.message.as_ref(),
            Self::Selfie(val) => val.message.as_ref(),
            Self::File(val) => val.message.as_ref(),
            Self::Files(val) => val.message.as_ref(),
            Self::TranslationFile(val) => val.message.as_ref(),
            Self::TranslationFiles(val) => val.message.as_ref(),
            Self::Unspecified(val) => val.message.as_ref(),
        }
    }

    /// Helper method for field `type`.
    ///
    /// # Variants
    /// - `PassportElementErrorDataField`. The section of the user's Telegram Passport which has the error, one of `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport`, `address`
    /// - `PassportElementErrorFrontSide`, `PassportElementErrorSelfie`. The section of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`
    /// - `PassportElementErrorReverseSide`. The section of the user's Telegram Passport which has the issue, one of `driver_license`, `identity_card`
    /// - `PassportElementErrorFile`, `PassportElementErrorFiles`. The section of the user's Telegram Passport which has the issue, one of `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    /// - `PassportElementErrorTranslationFile`, `PassportElementErrorTranslationFiles`. Type of element of the user's Telegram Passport which has the issue, one of `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration`, `temporary_registration`
    /// - `PassportElementErrorUnspecified`. Type of element of the user's Telegram Passport which has the issue
    #[must_use]
    pub fn r#type(&self) -> &str {
        match self {
            Self::Data(val) => val.r#type.as_ref(),
            Self::FrontSide(val) => val.r#type.as_ref(),
            Self::ReverseSide(val) => val.r#type.as_ref(),
            Self::Selfie(val) => val.r#type.as_ref(),
            Self::File(val) => val.r#type.as_ref(),
            Self::Files(val) => val.r#type.as_ref(),
            Self::TranslationFile(val) => val.r#type.as_ref(),
            Self::TranslationFiles(val) => val.r#type.as_ref(),
            Self::Unspecified(val) => val.r#type.as_ref(),
        }
    }
}
impl From<crate::types::PassportElementErrorDataField> for PassportElementError {
    fn from(val: crate::types::PassportElementErrorDataField) -> Self {
        Self::Data(val)
    }
}
impl TryFrom<PassportElementError> for crate::types::PassportElementErrorDataField {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PassportElementError) -> Result<Self, Self::Error> {
        if let PassportElementError::Data(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PassportElementError),
                stringify!(PassportElementErrorDataField),
            ))
        }
    }
}
impl From<crate::types::PassportElementErrorFrontSide> for PassportElementError {
    fn from(val: crate::types::PassportElementErrorFrontSide) -> Self {
        Self::FrontSide(val)
    }
}
impl TryFrom<PassportElementError> for crate::types::PassportElementErrorFrontSide {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PassportElementError) -> Result<Self, Self::Error> {
        if let PassportElementError::FrontSide(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PassportElementError),
                stringify!(PassportElementErrorFrontSide),
            ))
        }
    }
}
impl From<crate::types::PassportElementErrorReverseSide> for PassportElementError {
    fn from(val: crate::types::PassportElementErrorReverseSide) -> Self {
        Self::ReverseSide(val)
    }
}
impl TryFrom<PassportElementError> for crate::types::PassportElementErrorReverseSide {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PassportElementError) -> Result<Self, Self::Error> {
        if let PassportElementError::ReverseSide(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PassportElementError),
                stringify!(PassportElementErrorReverseSide),
            ))
        }
    }
}
impl From<crate::types::PassportElementErrorSelfie> for PassportElementError {
    fn from(val: crate::types::PassportElementErrorSelfie) -> Self {
        Self::Selfie(val)
    }
}
impl TryFrom<PassportElementError> for crate::types::PassportElementErrorSelfie {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PassportElementError) -> Result<Self, Self::Error> {
        if let PassportElementError::Selfie(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PassportElementError),
                stringify!(PassportElementErrorSelfie),
            ))
        }
    }
}
impl From<crate::types::PassportElementErrorFile> for PassportElementError {
    fn from(val: crate::types::PassportElementErrorFile) -> Self {
        Self::File(val)
    }
}
impl TryFrom<PassportElementError> for crate::types::PassportElementErrorFile {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PassportElementError) -> Result<Self, Self::Error> {
        if let PassportElementError::File(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PassportElementError),
                stringify!(PassportElementErrorFile),
            ))
        }
    }
}
impl From<crate::types::PassportElementErrorFiles> for PassportElementError {
    fn from(val: crate::types::PassportElementErrorFiles) -> Self {
        Self::Files(val)
    }
}
impl TryFrom<PassportElementError> for crate::types::PassportElementErrorFiles {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PassportElementError) -> Result<Self, Self::Error> {
        if let PassportElementError::Files(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PassportElementError),
                stringify!(PassportElementErrorFiles),
            ))
        }
    }
}
impl From<crate::types::PassportElementErrorTranslationFile> for PassportElementError {
    fn from(val: crate::types::PassportElementErrorTranslationFile) -> Self {
        Self::TranslationFile(val)
    }
}
impl TryFrom<PassportElementError> for crate::types::PassportElementErrorTranslationFile {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PassportElementError) -> Result<Self, Self::Error> {
        if let PassportElementError::TranslationFile(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PassportElementError),
                stringify!(PassportElementErrorTranslationFile),
            ))
        }
    }
}
impl From<crate::types::PassportElementErrorTranslationFiles> for PassportElementError {
    fn from(val: crate::types::PassportElementErrorTranslationFiles) -> Self {
        Self::TranslationFiles(val)
    }
}
impl TryFrom<PassportElementError> for crate::types::PassportElementErrorTranslationFiles {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PassportElementError) -> Result<Self, Self::Error> {
        if let PassportElementError::TranslationFiles(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PassportElementError),
                stringify!(PassportElementErrorTranslationFiles),
            ))
        }
    }
}
impl From<crate::types::PassportElementErrorUnspecified> for PassportElementError {
    fn from(val: crate::types::PassportElementErrorUnspecified) -> Self {
        Self::Unspecified(val)
    }
}
impl TryFrom<PassportElementError> for crate::types::PassportElementErrorUnspecified {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: PassportElementError) -> Result<Self, Self::Error> {
        if let PassportElementError::Unspecified(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(PassportElementError),
                stringify!(PassportElementErrorUnspecified),
            ))
        }
    }
}
