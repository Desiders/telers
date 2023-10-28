use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};

/// This enum represents all possible types of the chat action
/// # Documentation
/// <https://core.telegram.org/bots/api#sendchataction>
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
    RecordVideoNote,
    UploadVideoNote,
}

impl ChatAction {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            ChatAction::Typing => "typing",
            ChatAction::UploadPhoto => "upload_photo",
            ChatAction::RecordVideo => "record_video",
            ChatAction::UploadVideo => "upload_video",
            ChatAction::RecordAudio => "record_audio",
            ChatAction::UploadAudio => "upload_audio",
            ChatAction::UploadDocument => "upload_document",
            ChatAction::FindLocation => "find_location",
            ChatAction::RecordVideoNote => "record_video_note",
            ChatAction::UploadVideoNote => "upload_video_note",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [ChatAction; 10] {
        &[
            ChatAction::Typing,
            ChatAction::UploadPhoto,
            ChatAction::RecordVideo,
            ChatAction::UploadVideo,
            ChatAction::RecordAudio,
            ChatAction::UploadAudio,
            ChatAction::UploadDocument,
            ChatAction::FindLocation,
            ChatAction::RecordVideoNote,
            ChatAction::UploadVideoNote,
        ]
    }
}

impl Deref for ChatAction {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for ChatAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<ChatAction> for String {
    fn from(action: ChatAction) -> Self {
        action.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for ChatAction {
    fn eq(&self, other: &&'a str) -> bool {
        self == other
    }
}
