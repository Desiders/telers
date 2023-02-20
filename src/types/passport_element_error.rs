use super::{
    PassportElementErrorDataField, PassportElementErrorFile, PassportElementErrorFiles,
    PassportElementErrorFrontSide, PassportElementErrorReverseSide, PassportElementErrorSelfie,
    PassportElementErrorTranslationFile, PassportElementErrorTranslationFiles,
    PassportElementErrorUnspecified,
};

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
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(tag = "source", rename_all = "snake_case")]
pub enum PassportElementError {
    DataField(PassportElementErrorDataField),
    FrontSide(PassportElementErrorFrontSide),
    ReverseSide(PassportElementErrorReverseSide),
    Selfie(PassportElementErrorSelfie),
    File(PassportElementErrorFile),
    Files(PassportElementErrorFiles),
    TranslationFile(PassportElementErrorTranslationFile),
    TranslationFiles(PassportElementErrorTranslationFiles),
    Unspecified(PassportElementErrorUnspecified),
}

impl From<PassportElementErrorDataField> for PassportElementError {
    fn from(val: PassportElementErrorDataField) -> Self {
        Self::DataField(val)
    }
}

impl From<PassportElementErrorFrontSide> for PassportElementError {
    fn from(val: PassportElementErrorFrontSide) -> Self {
        Self::FrontSide(val)
    }
}

impl From<PassportElementErrorReverseSide> for PassportElementError {
    fn from(val: PassportElementErrorReverseSide) -> Self {
        Self::ReverseSide(val)
    }
}

impl From<PassportElementErrorSelfie> for PassportElementError {
    fn from(val: PassportElementErrorSelfie) -> Self {
        Self::Selfie(val)
    }
}

impl From<PassportElementErrorFile> for PassportElementError {
    fn from(val: PassportElementErrorFile) -> Self {
        Self::File(val)
    }
}

impl From<PassportElementErrorFiles> for PassportElementError {
    fn from(val: PassportElementErrorFiles) -> Self {
        Self::Files(val)
    }
}

impl From<PassportElementErrorTranslationFile> for PassportElementError {
    fn from(val: PassportElementErrorTranslationFile) -> Self {
        Self::TranslationFile(val)
    }
}

impl From<PassportElementErrorTranslationFiles> for PassportElementError {
    fn from(val: PassportElementErrorTranslationFiles) -> Self {
        Self::TranslationFiles(val)
    }
}

impl From<PassportElementErrorUnspecified> for PassportElementError {
    fn from(val: PassportElementErrorUnspecified) -> Self {
        Self::Unspecified(val)
    }
}
