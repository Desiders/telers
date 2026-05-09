use crate::client::Bot;
use serde::Serialize;
/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status). Returns `true` on success.
/// We only recommend using this method when a response from the bot will take a noticeable amount of time to arrive.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendchataction>
/// # Returns
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct SendChatAction {
    /// Unique identifier of the business connection on behalf of which the action will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username. Channel chats and channel direct messages chats aren't supported.
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread or topic of a forum; for supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, `upload_photo` for photos, `record_video` or `upload_video` for videos, `record_voice` or `upload_voice` for voice notes, `upload_document` for general files, `choose_sticker` for stickers, `find_location` for location data, `record_video_note` or `upload_video_note` for video notes.
    pub action: Box<str>,
}
impl SendChatAction {
    /// Creates a new `SendChatAction`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username. Channel chats and channel direct messages chats aren't supported.
    /// * `action` - Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, `upload_photo` for photos, `record_video` or `upload_video` for videos, `record_voice` or `upload_voice` for voice notes, `upload_document` for general files, `choose_sticker` for stickers, `find_location` for location data, `record_video_note` or `upload_video_note` for video notes.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<Box<str>>>(
        chat_id: T0,
        action: T1,
    ) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            action: action.into(),
        }
    }

    /// Unique identifier of the business connection on behalf of which the action will be sent
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = Some(val.into());
        this
    }

    /// Unique identifier of the business connection on behalf of which the action will be sent
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.business_connection_id = val.map(Into::into);
        this
    }

    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username. Channel chats and channel direct messages chats aren't supported.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }

    /// Unique identifier for the target message thread or topic of a forum; for supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_thread_id = Some(val.into());
        this
    }

    /// Unique identifier for the target message thread or topic of a forum; for supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_thread_id = val.map(Into::into);
        this
    }

    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: typing for text messages, `upload_photo` for photos, `record_video` or `upload_video` for videos, `record_voice` or `upload_voice` for voice notes, `upload_document` for general files, `choose_sticker` for stickers, `find_location` for location data, `record_video_note` or `upload_video_note` for video notes.
    #[must_use]
    pub fn action<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.action = val.into();
        this
    }
}
impl super::TelegramMethod for SendChatAction {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("sendChatAction", self, None)
    }
}
