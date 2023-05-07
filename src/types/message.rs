use super::{
    Animation, Audio, Chat, Contact, Dice, Document, ForumTopicClosed, ForumTopicCreated,
    ForumTopicEdited, ForumTopicReopened, Game, GeneralForumTopicHidden, GeneralForumTopicUnhidden,
    InlineKeyboardMarkup, Invoice, Location, MessageAutoDeleteTimerChanged, MessageEntity,
    PassportData, PhotoSize, Poll, ProximityAlertTriggered, Sticker, SuccessfulPayment, Update,
    User, Venue, Video, VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled,
    VideoChatStarted, VideoNote, Voice, WebAppData, WriteAccessAllowed,
};

use crate::{enums::ContentType, error::ConvertUpdateToTypeError};

use serde::Deserialize;

/// This object represents a message.
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
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
    pub forward_signature: Option<String>,
    /// Sender's name for messages forwarded from users who disallow adding a link to their account in forwarded messages
    pub forward_sender_name: Option<String>,
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
    pub media_group_id: Option<String>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<String>,
    /// For text messages, the actual UTF-8 text of the message
    pub text: Option<String>,
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
    /// Message is a video, information about the video
    pub video: Option<Video>,
    /// Message is a [`video note`](https://telegram.org/blog/video-messages-and-telescope), information about the video message
    pub video_note: Option<VideoNote>,
    /// Message is a voice message, information about the file
    pub voice: Option<Voice>,
    /// Caption for the animation, audio, document, photo, video or voice
    pub caption: Option<String>,
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    pub caption_entities: Option<Vec<MessageEntity>>,
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
    pub new_chat_members: Option<Vec<User>>,
    /// A member was removed from the group, information about them (this member may be the bot itself)
    pub left_chat_member: Option<User>,
    /// A chat title was changed to this value
    pub new_chat_title: Option<String>,
    /// A chat photo was change to this value
    pub new_chat_photo: Option<Vec<PhotoSize>>,
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
    /// The domain name of the website on which the user has logged in. [`More about Telegram Login`](https://core.telegram.org/widgets/login)
    pub connected_website: Option<String>,
    /// Service message: the user allowed the bot added to the attachment menu to write messages
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

    #[must_use]
    pub fn content_type(&self) -> ContentType {
        if self.text.is_some() {
            ContentType::Text
        } else if self.audio.is_some() {
            ContentType::Audio
        } else if self.document.is_some() {
            ContentType::Document
        } else if self.photo.is_some() {
            ContentType::Photo
        } else if self.sticker.is_some() {
            ContentType::Sticker
        } else if self.video.is_some() {
            ContentType::Video
        } else if self.voice.is_some() {
            ContentType::Voice
        } else if self.video_note.is_some() {
            ContentType::VideoNote
        } else if self.contact.is_some() {
            ContentType::Contact
        } else if self.location.is_some() {
            ContentType::Location
        } else if self.venue.is_some() {
            ContentType::Venue
        } else if self.poll.is_some() {
            ContentType::Poll
        } else if self.new_chat_members.is_some() {
            ContentType::NewChatMembers
        } else if self.left_chat_member.is_some() {
            ContentType::LeftChatMember
        } else if self.new_chat_title.is_some() {
            ContentType::NewChatTitle
        } else if self.new_chat_photo.is_some() {
            ContentType::NewChatPhoto
        } else if self.delete_chat_photo.is_some() {
            ContentType::DeleteChatPhoto
        } else if self.group_chat_created.is_some() {
            ContentType::GroupChatCreated
        } else if self.supergroup_chat_created.is_some() {
            ContentType::SupergroupChatCreated
        } else if self.channel_chat_created.is_some() {
            ContentType::ChannelChatCreated
        } else if self.migrate_to_chat_id.is_some() {
            ContentType::MigrateToChatId
        } else if self.migrate_from_chat_id.is_some() {
            ContentType::MigrateFromChatId
        } else if self.pinned_message.is_some() {
            ContentType::PinnedMessage
        } else if self.invoice.is_some() {
            ContentType::Invoice
        } else if self.successful_payment.is_some() {
            ContentType::SuccessfulPayment
        } else if self.connected_website.is_some() {
            ContentType::ConnectedWebsite
        } else if self.write_access_allowed.is_some() {
            ContentType::WriteAccessAllowed
        } else if self.passport_data.is_some() {
            ContentType::PassportData
        } else if self.proximity_alert_triggered.is_some() {
            ContentType::ProximityAlertTriggered
        } else if self.forum_topic_created.is_some() {
            ContentType::ForumTopicCreated
        } else if self.forum_topic_edited.is_some() {
            ContentType::ForumTopicEdited
        } else if self.forum_topic_closed.is_some() {
            ContentType::ForumTopicClosed
        } else if self.forum_topic_reopened.is_some() {
            ContentType::ForumTopicReopened
        } else if self.general_forum_topic_hidden.is_some() {
            ContentType::GeneralForumTopicHidden
        } else if self.general_forum_topic_unhidden.is_some() {
            ContentType::GeneralForumTopicUnhidden
        } else if self.video_chat_scheduled.is_some() {
            ContentType::VideoChatScheduled
        } else if self.video_chat_started.is_some() {
            ContentType::VideoChatStarted
        } else if self.video_chat_ended.is_some() {
            ContentType::VideoChatEnded
        } else if self.video_chat_participants_invited.is_some() {
            ContentType::VideoChatParticipantsInvited
        } else if self.web_app_data.is_some() {
            ContentType::WebAppData
        } else {
            ContentType::Unknown
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
            Err(ConvertUpdateToTypeError::new(format!(
                "Update `{update:?}` doesn't contain `Message`",
            )))
        }
    }
}
