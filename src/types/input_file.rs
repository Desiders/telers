use serde::{Deserialize, Serialize};

/// This object represents the contents of a file to be uploaded. Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
/// <https://core.telegram.org/bots/api#inputfile>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InputFile {
    pub filename: Option<String>,
    pub chunk_size: i64,
}
