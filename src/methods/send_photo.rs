use super::base::{prepare_file_with_value, Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{InputFile, Message, MessageEntity, ReplyMarkup},
};

use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct SendPhoto {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: i64,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Photo to send. Pass a file_id as String to send a photo that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a photo from the Internet, or upload a new photo using multipart/form-data. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20. See `more information on Sending Files <https://core.telegram.org/bots/api#sending-files>`.
    pub photo: InputFile,
    /// Photo caption (may also be used when resending photos by *file_id*), 0-1024 characters after entities parsing
    pub caption: Option<String>,
    /// Mode for parsing entities in the photo caption. See `formatting options <https://core.telegram.org/bots/api#formatting-options>` for more details.
    pub parse_mode: Option<String>,
    /// List of special entities that appear in the caption, which can be specified instead of *parse_mode*
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, *id* of the original message
    pub reply_to_message_id: Option<i64>,
    /// Pass `True` if the message should be sent even if the specified replied-to message is not found
    pub allow_sending_without_reply: Option<bool>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl TelegramMethod for SendPhoto {
    type Method = Self;
    type Return = Message;

    fn build_request(&self, _bot: &Bot) -> Request<Self::Method> {
        let mut files = HashMap::new();
        prepare_file_with_value(&mut files, &self.photo, "photo");

        Request::new("sendPhoto", self, Some(files))
    }
}
