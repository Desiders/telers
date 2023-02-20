use super::{
    InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaPhoto, InputMediaVideo,
};

use serde::Serialize;

/// This object represents the content of a media message to be sent. It should be one of
/// - [`InputMediaAnimation`]
/// - [`InputMediaDocument`]
/// - [`InputMediaAudio`]
/// - [`InputMediaPhoto`]
/// - [`InputMediaVideo`]
/// # Documentation
/// <https://core.telegram.org/bots/api#inputmedia>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputMedia<'a> {
    Animation(InputMediaAnimation<'a>),
    Document(InputMediaDocument<'a>),
    Audio(InputMediaAudio<'a>),
    Photo(InputMediaPhoto<'a>),
    Video(InputMediaVideo<'a>),
}

impl<'a> From<InputMediaAnimation<'a>> for InputMedia<'a> {
    fn from(input_media_animation: InputMediaAnimation<'a>) -> Self {
        Self::Animation(input_media_animation)
    }
}

impl<'a> From<InputMediaDocument<'a>> for InputMedia<'a> {
    fn from(input_media_document: InputMediaDocument<'a>) -> Self {
        Self::Document(input_media_document)
    }
}

impl<'a> From<InputMediaAudio<'a>> for InputMedia<'a> {
    fn from(input_media_audio: InputMediaAudio<'a>) -> Self {
        Self::Audio(input_media_audio)
    }
}

impl<'a> From<InputMediaPhoto<'a>> for InputMedia<'a> {
    fn from(input_media_photo: InputMediaPhoto<'a>) -> Self {
        Self::Photo(input_media_photo)
    }
}

impl<'a> From<InputMediaVideo<'a>> for InputMedia<'a> {
    fn from(input_media_video: InputMediaVideo<'a>) -> Self {
        Self::Video(input_media_video)
    }
}
