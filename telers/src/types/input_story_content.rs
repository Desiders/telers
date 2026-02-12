use super::{InputStoryContentPhoto, InputStoryContentVideo};

use serde::Serialize;

/// This object describes the content of a story to post. Currently, it can be one of
/// - [`InputStoryContentPhoto`]
/// - [`InputStoryContentVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputstorycontent>
#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputStoryContent {
    Photo(InputStoryContentPhoto),
    Video(InputStoryContentVideo),
}

impl From<InputStoryContentPhoto> for InputStoryContent {
    fn from(content: InputStoryContentPhoto) -> Self {
        Self::Photo(content)
    }
}

impl From<InputStoryContentVideo> for InputStoryContent {
    fn from(content: InputStoryContentVideo) -> Self {
        Self::Video(content)
    }
}
