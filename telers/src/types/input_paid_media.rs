use super::{InputPaidMediaPhoto, InputPaidMediaVideo};

use serde::Serialize;

/// This object describes the paid media to be sent. Currently, it can be one of
/// - [`InputPaidMediaPhoto`]
/// - [`InputPaidMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputpaidmedia>
#[derive(Debug, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputPaidMedia {
    Photo(InputPaidMediaPhoto),
    Video(InputPaidMediaVideo),
}

impl From<InputPaidMediaPhoto> for InputPaidMedia {
    fn from(fill: InputPaidMediaPhoto) -> Self {
        Self::Photo(fill)
    }
}

impl From<InputPaidMediaVideo> for InputPaidMedia {
    fn from(fill: InputPaidMediaVideo) -> Self {
        Self::Video(fill)
    }
}
