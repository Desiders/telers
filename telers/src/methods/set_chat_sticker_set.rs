use crate::client::Bot;
use serde::Serialize;
/// Use this method to set a new group sticker set for a supergroup. The bot must be an administrator in the chat for this to work and must have the appropriate administrator rights. Use the field `can_set_sticker_set` optionally returned in getChat requests to check if the bot can use this method. Returns `true` on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#setchatstickerset>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SetChatStickerSet {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Name of the sticker set to be set as the group sticker set
    pub sticker_set_name: Box<str>,
}
impl SetChatStickerSet {
    /// Creates a new `SetChatStickerSet`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username
    /// * `sticker_set_name` - Name of the sticker set to be set as the group sticker set
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<Box<str>>>(
        chat_id: T0,
        sticker_set_name: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            sticker_set_name: sticker_set_name.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Name of the sticker set to be set as the group sticker set
    #[must_use]
    pub fn sticker_set_name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sticker_set_name = val.into();
        self
    }
}
impl super::TelegramMethod for SetChatStickerSet {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("setChatStickerSet", self, None)
    }
}
