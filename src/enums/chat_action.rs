use std::fmt::{self, Debug};

#[derive(Clone, Eq, PartialEq, Hash)]
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

impl Debug for ChatAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
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

impl From<ChatAction> for String {
    fn from(action: ChatAction) -> Self {
        action.as_str().to_string()
    }
}

impl<'a> From<&'a ChatAction> for String {
    fn from(action: &'a ChatAction) -> Self {
        action.as_str().to_string()
    }
}
