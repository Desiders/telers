use super::{
    Animation, Audio, Chat, ChatShared, Contact, Dice, Document, ForumTopicClosed,
    ForumTopicCreated, ForumTopicEdited, ForumTopicReopened, Game, GeneralForumTopicHidden,
    GeneralForumTopicUnhidden, InlineKeyboardMarkup, Invoice, Location,
    MessageAutoDeleteTimerChanged, MessageEntity, PassportData, PhotoSize, Poll,
    ProximityAlertTriggered, Sticker, Story, SuccessfulPayment, Update, User, UserShared, Venue,
    Video, VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted,
    VideoNote, Voice, WebAppData, WriteAccessAllowed,
};

use crate::{enums::ContentType, errors::ConvertUpdateToTypeError};

use serde::Deserialize;

/// This object represents a message.
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
/// # Warnings
/// This structure has so big size, so it's recommended to use it inside [`std::sync::Arc`], [`Box`] and other smart pointers
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct Message {
    /// Unique message identifier inside this chat
    pub message_id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    pub message_thread_id: Option<i64>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Box<Chat>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Box<Chat>>,
    /// For forwarded messages, sender of the original message
    pub forward_from: Option<User>,
    /// For messages forwarded from channels or from anonymous administrators, information about the original sender chat
    pub forward_from_chat: Option<Box<Chat>>,
    /// For messages forwarded from channels, identifier of the original message in the channel
    pub forward_from_message_id: Option<i64>,
    /// For forwarded messages that were originally sent in channels or by an anonymous chat administrator, signature of the message sender if present
    pub forward_signature: Option<Box<str>>,
    /// Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    pub forward_sender_name: Option<Box<str>>,
    /// For forwarded messages, date the original message was sent in Unix time
    pub forward_date: Option<i64>,
    /// `True`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `True`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `True`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<Box<str>>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// For text messages, the actual UTF-8 text of the message
    pub text: Option<Box<str>>,
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    pub entities: Option<Vec<MessageEntity>>,
    /// Message is an animation, information about the animation. For backward compatibility, when this field is set, the *document* field will also be set
    pub animation: Option<Animation>,
    /// Message is an audio file, information about the file
    pub audio: Option<Audio>,
    /// Message is a general file, information about the file
    pub document: Option<Document>,
    /// Message is a photo, available sizes of the photo
    pub photo: Option<Vec<PhotoSize>>,
    /// Message is a sticker, information about the sticker
    pub sticker: Option<Sticker>,
    /// Message is a forwarded story
    pub story: Option<Story>,
    /// Message is a video, information about the video
    pub video: Option<Video>,
    /// Message is a [`video note`](https://telegram.org/blog/video-messages-and-telescope), information about the video message
    pub video_note: Option<VideoNote>,
    /// Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// Caption for the animation, audio, document, photo, video or voice
    pub caption: Option<Box<str>>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Box<[MessageEntity]>>,
    /// `True`, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
    /// Message is a shared contact, information about the contact
    pub contact: Option<Contact>,
    /// Message is a dice with random value
    pub dice: Option<Dice>,
    /// Message is a game, information about the game. [`More about games`](https://core.telegram.org/bots/api#games)
    pub game: Option<Game>,
    /// Message is a native poll, information about the poll
    pub poll: Option<Poll>,
    /// Message is a venue, information about the venue. For backward compatibility, when this field is set, the *location* field will also be set
    pub venue: Option<Venue>,
    /// Message is a shared location, information about the location
    pub location: Option<Location>,
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    pub new_chat_members: Option<Box<[User]>>,
    /// A member was removed from the group, information about them (this member may be the bot itself)
    pub left_chat_member: Option<User>,
    /// A chat title was changed to this value
    pub new_chat_title: Option<Box<str>>,
    /// A chat photo was change to this value
    pub new_chat_photo: Option<Box<[PhotoSize]>>,
    /// Service message: the chat photo was deleted
    pub delete_chat_photo: Option<bool>,
    /// Service message: the group has been created
    pub group_chat_created: Option<bool>,
    /// Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    pub supergroup_chat_created: Option<bool>,
    /// Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    pub channel_chat_created: Option<bool>,
    /// Service message: auto-delete timer settings changed in the chat
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_from_chat_id: Option<i64>,
    /// Specified message was pinned. Note that the Message object in this field will not contain further *reply_to_message* fields even if it is itself a reply.
    pub pinned_message: Option<Box<Message>>,
    /// Message is an invoice for a [`payment`](https://core.telegram.org/bots/api#payments), information about the invoice. [`More about payments`](https://core.telegram.org/bots/api#payments)
    pub invoice: Option<Invoice>,
    /// Message is a service message about a successful payment, information about the payment. [`More about payments`](https://core.telegram.org/bots/api#payments)
    pub successful_payment: Option<SuccessfulPayment>,
    /// Service message: a user was shared with the bot
    pub user_shared: Option<UserShared>,
    /// Service message: a chat was shared with the bot
    pub chat_shared: Option<ChatShared>,
    /// The domain name of the website on which the user has logged in. [`More about Telegram Login`](https://core.telegram.org/widgets/login)
    pub connected_website: Option<Box<str>>,
    /// Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method requestWriteAccess
    pub write_access_allowed: Option<WriteAccessAllowed>,
    /// Telegram Passport data
    pub passport_data: Option<PassportData>,
    /// Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    /// Service message: forum topic created
    pub forum_topic_created: Option<ForumTopicCreated>,
    /// Service message: forum topic edited
    pub forum_topic_edited: Option<ForumTopicEdited>,
    /// Service message: forum topic closed
    pub forum_topic_closed: Option<ForumTopicClosed>,
    /// Service message: forum topic reopened
    pub forum_topic_reopened: Option<ForumTopicReopened>,
    /// Service message: the `General` forum topic hidden
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    /// Service message: the `General` forum topic unhidden
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    /// Service message: video chat scheduled
    pub video_chat_scheduled: Option<VideoChatScheduled>,
    /// Optional*. Service message: video chat started
    pub video_chat_started: Option<VideoChatStarted>,
    /// Service message: video chat ended
    pub video_chat_ended: Option<VideoChatEnded>,
    /// Service message: new participants invited to a video chat
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    /// Service message: data sent by a Web App
    pub web_app_data: Option<WebAppData>,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl Message {
    /// Gets the sender user ID from the message
    #[must_use]
    pub const fn sender_user_id(&self) -> Option<i64> {
        if let Some(from) = &self.from {
            Some(from.id)
        } else {
            None
        }
    }

    /// Gets the sender user ID from the message
    /// # Notes
    /// Alias to `sender_user_id` method
    #[must_use]
    pub const fn user_id(&self) -> Option<i64> {
        self.sender_user_id()
    }

    /// Gets the forward user ID from the message
    #[must_use]
    pub const fn forward_user_id(&self) -> Option<i64> {
        if let Some(forward_from) = &self.forward_from {
            Some(forward_from.id)
        } else {
            None
        }
    }

    /// Gets the sender chat ID from the message
    #[must_use]
    pub const fn sender_chat_id(&self) -> i64 {
        self.chat.id
    }

    /// Gets the sender chat ID from the message
    /// # Notes
    /// Alias to `sender_chat_id` method
    #[must_use]
    pub const fn chat_id(&self) -> i64 {
        self.sender_chat_id()
    }

    /// Gets the forward chat ID from the message
    #[must_use]
    pub const fn forward_chat_id(&self) -> Option<i64> {
        if let Some(forward_from_chat) = &self.forward_from_chat {
            Some(forward_from_chat.id)
        } else {
            None
        }
    }

    /// Gets the via bot ID from the message
    #[must_use]
    pub const fn via_bot_id(&self) -> Option<i64> {
        if let Some(via_bot) = &self.via_bot {
            Some(via_bot.id)
        } else {
            None
        }
    }

    /// Gets the user shared ID from the message
    #[must_use]
    pub const fn user_shared_id(&self) -> Option<i64> {
        if let Some(user_shared) = &self.user_shared {
            Some(user_shared.user_id)
        } else {
            None
        }
    }

    /// Gets the text or caption from the message
    #[must_use]
    pub fn get_text_or_caption(&self) -> Option<&str> {
        if let Some(text) = &self.text {
            Some(text)
        } else if let Some(caption) = &self.caption {
            Some(caption)
        } else {
            None
        }
    }

    /// Gets the content type of the message
    #[must_use]
    pub fn content_type(&self) -> ContentType {
        ContentType::from(self)
    }

    /// Gets file ID from the message
    /// # Notes
    /// If the content type is `ContentType::Photo`, it will return the last photo's file ID
    #[must_use]
    pub fn file_id(&self) -> Option<&str> {
        if let Some(audio) = &self.audio {
            Some(&audio.file_id)
        } else if let Some(document) = &self.document {
            Some(&document.file_id)
        } else if let Some(photo) = &self.photo {
            Some(&photo[photo.len() - 1].file_id)
        } else if let Some(sticker) = &self.sticker {
            Some(&sticker.file_id)
        } else if let Some(video) = &self.video {
            Some(&video.file_id)
        } else if let Some(voice) = &self.voice {
            Some(&voice.file_id)
        } else if let Some(video_note) = &self.video_note {
            Some(&video_note.file_id)
        } else if let Some(animation) = &self.animation {
            Some(&animation.file_id)
        } else {
            None
        }
    }
}

impl TryFrom<Update> for Message {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(message) = update.message {
            Ok(message)
        } else if let Some(message) = update.edited_message {
            Ok(message)
        } else if let Some(message) = update.channel_post {
            Ok(message)
        } else if let Some(message) = update.edited_channel_post {
            Ok(message)
        } else {
            Err(ConvertUpdateToTypeError::new("Message"))
        }
    }
}
