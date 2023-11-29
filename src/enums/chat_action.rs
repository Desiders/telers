use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the chat action
/// # Documentation
/// <https://core.telegram.org/bots/api#sendchataction>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum ChatAction {
    #[strum(serialize = "typing")]
    Typing,
    #[strum(serialize = "upload_photo")]
    UploadPhoto,
    #[strum(serialize = "record_video")]
    RecordVideo,
    #[strum(serialize = "upload_video")]
    UploadVideo,
    #[strum(serialize = "record_voice")]
    RecordVoice,
    #[strum(serialize = "upload_voice")]
    UploadVoice,
    #[strum(serialize = "upload_document")]
    UploadDocument,
    #[strum(serialize = "choose_sticker")]
    ChooseSticker,
    #[strum(serialize = "find_location")]
    FindLocation,
    #[strum(serialize = "record_video_note")]
    RecordVideoNote,
    #[strum(serialize = "upload_video_note")]
    UploadVideoNote,
}

impl ChatAction {
    #[must_use]
    pub const fn all() -> [ChatAction; 11] {
        [
            ChatAction::Typing,
            ChatAction::UploadPhoto,
            ChatAction::RecordVideo,
            ChatAction::UploadVideo,
            ChatAction::RecordVoice,
            ChatAction::UploadVoice,
            ChatAction::UploadDocument,
            ChatAction::ChooseSticker,
            ChatAction::FindLocation,
            ChatAction::RecordVideoNote,
            ChatAction::UploadVideoNote,
        ]
    }
}

impl From<ChatAction> for Box<str> {
    fn from(chat_action: ChatAction) -> Self {
        Into::<&'static str>::into(chat_action).into()
    }
}

impl<'a> PartialEq<&'a str> for ChatAction {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
