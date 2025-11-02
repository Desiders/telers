use super::{InputStoryContentPhoto, InputStoryContentVideo};

use serde::Serialize;

/// This object describes the content of a story to post. Currently, it can be one of
/// - [`InputStoryContentPhoto`]
/// - [`InputStoryContentVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputstorycontent>
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputStoryContent<'a> {
    Photo(InputStoryContentPhoto<'a>),
    Video(InputStoryContentVideo<'a>),
}

impl<'a> From<InputStoryContentPhoto<'a>> for InputStoryContent<'a> {
    fn from(content: InputStoryContentPhoto<'a>) -> Self {
        Self::Photo(content)
    }
}

impl<'a> From<InputStoryContentVideo<'a>> for InputStoryContent<'a> {
    fn from(content: InputStoryContentVideo<'a>) -> Self {
        Self::Video(content)
    }
}
