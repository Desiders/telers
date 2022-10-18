use super::{
    PassportElementErrorDataField, PassportElementErrorFile, PassportElementErrorFiles,
    PassportElementErrorFrontSide, PassportElementErrorReverseSide, PassportElementErrorSelfie,
    PassportElementErrorTranslationFile, PassportElementErrorTranslationFiles,
    PassportElementErrorUnspecified,
};

use serde::{Deserialize, Serialize};

/// This object represents an error in the Telegram Passport element which was submitted that should be resolved by the user. It should be one of:
/// - :class:`aiogram_rs.types.passport_element_error_data_field.PassportElementErrorDataField`
/// - :class:`aiogram_rs.types.passport_element_error_front_side.PassportElementErrorFrontSide`
/// - :class:`aiogram_rs.types.passport_element_error_reverse_side.PassportElementErrorReverseSide`
/// - :class:`aiogram_rs.types.passport_element_error_selfie.PassportElementErrorSelfie`
/// - :class:`aiogram_rs.types.passport_element_error_file.PassportElementErrorFile`
/// - :class:`aiogram_rs.types.passport_element_error_files.PassportElementErrorFiles`
/// - :class:`aiogram_rs.types.passport_element_error_translation_file.PassportElementErrorTranslationFile`
/// - :class:`aiogram_rs.types.passport_element_error_translation_files.PassportElementErrorTranslationFiles`
/// - :class:`aiogram_rs.types.passport_element_error_unspecified.PassportElementErrorUnspecified`
/// <https://core.telegram.org/bots/api#passportelementerror>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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
