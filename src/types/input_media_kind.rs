use super::InputFile;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMediaKind {
    FileIdOrUrl(String),
    File(InputFile),
}
