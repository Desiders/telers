use crate::client::Bot;
use serde::Serialize;
/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video without sound). On success, the sent Message is returned. Bots can currently send animation files of up to 50 MB in size, this limit may be changed in the future.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendanimation>
/// # Returns
/// - `crate::types::Message`
#[derive(Clone, Debug, Serialize)]
pub struct SendAnimation {
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
    /// Animation to send. Pass a `file_id` as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    pub animation: crate::types::InputFile,
    /// Duration of sent animation in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Animation width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Animation height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<crate::types::InputFile>,
    /// Animation caption (may also be used when resending animation by `file_id`), 0-1024 characters after entities parsing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<str>>,
    /// Mode for parsing entities in the animation caption. See formatting options for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<Box<str>>,
    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Pass `true` if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// Pass `true` if the animation needs to be covered with a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
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
impl SendAnimation {
    /// Creates a new `SendAnimation`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target bot, supergroup or channel in the format @username
    /// * `animation` - Animation to send. Pass a `file_id` as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<crate::types::InputFile>>(
        chat_id: T0,
        animation: T1,
    ) -> Self {
        Self {
            business_connection_id: None,
            chat_id: chat_id.into(),
            message_thread_id: None,
            direct_messages_topic_id: None,
            receiver_user_id: None,
            callback_query_id: None,
            animation: animation.into(),
            duration: None,
            width: None,
            height: None,
            thumbnail: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_spoiler: None,
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

    /// Animation to send. Pass a `file_id` as String to send an animation that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an animation from the Internet, or upload a new animation using multipart/form-data. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn animation<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.animation = val.into();
        self
    }

    /// Duration of sent animation in seconds
    #[must_use]
    pub fn duration<T: Into<i64>>(mut self, val: T) -> Self {
        self.duration = Some(val.into());
        self
    }

    /// Duration of sent animation in seconds
    #[must_use]
    pub fn duration_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.duration = val.map(Into::into);
        self
    }

    /// Animation width
    #[must_use]
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = Some(val.into());
        self
    }

    /// Animation width
    #[must_use]
    pub fn width_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.width = val.map(Into::into);
        self
    }

    /// Animation height
    #[must_use]
    pub fn height<T: Into<i64>>(mut self, val: T) -> Self {
        self.height = Some(val.into());
        self
    }

    /// Animation height
    #[must_use]
    pub fn height_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.height = val.map(Into::into);
        self
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail<T: Into<crate::types::InputFile>>(mut self, val: T) -> Self {
        self.thumbnail = Some(val.into());
        self
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail's width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can't be reused and can be only uploaded as a new file, so you can pass `attach://<file_attach_name>` if the thumbnail was uploaded using multipart/form-data under <`file_attach_name`>. More information on Sending Files: <https://core.telegram.org/bots/api#sending-files>
    #[must_use]
    pub fn thumbnail_option<T: Into<crate::types::InputFile>>(mut self, val: Option<T>) -> Self {
        self.thumbnail = val.map(Into::into);
        self
    }

    /// Animation caption (may also be used when resending animation by `file_id`), 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Animation caption (may also be used when resending animation by `file_id`), 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the animation caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the animation caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.parse_mode = val.map(Into::into);
        self
    }

    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn caption_entities<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: T,
    ) -> Self {
        self.caption_entities = Some(
            self.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.caption_entities = Some(
            self.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// A JSON-serialized list of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn caption_entities_option<
        TItem: Into<crate::types::MessageEntity>,
        T: IntoIterator<Item = TItem>,
    >(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.caption_entities = val.map(|v| v.into_iter().map(Into::into).collect());
        self
    }

    /// Pass `true` if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media<T: Into<bool>>(mut self, val: T) -> Self {
        self.show_caption_above_media = Some(val.into());
        self
    }

    /// Pass `true` if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.show_caption_above_media = val.map(Into::into);
        self
    }

    /// Pass `true` if the animation needs to be covered with a spoiler animation
    #[must_use]
    pub fn has_spoiler<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_spoiler = Some(val.into());
        self
    }

    /// Pass `true` if the animation needs to be covered with a spoiler animation
    #[must_use]
    pub fn has_spoiler_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_spoiler = val.map(Into::into);
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
impl super::TelegramMethod for SendAnimation {
    type Method = Self;
    type Return = crate::types::Message;

    fn build_request<Client>(mut self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        let mut files = vec![];
        super::prepare_file(&mut files, &mut self.animation);
        if let Some(file) = &mut self.thumbnail {
            super::prepare_file(&mut files, file);
        }
        super::Request::new("sendAnimation", self, Some(files))
    }
}
