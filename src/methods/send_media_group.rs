use super::base::{prepare_input_media, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, InputMedia, Message},
};

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

/// Use this method to send a group of photos, videos, documents or audios as an album. Documents and audio files can be only grouped in an album with messages of the same type.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendmediagroup>
/// # Returns
/// On success, an array of [`Message`]s that were sent is returned
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SendMediaGroup<'a> {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// A JSON-serialized array describing messages to be sent, must include 2-10 items
    pub media: Vec<InputMedia<'a>>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    /// Pass `True`, if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
}

impl<'a> SendMediaGroup<'a> {
    #[must_use]
    pub fn new<C: Into<ChatIdKind>, M: Into<InputMedia<'a>>>(chat_id: C, media: Vec<M>) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            media: media.into_iter().map(Into::into).collect(),
            disable_notification: None,
            protect_content: None,
            reply_to_message_id: None,
            allow_sending_without_reply: None,
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
    pub fn media<T: Into<InputMedia<'a>>>(mut self, val: Vec<T>) -> Self {
        self.media = val.into_iter().map(Into::into).collect();
        self
    }

    #[must_use]
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    #[must_use]
    pub fn protect_content(mut self, val: bool) -> Self {
        self.protect_content = Some(val);
        self
    }

    #[must_use]
    pub fn reply_to_message_id(mut self, val: i64) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    #[must_use]
    pub fn allow_sending_without_reply(mut self, val: bool) -> Self {
        self.allow_sending_without_reply = Some(val);
        self
    }
}

impl<'a> TelegramMethod for SendMediaGroup<'a> {
    type Method = Self;
    type Return = Vec<Message>;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        let mut files = HashMap::new();
        prepare_input_media(&mut files, &self.media);

        Request::new("sendMediaGroup", self, Some(files))
    }
}
