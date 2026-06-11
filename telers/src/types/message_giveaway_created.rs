use serde::{Deserialize, Serialize};
/// Service message: a scheduled giveaway was created
/// # Notes
/// This object represents a service message from original message field `giveaway_created`.
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageGiveawayCreated {
    /// Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent.
    pub message_id: i64,
    /// Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Information about the direct messages chat topic that contains the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic: Option<crate::types::DirectMessagesTopic>,
    /// Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Box<crate::types::User>>,
    /// Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Box<crate::types::Chat>>,
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_boost_count: Option<i64>,
    /// The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<Box<crate::types::User>>,
    /// Tag or custom title of the sender of the message; for supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_tag: Option<Box<str>>,
    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    pub date: i64,
    /// The unique identifier for the guest query. Use this identifier with the method answerGuestQuery to send a response message. If non-empty, the message belongs to the chat where the guest bot was summoned, which may not coincide with other existing bot chats sharing the same identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_query_id: Option<Box<str>>,
    /// Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Chat the message belongs to
    pub chat: Box<crate::types::Chat>,
    /// Information about the original message for forwarded messages
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<crate::types::MessageOrigin>,
    /// `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,
    /// For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<crate::types::Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<Box<crate::types::ExternalReplyInfo>>,
    /// For replies that quote part of the original message, the quoted part of the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<crate::types::TextQuote>,
    /// For replies to a story, the original story
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<crate::types::Story>,
    /// Identifier of the specific checklist task that is being replied to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_checklist_task_id: Option<i64>,
    /// Persistent identifier of the specific poll option that is being replied to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_poll_option_id: Option<Box<str>>,
    /// Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<Box<crate::types::User>>,
    /// For a message sent by a guest bot, this is the user whose original message triggered the bot's response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_bot_caller_user: Option<Box<crate::types::User>>,
    /// For a message sent by a guest bot, this is the chat whose original message triggered the bot's response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_bot_caller_chat: Option<Box<crate::types::Chat>>,
    /// Date the message was last edited in Unix time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    /// `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<bool>,
    /// `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_paid_post: Option<bool>,
    /// The unique identifier inside this chat of a media message group this message belongs to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<Box<str>>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<Box<str>>,
    /// The number of Telegram Stars that were paid by the sender of the message to send it
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_star_count: Option<i64>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<crate::types::LinkPreviewOptions>,
    /// Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_info: Option<crate::types::SuggestedPostInfo>,
    /// Unique identifier of the message effect added to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<Box<str>>,
    /// Message is a rich formatted message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_message: Option<crate::types::RichMessage>,
    /// Caption for the animation, audio, document, paid media, photo, video or voice
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<str>>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// `true`, if the caption must be shown above the message media
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// `true`, if the message media is covered by a spoiler animation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Service message: a scheduled giveaway was created
    pub giveaway_created: crate::types::GiveawayCreated,
}
impl MessageGiveawayCreated {
    /// Creates a new `MessageGiveawayCreated`.
    ///
    /// # Arguments
    /// * `message_id` - Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent.
    /// * `date` - Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// * `chat` - Chat the message belongs to
    /// * `giveaway_created` - Service message: a scheduled giveaway was created
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1: Into<i64>,
        T2: Into<crate::types::Chat>,
        T3: Into<crate::types::GiveawayCreated>,
    >(
        message_id: T0,
        date: T1,
        chat: T2,
        giveaway_created: T3,
    ) -> Self {
        Self {
            message_id: message_id.into(),
            message_thread_id: None,
            direct_messages_topic: None,
            from: None,
            sender_chat: None,
            sender_boost_count: None,
            sender_business_bot: None,
            sender_tag: None,
            date: date.into(),
            guest_query_id: None,
            business_connection_id: None,
            chat: Box::new(chat.into()),
            forward_origin: None,
            is_topic_message: None,
            is_automatic_forward: None,
            reply_to_message: None,
            external_reply: None,
            quote: None,
            reply_to_story: None,
            reply_to_checklist_task_id: None,
            reply_to_poll_option_id: None,
            via_bot: None,
            guest_bot_caller_user: None,
            guest_bot_caller_chat: None,
            edit_date: None,
            has_protected_content: None,
            is_from_offline: None,
            is_paid_post: None,
            media_group_id: None,
            author_signature: None,
            paid_star_count: None,
            entities: None,
            link_preview_options: None,
            suggested_post_info: None,
            effect_id: None,
            rich_message: None,
            caption: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_media_spoiler: None,
            reply_markup: None,
            giveaway_created: giveaway_created.into(),
        }
    }

    /// Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent.
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    /// Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_thread_id = Some(val.into());
        self
    }

    /// Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_thread_id = val.map(Into::into);
        self
    }

    /// Information about the direct messages chat topic that contains the message
    #[must_use]
    pub fn direct_messages_topic<T: Into<crate::types::DirectMessagesTopic>>(
        mut self,
        val: T,
    ) -> Self {
        self.direct_messages_topic = Some(val.into());
        self
    }

    /// Information about the direct messages chat topic that contains the message
    #[must_use]
    pub fn direct_messages_topic_option<T: Into<crate::types::DirectMessagesTopic>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.direct_messages_topic = val.map(Into::into);
        self
    }

    /// Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.from = Some(Box::new(val.into()));
        self
    }

    /// Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn from_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.from = val.map(|val| Box::new(val.into()));
        self
    }

    /// Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn sender_chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.sender_chat = Some(Box::new(val.into()));
        self
    }

    /// Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn sender_chat_option<T: Into<crate::types::Chat>>(mut self, val: Option<T>) -> Self {
        self.sender_chat = val.map(|val| Box::new(val.into()));
        self
    }

    /// If the sender of the message boosted the chat, the number of boosts added by the user
    #[must_use]
    pub fn sender_boost_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.sender_boost_count = Some(val.into());
        self
    }

    /// If the sender of the message boosted the chat, the number of boosts added by the user
    #[must_use]
    pub fn sender_boost_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.sender_boost_count = val.map(Into::into);
        self
    }

    /// The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[must_use]
    pub fn sender_business_bot<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.sender_business_bot = Some(Box::new(val.into()));
        self
    }

    /// The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[must_use]
    pub fn sender_business_bot_option<T: Into<crate::types::User>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.sender_business_bot = val.map(|val| Box::new(val.into()));
        self
    }

    /// Tag or custom title of the sender of the message; for supergroups only
    #[must_use]
    pub fn sender_tag<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.sender_tag = Some(val.into());
        self
    }

    /// Tag or custom title of the sender of the message; for supergroups only
    #[must_use]
    pub fn sender_tag_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.sender_tag = val.map(Into::into);
        self
    }

    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// The unique identifier for the guest query. Use this identifier with the method answerGuestQuery to send a response message. If non-empty, the message belongs to the chat where the guest bot was summoned, which may not coincide with other existing bot chats sharing the same identifier.
    #[must_use]
    pub fn guest_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.guest_query_id = Some(val.into());
        self
    }

    /// The unique identifier for the guest query. Use this identifier with the method answerGuestQuery to send a response message. If non-empty, the message belongs to the chat where the guest bot was summoned, which may not coincide with other existing bot chats sharing the same identifier.
    #[must_use]
    pub fn guest_query_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.guest_query_id = val.map(Into::into);
        self
    }

    /// Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.business_connection_id = val.map(Into::into);
        self
    }

    /// Chat the message belongs to
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Box::new(val.into());
        self
    }

    /// Information about the original message for forwarded messages
    #[must_use]
    pub fn forward_origin<T: Into<crate::types::MessageOrigin>>(mut self, val: T) -> Self {
        self.forward_origin = Some(val.into());
        self
    }

    /// Information about the original message for forwarded messages
    #[must_use]
    pub fn forward_origin_option<T: Into<crate::types::MessageOrigin>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.forward_origin = val.map(Into::into);
        self
    }

    /// `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    #[must_use]
    pub fn is_topic_message<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_topic_message = Some(val.into());
        self
    }

    /// `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    #[must_use]
    pub fn is_topic_message_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_topic_message = val.map(Into::into);
        self
    }

    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[must_use]
    pub fn is_automatic_forward<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_automatic_forward = Some(val.into());
        self
    }

    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[must_use]
    pub fn is_automatic_forward_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_automatic_forward = val.map(Into::into);
        self
    }

    /// For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[must_use]
    pub fn reply_to_message<T: Into<crate::types::Message>>(mut self, val: T) -> Self {
        self.reply_to_message = Some(Box::new(val.into()));
        self
    }

    /// For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[must_use]
    pub fn reply_to_message_option<T: Into<crate::types::Message>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_to_message = val.map(|val| Box::new(val.into()));
        self
    }

    /// Information about the message that is being replied to, which may come from another chat or forum topic
    #[must_use]
    pub fn external_reply<T: Into<crate::types::ExternalReplyInfo>>(mut self, val: T) -> Self {
        self.external_reply = Some(Box::new(val.into()));
        self
    }

    /// Information about the message that is being replied to, which may come from another chat or forum topic
    #[must_use]
    pub fn external_reply_option<T: Into<crate::types::ExternalReplyInfo>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.external_reply = val.map(|val| Box::new(val.into()));
        self
    }

    /// For replies that quote part of the original message, the quoted part of the message
    #[must_use]
    pub fn quote<T: Into<crate::types::TextQuote>>(mut self, val: T) -> Self {
        self.quote = Some(val.into());
        self
    }

    /// For replies that quote part of the original message, the quoted part of the message
    #[must_use]
    pub fn quote_option<T: Into<crate::types::TextQuote>>(mut self, val: Option<T>) -> Self {
        self.quote = val.map(Into::into);
        self
    }

    /// For replies to a story, the original story
    #[must_use]
    pub fn reply_to_story<T: Into<crate::types::Story>>(mut self, val: T) -> Self {
        self.reply_to_story = Some(val.into());
        self
    }

    /// For replies to a story, the original story
    #[must_use]
    pub fn reply_to_story_option<T: Into<crate::types::Story>>(mut self, val: Option<T>) -> Self {
        self.reply_to_story = val.map(Into::into);
        self
    }

    /// Identifier of the specific checklist task that is being replied to
    #[must_use]
    pub fn reply_to_checklist_task_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.reply_to_checklist_task_id = Some(val.into());
        self
    }

    /// Identifier of the specific checklist task that is being replied to
    #[must_use]
    pub fn reply_to_checklist_task_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.reply_to_checklist_task_id = val.map(Into::into);
        self
    }

    /// Persistent identifier of the specific poll option that is being replied to
    #[must_use]
    pub fn reply_to_poll_option_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.reply_to_poll_option_id = Some(val.into());
        self
    }

    /// Persistent identifier of the specific poll option that is being replied to
    #[must_use]
    pub fn reply_to_poll_option_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.reply_to_poll_option_id = val.map(Into::into);
        self
    }

    /// Bot through which the message was sent
    #[must_use]
    pub fn via_bot<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.via_bot = Some(Box::new(val.into()));
        self
    }

    /// Bot through which the message was sent
    #[must_use]
    pub fn via_bot_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.via_bot = val.map(|val| Box::new(val.into()));
        self
    }

    /// For a message sent by a guest bot, this is the user whose original message triggered the bot's response
    #[must_use]
    pub fn guest_bot_caller_user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.guest_bot_caller_user = Some(Box::new(val.into()));
        self
    }

    /// For a message sent by a guest bot, this is the user whose original message triggered the bot's response
    #[must_use]
    pub fn guest_bot_caller_user_option<T: Into<crate::types::User>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.guest_bot_caller_user = val.map(|val| Box::new(val.into()));
        self
    }

    /// For a message sent by a guest bot, this is the chat whose original message triggered the bot's response
    #[must_use]
    pub fn guest_bot_caller_chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.guest_bot_caller_chat = Some(Box::new(val.into()));
        self
    }

    /// For a message sent by a guest bot, this is the chat whose original message triggered the bot's response
    #[must_use]
    pub fn guest_bot_caller_chat_option<T: Into<crate::types::Chat>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.guest_bot_caller_chat = val.map(|val| Box::new(val.into()));
        self
    }

    /// Date the message was last edited in Unix time
    #[must_use]
    pub fn edit_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.edit_date = Some(val.into());
        self
    }

    /// Date the message was last edited in Unix time
    #[must_use]
    pub fn edit_date_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.edit_date = val.map(Into::into);
        self
    }

    /// `true`, if the message can't be forwarded
    #[must_use]
    pub fn has_protected_content<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_protected_content = Some(val.into());
        self
    }

    /// `true`, if the message can't be forwarded
    #[must_use]
    pub fn has_protected_content_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_protected_content = val.map(Into::into);
        self
    }

    /// `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[must_use]
    pub fn is_from_offline<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_from_offline = Some(val.into());
        self
    }

    /// `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[must_use]
    pub fn is_from_offline_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_from_offline = val.map(Into::into);
        self
    }

    /// `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    #[must_use]
    pub fn is_paid_post<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_paid_post = Some(val.into());
        self
    }

    /// `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    #[must_use]
    pub fn is_paid_post_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_paid_post = val.map(Into::into);
        self
    }

    /// The unique identifier inside this chat of a media message group this message belongs to
    #[must_use]
    pub fn media_group_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.media_group_id = Some(val.into());
        self
    }

    /// The unique identifier inside this chat of a media message group this message belongs to
    #[must_use]
    pub fn media_group_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.media_group_id = val.map(Into::into);
        self
    }

    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[must_use]
    pub fn author_signature<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.author_signature = Some(val.into());
        self
    }

    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[must_use]
    pub fn author_signature_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.author_signature = val.map(Into::into);
        self
    }

    /// The number of Telegram Stars that were paid by the sender of the message to send it
    #[must_use]
    pub fn paid_star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.paid_star_count = Some(val.into());
        self
    }

    /// The number of Telegram Stars that were paid by the sender of the message to send it
    #[must_use]
    pub fn paid_star_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.paid_star_count = val.map(Into::into);
        self
    }

    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.entities = Some(
            self.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.entities = val.map(Into::into);
        self
    }

    /// Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[must_use]
    pub fn link_preview_options<T: Into<crate::types::LinkPreviewOptions>>(
        mut self,
        val: T,
    ) -> Self {
        self.link_preview_options = Some(val.into());
        self
    }

    /// Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[must_use]
    pub fn link_preview_options_option<T: Into<crate::types::LinkPreviewOptions>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.link_preview_options = val.map(Into::into);
        self
    }

    /// Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    #[must_use]
    pub fn suggested_post_info<T: Into<crate::types::SuggestedPostInfo>>(mut self, val: T) -> Self {
        self.suggested_post_info = Some(val.into());
        self
    }

    /// Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    #[must_use]
    pub fn suggested_post_info_option<T: Into<crate::types::SuggestedPostInfo>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.suggested_post_info = val.map(Into::into);
        self
    }

    /// Unique identifier of the message effect added to the message
    #[must_use]
    pub fn effect_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.effect_id = Some(val.into());
        self
    }

    /// Unique identifier of the message effect added to the message
    #[must_use]
    pub fn effect_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.effect_id = val.map(Into::into);
        self
    }

    /// Message is a rich formatted message
    #[must_use]
    pub fn rich_message<T: Into<crate::types::RichMessage>>(mut self, val: T) -> Self {
        self.rich_message = Some(val.into());
        self
    }

    /// Message is a rich formatted message
    #[must_use]
    pub fn rich_message_option<T: Into<crate::types::RichMessage>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.rich_message = val.map(Into::into);
        self
    }

    /// Caption for the animation, audio, document, paid media, photo, video or voice
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Caption for the animation, audio, document, paid media, photo, video or voice
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into);
        self
    }

    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn caption_entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.caption_entities = Some(
            self.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
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

    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.caption_entities = val.map(Into::into);
        self
    }

    /// `true`, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media<T: Into<bool>>(mut self, val: T) -> Self {
        self.show_caption_above_media = Some(val.into());
        self
    }

    /// `true`, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.show_caption_above_media = val.map(Into::into);
        self
    }

    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_media_spoiler = Some(val.into());
        self
    }

    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_media_spoiler = val.map(Into::into);
        self
    }

    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }

    /// Service message: a scheduled giveaway was created
    #[must_use]
    pub fn giveaway_created<T: Into<crate::types::GiveawayCreated>>(mut self, val: T) -> Self {
        self.giveaway_created = val.into();
        self
    }
}
