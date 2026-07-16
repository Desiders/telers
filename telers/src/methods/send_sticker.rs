use crate::client::Bot;
use serde::Serialize;
/// Use this method to send static .WEBP, animated .TGS, or video .WEBM stickers. On success, the sent Message is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendsticker>
/// # Returns
/// - `crate::types::Message`
#[derive(Clone, Debug, Serialize)]
pub struct SendSticker {
    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i64>,
    /// For outgoing ephemeral messages, unique identifier of the user who will receive the message; for group and supergroup chats only. It is not guaranteed that the user will receive the message, especially if they are offline. See ephemeral message sending for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<i64>,
    /// For outgoing ephemeral messages, identifier of the callback query which triggered the message if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query_id: Option<Box<str>>,
    /// Sticker to send. Pass a `file_id` as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Video and animated stickers can't be sent via an HTTP URL.
    pub sticker: crate::types::InputFile,
    /// Emoji associated with the sticker; only for just uploaded stickers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<Box<str>>,
    /// Sends the message silently. Users will receive a notification with no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<Box<str>>,
    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<crate::types::SuggestedPostParameters>,
    /// Description of the message to reply to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<crate::types::ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::ReplyMarkup>,
}
impl SendSticker {
    /// Creates a new `SendSticker`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    /// * `sticker` - Sticker to send. Pass a `file_id` as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Video and animated stickers can't be sent via an HTTP URL.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<crate::types::InputFile>>(
        chat_id: T0,
        sticker: T1,
    ) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            receiver_user_id: None,
            callback_query_id: None,
            sticker: sticker.into(),
            emoji: None,
            disable_notification: None,
            protect_content: None,
            allow_paid_broadcast: None,
            message_effect_id: None,
            suggested_post_parameters: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Unique identifier of the business connection on behalf of which the message will be sent
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.business_connection_id = val.map(Into::into);
        self
    }

    /// Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_thread_id = Some(val.into());
        self
    }

    /// Unique identifier for the target message thread (topic) of a forum; for forum supergroups and private chats of bots with forum topic mode enabled only
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_thread_id = val.map(Into::into);
        self
    }

    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.direct_messages_topic_id = Some(val.into());
        self
    }

    /// Identifier of the direct messages topic to which the message will be sent; required if the message is sent to a direct messages chat
    #[must_use]
    pub fn direct_messages_topic_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.direct_messages_topic_id = val.map(Into::into);
        self
    }

    /// For outgoing ephemeral messages, unique identifier of the user who will receive the message; for group and supergroup chats only. It is not guaranteed that the user will receive the message, especially if they are offline. See ephemeral message sending for more details.
    #[must_use]
    pub fn receiver_user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.receiver_user_id = Some(val.into());
        self
    }

    /// For outgoing ephemeral messages, unique identifier of the user who will receive the message; for group and supergroup chats only. It is not guaranteed that the user will receive the message, especially if they are offline. See ephemeral message sending for more details.
    #[must_use]
    pub fn receiver_user_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.receiver_user_id = val.map(Into::into);
        self
    }

    /// For outgoing ephemeral messages, identifier of the callback query which triggered the message if any
    #[must_use]
    pub fn callback_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.callback_query_id = Some(val.into());
        self
    }

    /// For outgoing ephemeral messages, identifier of the callback query which triggered the message if any
    #[must_use]
    pub fn callback_query_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.callback_query_id = val.map(Into::into);
        self
    }

    /// Sticker to send. Pass a `file_id` as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .WEBP sticker from the Internet, or upload a new .WEBP, .TGS, or .WEBM sticker using multipart/form-data. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>. Video and animated stickers can't be sent via an HTTP URL.
    #[must_use]
    pub fn sticker<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.sticker = val.into();
        self
    }

    /// Emoji associated with the sticker; only for just uploaded stickers
    #[must_use]
    pub fn emoji<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.emoji = Some(val.into());
        self
    }

    /// Emoji associated with the sticker; only for just uploaded stickers
    #[must_use]
    pub fn emoji_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.emoji = val.map(Into::into);
        self
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }

    /// Sends the message silently. Users will receive a notification with no sound.
    #[must_use]
    pub fn disable_notification_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.disable_notification = val.map(Into::into);
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content<T: Into<bool>>(mut self, val: T) -> Self {
        self.protect_content = Some(val.into());
        self
    }

    /// Protects the contents of the sent message from forwarding and saving
    #[must_use]
    pub fn protect_content_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.protect_content = val.map(Into::into);
        self
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[must_use]
    pub fn allow_paid_broadcast<T: Into<bool>>(mut self, val: T) -> Self {
        self.allow_paid_broadcast = Some(val.into());
        self
    }

    /// Pass `true` to allow up to 1000 messages per second, ignoring broadcasting limits for a fee of 0.1 Telegram Stars per message. The relevant Stars will be withdrawn from the bot's balance.
    #[must_use]
    pub fn allow_paid_broadcast_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allow_paid_broadcast = val.map(Into::into);
        self
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.message_effect_id = Some(val.into());
        self
    }

    /// Unique identifier of the message effect to be added to the message; for private chats only
    #[must_use]
    pub fn message_effect_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.message_effect_id = val.map(Into::into);
        self
    }

    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[must_use]
    pub fn suggested_post_parameters<T: Into<crate::types::SuggestedPostParameters>>(
        mut self,
        val: T,
    ) -> Self {
        self.suggested_post_parameters = Some(val.into());
        self
    }

    /// A JSON-serialized object containing the parameters of the suggested post to send; for direct messages chats only. If the message is sent as a reply to another suggested post, then that suggested post is automatically declined.
    #[must_use]
    pub fn suggested_post_parameters_option<T: Into<crate::types::SuggestedPostParameters>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.suggested_post_parameters = val.map(Into::into);
        self
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters<T: Into<crate::types::ReplyParameters>>(mut self, val: T) -> Self {
        self.reply_parameters = Some(val.into());
        self
    }

    /// Description of the message to reply to
    #[must_use]
    pub fn reply_parameters_option<T: Into<crate::types::ReplyParameters>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_parameters = val.map(Into::into);
        self
    }

    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user.
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::ReplyMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove a reply keyboard or to force a reply from the user.
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::ReplyMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for SendSticker {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_file(&mut files, &mut self.sticker);
        super::Request::new("sendSticker", self, Some(files))
    }
}
