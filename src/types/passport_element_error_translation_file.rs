use serde::{Deserialize, Serialize};

/// Represents an issue with one of the files that constitute the translation of a document. The error is considered resolved when the file changes.
/// <https://core.telegram.org/bots/api#passportelementerrortranslationfile>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct PassportElementErrorTranslationFile {
    /// Error source, must be *translation_file*
    #[serde(default = "translation_file")]
    pub source: String,
    #[serde(rename = "type")]
    pub element_type: String,
    /// Base64-encoded file hash
    pub file_hash: String,
    /// Error message
    pub message: String,
}

impl Default for PassportElementErrorTranslationFile {
    fn default() -> Self {
        Self {
            source: translation_file(),
            element_type: String::default(),
            file_hash: String::default(),
            message: String::default(),
        }
    }
}

fn translation_file() -> String {
    "translation_file".to_string()
}
