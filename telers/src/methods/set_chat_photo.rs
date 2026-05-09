use crate::client::Bot;
use serde::Serialize;
/// Use this method to set a new profile photo for the chat. Photos can't be changed for private chats. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatphoto>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// New chat photo, uploaded using multipart/form-data
    pub photo: crate::types::InputFile,
}
impl SetChatPhoto {
    /// Creates a new `SetChatPhoto`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target channel in the format @username
    /// * `photo` - New chat photo, uploaded using multipart/form-data
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<crate::types::InputFile>>(
        chat_id: T0,
        photo: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            photo: photo.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// New chat photo, uploaded using multipart/form-data
    #[must_use]
    pub fn photo<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.photo = val.into();
        self
    }
}
impl super::TelegramMethod for SetChatPhoto {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_file(&mut files, &mut self.photo);
        super::Request::new("setChatPhoto", self, Some(files))
    }
}
