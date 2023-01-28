use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, Message},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method when you need to tell the user that something is happening on the bot's side. The status is set for 5 seconds or less (when a message arrives from your bot, Telegram clients clear its typing status).
/// # Documentation
/// <https://core.telegram.org/bots/api#sendchataction>
/// # Notes
/// We only recommend using this method when a response from the bot will take a **noticeable** amount of time to arrive.
/// # Example
/// The [ImageBot](https://t.me/imagebot) needs some time to process a request and upload the image. Instead of sending a text message along the lines of “Retrieving image, please wait…”, the bot may use [`SendChatAction`](crate::methods::SendChatAction) with `action = upload_photo`. The user will see a “sending photo” status for the bot.
/// # Returns
/// Returns `True` on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SendChatAction {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread; supergroups only
    pub message_thread_id: Option<i64>,
    /// Type of action to broadcast. Choose one, depending on what the user is about to receive: `typing` for [`text messages`](crate::methods::SendMessage), `upload_photo` for [`photos`](crate::methods::SendPhoto), `record_video` or `upload_video` for [`videos`](crate::methods::SendVideo), `record_voice` or `upload_voice` for [`voice notes`](crate::methods::SendVoice), `upload_document` for [`general files`](crate::methods::SendDocument), `choose_sticker` for [`stickers`](crate::methods::SendSticker), `find_location` for [`location data`](crate::methods::SendLocation), `record_video_note` or `upload_video_note` for [`video notes`](crate::methods::SendVideoNote).
    pub action: String,
}

impl SendChatAction {
    #[must_use]
    pub fn new<C: Into<ChatIdKind>, T: Into<String>>(chat_id: C, action: T) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            action: action.into(),
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn message_thread_id(mut self, val: i64) -> Self {
        self.message_thread_id = Some(val);
        self
    }

    #[must_use]
    pub fn action<T: Into<String>>(mut self, val: T) -> Self {
        self.action = val.into();
        self
    }
}

impl TelegramMethod for SendChatAction {
    type Method = Self;
    type Return = Message;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("sendChatAction", self, None)
    }
}
