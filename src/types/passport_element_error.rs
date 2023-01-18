use super::{
    PassportElementErrorDataField, PassportElementErrorFile, PassportElementErrorFiles,
    PassportElementErrorFrontSide, PassportElementErrorReverseSide, PassportElementErrorSelfie,
    PassportElementErrorTranslationFile, PassportElementErrorTranslationFiles,
    PassportElementErrorUnspecified,
};

use serde::Deserialize;

/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
/// - `aiogram_rs.types.passport_element_error_data_field.PassportElementErrorDataField`
/// - `aiogram_rs.types.passport_element_error_front_side.PassportElementErrorFrontSide`
/// - `aiogram_rs.types.passport_element_error_reverse_side.PassportElementErrorReverseSide`
/// - `aiogram_rs.types.passport_element_error_selfie.PassportElementErrorSelfie`
/// - `aiogram_rs.types.passport_element_error_file.PassportElementErrorFile`
/// - `aiogram_rs.types.passport_element_error_files.PassportElementErrorFiles`
/// - `aiogram_rs.types.passport_element_error_translation_file.PassportElementErrorTranslationFile`
/// - `aiogram_rs.types.passport_element_error_translation_files.PassportElementErrorTranslationFiles`
/// - `aiogram_rs.types.passport_element_error_unspecified.PassportElementErrorUnspecified`
/// <https://core.telegram.org/bots/api#passportelementerror>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
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
