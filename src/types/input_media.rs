use super::{
    InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaPhoto, InputMediaVideo,
};

use serde::{Deserialize, Serialize};

/// This object represents the content of a media message to be sent. It should be one of
/// - `aiogram_rs.types.input_media_animation.InputMediaAnimation`
/// - `aiogram_rs.types.input_media_document.InputMediaDocument`
/// - `aiogram_rs.types.input_media_audio.InputMediaAudio`
/// - `aiogram_rs.types.input_media_photo.InputMediaPhoto`
/// - `aiogram_rs.types.input_media_video.InputMediaVideo`
/// <https://core.telegram.org/bots/api#inputmedia>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputMedia {
    Animation(InputMediaAnimation),
    Document(InputMediaDocument),
    Audio(InputMediaAudio),
    Photo(InputMediaPhoto),
    Video(InputMediaVideo),
}
