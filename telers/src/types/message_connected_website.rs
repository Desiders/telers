use serde::{Deserialize, Serialize};
/// The domain name of the website on which the user has logged in. More about Telegram Login: <https://core.telegram.org/widgets/login>
/// # Notes
/// This object represents a service message from original message field `connected_website`.
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageConnectedWebsite {
    /// Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    pub message_id: i64,
    /// Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i64>,
    /// Information about the direct messages chat topic that contains the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic: Option<crate::types::DirectMessagesTopic>,
    /// Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
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
    /// Bot through which the message was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<Box<crate::types::User>>,
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
    /// The domain name of the website on which the user has logged in. More about Telegram Login: <https://core.telegram.org/widgets/login>
    pub connected_website: Box<str>,
}
impl MessageConnectedWebsite {
    /// Creates a new `MessageConnectedWebsite`.
    ///
    /// # Arguments
    /// * `message_id` - Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// * `date` - Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// * `chat` - Chat the message belongs to
    /// * `connected_website` - The domain name of the website on which the user has logged in. More about Telegram Login: <https://core.telegram.org/widgets/login>
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<crate::types::Chat>, T3: Into<Box<str>>>(
        message_id: T0,
        date: T1,
        chat: T2,
        connected_website: T3,
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
            via_bot: None,
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
            caption: None,
            caption_entities: None,
            show_caption_above_media: None,
            has_media_spoiler: None,
            reply_markup: None,
            connected_website: connected_website.into(),
        }
    }

    /// Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = val.into();
        this
    }

    /// Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[must_use]
    pub fn message_thread_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_thread_id = Some(val.into());
        this
    }

    /// Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[must_use]
    pub fn message_thread_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_thread_id = val.map(Into::into);
        this
    }

    /// Information about the direct messages chat topic that contains the message
    #[must_use]
    pub fn direct_messages_topic<T: Into<crate::types::DirectMessagesTopic>>(self, val: T) -> Self {
        let mut this = self;
        this.direct_messages_topic = Some(val.into());
        this
    }

    /// Information about the direct messages chat topic that contains the message
    #[must_use]
    pub fn direct_messages_topic_option<T: Into<crate::types::DirectMessagesTopic>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.direct_messages_topic = val.map(Into::into);
        this
    }

    /// Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.from = Some(Box::new(val.into()));
        this
    }

    /// Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    #[must_use]
    pub fn from_option<T: Into<crate::types::User>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.from = val.map(|val| Box::new(val.into()));
        this
    }

    /// Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn sender_chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.sender_chat = Some(Box::new(val.into()));
        this
    }

    /// Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn sender_chat_option<T: Into<crate::types::Chat>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.sender_chat = val.map(|val| Box::new(val.into()));
        this
    }

    /// If the sender of the message boosted the chat, the number of boosts added by the user
    #[must_use]
    pub fn sender_boost_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.sender_boost_count = Some(val.into());
        this
    }

    /// If the sender of the message boosted the chat, the number of boosts added by the user
    #[must_use]
    pub fn sender_boost_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.sender_boost_count = val.map(Into::into);
        this
    }

    /// The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[must_use]
    pub fn sender_business_bot<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.sender_business_bot = Some(Box::new(val.into()));
        this
    }

    /// The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[must_use]
    pub fn sender_business_bot_option<T: Into<crate::types::User>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.sender_business_bot = val.map(|val| Box::new(val.into()));
        this
    }

    /// Tag or custom title of the sender of the message; for supergroups only
    #[must_use]
    pub fn sender_tag<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.sender_tag = Some(val.into());
        this
    }

    /// Tag or custom title of the sender of the message; for supergroups only
    #[must_use]
    pub fn sender_tag_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.sender_tag = val.map(Into::into);
        this
    }

    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    #[must_use]
    pub fn date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.date = val.into();
        this
    }

    /// Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = Some(val.into());
        this
    }

    /// Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.business_connection_id = val.map(Into::into);
        this
    }

    /// Chat the message belongs to
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.chat = Box::new(val.into());
        this
    }

    /// Information about the original message for forwarded messages
    #[must_use]
    pub fn forward_origin<T: Into<crate::types::MessageOrigin>>(self, val: T) -> Self {
        let mut this = self;
        this.forward_origin = Some(val.into());
        this
    }

    /// Information about the original message for forwarded messages
    #[must_use]
    pub fn forward_origin_option<T: Into<crate::types::MessageOrigin>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.forward_origin = val.map(Into::into);
        this
    }

    /// `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    #[must_use]
    pub fn is_topic_message<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_topic_message = Some(val.into());
        this
    }

    /// `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    #[must_use]
    pub fn is_topic_message_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_topic_message = val.map(Into::into);
        this
    }

    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[must_use]
    pub fn is_automatic_forward<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_automatic_forward = Some(val.into());
        this
    }

    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[must_use]
    pub fn is_automatic_forward_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_automatic_forward = val.map(Into::into);
        this
    }

    /// For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[must_use]
    pub fn reply_to_message<T: Into<crate::types::Message>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_to_message = Some(Box::new(val.into()));
        this
    }

    /// For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[must_use]
    pub fn reply_to_message_option<T: Into<crate::types::Message>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.reply_to_message = val.map(|val| Box::new(val.into()));
        this
    }

    /// Information about the message that is being replied to, which may come from another chat or forum topic
    #[must_use]
    pub fn external_reply<T: Into<crate::types::ExternalReplyInfo>>(self, val: T) -> Self {
        let mut this = self;
        this.external_reply = Some(Box::new(val.into()));
        this
    }

    /// Information about the message that is being replied to, which may come from another chat or forum topic
    #[must_use]
    pub fn external_reply_option<T: Into<crate::types::ExternalReplyInfo>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.external_reply = val.map(|val| Box::new(val.into()));
        this
    }

    /// For replies that quote part of the original message, the quoted part of the message
    #[must_use]
    pub fn quote<T: Into<crate::types::TextQuote>>(self, val: T) -> Self {
        let mut this = self;
        this.quote = Some(val.into());
        this
    }

    /// For replies that quote part of the original message, the quoted part of the message
    #[must_use]
    pub fn quote_option<T: Into<crate::types::TextQuote>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.quote = val.map(Into::into);
        this
    }

    /// For replies to a story, the original story
    #[must_use]
    pub fn reply_to_story<T: Into<crate::types::Story>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_to_story = Some(val.into());
        this
    }

    /// For replies to a story, the original story
    #[must_use]
    pub fn reply_to_story_option<T: Into<crate::types::Story>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.reply_to_story = val.map(Into::into);
        this
    }

    /// Identifier of the specific checklist task that is being replied to
    #[must_use]
    pub fn reply_to_checklist_task_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_to_checklist_task_id = Some(val.into());
        this
    }

    /// Identifier of the specific checklist task that is being replied to
    #[must_use]
    pub fn reply_to_checklist_task_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.reply_to_checklist_task_id = val.map(Into::into);
        this
    }

    /// Bot through which the message was sent
    #[must_use]
    pub fn via_bot<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.via_bot = Some(Box::new(val.into()));
        this
    }

    /// Bot through which the message was sent
    #[must_use]
    pub fn via_bot_option<T: Into<crate::types::User>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.via_bot = val.map(|val| Box::new(val.into()));
        this
    }

    /// Date the message was last edited in Unix time
    #[must_use]
    pub fn edit_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.edit_date = Some(val.into());
        this
    }

    /// Date the message was last edited in Unix time
    #[must_use]
    pub fn edit_date_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.edit_date = val.map(Into::into);
        this
    }

    /// `true`, if the message can't be forwarded
    #[must_use]
    pub fn has_protected_content<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.has_protected_content = Some(val.into());
        this
    }

    /// `true`, if the message can't be forwarded
    #[must_use]
    pub fn has_protected_content_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.has_protected_content = val.map(Into::into);
        this
    }

    /// `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[must_use]
    pub fn is_from_offline<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_from_offline = Some(val.into());
        this
    }

    /// `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[must_use]
    pub fn is_from_offline_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_from_offline = val.map(Into::into);
        this
    }

    /// `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    #[must_use]
    pub fn is_paid_post<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_paid_post = Some(val.into());
        this
    }

    /// `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    #[must_use]
    pub fn is_paid_post_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_paid_post = val.map(Into::into);
        this
    }

    /// The unique identifier inside this chat of a media message group this message belongs to
    #[must_use]
    pub fn media_group_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.media_group_id = Some(val.into());
        this
    }

    /// The unique identifier inside this chat of a media message group this message belongs to
    #[must_use]
    pub fn media_group_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.media_group_id = val.map(Into::into);
        this
    }

    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[must_use]
    pub fn author_signature<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.author_signature = Some(val.into());
        this
    }

    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[must_use]
    pub fn author_signature_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.author_signature = val.map(Into::into);
        this
    }

    /// The number of Telegram Stars that were paid by the sender of the message to send it
    #[must_use]
    pub fn paid_star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.paid_star_count = Some(val.into());
        this
    }

    /// The number of Telegram Stars that were paid by the sender of the message to send it
    #[must_use]
    pub fn paid_star_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.paid_star_count = val.map(Into::into);
        this
    }

    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.entities = Some(
            this.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.entities = Some(
            this.entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.entities = val.map(Into::into);
        this
    }

    /// Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[must_use]
    pub fn link_preview_options<T: Into<crate::types::LinkPreviewOptions>>(self, val: T) -> Self {
        let mut this = self;
        this.link_preview_options = Some(val.into());
        this
    }

    /// Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[must_use]
    pub fn link_preview_options_option<T: Into<crate::types::LinkPreviewOptions>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.link_preview_options = val.map(Into::into);
        this
    }

    /// Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    #[must_use]
    pub fn suggested_post_info<T: Into<crate::types::SuggestedPostInfo>>(self, val: T) -> Self {
        let mut this = self;
        this.suggested_post_info = Some(val.into());
        this
    }

    /// Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    #[must_use]
    pub fn suggested_post_info_option<T: Into<crate::types::SuggestedPostInfo>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.suggested_post_info = val.map(Into::into);
        this
    }

    /// Unique identifier of the message effect added to the message
    #[must_use]
    pub fn effect_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.effect_id = Some(val.into());
        this
    }

    /// Unique identifier of the message effect added to the message
    #[must_use]
    pub fn effect_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.effect_id = val.map(Into::into);
        this
    }

    /// Caption for the animation, audio, document, paid media, photo, video or voice
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.caption = Some(val.into());
        this
    }

    /// Caption for the animation, audio, document, paid media, photo, video or voice
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.caption = val.map(Into::into);
        this
    }

    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn caption_entities<T: Into<Box<[crate::types::MessageEntity]>>>(self, val: T) -> Self {
        let mut this = self;
        this.caption_entities = Some(
            this.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        this
    }

    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entity<T: Into<crate::types::MessageEntity>>(self, val: T) -> Self {
        let mut this = self;
        this.caption_entities = Some(
            this.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        this
    }

    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.caption_entities = val.map(Into::into);
        this
    }

    /// `true`, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.show_caption_above_media = Some(val.into());
        this
    }

    /// `true`, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.show_caption_above_media = val.map(Into::into);
        this
    }

    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.has_media_spoiler = Some(val.into());
        this
    }

    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.has_media_spoiler = val.map(Into::into);
        this
    }

    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_markup = Some(val.into());
        this
    }

    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_markup = val.map(Into::into);
        this
    }

    /// The domain name of the website on which the user has logged in. More about Telegram Login: <https://core.telegram.org/widgets/login>
    #[must_use]
    pub fn connected_website<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.connected_website = val.into();
        this
    }
}
