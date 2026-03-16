use crate::types::PassportElementError;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
/// - [`crate::types::PassportElementErrorDataField`]
/// - [`crate::types::PassportElementErrorFrontSide`]
/// - [`crate::types::PassportElementErrorReverseSide`]
/// - [`crate::types::PassportElementErrorSelfie`]
/// - [`crate::types::PassportElementErrorFile`]
/// - [`crate::types::PassportElementErrorFiles`]
/// - [`crate::types::PassportElementErrorTranslationFile`]
/// - [`crate::types::PassportElementErrorTranslationFiles`]
/// - [`crate::types::PassportElementErrorUnspecified`]
/// # Documentation
/// <https://core.telegram.org/bots/api#passportelementerror>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum PassportElementErrorType {
    #[strum(serialize = "data")]
    Data,
    #[strum(serialize = "front_side")]
    FrontSide,
    #[strum(serialize = "reverse_side")]
    ReverseSide,
    #[strum(serialize = "selfie")]
    Selfie,
    #[strum(serialize = "file")]
    File,
    #[strum(serialize = "files")]
    Files,
    #[strum(serialize = "translation_file")]
    TranslationFile,
    #[strum(serialize = "translation_files")]
    TranslationFiles,
    #[strum(serialize = "unspecified")]
    Unspecified,
}
impl PassportElementErrorType {
    #[must_use]
    pub const fn all() -> [PassportElementErrorType; 9usize] {
        [
            PassportElementErrorType::Data,
            PassportElementErrorType::FrontSide,
            PassportElementErrorType::ReverseSide,
            PassportElementErrorType::Selfie,
            PassportElementErrorType::File,
            PassportElementErrorType::Files,
            PassportElementErrorType::TranslationFile,
            PassportElementErrorType::TranslationFiles,
            PassportElementErrorType::Unspecified,
        ]
    }
}
impl From<PassportElementErrorType> for Box<str> {
    fn from(val: PassportElementErrorType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<PassportElementErrorType> for String {
    fn from(val: PassportElementErrorType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for PassportElementErrorType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a PassportElementError> for PassportElementErrorType {
    fn from(val: &'a PassportElementError) -> Self {
        match val {
            PassportElementError::Data(_) => PassportElementErrorType::Data,
            PassportElementError::FrontSide(_) => PassportElementErrorType::FrontSide,
            PassportElementError::ReverseSide(_) => PassportElementErrorType::ReverseSide,
            PassportElementError::Selfie(_) => PassportElementErrorType::Selfie,
            PassportElementError::File(_) => PassportElementErrorType::File,
            PassportElementError::Files(_) => PassportElementErrorType::Files,
            PassportElementError::TranslationFile(_) => PassportElementErrorType::TranslationFile,
            PassportElementError::TranslationFiles(_) => PassportElementErrorType::TranslationFiles,
            PassportElementError::Unspecified(_) => PassportElementErrorType::Unspecified,
        }
    }
}
impl From<PassportElementError> for PassportElementErrorType {
    fn from(val: PassportElementError) -> Self {
        PassportElementErrorType::from(&val)
    }
}
