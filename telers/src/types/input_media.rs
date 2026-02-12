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
#[derive(Debug, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}

impl From<InputMediaAnimation> for InputMedia {
    fn from(input_media_animation: InputMediaAnimation) -> Self {
        Self::Animation(input_media_animation)
    }
}

impl From<InputMediaDocument> for InputMedia {
    fn from(input_media_document: InputMediaDocument) -> Self {
        Self::Document(input_media_document)
    }
}

impl From<InputMediaAudio> for InputMedia {
    fn from(input_media_audio: InputMediaAudio) -> Self {
        Self::Audio(input_media_audio)
    }
}

impl From<InputMediaPhoto> for InputMedia {
    fn from(input_media_photo: InputMediaPhoto) -> Self {
        Self::Photo(input_media_photo)
    }
}

impl From<InputMediaVideo> for InputMedia {
    fn from(input_media_video: InputMediaVideo) -> Self {
        Self::Video(input_media_video)
    }
}
