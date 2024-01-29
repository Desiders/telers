use super::{
    Chat, ExternalReplyInfo, InlineKeyboardMarkup, LinkPreviewOptions, MaybeInaccessibleMessage,
    MessageEntity, MessageOrigin, PhotoSize, TextQuote, Update, UpdateKind, User,
};

use crate::{errors::ConvertToTypeError, extractors::FromEvent, types};

use serde::Deserialize;

/// This object represents a message.
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
#[serde(untagged)]
pub enum Message {
    Text(Text),
    Animation(Animation),
    Audio(Audio),
    Document(Document),
    Photo(Photo),
    Sticker(Sticker),
    Story(Story),
    Video(Video),
    VideoNote(VideoNote),
    Voice(Voice),
    Contact(Contact),
    Dice(Dice),
    Game(Game),
    Poll(Poll),
    Venue(Venue),
    Location(Location),
    NewChatMembers(NewChatMembers),
    LeftChatMember(LeftChatMember),
    NewChatTitle(NewChatTitle),
    NewChatPhoto(NewChatPhoto),
    DeleteChatPhoto(DeleteChatPhoto),
    GroupChatCreated(GroupChatCreated),
    SupergroupChatCreated(SupergroupChatCreated),
    ChannelChatCreated(ChannelChatCreated),
    MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged),
    MigrateToChat(MigrateToChat),
    MigrateFromChat(MigrateFromChat),
    Pinned(Pinned),
    Invoice(Invoice),
    SuccessfulPayment(SuccessfulPayment),
    UsersShared(UsersShared),
    ChatShared(ChatShared),
    ConnectedWebsite(ConnectedWebsite),
    WriteAccessAllowed(WriteAccessAllowed),
    PassportData(PassportData),
    ProximityAlertTriggered(ProximityAlertTriggered),
    ForumTopicCreated(ForumTopicCreated),
    ForumTopicEdited(ForumTopicEdited),
    ForumTopicClosed(ForumTopicClosed),
    ForumTopicReopened(ForumTopicReopened),
    GeneralForumTopicHidden(GeneralForumTopicHidden),
    GeneralForumTopicUnhidden(GeneralForumTopicUnhidden),
    GiveawayCreated(GiveawayCreated),
    Giveaway(Giveaway),
    GiveawayWinners(GiveawayWinners),
    GiveawayCompleted(GiveawayCompleted),
    VideoChatScheduled(VideoChatScheduled),
    VideoChatStarted(VideoChatStarted),
    VideoChatEnded(VideoChatEnded),
    VideoChatParticipantsInvited(VideoChatParticipantsInvited),
    WebAppData(WebAppData),
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Animation {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// For replies that quote part of the original message, the quoted part of the message
    pub quote: Option<TextQuote>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the animation
    pub animation: types::Animation,
    /// Caption
    pub caption: Option<Box<str>>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(rename = "caption_entities")]
    pub entities: Option<Box<[MessageEntity]>>,
    /// `true`, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Audio {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// For replies that quote part of the original message, the quoted part of the message
    pub quote: Option<TextQuote>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<Box<str>>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the file
    pub audio: types::Audio,
    /// Caption
    pub caption: Option<Box<str>>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(rename = "caption_entities")]
    pub entities: Option<Box<[MessageEntity]>>,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Contact {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the contact
    pub contact: types::Contact,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Dice {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Message is a dice with random value
    pub dice: types::Dice,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Document {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// For replies that quote part of the original message, the quoted part of the message
    pub quote: Option<TextQuote>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<Box<str>>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the file
    pub document: types::Document,
    /// Caption
    pub caption: Option<Box<str>>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(rename = "caption_entities")]
    pub entities: Option<Box<[MessageEntity]>>,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Game {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the game. [`More about games`](https://core.telegram.org/bots/api#games)
    pub game: types::Game,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Poll {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the poll
    pub poll: types::Poll,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Venue {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the venue
    pub venue: types::Venue,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Location {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the location
    pub location: types::Location,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Photo {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// For replies that quote part of the original message, the quoted part of the message
    pub quote: Option<TextQuote>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<Box<str>>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Available sizes of the photo
    pub photo: Box<[PhotoSize]>,
    /// Caption
    pub caption: Option<Box<str>>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(rename = "caption_entities")]
    pub entities: Option<Box<[MessageEntity]>>,
    /// `true`, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl Photo {
    /// Returns the largest photo in the message by `width` + `height`
    #[must_use]
    pub fn largest_photo(&self) -> Option<&PhotoSize> {
        self.photo.iter().max_by_key(|x| x.width + x.height)
    }

    /// Returns the smallest photo in the message by `width` + `height`
    #[must_use]
    pub fn smallest_photo(&self) -> Option<&PhotoSize> {
        self.photo.iter().min_by_key(|x| x.width + x.height)
    }

    /// Returns the largest photo in the message by `file_size`
    #[must_use]
    pub fn largest_photo_by_file_size(&self) -> Option<&PhotoSize> {
        self.photo.iter().max_by_key(|x| x.file_size)
    }

    /// Returns the smallest photo in the message by `file_size`
    #[must_use]
    pub fn smallest_photo_by_file_size(&self) -> Option<&PhotoSize> {
        self.photo.iter().min_by_key(|x| x.file_size)
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Story {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Forwarded story
    pub story: types::Story,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Sticker {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the sticker
    pub sticker: types::Sticker,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Text {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// For replies that quote part of the original message, the quoted part of the message
    pub quote: Option<TextQuote>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// The actual UTF-8 text of the message
    pub text: Box<str>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the text
    pub entities: Option<Box<[MessageEntity]>>,
    /// Options used for link preview generation for the message, if it is a text message an
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Video {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// For replies that quote part of the original message, the quoted part of the message
    pub quote: Option<TextQuote>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Date the message was last edited in Unix time
    pub edit_date: Option<i64>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// The unique identifier of a media message group this message belongs to
    pub media_group_id: Option<Box<str>>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the video
    pub video: types::Video,
    /// Caption
    pub caption: Option<Box<str>>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(rename = "caption_entities")]
    pub entities: Option<Box<[MessageEntity]>>,
    /// `true`, if the message media is covered by a spoiler animation
    pub has_media_spoiler: Option<bool>,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct VideoNote {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the video message
    pub video_note: types::VideoNote,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Voice {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    pub is_automatic_forward: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// For replies that quote part of the original message, the quoted part of the message
    pub quote: Option<TextQuote>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Information about the file
    pub voice: types::Voice,
    /// Caption
    pub caption: Option<Box<str>>,
    /// Special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[serde(rename = "caption_entities")]
    pub entities: Option<Box<[MessageEntity]>>,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct MigrateToChat {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(rename = "migrate_to_chat_id")]
    pub to_chat_id: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct MigrateFromChat {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[serde(rename = "migrate_from_chat_id")]
    pub from_chat_id: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct NewChatMembers {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[serde(rename = "new_chat_members")]
    pub members: Box<[User]>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct LeftChatMember {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// A member was removed from the group, information about them (this member may be the bot itself).
    #[serde(rename = "left_chat_member")]
    pub member: User,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct NewChatTitle {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// A chat title was changed to this value
    #[serde(rename = "new_chat_title")]
    pub title: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct NewChatPhoto {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// A chat photo was change to this value
    #[serde(rename = "new_chat_photo")]
    pub photo: Box<[PhotoSize]>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct DeleteChatPhoto {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Service message: the chat photo was deleted
    #[serde(rename = "delete_chat_photo")]
    pub photo: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct GroupChatCreated {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: the group has been created
    #[serde(rename = "group_chat_created")]
    pub created: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct SupergroupChatCreated {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in reply_to_message if someone replies to a very first message in a directly created supergroup.
    #[serde(rename = "supergroup_chat_created")]
    pub created: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct ChannelChatCreated {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in reply_to_message if someone replies to a very first message in a channel.
    #[serde(rename = "channel_chat_created")]
    pub created: bool,
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct MessageAutoDeleteTimerChanged {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Service message: auto-delete timer settings changed in the chat
    #[serde(rename = "message_auto_delete_timer_changed")]
    pub timer: types::MessageAutoDeleteTimerChanged,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Pinned {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Specified message was pinned. Note that the Message object in this field will not contain further *reply_to_message* fields even if it is itself a reply.
    #[serde(rename = "pinned_message")]
    pub message: Box<MaybeInaccessibleMessage>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Invoice {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// `true`, if the message can't be forwarded
    pub has_protected_content: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Message is an invoice for a [`payment`](https://core.telegram.org/bots/api#payments), information about the invoice. [`More about payments`](https://core.telegram.org/bots/api#payments)
    pub invoice: types::Invoice,
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary `url` buttons.
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct SuccessfulPayment {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Message is a service message about a successful payment, information about the payment. [`More about payments`](https://core.telegram.org/bots/api#payments)
    #[serde(rename = "successful_payment")]
    pub payment: types::SuccessfulPayment,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct UsersShared {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Service message: users were shared with the bot
    #[serde(rename = "users_shared")]
    pub shared: types::UsersShared,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct ChatShared {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Service message: a chat was shared with the bot
    #[serde(rename = "chat_shared")]
    pub shared: types::ChatShared,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct ConnectedWebsite {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// The domain name of the website on which the user has logged in. [`More about Telegram Login`](https://core.telegram.org/widgets/login)
    #[serde(rename = "connected_website")]
    pub website: Box<str>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct WriteAccessAllowed {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method requestWriteAccess
    #[serde(rename = "write_access_allowed")]
    pub allowed: types::WriteAccessAllowed,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct PassportData {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Telegram Passport data
    #[serde(rename = "passport_data")]
    pub data: types::PassportData,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct ProximityAlertTriggered {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[serde(rename = "proximity_alert_triggered")]
    pub triggered: types::ProximityAlertTriggered,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct ForumTopicCreated {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Service message: forum topic created
    #[serde(rename = "forum_topic_created")]
    pub created: types::ForumTopicCreated,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct ForumTopicEdited {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Service message: forum topic edited
    #[serde(rename = "forum_topic_edited")]
    pub edited: types::ForumTopicEdited,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct ForumTopicClosed {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Service message: forum topic closed
    #[serde(rename = "forum_topic_closed")]
    pub closed: types::ForumTopicClosed,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct ForumTopicReopened {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Service message: forum topic reopened
    #[serde(rename = "forum_topic_reopened")]
    pub reopened: types::ForumTopicReopened,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct GeneralForumTopicHidden {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Service message: the `General` forum topic hidden
    #[serde(rename = "general_forum_topic_hidden")]
    pub hidden: types::GeneralForumTopicHidden,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct GeneralForumTopicUnhidden {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Box<Message>>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Service message: the `General` forum topic unhidden
    #[serde(rename = "general_forum_topic_unhidden")]
    pub unhidden: types::GeneralForumTopicUnhidden,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct GiveawayCreated {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: a scheduled giveaway was created
    #[serde(rename = "giveaway_created")]
    pub created: types::GiveawayCreated,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct Giveaway {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// The message is a scheduled giveaway message
    pub giveaway: types::Giveaway,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct GiveawayWinners {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    pub external_reply: Option<ExternalReplyInfo>,
    /// A giveaway with public winners was completed
    #[serde(rename = "giveaway_winners")]
    pub winners: types::GiveawayWinners,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct GiveawayCompleted {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: a giveaway without public winners was completed
    #[serde(rename = "giveaway_completed")]
    pub completed: types::GiveawayCompleted,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct VideoChatScheduled {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: video chat scheduled
    #[serde(rename = "video_chat_scheduled")]
    pub scheduled: types::VideoChatScheduled,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct VideoChatStarted {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: video chat started
    #[serde(rename = "video_chat_started")]
    pub started: types::VideoChatStarted,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct VideoChatEnded {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: video chat ended
    #[serde(rename = "video_chat_ended")]
    pub ended: types::VideoChatEnded,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct VideoChatParticipantsInvited {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: new participants invited to a video chat
    #[serde(rename = "video_chat_participants_invited")]
    pub invited: types::VideoChatParticipantsInvited,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct WebAppData {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Unique identifier of a message thread to which the message belongs; for supergroups only
    #[serde(rename = "message_thread_id")]
    pub thread_id: Option<i64>,
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// Bot through which the message was sent
    pub via_bot: Option<User>,
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    pub author_signature: Option<Box<str>>,
    /// Service message: data sent by a Web App
    #[serde(rename = "web_app_data")]
    pub data: types::WebAppData,
}

impl Message {
    #[must_use]
    pub const fn id(&self) -> i64 {
        match self {
            Message::Text(Text { id, .. })
            | Message::Animation(Animation { id, .. })
            | Message::Audio(Audio { id, .. })
            | Message::Document(Document { id, .. })
            | Message::Photo(Photo { id, .. })
            | Message::Sticker(Sticker { id, .. })
            | Message::Story(Story { id, .. })
            | Message::Video(Video { id, .. })
            | Message::VideoNote(VideoNote { id, .. })
            | Message::Voice(Voice { id, .. })
            | Message::Contact(Contact { id, .. })
            | Message::Dice(Dice { id, .. })
            | Message::Game(Game { id, .. })
            | Message::Poll(Poll { id, .. })
            | Message::Venue(Venue { id, .. })
            | Message::Location(Location { id, .. })
            | Message::NewChatMembers(NewChatMembers { id, .. })
            | Message::LeftChatMember(LeftChatMember { id, .. })
            | Message::NewChatTitle(NewChatTitle { id, .. })
            | Message::NewChatPhoto(NewChatPhoto { id, .. })
            | Message::DeleteChatPhoto(DeleteChatPhoto { id, .. })
            | Message::GroupChatCreated(GroupChatCreated { id, .. })
            | Message::SupergroupChatCreated(SupergroupChatCreated { id, .. })
            | Message::ChannelChatCreated(ChannelChatCreated { id, .. })
            | Message::MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged {
                id, ..
            })
            | Message::MigrateToChat(MigrateToChat { id, .. })
            | Message::MigrateFromChat(MigrateFromChat { id, .. })
            | Message::Pinned(Pinned { id, .. })
            | Message::Invoice(Invoice { id, .. })
            | Message::SuccessfulPayment(SuccessfulPayment { id, .. })
            | Message::UsersShared(UsersShared { id, .. })
            | Message::ChatShared(ChatShared { id, .. })
            | Message::ConnectedWebsite(ConnectedWebsite { id, .. })
            | Message::WriteAccessAllowed(WriteAccessAllowed { id, .. })
            | Message::PassportData(PassportData { id, .. })
            | Message::ProximityAlertTriggered(ProximityAlertTriggered { id, .. })
            | Message::ForumTopicCreated(ForumTopicCreated { id, .. })
            | Message::ForumTopicEdited(ForumTopicEdited { id, .. })
            | Message::ForumTopicClosed(ForumTopicClosed { id, .. })
            | Message::ForumTopicReopened(ForumTopicReopened { id, .. })
            | Message::GeneralForumTopicHidden(GeneralForumTopicHidden { id, .. })
            | Message::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden { id, .. })
            | Message::VideoChatScheduled(VideoChatScheduled { id, .. })
            | Message::VideoChatStarted(VideoChatStarted { id, .. })
            | Message::VideoChatEnded(VideoChatEnded { id, .. })
            | Message::VideoChatParticipantsInvited(VideoChatParticipantsInvited { id, .. })
            | Message::WebAppData(WebAppData { id, .. })
            | Message::GiveawayCreated(GiveawayCreated { id, .. })
            | Message::Giveaway(Giveaway { id, .. })
            | Message::GiveawayWinners(GiveawayWinners { id, .. })
            | Message::GiveawayCompleted(GiveawayCompleted { id, .. }) => *id,
        }
    }

    #[must_use]
    pub const fn thread_id(&self) -> Option<i64> {
        match self {
            Message::Text(Text { thread_id, .. })
            | Message::Animation(Animation { thread_id, .. })
            | Message::Audio(Audio { thread_id, .. })
            | Message::Document(Document { thread_id, .. })
            | Message::Photo(Photo { thread_id, .. })
            | Message::Sticker(Sticker { thread_id, .. })
            | Message::Story(Story { thread_id, .. })
            | Message::Video(Video { thread_id, .. })
            | Message::VideoNote(VideoNote { thread_id, .. })
            | Message::Voice(Voice { thread_id, .. })
            | Message::Contact(Contact { thread_id, .. })
            | Message::Dice(Dice { thread_id, .. })
            | Message::Game(Game { thread_id, .. })
            | Message::Poll(Poll { thread_id, .. })
            | Message::Venue(Venue { thread_id, .. })
            | Message::Location(Location { thread_id, .. })
            | Message::NewChatMembers(NewChatMembers { thread_id, .. })
            | Message::LeftChatMember(LeftChatMember { thread_id, .. })
            | Message::NewChatTitle(NewChatTitle { thread_id, .. })
            | Message::NewChatPhoto(NewChatPhoto { thread_id, .. })
            | Message::DeleteChatPhoto(DeleteChatPhoto { thread_id, .. })
            | Message::GroupChatCreated(GroupChatCreated { thread_id, .. })
            | Message::SupergroupChatCreated(SupergroupChatCreated { thread_id, .. })
            | Message::ChannelChatCreated(ChannelChatCreated { thread_id, .. })
            | Message::MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged {
                thread_id,
                ..
            })
            | Message::MigrateToChat(MigrateToChat { thread_id, .. })
            | Message::MigrateFromChat(MigrateFromChat { thread_id, .. })
            | Message::Pinned(Pinned { thread_id, .. })
            | Message::Invoice(Invoice { thread_id, .. })
            | Message::SuccessfulPayment(SuccessfulPayment { thread_id, .. })
            | Message::UsersShared(UsersShared { thread_id, .. })
            | Message::ChatShared(ChatShared { thread_id, .. })
            | Message::ConnectedWebsite(ConnectedWebsite { thread_id, .. })
            | Message::WriteAccessAllowed(WriteAccessAllowed { thread_id, .. })
            | Message::PassportData(PassportData { thread_id, .. })
            | Message::ProximityAlertTriggered(ProximityAlertTriggered { thread_id, .. })
            | Message::ForumTopicCreated(ForumTopicCreated { thread_id, .. })
            | Message::ForumTopicEdited(ForumTopicEdited { thread_id, .. })
            | Message::ForumTopicClosed(ForumTopicClosed { thread_id, .. })
            | Message::ForumTopicReopened(ForumTopicReopened { thread_id, .. })
            | Message::GeneralForumTopicHidden(GeneralForumTopicHidden { thread_id, .. })
            | Message::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden { thread_id, .. })
            | Message::VideoChatScheduled(VideoChatScheduled { thread_id, .. })
            | Message::VideoChatStarted(VideoChatStarted { thread_id, .. })
            | Message::VideoChatEnded(VideoChatEnded { thread_id, .. })
            | Message::VideoChatParticipantsInvited(VideoChatParticipantsInvited {
                thread_id,
                ..
            })
            | Message::WebAppData(WebAppData { thread_id, .. })
            | Message::GiveawayCreated(GiveawayCreated { thread_id, .. })
            | Message::Giveaway(Giveaway { thread_id, .. })
            | Message::GiveawayWinners(GiveawayWinners { thread_id, .. })
            | Message::GiveawayCompleted(GiveawayCompleted { thread_id, .. }) => *thread_id,
        }
    }

    #[must_use]
    pub const fn date(&self) -> i64 {
        match self {
            Message::Text(Text { date, .. })
            | Message::Animation(Animation { date, .. })
            | Message::Audio(Audio { date, .. })
            | Message::Document(Document { date, .. })
            | Message::Photo(Photo { date, .. })
            | Message::Sticker(Sticker { date, .. })
            | Message::Story(Story { date, .. })
            | Message::Video(Video { date, .. })
            | Message::VideoNote(VideoNote { date, .. })
            | Message::Voice(Voice { date, .. })
            | Message::Contact(Contact { date, .. })
            | Message::Dice(Dice { date, .. })
            | Message::Game(Game { date, .. })
            | Message::Poll(Poll { date, .. })
            | Message::Venue(Venue { date, .. })
            | Message::Location(Location { date, .. })
            | Message::NewChatMembers(NewChatMembers { date, .. })
            | Message::LeftChatMember(LeftChatMember { date, .. })
            | Message::NewChatTitle(NewChatTitle { date, .. })
            | Message::NewChatPhoto(NewChatPhoto { date, .. })
            | Message::DeleteChatPhoto(DeleteChatPhoto { date, .. })
            | Message::GroupChatCreated(GroupChatCreated { date, .. })
            | Message::SupergroupChatCreated(SupergroupChatCreated { date, .. })
            | Message::ChannelChatCreated(ChannelChatCreated { date, .. })
            | Message::MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged {
                date, ..
            })
            | Message::MigrateToChat(MigrateToChat { date, .. })
            | Message::MigrateFromChat(MigrateFromChat { date, .. })
            | Message::Pinned(Pinned { date, .. })
            | Message::Invoice(Invoice { date, .. })
            | Message::SuccessfulPayment(SuccessfulPayment { date, .. })
            | Message::UsersShared(UsersShared { date, .. })
            | Message::ChatShared(ChatShared { date, .. })
            | Message::ConnectedWebsite(ConnectedWebsite { date, .. })
            | Message::WriteAccessAllowed(WriteAccessAllowed { date, .. })
            | Message::PassportData(PassportData { date, .. })
            | Message::ProximityAlertTriggered(ProximityAlertTriggered { date, .. })
            | Message::ForumTopicCreated(ForumTopicCreated { date, .. })
            | Message::ForumTopicEdited(ForumTopicEdited { date, .. })
            | Message::ForumTopicClosed(ForumTopicClosed { date, .. })
            | Message::ForumTopicReopened(ForumTopicReopened { date, .. })
            | Message::GeneralForumTopicHidden(GeneralForumTopicHidden { date, .. })
            | Message::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden { date, .. })
            | Message::VideoChatScheduled(VideoChatScheduled { date, .. })
            | Message::VideoChatStarted(VideoChatStarted { date, .. })
            | Message::VideoChatEnded(VideoChatEnded { date, .. })
            | Message::VideoChatParticipantsInvited(VideoChatParticipantsInvited {
                date, ..
            })
            | Message::WebAppData(WebAppData { date, .. })
            | Message::GiveawayCreated(GiveawayCreated { date, .. })
            | Message::Giveaway(Giveaway { date, .. })
            | Message::GiveawayWinners(GiveawayWinners { date, .. })
            | Message::GiveawayCompleted(GiveawayCompleted { date, .. }) => *date,
        }
    }

    #[must_use]
    pub const fn chat(&self) -> &Chat {
        match self {
            Message::Text(Text { chat, .. })
            | Message::Animation(Animation { chat, .. })
            | Message::Audio(Audio { chat, .. })
            | Message::Document(Document { chat, .. })
            | Message::Photo(Photo { chat, .. })
            | Message::Sticker(Sticker { chat, .. })
            | Message::Story(Story { chat, .. })
            | Message::Video(Video { chat, .. })
            | Message::VideoNote(VideoNote { chat, .. })
            | Message::Voice(Voice { chat, .. })
            | Message::Contact(Contact { chat, .. })
            | Message::Dice(Dice { chat, .. })
            | Message::Game(Game { chat, .. })
            | Message::Poll(Poll { chat, .. })
            | Message::Venue(Venue { chat, .. })
            | Message::Location(Location { chat, .. })
            | Message::NewChatMembers(NewChatMembers { chat, .. })
            | Message::LeftChatMember(LeftChatMember { chat, .. })
            | Message::NewChatTitle(NewChatTitle { chat, .. })
            | Message::NewChatPhoto(NewChatPhoto { chat, .. })
            | Message::DeleteChatPhoto(DeleteChatPhoto { chat, .. })
            | Message::GroupChatCreated(GroupChatCreated { chat, .. })
            | Message::SupergroupChatCreated(SupergroupChatCreated { chat, .. })
            | Message::ChannelChatCreated(ChannelChatCreated { chat, .. })
            | Message::MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged {
                chat, ..
            })
            | Message::MigrateToChat(MigrateToChat { chat, .. })
            | Message::MigrateFromChat(MigrateFromChat { chat, .. })
            | Message::Pinned(Pinned { chat, .. })
            | Message::Invoice(Invoice { chat, .. })
            | Message::SuccessfulPayment(SuccessfulPayment { chat, .. })
            | Message::UsersShared(UsersShared { chat, .. })
            | Message::ChatShared(ChatShared { chat, .. })
            | Message::ConnectedWebsite(ConnectedWebsite { chat, .. })
            | Message::WriteAccessAllowed(WriteAccessAllowed { chat, .. })
            | Message::PassportData(PassportData { chat, .. })
            | Message::ProximityAlertTriggered(ProximityAlertTriggered { chat, .. })
            | Message::ForumTopicCreated(ForumTopicCreated { chat, .. })
            | Message::ForumTopicEdited(ForumTopicEdited { chat, .. })
            | Message::ForumTopicClosed(ForumTopicClosed { chat, .. })
            | Message::ForumTopicReopened(ForumTopicReopened { chat, .. })
            | Message::GeneralForumTopicHidden(GeneralForumTopicHidden { chat, .. })
            | Message::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden { chat, .. })
            | Message::VideoChatScheduled(VideoChatScheduled { chat, .. })
            | Message::VideoChatStarted(VideoChatStarted { chat, .. })
            | Message::VideoChatEnded(VideoChatEnded { chat, .. })
            | Message::VideoChatParticipantsInvited(VideoChatParticipantsInvited {
                chat, ..
            })
            | Message::WebAppData(WebAppData { chat, .. })
            | Message::GiveawayCreated(GiveawayCreated { chat, .. })
            | Message::Giveaway(Giveaway { chat, .. })
            | Message::GiveawayWinners(GiveawayWinners { chat, .. })
            | Message::GiveawayCompleted(GiveawayCompleted { chat, .. }) => chat,
        }
    }

    #[must_use]
    pub const fn via_bot(&self) -> Option<&User> {
        match self {
            Message::Text(Text { via_bot, .. })
            | Message::Animation(Animation { via_bot, .. })
            | Message::Audio(Audio { via_bot, .. })
            | Message::Document(Document { via_bot, .. })
            | Message::Photo(Photo { via_bot, .. })
            | Message::Sticker(Sticker { via_bot, .. })
            | Message::Video(Video { via_bot, .. })
            | Message::Voice(Voice { via_bot, .. })
            | Message::Contact(Contact { via_bot, .. })
            | Message::Game(Game { via_bot, .. })
            | Message::Venue(Venue { via_bot, .. })
            | Message::Location(Location { via_bot, .. })
            | Message::LeftChatMember(LeftChatMember { via_bot, .. })
            | Message::NewChatTitle(NewChatTitle { via_bot, .. })
            | Message::NewChatPhoto(NewChatPhoto { via_bot, .. })
            | Message::DeleteChatPhoto(DeleteChatPhoto { via_bot, .. })
            | Message::Pinned(Pinned { via_bot, .. })
            | Message::Invoice(Invoice { via_bot, .. })
            | Message::SuccessfulPayment(SuccessfulPayment { via_bot, .. })
            | Message::ConnectedWebsite(ConnectedWebsite { via_bot, .. })
            | Message::WriteAccessAllowed(WriteAccessAllowed { via_bot, .. })
            | Message::ForumTopicCreated(ForumTopicCreated { via_bot, .. })
            | Message::ForumTopicEdited(ForumTopicEdited { via_bot, .. })
            | Message::ForumTopicClosed(ForumTopicClosed { via_bot, .. })
            | Message::ForumTopicReopened(ForumTopicReopened { via_bot, .. })
            | Message::GeneralForumTopicHidden(GeneralForumTopicHidden { via_bot, .. })
            | Message::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden { via_bot, .. })
            | Message::WebAppData(WebAppData { via_bot, .. }) => via_bot,
            _ => &None,
        }
        .as_ref()
    }

    #[must_use]
    pub const fn text(&self) -> Option<&str> {
        match self {
            Message::Text(Text { text, .. }) => Some(text),
            _ => None,
        }
    }

    #[must_use]
    pub const fn caption(&self) -> Option<&str> {
        match self {
            Message::Animation(Animation { caption, .. })
            | Message::Audio(Audio { caption, .. })
            | Message::Document(Document { caption, .. })
            | Message::Video(Video { caption, .. })
            | Message::Voice(Voice { caption, .. })
            | Message::Photo(Photo { caption, .. }) => match caption {
                Some(caption) => Some(caption),
                None => None,
            },
            _ => None,
        }
    }

    #[must_use]
    pub const fn text_or_caption(&self) -> Option<&str> {
        if let Some(text) = self.text() {
            Some(text)
        } else {
            self.caption()
        }
    }

    #[must_use]
    pub const fn from(&self) -> Option<&User> {
        match self {
            Message::Text(Text { from, .. })
            | Message::Animation(Animation { from, .. })
            | Message::Audio(Audio { from, .. })
            | Message::Document(Document { from, .. })
            | Message::Photo(Photo { from, .. })
            | Message::Sticker(Sticker { from, .. })
            | Message::Story(Story { from, .. })
            | Message::Video(Video { from, .. })
            | Message::VideoNote(VideoNote { from, .. })
            | Message::Voice(Voice { from, .. })
            | Message::Contact(Contact { from, .. })
            | Message::Dice(Dice { from, .. })
            | Message::Game(Game { from, .. })
            | Message::Poll(Poll { from, .. })
            | Message::Venue(Venue { from, .. })
            | Message::Location(Location { from, .. })
            | Message::NewChatMembers(NewChatMembers { from, .. })
            | Message::LeftChatMember(LeftChatMember { from, .. })
            | Message::NewChatTitle(NewChatTitle { from, .. })
            | Message::NewChatPhoto(NewChatPhoto { from, .. })
            | Message::DeleteChatPhoto(DeleteChatPhoto { from, .. })
            | Message::GroupChatCreated(GroupChatCreated { from, .. })
            | Message::SupergroupChatCreated(SupergroupChatCreated { from, .. })
            | Message::ChannelChatCreated(ChannelChatCreated { from, .. })
            | Message::MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged {
                from, ..
            })
            | Message::MigrateToChat(MigrateToChat { from, .. })
            | Message::MigrateFromChat(MigrateFromChat { from, .. })
            | Message::Pinned(Pinned { from, .. })
            | Message::Invoice(Invoice { from, .. })
            | Message::SuccessfulPayment(SuccessfulPayment { from, .. })
            | Message::UsersShared(UsersShared { from, .. })
            | Message::ChatShared(ChatShared { from, .. })
            | Message::ConnectedWebsite(ConnectedWebsite { from, .. })
            | Message::WriteAccessAllowed(WriteAccessAllowed { from, .. })
            | Message::PassportData(PassportData { from, .. })
            | Message::ProximityAlertTriggered(ProximityAlertTriggered { from, .. })
            | Message::ForumTopicCreated(ForumTopicCreated { from, .. })
            | Message::ForumTopicEdited(ForumTopicEdited { from, .. })
            | Message::ForumTopicClosed(ForumTopicClosed { from, .. })
            | Message::ForumTopicReopened(ForumTopicReopened { from, .. })
            | Message::GeneralForumTopicHidden(GeneralForumTopicHidden { from, .. })
            | Message::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden { from, .. })
            | Message::VideoChatScheduled(VideoChatScheduled { from, .. })
            | Message::VideoChatStarted(VideoChatStarted { from, .. })
            | Message::VideoChatEnded(VideoChatEnded { from, .. })
            | Message::VideoChatParticipantsInvited(VideoChatParticipantsInvited {
                from, ..
            })
            | Message::WebAppData(WebAppData { from, .. })
            | Message::GiveawayCreated(GiveawayCreated { from, .. })
            | Message::Giveaway(Giveaway { from, .. })
            | Message::GiveawayWinners(GiveawayWinners { from, .. })
            | Message::GiveawayCompleted(GiveawayCompleted { from, .. }) => from,
        }
        .as_ref()
    }

    #[must_use]
    pub const fn from_id(&self) -> Option<i64> {
        match self.from() {
            Some(user) => Some(user.id),
            None => None,
        }
    }

    #[must_use]
    pub const fn sender_chat(&self) -> Option<&Chat> {
        match self {
            Message::Text(Text { sender_chat, .. })
            | Message::Animation(Animation { sender_chat, .. })
            | Message::Audio(Audio { sender_chat, .. })
            | Message::Document(Document { sender_chat, .. })
            | Message::Photo(Photo { sender_chat, .. })
            | Message::Sticker(Sticker { sender_chat, .. })
            | Message::Story(Story { sender_chat, .. })
            | Message::Video(Video { sender_chat, .. })
            | Message::VideoNote(VideoNote { sender_chat, .. })
            | Message::Voice(Voice { sender_chat, .. })
            | Message::Contact(Contact { sender_chat, .. })
            | Message::Dice(Dice { sender_chat, .. })
            | Message::Game(Game { sender_chat, .. })
            | Message::Poll(Poll { sender_chat, .. })
            | Message::Venue(Venue { sender_chat, .. })
            | Message::Location(Location { sender_chat, .. })
            | Message::NewChatMembers(NewChatMembers { sender_chat, .. })
            | Message::LeftChatMember(LeftChatMember { sender_chat, .. })
            | Message::NewChatTitle(NewChatTitle { sender_chat, .. })
            | Message::NewChatPhoto(NewChatPhoto { sender_chat, .. })
            | Message::DeleteChatPhoto(DeleteChatPhoto { sender_chat, .. })
            | Message::GroupChatCreated(GroupChatCreated { sender_chat, .. })
            | Message::SupergroupChatCreated(SupergroupChatCreated { sender_chat, .. })
            | Message::ChannelChatCreated(ChannelChatCreated { sender_chat, .. })
            | Message::MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged {
                sender_chat,
                ..
            })
            | Message::MigrateToChat(MigrateToChat { sender_chat, .. })
            | Message::MigrateFromChat(MigrateFromChat { sender_chat, .. })
            | Message::Pinned(Pinned { sender_chat, .. })
            | Message::Invoice(Invoice { sender_chat, .. })
            | Message::SuccessfulPayment(SuccessfulPayment { sender_chat, .. })
            | Message::UsersShared(UsersShared { sender_chat, .. })
            | Message::ChatShared(ChatShared { sender_chat, .. })
            | Message::ConnectedWebsite(ConnectedWebsite { sender_chat, .. })
            | Message::WriteAccessAllowed(WriteAccessAllowed { sender_chat, .. })
            | Message::PassportData(PassportData { sender_chat, .. })
            | Message::ProximityAlertTriggered(ProximityAlertTriggered { sender_chat, .. })
            | Message::ForumTopicCreated(ForumTopicCreated { sender_chat, .. })
            | Message::ForumTopicEdited(ForumTopicEdited { sender_chat, .. })
            | Message::ForumTopicClosed(ForumTopicClosed { sender_chat, .. })
            | Message::ForumTopicReopened(ForumTopicReopened { sender_chat, .. })
            | Message::GeneralForumTopicHidden(GeneralForumTopicHidden { sender_chat, .. })
            | Message::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden {
                sender_chat, ..
            })
            | Message::VideoChatScheduled(VideoChatScheduled { sender_chat, .. })
            | Message::VideoChatStarted(VideoChatStarted { sender_chat, .. })
            | Message::VideoChatEnded(VideoChatEnded { sender_chat, .. })
            | Message::VideoChatParticipantsInvited(VideoChatParticipantsInvited {
                sender_chat,
                ..
            })
            | Message::WebAppData(WebAppData { sender_chat, .. })
            | Message::GiveawayCreated(GiveawayCreated { sender_chat, .. })
            | Message::Giveaway(Giveaway { sender_chat, .. })
            | Message::GiveawayWinners(GiveawayWinners { sender_chat, .. })
            | Message::GiveawayCompleted(GiveawayCompleted { sender_chat, .. }) => sender_chat,
        }
        .as_ref()
    }

    #[must_use]
    pub const fn sender_chat_id(&self) -> Option<i64> {
        match self.sender_chat() {
            Some(chat) => Some(chat.id()),
            None => None,
        }
    }

    #[must_use]
    pub const fn author_signature(&self) -> Option<&str> {
        match self {
            Message::Text(Text {
                author_signature, ..
            })
            | Message::Animation(Animation {
                author_signature, ..
            })
            | Message::Audio(Audio {
                author_signature, ..
            })
            | Message::Document(Document {
                author_signature, ..
            })
            | Message::Photo(Photo {
                author_signature, ..
            })
            | Message::Sticker(Sticker {
                author_signature, ..
            })
            | Message::Story(Story {
                author_signature, ..
            })
            | Message::Video(Video {
                author_signature, ..
            })
            | Message::VideoNote(VideoNote {
                author_signature, ..
            })
            | Message::Voice(Voice {
                author_signature, ..
            })
            | Message::Contact(Contact {
                author_signature, ..
            })
            | Message::Dice(Dice {
                author_signature, ..
            })
            | Message::Game(Game {
                author_signature, ..
            })
            | Message::Poll(Poll {
                author_signature, ..
            })
            | Message::Venue(Venue {
                author_signature, ..
            })
            | Message::Location(Location {
                author_signature, ..
            })
            | Message::PassportData(PassportData {
                author_signature, ..
            })
            | Message::WebAppData(WebAppData {
                author_signature, ..
            })
            | Message::Invoice(Invoice {
                author_signature, ..
            }) => match author_signature {
                Some(author_signature) => Some(author_signature),
                None => None,
            },
            _ => None,
        }
    }

    #[must_use]
    pub const fn reply_to_message(&self) -> Option<&Message> {
        match self {
            Message::Text(Text {
                reply_to_message, ..
            })
            | Message::Animation(Animation {
                reply_to_message, ..
            })
            | Message::Audio(Audio {
                reply_to_message, ..
            })
            | Message::Document(Document {
                reply_to_message, ..
            })
            | Message::Photo(Photo {
                reply_to_message, ..
            })
            | Message::Sticker(Sticker {
                reply_to_message, ..
            })
            | Message::Video(Video {
                reply_to_message, ..
            })
            | Message::VideoNote(VideoNote {
                reply_to_message, ..
            })
            | Message::Voice(Voice {
                reply_to_message, ..
            })
            | Message::Contact(Contact {
                reply_to_message, ..
            })
            | Message::Dice(Dice {
                reply_to_message, ..
            })
            | Message::Game(Game {
                reply_to_message, ..
            })
            | Message::Poll(Poll {
                reply_to_message, ..
            })
            | Message::Venue(Venue {
                reply_to_message, ..
            })
            | Message::Location(Location {
                reply_to_message, ..
            })
            | Message::Pinned(Pinned {
                reply_to_message, ..
            })
            | Message::Invoice(Invoice {
                reply_to_message, ..
            })
            | Message::SuccessfulPayment(SuccessfulPayment {
                reply_to_message, ..
            })
            | Message::ForumTopicCreated(ForumTopicCreated {
                reply_to_message, ..
            })
            | Message::ForumTopicEdited(ForumTopicEdited {
                reply_to_message, ..
            })
            | Message::ForumTopicClosed(ForumTopicClosed {
                reply_to_message, ..
            })
            | Message::ForumTopicReopened(ForumTopicReopened {
                reply_to_message, ..
            })
            | Message::GeneralForumTopicHidden(GeneralForumTopicHidden {
                reply_to_message, ..
            })
            | Message::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden {
                reply_to_message,
                ..
            }) => match reply_to_message {
                Some(reply_to_message) => Some(reply_to_message),
                None => None,
            },
            _ => None,
        }
    }

    #[must_use]
    pub const fn external_reply(&self) -> Option<&ExternalReplyInfo> {
        match self {
            Message::Text(Text { external_reply, .. })
            | Message::Animation(Animation { external_reply, .. })
            | Message::Audio(Audio { external_reply, .. })
            | Message::Document(Document { external_reply, .. })
            | Message::Photo(Photo { external_reply, .. })
            | Message::Sticker(Sticker { external_reply, .. })
            | Message::Story(Story { external_reply, .. })
            | Message::Video(Video { external_reply, .. })
            | Message::VideoNote(VideoNote { external_reply, .. })
            | Message::Voice(Voice { external_reply, .. })
            | Message::Contact(Contact { external_reply, .. })
            | Message::Dice(Dice { external_reply, .. })
            | Message::Game(Game { external_reply, .. })
            | Message::Giveaway(Giveaway { external_reply, .. })
            | Message::GiveawayWinners(GiveawayWinners { external_reply, .. })
            | Message::Poll(Poll { external_reply, .. })
            | Message::Venue(Venue { external_reply, .. })
            | Message::Location(Location { external_reply, .. })
            | Message::Invoice(Invoice { external_reply, .. }) => match external_reply {
                Some(external_reply) => Some(external_reply),
                None => None,
            },
            _ => None,
        }
    }

    #[must_use]
    pub const fn quote(&self) -> Option<&TextQuote> {
        match self {
            Message::Text(Text { quote, .. })
            | Message::Animation(Animation { quote, .. })
            | Message::Audio(Audio { quote, .. })
            | Message::Document(Document { quote, .. })
            | Message::Video(Video { quote, .. })
            | Message::Voice(Voice { quote, .. })
            | Message::Photo(Photo { quote, .. }) => match quote {
                Some(quote) => Some(quote),
                None => None,
            },
            _ => None,
        }
    }

    #[must_use]
    pub const fn edit_date(&self) -> Option<i64> {
        *match self {
            Message::Text(Text { edit_date, .. })
            | Message::Animation(Animation { edit_date, .. })
            | Message::Audio(Audio { edit_date, .. })
            | Message::Document(Document { edit_date, .. })
            | Message::Photo(Photo { edit_date, .. })
            | Message::Video(Video { edit_date, .. })
            | Message::Game(Game { edit_date, .. })
            | Message::Poll(Poll { edit_date, .. })
            | Message::Venue(Venue { edit_date, .. })
            | Message::Location(Location { edit_date, .. }) => edit_date,
            _ => &None,
        }
    }

    #[must_use]
    pub const fn reply_markup(&self) -> Option<&InlineKeyboardMarkup> {
        match self {
            Message::Text(Text { reply_markup, .. })
            | Message::Animation(Animation { reply_markup, .. })
            | Message::Audio(Audio { reply_markup, .. })
            | Message::Document(Document { reply_markup, .. })
            | Message::Photo(Photo { reply_markup, .. })
            | Message::Video(Video { reply_markup, .. })
            | Message::VideoNote(VideoNote { reply_markup, .. })
            | Message::Voice(Voice { reply_markup, .. })
            | Message::Contact(Contact { reply_markup, .. })
            | Message::Dice(Dice { reply_markup, .. })
            | Message::Game(Game { reply_markup, .. })
            | Message::Poll(Poll { reply_markup, .. })
            | Message::Venue(Venue { reply_markup, .. })
            | Message::Location(Location { reply_markup, .. })
            | Message::Invoice(Invoice { reply_markup, .. }) => reply_markup,
            _ => &None,
        }
        .as_ref()
    }

    #[must_use]
    pub const fn is_automatic_forward(&self) -> Option<bool> {
        match self {
            Message::Text(Text {
                is_automatic_forward,
                ..
            })
            | Message::Animation(Animation {
                is_automatic_forward,
                ..
            })
            | Message::Audio(Audio {
                is_automatic_forward,
                ..
            })
            | Message::Document(Document {
                is_automatic_forward,
                ..
            })
            | Message::Video(Video {
                is_automatic_forward,
                ..
            })
            | Message::Voice(Voice {
                is_automatic_forward,
                ..
            })
            | Message::Photo(Photo {
                is_automatic_forward,
                ..
            }) => *is_automatic_forward,
            _ => None,
        }
    }

    #[must_use]
    pub const fn has_protected_content(&self) -> Option<bool> {
        match self {
            Message::Text(Text {
                has_protected_content,
                ..
            })
            | Message::Animation(Animation {
                has_protected_content,
                ..
            })
            | Message::Audio(Audio {
                has_protected_content,
                ..
            })
            | Message::Document(Document {
                has_protected_content,
                ..
            })
            | Message::Video(Video {
                has_protected_content,
                ..
            })
            | Message::Voice(Voice {
                has_protected_content,
                ..
            })
            | Message::Photo(Photo {
                has_protected_content,
                ..
            }) => *has_protected_content,
            _ => None,
        }
    }

    #[must_use]
    pub const fn forward_origin(&self) -> Option<&MessageOrigin> {
        match self {
            Message::Text(Text { forward_origin, .. })
            | Message::Animation(Animation { forward_origin, .. })
            | Message::Audio(Audio { forward_origin, .. })
            | Message::Document(Document { forward_origin, .. })
            | Message::Photo(Photo { forward_origin, .. })
            | Message::Sticker(Sticker { forward_origin, .. })
            | Message::Story(Story { forward_origin, .. })
            | Message::Video(Video { forward_origin, .. })
            | Message::VideoNote(VideoNote { forward_origin, .. })
            | Message::Voice(Voice { forward_origin, .. })
            | Message::Contact(Contact { forward_origin, .. })
            | Message::Dice(Dice { forward_origin, .. })
            | Message::Game(Game { forward_origin, .. })
            | Message::Poll(Poll { forward_origin, .. })
            | Message::Venue(Venue { forward_origin, .. })
            | Message::Location(Location { forward_origin, .. })
            | Message::Invoice(Invoice { forward_origin, .. })
            | Message::SuccessfulPayment(SuccessfulPayment { forward_origin, .. })
            | Message::ConnectedWebsite(ConnectedWebsite { forward_origin, .. }) => forward_origin,
            _ => &None,
        }
        .as_ref()
    }

    #[must_use]
    pub const fn animation(&self) -> Option<&types::Animation> {
        match self {
            Message::Animation(Animation { animation, .. }) => Some(animation),
            _ => None,
        }
    }

    #[must_use]
    pub const fn audio(&self) -> Option<&types::Audio> {
        match self {
            Message::Audio(Audio { audio, .. }) => Some(audio),
            _ => None,
        }
    }

    #[must_use]
    pub const fn contact(&self) -> Option<&types::Contact> {
        match self {
            Message::Contact(Contact { contact, .. }) => Some(contact),
            _ => None,
        }
    }

    #[must_use]
    pub const fn dice(&self) -> Option<&types::Dice> {
        match self {
            Message::Dice(Dice { dice, .. }) => Some(dice),
            _ => None,
        }
    }

    #[must_use]
    pub const fn document(&self) -> Option<&types::Document> {
        match self {
            Message::Document(Document { document, .. }) => Some(document),
            _ => None,
        }
    }

    #[must_use]
    pub const fn game(&self) -> Option<&types::Game> {
        match self {
            Message::Game(Game { game, .. }) => Some(game),
            _ => None,
        }
    }

    #[must_use]
    pub const fn poll(&self) -> Option<&types::Poll> {
        match self {
            Message::Poll(Poll { poll, .. }) => Some(poll),
            _ => None,
        }
    }

    #[must_use]
    pub const fn venue(&self) -> Option<&types::Venue> {
        match self {
            Message::Venue(Venue { venue, .. }) => Some(venue),
            _ => None,
        }
    }

    #[must_use]
    pub const fn location(&self) -> Option<&types::Location> {
        match self {
            Message::Location(Location { location, .. }) => Some(location),
            _ => None,
        }
    }

    #[must_use]
    pub const fn new_chat_members(&self) -> Option<&[User]> {
        match self {
            Message::NewChatMembers(NewChatMembers { members, .. }) => Some(members),
            _ => None,
        }
    }

    #[must_use]
    pub const fn left_chat_member(&self) -> Option<&User> {
        match self {
            Message::LeftChatMember(LeftChatMember { member, .. }) => Some(member),
            _ => None,
        }
    }

    #[must_use]
    pub const fn new_chat_title(&self) -> Option<&str> {
        match self {
            Message::NewChatTitle(NewChatTitle { title, .. }) => Some(title),
            _ => None,
        }
    }

    #[must_use]
    pub const fn new_chat_photo(&self) -> Option<&[PhotoSize]> {
        match self {
            Message::NewChatPhoto(NewChatPhoto { photo, .. }) => Some(photo),
            _ => None,
        }
    }

    #[must_use]
    pub const fn delete_chat_photo(&self) -> Option<bool> {
        match self {
            Message::DeleteChatPhoto(DeleteChatPhoto { photo, .. }) => Some(*photo),
            _ => None,
        }
    }

    #[must_use]
    pub const fn group_chat_created(&self) -> Option<bool> {
        match self {
            Message::GroupChatCreated(GroupChatCreated { created, .. }) => Some(*created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn supergroup_chat_created(&self) -> Option<bool> {
        match self {
            Message::SupergroupChatCreated(SupergroupChatCreated { created, .. }) => Some(*created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn channel_chat_created(&self) -> Option<bool> {
        match self {
            Message::ChannelChatCreated(ChannelChatCreated { created, .. }) => Some(*created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn message_auto_delete_timer_changed(
        &self,
    ) -> Option<&types::MessageAutoDeleteTimerChanged> {
        match self {
            Message::MessageAutoDeleteTimerChanged(MessageAutoDeleteTimerChanged {
                timer, ..
            }) => Some(timer),
            _ => None,
        }
    }

    #[must_use]
    pub const fn pinned(&self) -> Option<&MaybeInaccessibleMessage> {
        match self {
            Message::Pinned(Pinned { message, .. }) => Some(message),
            _ => None,
        }
    }

    #[must_use]
    pub const fn invoice(&self) -> Option<&types::Invoice> {
        match self {
            Message::Invoice(Invoice { invoice, .. }) => Some(invoice),
            _ => None,
        }
    }

    #[must_use]
    pub const fn successful_payment(&self) -> Option<&types::SuccessfulPayment> {
        match self {
            Message::SuccessfulPayment(SuccessfulPayment { payment, .. }) => Some(payment),
            _ => None,
        }
    }

    #[must_use]
    pub const fn users_shared(&self) -> Option<&types::UsersShared> {
        match self {
            Message::UsersShared(UsersShared { shared, .. }) => Some(shared),
            _ => None,
        }
    }

    #[must_use]
    pub const fn chat_shared(&self) -> Option<&types::ChatShared> {
        match self {
            Message::ChatShared(ChatShared { shared, .. }) => Some(shared),
            _ => None,
        }
    }

    #[must_use]
    pub const fn connected_website(&self) -> Option<&str> {
        match self {
            Message::ConnectedWebsite(ConnectedWebsite { website, .. }) => Some(website),
            _ => None,
        }
    }

    #[must_use]
    pub const fn write_access_allowed(&self) -> Option<&types::WriteAccessAllowed> {
        match self {
            Message::WriteAccessAllowed(WriteAccessAllowed { allowed, .. }) => Some(allowed),
            _ => None,
        }
    }

    #[must_use]
    pub const fn passport_data(&self) -> Option<&types::PassportData> {
        match self {
            Message::PassportData(PassportData { data, .. }) => Some(data),
            _ => None,
        }
    }

    #[must_use]
    pub const fn proximity_alert_triggered(&self) -> Option<&types::ProximityAlertTriggered> {
        match self {
            Message::ProximityAlertTriggered(ProximityAlertTriggered { triggered, .. }) => {
                Some(triggered)
            }
            _ => None,
        }
    }

    #[must_use]
    pub const fn forum_topic_created(&self) -> Option<&types::ForumTopicCreated> {
        match self {
            Message::ForumTopicCreated(ForumTopicCreated { created, .. }) => Some(created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn forum_topic_edited(&self) -> Option<&types::ForumTopicEdited> {
        match self {
            Message::ForumTopicEdited(ForumTopicEdited { edited, .. }) => Some(edited),
            _ => None,
        }
    }

    #[must_use]
    pub const fn forum_topic_closed(&self) -> Option<&types::ForumTopicClosed> {
        match self {
            Message::ForumTopicClosed(ForumTopicClosed { closed, .. }) => Some(closed),
            _ => None,
        }
    }

    #[must_use]
    pub const fn forum_topic_reopened(&self) -> Option<&types::ForumTopicReopened> {
        match self {
            Message::ForumTopicReopened(ForumTopicReopened { reopened, .. }) => Some(reopened),
            _ => None,
        }
    }

    #[must_use]
    pub const fn general_forum_topic_hidden(&self) -> Option<&types::GeneralForumTopicHidden> {
        match self {
            Message::GeneralForumTopicHidden(GeneralForumTopicHidden { hidden, .. }) => {
                Some(hidden)
            }
            _ => None,
        }
    }

    #[must_use]
    pub const fn general_forum_topic_unhidden(&self) -> Option<&types::GeneralForumTopicUnhidden> {
        match self {
            Message::GeneralForumTopicUnhidden(GeneralForumTopicUnhidden { unhidden, .. }) => {
                Some(unhidden)
            }
            _ => None,
        }
    }

    #[must_use]
    pub const fn giveaway_created(&self) -> Option<&types::GiveawayCreated> {
        match self {
            Message::GiveawayCreated(GiveawayCreated { created, .. }) => Some(created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn giveaway(&self) -> Option<&types::Giveaway> {
        match self {
            Message::Giveaway(Giveaway { giveaway, .. }) => Some(giveaway),
            _ => None,
        }
    }

    #[must_use]
    pub const fn giveaway_winners(&self) -> Option<&types::GiveawayWinners> {
        match self {
            Message::GiveawayWinners(GiveawayWinners { winners, .. }) => Some(winners),
            _ => None,
        }
    }

    #[must_use]
    pub const fn giveaway_completed(&self) -> Option<&types::GiveawayCompleted> {
        match self {
            Message::GiveawayCompleted(GiveawayCompleted { completed, .. }) => Some(completed),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_chat_scheduled(&self) -> Option<&types::VideoChatScheduled> {
        match self {
            Message::VideoChatScheduled(VideoChatScheduled { scheduled, .. }) => Some(scheduled),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_chat_started(&self) -> Option<&types::VideoChatStarted> {
        match self {
            Message::VideoChatStarted(VideoChatStarted { started, .. }) => Some(started),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_chat_ended(&self) -> Option<&types::VideoChatEnded> {
        match self {
            Message::VideoChatEnded(VideoChatEnded { ended, .. }) => Some(ended),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_chat_participants_invited(
        &self,
    ) -> Option<&types::VideoChatParticipantsInvited> {
        match self {
            Message::VideoChatParticipantsInvited(VideoChatParticipantsInvited {
                invited, ..
            }) => Some(invited),
            _ => None,
        }
    }

    #[must_use]
    pub const fn web_app_data(&self) -> Option<&types::WebAppData> {
        match self {
            Message::WebAppData(WebAppData { data, .. }) => Some(data),
            _ => None,
        }
    }

    #[must_use]
    pub const fn photo(&self) -> Option<&[PhotoSize]> {
        match self {
            Message::Photo(Photo { photo, .. }) => Some(photo),
            _ => None,
        }
    }

    /// Returns the largest photo in the message by `width` + `height`
    #[must_use]
    pub fn largest_photo(&self) -> Option<&PhotoSize> {
        match self {
            Message::Photo(message_photo) => message_photo.largest_photo(),
            _ => None,
        }
    }

    /// Returns the smallest photo in the message by `width` + `height`
    #[must_use]
    pub fn smallest_photo(&self) -> Option<&PhotoSize> {
        match self {
            Message::Photo(message_photo) => message_photo.smallest_photo(),
            _ => None,
        }
    }

    /// Returns the largest photo in the message by `file_size`
    #[must_use]
    pub fn largest_photo_by_file_size(&self) -> Option<&PhotoSize> {
        match self {
            Message::Photo(message_photo) => message_photo.largest_photo_by_file_size(),
            _ => None,
        }
    }

    /// Returns the smallest photo in the message by `file_size`
    #[must_use]
    pub fn smallest_photo_by_file_size(&self) -> Option<&PhotoSize> {
        match self {
            Message::Photo(message_photo) => message_photo.smallest_photo_by_file_size(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn story(&self) -> Option<&types::Story> {
        match self {
            Message::Story(Story { story, .. }) => Some(story),
            _ => None,
        }
    }

    #[must_use]
    pub const fn sticker(&self) -> Option<&types::Sticker> {
        match self {
            Message::Sticker(Sticker { sticker, .. }) => Some(sticker),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video(&self) -> Option<&types::Video> {
        match self {
            Message::Video(Video { video, .. }) => Some(video),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_note(&self) -> Option<&types::VideoNote> {
        match self {
            Message::VideoNote(VideoNote { video_note, .. }) => Some(video_note),
            _ => None,
        }
    }

    #[must_use]
    pub const fn voice(&self) -> Option<&types::Voice> {
        match self {
            Message::Voice(Voice { voice, .. }) => Some(voice),
            _ => None,
        }
    }

    #[must_use]
    pub const fn migrate_to_chat_id(&self) -> Option<i64> {
        match self {
            Message::MigrateToChat(MigrateToChat {
                to_chat_id: chat_id,
                ..
            }) => Some(*chat_id),
            _ => None,
        }
    }

    #[must_use]
    pub const fn migrate_from_chat_id(&self) -> Option<i64> {
        match self {
            Message::MigrateFromChat(MigrateFromChat {
                from_chat_id: chat_id,
                ..
            }) => Some(*chat_id),
            _ => None,
        }
    }
}

impl Default for Message {
    #[must_use]
    fn default() -> Self {
        Message::Text(Text::default())
    }
}

macro_rules! impl_try_from_message {
    ($variant:ident, $ty:ty) => {
        impl TryFrom<Message> for $ty {
            type Error = ConvertToTypeError;

            fn try_from(value: Message) -> Result<Self, Self::Error> {
                if let Message::$variant(val) = value {
                    Ok(val)
                } else {
                    Err(Self::Error::new("Message", stringify!($ty)))
                }
            }
        }
    };
}

impl_try_from_message!(Text, Text);
impl_try_from_message!(Animation, Animation);
impl_try_from_message!(Audio, Audio);
impl_try_from_message!(Contact, Contact);
impl_try_from_message!(Dice, Dice);
impl_try_from_message!(Document, Document);
impl_try_from_message!(Game, Game);
impl_try_from_message!(Invoice, Invoice);
impl_try_from_message!(Location, Location);
impl_try_from_message!(NewChatMembers, NewChatMembers);
impl_try_from_message!(LeftChatMember, LeftChatMember);
impl_try_from_message!(NewChatTitle, NewChatTitle);
impl_try_from_message!(NewChatPhoto, NewChatPhoto);
impl_try_from_message!(DeleteChatPhoto, DeleteChatPhoto);
impl_try_from_message!(GroupChatCreated, GroupChatCreated);
impl_try_from_message!(SupergroupChatCreated, SupergroupChatCreated);
impl_try_from_message!(ChannelChatCreated, ChannelChatCreated);
impl_try_from_message!(MigrateToChat, MigrateToChat);
impl_try_from_message!(MigrateFromChat, MigrateFromChat);
impl_try_from_message!(Pinned, Pinned);
impl_try_from_message!(SuccessfulPayment, SuccessfulPayment);
impl_try_from_message!(ConnectedWebsite, ConnectedWebsite);
impl_try_from_message!(PassportData, PassportData);
impl_try_from_message!(ProximityAlertTriggered, ProximityAlertTriggered);
impl_try_from_message!(ForumTopicCreated, ForumTopicCreated);
impl_try_from_message!(ForumTopicEdited, ForumTopicEdited);
impl_try_from_message!(ForumTopicClosed, ForumTopicClosed);
impl_try_from_message!(ForumTopicReopened, ForumTopicReopened);
impl_try_from_message!(GeneralForumTopicHidden, GeneralForumTopicHidden);
impl_try_from_message!(GeneralForumTopicUnhidden, GeneralForumTopicUnhidden);
impl_try_from_message!(GiveawayCreated, GiveawayCreated);
impl_try_from_message!(Giveaway, Giveaway);
impl_try_from_message!(GiveawayWinners, GiveawayWinners);
impl_try_from_message!(GiveawayCompleted, GiveawayCompleted);
impl_try_from_message!(VideoChatScheduled, VideoChatScheduled);
impl_try_from_message!(VideoChatStarted, VideoChatStarted);
impl_try_from_message!(VideoChatEnded, VideoChatEnded);
impl_try_from_message!(VideoChatParticipantsInvited, VideoChatParticipantsInvited);
impl_try_from_message!(WebAppData, WebAppData);
impl_try_from_message!(Poll, Poll);
impl_try_from_message!(Venue, Venue);
impl_try_from_message!(Photo, Photo);
impl_try_from_message!(Story, Story);
impl_try_from_message!(Sticker, Sticker);
impl_try_from_message!(Video, Video);
impl_try_from_message!(VideoNote, VideoNote);
impl_try_from_message!(Voice, Voice);
impl_try_from_message!(WriteAccessAllowed, WriteAccessAllowed);
impl_try_from_message!(UsersShared, UsersShared);
impl_try_from_message!(ChatShared, ChatShared);
impl_try_from_message!(MessageAutoDeleteTimerChanged, MessageAutoDeleteTimerChanged);

impl TryFrom<Update> for Message {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::Message(val)
            | UpdateKind::EditedMessage(val)
            | UpdateKind::ChannelPost(val)
            | UpdateKind::EditedChannelPost(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "Message")),
        }
    }
}

macro_rules! impl_try_from_update {
    ($ty:ty) => {
        impl TryFrom<Update> for $ty {
            type Error = ConvertToTypeError;

            fn try_from(update: Update) -> Result<Self, Self::Error> {
                Message::try_from(update)?.try_into()
            }
        }
    };
}

impl_try_from_update!(Text);
impl_try_from_update!(Animation);
impl_try_from_update!(Audio);
impl_try_from_update!(Contact);
impl_try_from_update!(Dice);
impl_try_from_update!(Document);
impl_try_from_update!(Game);
impl_try_from_update!(Invoice);
impl_try_from_update!(Location);
impl_try_from_update!(NewChatMembers);
impl_try_from_update!(LeftChatMember);
impl_try_from_update!(NewChatTitle);
impl_try_from_update!(NewChatPhoto);
impl_try_from_update!(DeleteChatPhoto);
impl_try_from_update!(GroupChatCreated);
impl_try_from_update!(SupergroupChatCreated);
impl_try_from_update!(ChannelChatCreated);
impl_try_from_update!(MigrateToChat);
impl_try_from_update!(MigrateFromChat);
impl_try_from_update!(Pinned);
impl_try_from_update!(SuccessfulPayment);
impl_try_from_update!(ConnectedWebsite);
impl_try_from_update!(PassportData);
impl_try_from_update!(ProximityAlertTriggered);
impl_try_from_update!(ForumTopicCreated);
impl_try_from_update!(ForumTopicEdited);
impl_try_from_update!(ForumTopicClosed);
impl_try_from_update!(ForumTopicReopened);
impl_try_from_update!(GeneralForumTopicHidden);
impl_try_from_update!(GeneralForumTopicUnhidden);
impl_try_from_update!(GiveawayCreated);
impl_try_from_update!(Giveaway);
impl_try_from_update!(GiveawayWinners);
impl_try_from_update!(GiveawayCompleted);
impl_try_from_update!(VideoChatScheduled);
impl_try_from_update!(VideoChatStarted);
impl_try_from_update!(VideoChatEnded);
impl_try_from_update!(VideoChatParticipantsInvited);
impl_try_from_update!(WebAppData);
impl_try_from_update!(Poll);
impl_try_from_update!(Venue);
impl_try_from_update!(Photo);
impl_try_from_update!(Story);
impl_try_from_update!(Sticker);
impl_try_from_update!(Video);
impl_try_from_update!(VideoNote);
impl_try_from_update!(Voice);
impl_try_from_update!(WriteAccessAllowed);
impl_try_from_update!(UsersShared);
impl_try_from_update!(ChatShared);
impl_try_from_update!(MessageAutoDeleteTimerChanged);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_text() {
        let jsons = [
            serde_json::json!({
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "text": "test",
            }),
            serde_json::json!({
                "message_id": 160,
                "date": 0,
                "chat": {
                    "id": 1,
                    "title": "test",
                    "type": "supergroup",
                },
                "sender_chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "from": {
                    "id": 777_000,
                    "is_bot": false,
                    "first_name": "test",
                },
                "forward_from_chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "forward_from_message_id": 1,
                "forward_signature": "test",
                "is_automatic_forward": true,
                "forward_date": 0,
                "text": "test",
            }),
        ];

        for json in jsons {
            let message_text: Text = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Text(message) => assert_eq!(message, message_text),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_text_with_forward() {
        let jsons = [
            serde_json::json!({
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "forward_from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
                "forward_date": 0,
                "text": "test",
            }),
            serde_json::json!({
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "forward_from_chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "forward_date": 0,
                "text": "test",
            }),
            serde_json::json!({
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "forward_sender_name": "test",
                "forward_date": 0,
                "text": "test",
            }),
            serde_json::json!({
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "forward_from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
                "forward_date": 0,
                "forward_from_message_id": 1,
                "forward_signature": "test",
                "text": "test",
            }),
        ];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Text(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_animation() {
        let jsons = [
            serde_json::json!({
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "animation": {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "duration": 1,
                },
            }),
            serde_json::json!({
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "animation": {
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                    "duration": 1,
                },
                "document": {
                    "file_id": "test",
                    "file_unique_id": "test",
                },
            }),
        ];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Animation(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_audio() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "audio": {
                "file_id": "test",
                "file_unique_id": "test",
                "duration": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Audio(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_document() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "document": {
                "file_id": "test",
                "file_unique_id": "test",
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Document(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_photo() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "photo": [{
                "file_id": "test",
                "file_unique_id": "test",
                "width": 1,
                "height": 1,
            }],
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Photo(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_sticker() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "sticker": {
                "file_id": "test",
                "file_unique_id": "test",
                "type": "regular",
                "width": 1,
                "height": 1,
                "is_animated": false,
                "is_video": false,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Sticker(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_story() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "story": {},
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Story(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_video() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "video": {
                "file_id": "test",
                "file_unique_id": "test",
                "width": 1,
                "height": 1,
                "duration": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Video(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_video_note() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "video_note": {
                "file_id": "test",
                "file_unique_id": "test",
                "length": 1,
                "duration": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::VideoNote(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_voice() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "voice": {
                "file_id": "test",
                "file_unique_id": "test",
                "duration": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Voice(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_contact() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "contact": {
                "phone_number": "test",
                "first_name": "test",
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Contact(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_dice() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "dice": {
                "emoji": "🎲",
                "value": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Dice(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_game() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "game": {
                "title": "test",
                "description": "test",
                "photo": [{
                    "file_id": "test",
                    "file_unique_id": "test",
                    "width": 1,
                    "height": 1,
                }],
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Game(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_poll() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "author_signature": "test",
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "poll": {
                "id": "test",
                "question": "test",
                "options": [
                    {
                        "text": "test",
                        "voter_count": 1,
                    },
                    {
                        "text": "test",
                        "voter_count": 1,
                    },
                ],
                "total_voter_count": 2,
                "is_closed": false,
                "is_anonymous": false,
                "type": "regular",
                "allows_multiple_answers": false,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Poll(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_venue() {
        let jsons = [
            serde_json::json!({
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "venue": {
                    "location": {
                        "latitude": 1.0,
                        "longitude": 1.0,
                    },
                    "title": "test",
                    "address": "test",
                },
            }),
            serde_json::json!({
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "venue": {
                    "location": {
                        "latitude": 1.0,
                        "longitude": 1.0,
                    },
                    "title": "test",
                    "address": "test",
                },
                "location": {
                    "latitude": 1.0,
                    "longitude": 1.0,
                },
            }),
        ];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Venue(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_location() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "location": {
                "latitude": 1.0,
                "longitude": 1.0,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Location(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_new_chat_members() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "new_chat_members": [{
                "id": 1,
                "is_bot": false,
                "first_name": "test",
            }],
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::NewChatMembers(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_left_chat_member() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "left_chat_member": {
                "id": 1,
                "is_bot": false,
                "first_name": "test",
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::LeftChatMember(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_new_chat_title() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "new_chat_title": "test",
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::NewChatTitle(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_new_chat_photo() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "new_chat_photo": [{
                "file_id": "test",
                "file_unique_id": "test",
                "width": 1,
                "height": 1,
            }],
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::NewChatPhoto(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_delete_chat_photo() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "delete_chat_photo": true,
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::DeleteChatPhoto(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_group_chat_created() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "group_chat_created": true,
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::GroupChatCreated(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_supergroup_chat_created() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "supergroup_chat_created": true,
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::SupergroupChatCreated(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_channel_chat_created() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "channel_chat_created": true,
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::ChannelChatCreated(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_auto_delete_timer_changed() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "message_auto_delete_timer_changed": {
                "message_auto_delete_time": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::MessageAutoDeleteTimerChanged(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_migrate_to_chat_id() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 1,
                "title": "test",
                "type": "group",
            },
            "migrate_to_chat_id": 2,
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::MigrateToChat(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_migrate_from_chat_id() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": 2,
                "title": "test",
                "type": "group",
            },
            "migrate_from_chat_id": 1,
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::MigrateFromChat(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_pinned_message() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "pinned_message": {
                "message_id": 2,
                "date": 0,
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "text": "test",
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Pinned(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_invoice() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "invoice": {
                "title": "test",
                "description": "test",
                "start_parameter": "test",
                "currency": "test",
                "total_amount": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Invoice(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_successful_payment() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "successful_payment": {
                "currency": "test",
                "total_amount": 1,
                "invoice_payload": "test",
                "telegram_payment_charge_id": "test",
                "provider_payment_charge_id": "test",
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::SuccessfulPayment(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_users_shared() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "users_shared": {
                "request_id": 1,
                "user_ids": [1, 2, 3],
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::UsersShared(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_chat_shared() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "chat_shared": {
                "request_id": 1,
                "chat_id": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::ChatShared(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_connected_website() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "connected_website": "test",
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::ConnectedWebsite(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_write_access_allowed() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "write_access_allowed": {},
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::WriteAccessAllowed(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_passport_data() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "passport_data": {
                "data": [{
                    "type": "personal_details",
                    "data": "test",
                    "hash": "test",
                }],
                "credentials": {
                    "data": "test",
                    "hash": "test",
                    "secret": "test",
                },
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::PassportData(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_proximity_alert_triggered() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "proximity_alert_triggered": {
                "traveler": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                },
                "watcher": {
                    "id": 2,
                    "is_bot": false,
                    "first_name": "test",
                },
                "distance": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::ProximityAlertTriggered(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_forum_topic_created() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "forum_topic_created": {
                "name": "test",
                "icon_color": 1,
                "icon_custom_emoji_id": "test",
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::ForumTopicCreated(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_forum_topic_edited() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "forum_topic_edited": {
                "name": "test",
                "icon_custom_emoji_id": "test",
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::ForumTopicEdited(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_forum_topic_closed() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "forum_topic_closed": {},
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::ForumTopicClosed(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_forum_topic_reopened() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "forum_topic_reopened": {},
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::ForumTopicReopened(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_general_forum_topic_hidden() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "general_forum_topic_hidden": {},
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::GeneralForumTopicHidden(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_general_forum_topic_unhidden() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "general_forum_topic_unhidden": {},
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::GeneralForumTopicUnhidden(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_giveaway_created() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "giveaway_created": {},
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::GiveawayCreated(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_giveaway() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "giveaway": {
                "chats": [{
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                }],
                "winners_selection_date": 0,
                "winner_count": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Giveaway(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_giveaway_winners() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "giveaway_winners": {
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "giveaway_message_id": 1,
                "winners_selection_date": 0,
                "winner_count": 1,
                "winners": [{
                    "id": 1,
                    "is_bot": false,
                    "first_name": "test",
                }],
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::GiveawayWinners(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_giveaway_completed() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "giveaway_completed": {
                "winner_count": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::GiveawayCompleted(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_video_chat_scheduled() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "video_chat_scheduled": {
                "start_date": 0,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::VideoChatScheduled(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_video_chat_started() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "video_chat_started": {},
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::VideoChatStarted(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_video_chat_ended() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "video_chat_ended": {
                "duration": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::VideoChatEnded(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_video_chat_participants_invited() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "video_chat_participants_invited": {
                "users": [
                    {
                        "id": 1,
                        "is_bot": false,
                        "first_name": "test",
                    },
                    {
                        "id": 2,
                        "is_bot": false,
                        "first_name": "test",
                    },
                ],
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::VideoChatParticipantsInvited(message) => {
                    assert_eq!(message, message_kind);
                }
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }

    #[test]
    fn deserialize_web_app_data() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "web_app_data": {
                "data": "test",
                "button_text": "test",
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::WebAppData(message) => assert_eq!(message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }
}
