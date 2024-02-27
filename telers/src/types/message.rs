use super::{
    Chat, ExternalReplyInfo, InlineKeyboardMarkup, LinkPreviewOptions, MaybeInaccessibleMessage,
    MessageEntity, MessageOrigin, PhotoSize, TextQuote, Update, UpdateKind, User,
};

use crate::{errors::ConvertToTypeError, extractors::FromEvent, types};

use serde::Deserialize;

/// This object represents a message.
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
/// # Notes
/// We use `Box` to avoid stack overflow in some cases but minus in usability in `match`-case.
/// To compensation this, you can use methods that simplify get some data.
/// For example, instead of get [`Text`] from [`Message`] to get [`Text::text`], you can just call [`Message::text`],
/// or if you want to get caption from all types where it has,
/// instead of using [`Animation::caption`], [`Audio::caption`], ..., you can use [`Message::caption`].
/// Similar methods are implemented for all major message types.
#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
#[serde(untagged)]
pub enum Message {
    Text(Box<Text>),
    Animation(Box<Animation>),
    Audio(Box<Audio>),
    Document(Box<Document>),
    Photo(Box<Photo>),
    Sticker(Box<Sticker>),
    Story(Box<Story>),
    Video(Box<Video>),
    VideoNote(Box<VideoNote>),
    Voice(Box<Voice>),
    Contact(Box<Contact>),
    Dice(Box<Dice>),
    Game(Box<Game>),
    Poll(Box<Poll>),
    Venue(Box<Venue>),
    Location(Box<Location>),
    NewChatMembers(Box<NewChatMembers>),
    LeftChatMember(Box<LeftChatMember>),
    NewChatTitle(Box<NewChatTitle>),
    NewChatPhoto(Box<NewChatPhoto>),
    DeleteChatPhoto(Box<DeleteChatPhoto>),
    GroupChatCreated(Box<GroupChatCreated>),
    SupergroupChatCreated(Box<SupergroupChatCreated>),
    ChannelChatCreated(Box<ChannelChatCreated>),
    MessageAutoDeleteTimerChanged(Box<MessageAutoDeleteTimerChanged>),
    MigrateToChat(Box<MigrateToChat>),
    MigrateFromChat(Box<MigrateFromChat>),
    Pinned(Box<Pinned>),
    Invoice(Box<Invoice>),
    SuccessfulPayment(Box<SuccessfulPayment>),
    UsersShared(Box<UsersShared>),
    ChatShared(Box<ChatShared>),
    ConnectedWebsite(Box<ConnectedWebsite>),
    WriteAccessAllowed(Box<WriteAccessAllowed>),
    PassportData(Box<PassportData>),
    ProximityAlertTriggered(Box<ProximityAlertTriggered>),
    ChatBoostAdded(Box<ChatBoostAdded>),
    ForumTopicCreated(Box<ForumTopicCreated>),
    ForumTopicEdited(Box<ForumTopicEdited>),
    ForumTopicClosed(Box<ForumTopicClosed>),
    ForumTopicReopened(Box<ForumTopicReopened>),
    GeneralForumTopicHidden(Box<GeneralForumTopicHidden>),
    GeneralForumTopicUnhidden(Box<GeneralForumTopicUnhidden>),
    GiveawayCreated(Box<GiveawayCreated>),
    Giveaway(Box<Giveaway>),
    GiveawayWinners(Box<GiveawayWinners>),
    GiveawayCompleted(Box<GiveawayCompleted>),
    VideoChatScheduled(Box<VideoChatScheduled>),
    VideoChatStarted(Box<VideoChatStarted>),
    VideoChatEnded(Box<VideoChatEnded>),
    VideoChatParticipantsInvited(Box<VideoChatParticipantsInvited>),
    WebAppData(Box<WebAppData>),
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
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
    pub reply_to_message: Option<Message>,
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
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    pub reply_to_message: Option<Message>,
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
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    pub sender_boost_count: Option<i64>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Information about the original message for forwarded messages
    pub forward_origin: Option<MessageOrigin>,
    /// `true`, if the message is sent to a forum topic
    pub is_topic_message: Option<bool>,
    /// For replies, the original message. Note that the [Message object](https://core.telegram.org/bots/api#message) in this field will not contain further *reply_to_message* fields even if it itself is a reply.
    pub reply_to_message: Option<Message>,
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
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Sender of the message, sent on behalf of a chat. For example, the channel itself for channel posts, the supergroup itself for messages from anonymous group administrators, the linked channel for messages automatically forwarded to the discussion group. For backward compatibility, the field *from* contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub sender_chat: Option<Chat>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Sender of the message; empty for messages sent to channels. For backward compatibility, the field contains a fake sender user in non-channel chats, if the message was sent on behalf of a chat.
    pub from: Option<User>,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
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
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[serde(rename = "proximity_alert_triggered")]
    pub triggered: types::ProximityAlertTriggered,
}

#[derive(Debug, Clone, PartialEq, Deserialize, FromEvent)]
#[event(try_from = Update)]
pub struct ChatBoostAdded {
    /// Unique message identifier inside this chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: user boosted the chat
    #[serde(rename = "boost_added")]
    pub added: types::ChatBoostAdded,
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
    pub reply_to_message: Option<Message>,
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
    pub reply_to_message: Option<Message>,
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
    pub reply_to_message: Option<Message>,
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
    pub reply_to_message: Option<Message>,
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
    pub reply_to_message: Option<Message>,
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
    pub reply_to_message: Option<Message>,
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
    /// Date the message was sent in Unix time
    pub date: i64,
    /// Conversation the message belongs to
    pub chat: Chat,
    /// Service message: data sent by a Web App
    #[serde(rename = "web_app_data")]
    pub data: types::WebAppData,
}

impl Message {
    #[must_use]
    pub const fn id(&self) -> i64 {
        match self {
            Message::Text(message) => message.id,
            Message::Animation(message) => message.id,
            Message::Audio(message) => message.id,
            Message::Document(message) => message.id,
            Message::Photo(message) => message.id,
            Message::Sticker(message) => message.id,
            Message::Story(message) => message.id,
            Message::Video(message) => message.id,
            Message::VideoNote(message) => message.id,
            Message::Voice(message) => message.id,
            Message::Contact(message) => message.id,
            Message::Dice(message) => message.id,
            Message::Game(message) => message.id,
            Message::Poll(message) => message.id,
            Message::Venue(message) => message.id,
            Message::Location(message) => message.id,
            Message::NewChatMembers(message) => message.id,
            Message::LeftChatMember(message) => message.id,
            Message::NewChatTitle(message) => message.id,
            Message::NewChatPhoto(message) => message.id,
            Message::DeleteChatPhoto(message) => message.id,
            Message::GroupChatCreated(message) => message.id,
            Message::SupergroupChatCreated(message) => message.id,
            Message::ChannelChatCreated(message) => message.id,
            Message::MessageAutoDeleteTimerChanged(message) => message.id,
            Message::MigrateToChat(message) => message.id,
            Message::MigrateFromChat(message) => message.id,
            Message::Pinned(message) => message.id,
            Message::Invoice(message) => message.id,
            Message::SuccessfulPayment(message) => message.id,
            Message::UsersShared(message) => message.id,
            Message::ChatShared(message) => message.id,
            Message::ConnectedWebsite(message) => message.id,
            Message::WriteAccessAllowed(message) => message.id,
            Message::PassportData(message) => message.id,
            Message::ProximityAlertTriggered(message) => message.id,
            Message::ChatBoostAdded(message) => message.id,
            Message::ForumTopicCreated(message) => message.id,
            Message::ForumTopicEdited(message) => message.id,
            Message::ForumTopicClosed(message) => message.id,
            Message::ForumTopicReopened(message) => message.id,
            Message::GeneralForumTopicHidden(message) => message.id,
            Message::GeneralForumTopicUnhidden(message) => message.id,
            Message::VideoChatScheduled(message) => message.id,
            Message::VideoChatStarted(message) => message.id,
            Message::VideoChatEnded(message) => message.id,
            Message::VideoChatParticipantsInvited(message) => message.id,
            Message::WebAppData(message) => message.id,
            Message::GiveawayCreated(message) => message.id,
            Message::Giveaway(message) => message.id,
            Message::GiveawayWinners(message) => message.id,
            Message::GiveawayCompleted(message) => message.id,
        }
    }

    #[must_use]
    pub const fn thread_id(&self) -> Option<i64> {
        match self {
            Message::Text(message) => message.thread_id,
            Message::Animation(message) => message.thread_id,
            Message::Audio(message) => message.thread_id,
            Message::Document(message) => message.thread_id,
            Message::Photo(message) => message.thread_id,
            Message::Sticker(message) => message.thread_id,
            Message::Story(message) => message.thread_id,
            Message::Video(message) => message.thread_id,
            Message::VideoNote(message) => message.thread_id,
            Message::Voice(message) => message.thread_id,
            Message::Contact(message) => message.thread_id,
            Message::Dice(message) => message.thread_id,
            Message::Game(message) => message.thread_id,
            Message::Poll(message) => message.thread_id,
            Message::Venue(message) => message.thread_id,
            Message::Location(message) => message.thread_id,
            Message::Pinned(message) => message.thread_id,
            Message::Invoice(message) => message.thread_id,
            Message::PassportData(message) => message.thread_id,
            Message::ForumTopicCreated(message) => message.thread_id,
            Message::ForumTopicEdited(message) => message.thread_id,
            Message::ForumTopicClosed(message) => message.thread_id,
            Message::ForumTopicReopened(message) => message.thread_id,
            Message::GeneralForumTopicHidden(message) => message.thread_id,
            Message::GeneralForumTopicUnhidden(message) => message.thread_id,
            Message::GiveawayCreated(message) => message.thread_id,
            Message::Giveaway(message) => message.thread_id,
            Message::GiveawayWinners(message) => message.thread_id,
            Message::GiveawayCompleted(message) => message.thread_id,
            _ => None,
        }
    }

    #[must_use]
    pub const fn date(&self) -> i64 {
        match self {
            Message::Text(message) => message.date,
            Message::Animation(message) => message.date,
            Message::Audio(message) => message.date,
            Message::Document(message) => message.date,
            Message::Photo(message) => message.date,
            Message::Sticker(message) => message.date,
            Message::Story(message) => message.date,
            Message::Video(message) => message.date,
            Message::VideoNote(message) => message.date,
            Message::Voice(message) => message.date,
            Message::Contact(message) => message.date,
            Message::Dice(message) => message.date,
            Message::Game(message) => message.date,
            Message::Poll(message) => message.date,
            Message::Venue(message) => message.date,
            Message::Location(message) => message.date,
            Message::NewChatMembers(message) => message.date,
            Message::LeftChatMember(message) => message.date,
            Message::NewChatTitle(message) => message.date,
            Message::NewChatPhoto(message) => message.date,
            Message::DeleteChatPhoto(message) => message.date,
            Message::GroupChatCreated(message) => message.date,
            Message::SupergroupChatCreated(message) => message.date,
            Message::ChannelChatCreated(message) => message.date,
            Message::MessageAutoDeleteTimerChanged(message) => message.date,
            Message::MigrateToChat(message) => message.date,
            Message::MigrateFromChat(message) => message.date,
            Message::Pinned(message) => message.date,
            Message::Invoice(message) => message.date,
            Message::SuccessfulPayment(message) => message.date,
            Message::UsersShared(message) => message.date,
            Message::ChatShared(message) => message.date,
            Message::ConnectedWebsite(message) => message.date,
            Message::WriteAccessAllowed(message) => message.date,
            Message::PassportData(message) => message.date,
            Message::ProximityAlertTriggered(message) => message.date,
            Message::ChatBoostAdded(message) => message.date,
            Message::ForumTopicCreated(message) => message.date,
            Message::ForumTopicEdited(message) => message.date,
            Message::ForumTopicClosed(message) => message.date,
            Message::ForumTopicReopened(message) => message.date,
            Message::GeneralForumTopicHidden(message) => message.date,
            Message::GeneralForumTopicUnhidden(message) => message.date,
            Message::VideoChatScheduled(message) => message.date,
            Message::VideoChatStarted(message) => message.date,
            Message::VideoChatEnded(message) => message.date,
            Message::VideoChatParticipantsInvited(message) => message.date,
            Message::WebAppData(message) => message.date,
            Message::GiveawayCreated(message) => message.date,
            Message::Giveaway(message) => message.date,
            Message::GiveawayWinners(message) => message.date,
            Message::GiveawayCompleted(message) => message.date,
        }
    }

    #[must_use]
    pub const fn chat(&self) -> &Chat {
        match self {
            Message::Text(message) => &message.chat,
            Message::Animation(message) => &message.chat,
            Message::Audio(message) => &message.chat,
            Message::Document(message) => &message.chat,
            Message::Photo(message) => &message.chat,
            Message::Sticker(message) => &message.chat,
            Message::Story(message) => &message.chat,
            Message::Video(message) => &message.chat,
            Message::VideoNote(message) => &message.chat,
            Message::Voice(message) => &message.chat,
            Message::Contact(message) => &message.chat,
            Message::Dice(message) => &message.chat,
            Message::Game(message) => &message.chat,
            Message::Poll(message) => &message.chat,
            Message::Venue(message) => &message.chat,
            Message::Location(message) => &message.chat,
            Message::NewChatMembers(message) => &message.chat,
            Message::LeftChatMember(message) => &message.chat,
            Message::NewChatTitle(message) => &message.chat,
            Message::NewChatPhoto(message) => &message.chat,
            Message::DeleteChatPhoto(message) => &message.chat,
            Message::GroupChatCreated(message) => &message.chat,
            Message::SupergroupChatCreated(message) => &message.chat,
            Message::ChannelChatCreated(message) => &message.chat,
            Message::MessageAutoDeleteTimerChanged(message) => &message.chat,
            Message::MigrateToChat(message) => &message.chat,
            Message::MigrateFromChat(message) => &message.chat,
            Message::Pinned(message) => &message.chat,
            Message::Invoice(message) => &message.chat,
            Message::SuccessfulPayment(message) => &message.chat,
            Message::UsersShared(message) => &message.chat,
            Message::ChatShared(message) => &message.chat,
            Message::ConnectedWebsite(message) => &message.chat,
            Message::WriteAccessAllowed(message) => &message.chat,
            Message::PassportData(message) => &message.chat,
            Message::ProximityAlertTriggered(message) => &message.chat,
            Message::ChatBoostAdded(message) => &message.chat,
            Message::ForumTopicCreated(message) => &message.chat,
            Message::ForumTopicEdited(message) => &message.chat,
            Message::ForumTopicClosed(message) => &message.chat,
            Message::ForumTopicReopened(message) => &message.chat,
            Message::GeneralForumTopicHidden(message) => &message.chat,
            Message::GeneralForumTopicUnhidden(message) => &message.chat,
            Message::VideoChatScheduled(message) => &message.chat,
            Message::VideoChatStarted(message) => &message.chat,
            Message::VideoChatEnded(message) => &message.chat,
            Message::VideoChatParticipantsInvited(message) => &message.chat,
            Message::WebAppData(message) => &message.chat,
            Message::GiveawayCreated(message) => &message.chat,
            Message::Giveaway(message) => &message.chat,
            Message::GiveawayWinners(message) => &message.chat,
            Message::GiveawayCompleted(message) => &message.chat,
        }
    }

    #[must_use]
    pub const fn via_bot(&self) -> Option<&User> {
        match self {
            Message::Text(message) => message.via_bot.as_ref(),
            Message::Animation(message) => message.via_bot.as_ref(),
            Message::Audio(message) => message.via_bot.as_ref(),
            Message::Document(message) => message.via_bot.as_ref(),
            Message::Photo(message) => message.via_bot.as_ref(),
            Message::Sticker(message) => message.via_bot.as_ref(),
            Message::Video(message) => message.via_bot.as_ref(),
            Message::Voice(message) => message.via_bot.as_ref(),
            Message::Contact(message) => message.via_bot.as_ref(),
            Message::Game(message) => message.via_bot.as_ref(),
            Message::Venue(message) => message.via_bot.as_ref(),
            Message::Location(message) => message.via_bot.as_ref(),
            Message::Invoice(message) => message.via_bot.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn text(&self) -> Option<&str> {
        match self {
            Message::Text(message) => Some(&message.text),
            _ => None,
        }
    }

    #[must_use]
    #[allow(clippy::match_as_ref)]
    pub const fn caption(&self) -> Option<&str> {
        match self {
            Message::Animation(message) => match message.caption {
                Some(ref caption) => Some(caption),
                None => None,
            },
            Message::Audio(message) => match message.caption {
                Some(ref caption) => Some(caption),
                None => None,
            },
            Message::Document(message) => match message.caption {
                Some(ref caption) => Some(caption),
                None => None,
            },
            Message::Video(message) => match message.caption {
                Some(ref caption) => Some(caption),
                None => None,
            },
            Message::Voice(message) => match message.caption {
                Some(ref caption) => Some(caption),
                None => None,
            },
            Message::Photo(message) => match message.caption {
                Some(ref caption) => Some(caption),
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
            Message::Text(message) => message.from.as_ref(),
            Message::Animation(message) => message.from.as_ref(),
            Message::Audio(message) => message.from.as_ref(),
            Message::Document(message) => message.from.as_ref(),
            Message::Photo(message) => message.from.as_ref(),
            Message::Sticker(message) => message.from.as_ref(),
            Message::Story(message) => message.from.as_ref(),
            Message::Video(message) => message.from.as_ref(),
            Message::VideoNote(message) => message.from.as_ref(),
            Message::Voice(message) => message.from.as_ref(),
            Message::Contact(message) => message.from.as_ref(),
            Message::Dice(message) => message.from.as_ref(),
            Message::Game(message) => message.from.as_ref(),
            Message::Poll(message) => message.from.as_ref(),
            Message::Venue(message) => message.from.as_ref(),
            Message::Location(message) => message.from.as_ref(),
            Message::NewChatMembers(message) => message.from.as_ref(),
            Message::LeftChatMember(message) => message.from.as_ref(),
            Message::NewChatTitle(message) => message.from.as_ref(),
            Message::NewChatPhoto(message) => message.from.as_ref(),
            Message::DeleteChatPhoto(message) => message.from.as_ref(),
            Message::GroupChatCreated(message) => message.from.as_ref(),
            Message::SupergroupChatCreated(message) => message.from.as_ref(),
            Message::ChannelChatCreated(message) => message.from.as_ref(),
            Message::MessageAutoDeleteTimerChanged(message) => message.from.as_ref(),
            Message::Pinned(message) => message.from.as_ref(),
            Message::Invoice(message) => message.from.as_ref(),
            Message::SuccessfulPayment(message) => message.from.as_ref(),
            Message::UsersShared(message) => message.from.as_ref(),
            Message::ChatShared(message) => message.from.as_ref(),
            Message::PassportData(message) => message.from.as_ref(),
            Message::ForumTopicCreated(message) => message.from.as_ref(),
            Message::ForumTopicEdited(message) => message.from.as_ref(),
            Message::ForumTopicClosed(message) => message.from.as_ref(),
            Message::ForumTopicReopened(message) => message.from.as_ref(),
            Message::GeneralForumTopicHidden(message) => message.from.as_ref(),
            Message::GeneralForumTopicUnhidden(message) => message.from.as_ref(),
            Message::VideoChatScheduled(message) => message.from.as_ref(),
            Message::VideoChatStarted(message) => message.from.as_ref(),
            Message::VideoChatEnded(message) => message.from.as_ref(),
            Message::VideoChatParticipantsInvited(message) => message.from.as_ref(),
            Message::GiveawayCreated(message) => message.from.as_ref(),
            Message::Giveaway(message) => message.from.as_ref(),
            Message::GiveawayWinners(message) => message.from.as_ref(),
            Message::GiveawayCompleted(message) => message.from.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn from_id(&self) -> Option<i64> {
        match self.from() {
            Some(user) => Some(user.id),
            None => None,
        }
    }

    #[must_use]
    pub const fn sender_boost_count(&self) -> Option<i64> {
        match self {
            Message::Text(message) => message.sender_boost_count,
            Message::Animation(message) => message.sender_boost_count,
            Message::Audio(message) => message.sender_boost_count,
            Message::Document(message) => message.sender_boost_count,
            Message::Photo(message) => message.sender_boost_count,
            Message::Sticker(message) => message.sender_boost_count,
            Message::Story(message) => message.sender_boost_count,
            Message::Video(message) => message.sender_boost_count,
            Message::VideoNote(message) => message.sender_boost_count,
            Message::Voice(message) => message.sender_boost_count,
            Message::Contact(message) => message.sender_boost_count,
            Message::Dice(message) => message.sender_boost_count,
            Message::Game(message) => message.sender_boost_count,
            Message::Poll(message) => message.sender_boost_count,
            Message::Venue(message) => message.sender_boost_count,
            Message::Location(message) => message.sender_boost_count,
            Message::Invoice(message) => message.sender_boost_count,
            _ => None,
        }
    }

    #[must_use]
    pub const fn sender_chat(&self) -> Option<&Chat> {
        match self {
            Message::Text(message) => message.sender_chat.as_ref(),
            Message::Animation(message) => message.sender_chat.as_ref(),
            Message::Audio(message) => message.sender_chat.as_ref(),
            Message::Document(message) => message.sender_chat.as_ref(),
            Message::Photo(message) => message.sender_chat.as_ref(),
            Message::Sticker(message) => message.sender_chat.as_ref(),
            Message::Story(message) => message.sender_chat.as_ref(),
            Message::Video(message) => message.sender_chat.as_ref(),
            Message::VideoNote(message) => message.sender_chat.as_ref(),
            Message::Voice(message) => message.sender_chat.as_ref(),
            Message::Contact(message) => message.sender_chat.as_ref(),
            Message::Dice(message) => message.sender_chat.as_ref(),
            Message::Game(message) => message.sender_chat.as_ref(),
            Message::Poll(message) => message.sender_chat.as_ref(),
            Message::Venue(message) => message.sender_chat.as_ref(),
            Message::Location(message) => message.sender_chat.as_ref(),
            Message::NewChatMembers(message) => message.sender_chat.as_ref(),
            Message::LeftChatMember(message) => message.sender_chat.as_ref(),
            Message::NewChatTitle(message) => message.sender_chat.as_ref(),
            Message::NewChatPhoto(message) => message.sender_chat.as_ref(),
            Message::DeleteChatPhoto(message) => message.sender_chat.as_ref(),
            Message::GroupChatCreated(message) => message.sender_chat.as_ref(),
            Message::SupergroupChatCreated(message) => message.sender_chat.as_ref(),
            Message::ChannelChatCreated(message) => message.sender_chat.as_ref(),
            Message::MessageAutoDeleteTimerChanged(message) => message.sender_chat.as_ref(),
            Message::MigrateToChat(message) => message.sender_chat.as_ref(),
            Message::MigrateFromChat(message) => message.sender_chat.as_ref(),
            Message::Pinned(message) => message.sender_chat.as_ref(),
            Message::Invoice(message) => message.sender_chat.as_ref(),
            Message::SuccessfulPayment(message) => message.sender_chat.as_ref(),
            Message::PassportData(message) => message.sender_chat.as_ref(),
            Message::ForumTopicCreated(message) => message.sender_chat.as_ref(),
            Message::ForumTopicEdited(message) => message.sender_chat.as_ref(),
            Message::ForumTopicClosed(message) => message.sender_chat.as_ref(),
            Message::ForumTopicReopened(message) => message.sender_chat.as_ref(),
            Message::GeneralForumTopicHidden(message) => message.sender_chat.as_ref(),
            Message::GeneralForumTopicUnhidden(message) => message.sender_chat.as_ref(),
            Message::VideoChatScheduled(message) => message.sender_chat.as_ref(),
            Message::VideoChatStarted(message) => message.sender_chat.as_ref(),
            Message::VideoChatEnded(message) => message.sender_chat.as_ref(),
            Message::VideoChatParticipantsInvited(message) => message.sender_chat.as_ref(),
            Message::GiveawayCreated(message) => message.sender_chat.as_ref(),
            Message::Giveaway(message) => message.sender_chat.as_ref(),
            Message::GiveawayWinners(message) => message.sender_chat.as_ref(),
            Message::GiveawayCompleted(message) => message.sender_chat.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn sender_chat_id(&self) -> Option<i64> {
        match self.sender_chat() {
            Some(chat) => Some(chat.id()),
            None => None,
        }
    }

    #[must_use]
    #[allow(clippy::match_as_ref)]
    pub const fn author_signature(&self) -> Option<&str> {
        match self {
            Message::Text(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Animation(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Audio(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Document(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Photo(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Sticker(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Story(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Video(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::VideoNote(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Voice(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Contact(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Dice(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Game(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Poll(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Venue(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Location(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::PassportData(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            Message::Invoice(message) => match message.author_signature {
                Some(ref author_signature) => Some(author_signature),
                None => None,
            },
            _ => None,
        }
    }

    #[must_use]
    pub const fn reply_to_message(&self) -> Option<&Message> {
        match self {
            Message::Text(message) => message.reply_to_message.as_ref(),
            Message::Animation(message) => message.reply_to_message.as_ref(),
            Message::Audio(message) => message.reply_to_message.as_ref(),
            Message::Document(message) => message.reply_to_message.as_ref(),
            Message::Photo(message) => message.reply_to_message.as_ref(),
            Message::Sticker(message) => message.reply_to_message.as_ref(),
            Message::Video(message) => message.reply_to_message.as_ref(),
            Message::VideoNote(message) => message.reply_to_message.as_ref(),
            Message::Voice(message) => message.reply_to_message.as_ref(),
            Message::Contact(message) => message.reply_to_message.as_ref(),
            Message::Dice(message) => message.reply_to_message.as_ref(),
            Message::Game(message) => message.reply_to_message.as_ref(),
            Message::Poll(message) => message.reply_to_message.as_ref(),
            Message::Venue(message) => message.reply_to_message.as_ref(),
            Message::Location(message) => message.reply_to_message.as_ref(),
            Message::Pinned(message) => message.reply_to_message.as_ref(),
            Message::Invoice(message) => message.reply_to_message.as_ref(),
            Message::ForumTopicCreated(message) => message.reply_to_message.as_ref(),
            Message::ForumTopicEdited(message) => message.reply_to_message.as_ref(),
            Message::ForumTopicClosed(message) => message.reply_to_message.as_ref(),
            Message::ForumTopicReopened(message) => message.reply_to_message.as_ref(),
            Message::GeneralForumTopicHidden(message) => message.reply_to_message.as_ref(),
            Message::GeneralForumTopicUnhidden(message) => message.reply_to_message.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn external_reply(&self) -> Option<&ExternalReplyInfo> {
        match self {
            Message::Text(message) => message.external_reply.as_ref(),
            Message::Animation(message) => message.external_reply.as_ref(),
            Message::Audio(message) => message.external_reply.as_ref(),
            Message::Document(message) => message.external_reply.as_ref(),
            Message::Photo(message) => message.external_reply.as_ref(),
            Message::Sticker(message) => message.external_reply.as_ref(),
            Message::Story(message) => message.external_reply.as_ref(),
            Message::Video(message) => message.external_reply.as_ref(),
            Message::VideoNote(message) => message.external_reply.as_ref(),
            Message::Voice(message) => message.external_reply.as_ref(),
            Message::Contact(message) => message.external_reply.as_ref(),
            Message::Dice(message) => message.external_reply.as_ref(),
            Message::Game(message) => message.external_reply.as_ref(),
            Message::Giveaway(message) => message.external_reply.as_ref(),
            Message::GiveawayWinners(message) => message.external_reply.as_ref(),
            Message::Poll(message) => message.external_reply.as_ref(),
            Message::Venue(message) => message.external_reply.as_ref(),
            Message::Location(message) => message.external_reply.as_ref(),
            Message::Invoice(message) => message.external_reply.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn quote(&self) -> Option<&TextQuote> {
        match self {
            Message::Text(message) => message.quote.as_ref(),
            Message::Animation(message) => message.quote.as_ref(),
            Message::Audio(message) => message.quote.as_ref(),
            Message::Document(message) => message.quote.as_ref(),
            Message::Video(message) => message.quote.as_ref(),
            Message::Voice(message) => message.quote.as_ref(),
            Message::Photo(message) => message.quote.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn edit_date(&self) -> Option<i64> {
        match self {
            Message::Text(message) => message.edit_date,
            Message::Animation(message) => message.edit_date,
            Message::Audio(message) => message.edit_date,
            Message::Document(message) => message.edit_date,
            Message::Photo(message) => message.edit_date,
            Message::Video(message) => message.edit_date,
            Message::Game(message) => message.edit_date,
            Message::Poll(message) => message.edit_date,
            Message::Venue(message) => message.edit_date,
            Message::Location(message) => message.edit_date,
            _ => None,
        }
    }

    #[must_use]
    pub const fn reply_markup(&self) -> Option<&InlineKeyboardMarkup> {
        match self {
            Message::Text(message) => message.reply_markup.as_ref(),
            Message::Animation(message) => message.reply_markup.as_ref(),
            Message::Audio(message) => message.reply_markup.as_ref(),
            Message::Document(message) => message.reply_markup.as_ref(),
            Message::Photo(message) => message.reply_markup.as_ref(),
            Message::Video(message) => message.reply_markup.as_ref(),
            Message::VideoNote(message) => message.reply_markup.as_ref(),
            Message::Voice(message) => message.reply_markup.as_ref(),
            Message::Contact(message) => message.reply_markup.as_ref(),
            Message::Dice(message) => message.reply_markup.as_ref(),
            Message::Game(message) => message.reply_markup.as_ref(),
            Message::Poll(message) => message.reply_markup.as_ref(),
            Message::Venue(message) => message.reply_markup.as_ref(),
            Message::Location(message) => message.reply_markup.as_ref(),
            Message::Invoice(message) => message.reply_markup.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_automatic_forward(&self) -> Option<bool> {
        match self {
            Message::Text(message) => message.is_automatic_forward,
            Message::Animation(message) => message.is_automatic_forward,
            Message::Audio(message) => message.is_automatic_forward,
            Message::Document(message) => message.is_automatic_forward,
            Message::Video(message) => message.is_automatic_forward,
            Message::Voice(message) => message.is_automatic_forward,
            Message::Photo(message) => message.is_automatic_forward,
            _ => None,
        }
    }

    #[must_use]
    pub const fn has_protected_content(&self) -> Option<bool> {
        match self {
            Message::Text(message) => message.has_protected_content,
            Message::Animation(message) => message.has_protected_content,
            Message::Audio(message) => message.has_protected_content,
            Message::Document(message) => message.has_protected_content,
            Message::Video(message) => message.has_protected_content,
            Message::Voice(message) => message.has_protected_content,
            Message::Photo(message) => message.has_protected_content,
            _ => None,
        }
    }

    #[must_use]
    pub const fn forward_origin(&self) -> Option<&MessageOrigin> {
        match self {
            Message::Text(message) => message.forward_origin.as_ref(),
            Message::Animation(message) => message.forward_origin.as_ref(),
            Message::Audio(message) => message.forward_origin.as_ref(),
            Message::Document(message) => message.forward_origin.as_ref(),
            Message::Photo(message) => message.forward_origin.as_ref(),
            Message::Sticker(message) => message.forward_origin.as_ref(),
            Message::Story(message) => message.forward_origin.as_ref(),
            Message::Video(message) => message.forward_origin.as_ref(),
            Message::VideoNote(message) => message.forward_origin.as_ref(),
            Message::Voice(message) => message.forward_origin.as_ref(),
            Message::Contact(message) => message.forward_origin.as_ref(),
            Message::Dice(message) => message.forward_origin.as_ref(),
            Message::Game(message) => message.forward_origin.as_ref(),
            Message::Poll(message) => message.forward_origin.as_ref(),
            Message::Venue(message) => message.forward_origin.as_ref(),
            Message::Location(message) => message.forward_origin.as_ref(),
            Message::Invoice(message) => message.forward_origin.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn animation(&self) -> Option<&types::Animation> {
        match self {
            Message::Animation(message) => Some(&message.animation),
            _ => None,
        }
    }

    #[must_use]
    pub const fn audio(&self) -> Option<&types::Audio> {
        match self {
            Message::Audio(message) => Some(&message.audio),
            _ => None,
        }
    }

    #[must_use]
    pub const fn contact(&self) -> Option<&types::Contact> {
        match self {
            Message::Contact(message) => Some(&message.contact),
            _ => None,
        }
    }

    #[must_use]
    pub const fn dice(&self) -> Option<&types::Dice> {
        match self {
            Message::Dice(message) => Some(&message.dice),
            _ => None,
        }
    }

    #[must_use]
    pub const fn document(&self) -> Option<&types::Document> {
        match self {
            Message::Document(message) => Some(&message.document),
            _ => None,
        }
    }

    #[must_use]
    pub const fn game(&self) -> Option<&types::Game> {
        match self {
            Message::Game(message) => Some(&message.game),
            _ => None,
        }
    }

    #[must_use]
    pub const fn poll(&self) -> Option<&types::Poll> {
        match self {
            Message::Poll(message) => Some(&message.poll),
            _ => None,
        }
    }

    #[must_use]
    pub const fn venue(&self) -> Option<&types::Venue> {
        match self {
            Message::Venue(message) => Some(&message.venue),
            _ => None,
        }
    }

    #[must_use]
    pub const fn location(&self) -> Option<&types::Location> {
        match self {
            Message::Location(message) => Some(&message.location),
            _ => None,
        }
    }

    #[must_use]
    pub const fn new_chat_members(&self) -> Option<&[User]> {
        match self {
            Message::NewChatMembers(message) => Some(&message.members),
            _ => None,
        }
    }

    #[must_use]
    pub const fn left_chat_member(&self) -> Option<&User> {
        match self {
            Message::LeftChatMember(message) => Some(&message.member),
            _ => None,
        }
    }

    #[must_use]
    pub const fn new_chat_title(&self) -> Option<&str> {
        match self {
            Message::NewChatTitle(message) => Some(&message.title),
            _ => None,
        }
    }

    #[must_use]
    pub const fn new_chat_photo(&self) -> Option<&[PhotoSize]> {
        match self {
            Message::NewChatPhoto(message) => Some(&message.photo),
            _ => None,
        }
    }

    #[must_use]
    pub const fn delete_chat_photo(&self) -> Option<bool> {
        match self {
            Message::DeleteChatPhoto(message) => Some(message.photo),
            _ => None,
        }
    }

    #[must_use]
    pub const fn group_chat_created(&self) -> Option<bool> {
        match self {
            Message::GroupChatCreated(message) => Some(message.created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn supergroup_chat_created(&self) -> Option<bool> {
        match self {
            Message::SupergroupChatCreated(message) => Some(message.created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn channel_chat_created(&self) -> Option<bool> {
        match self {
            Message::ChannelChatCreated(message) => Some(message.created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn message_auto_delete_timer_changed(
        &self,
    ) -> Option<&types::MessageAutoDeleteTimerChanged> {
        match self {
            Message::MessageAutoDeleteTimerChanged(message) => Some(&message.timer),
            _ => None,
        }
    }

    #[must_use]
    pub const fn pinned(&self) -> Option<&MaybeInaccessibleMessage> {
        match self {
            Message::Pinned(message) => Some(&message.message),
            _ => None,
        }
    }

    #[must_use]
    pub const fn invoice(&self) -> Option<&types::Invoice> {
        match self {
            Message::Invoice(message) => Some(&message.invoice),
            _ => None,
        }
    }

    #[must_use]
    pub const fn successful_payment(&self) -> Option<&types::SuccessfulPayment> {
        match self {
            Message::SuccessfulPayment(message) => Some(&message.payment),
            _ => None,
        }
    }

    #[must_use]
    pub const fn users_shared(&self) -> Option<&types::UsersShared> {
        match self {
            Message::UsersShared(message) => Some(&message.shared),
            _ => None,
        }
    }

    #[must_use]
    pub const fn chat_shared(&self) -> Option<&types::ChatShared> {
        match self {
            Message::ChatShared(message) => Some(&message.shared),
            _ => None,
        }
    }

    #[must_use]
    pub const fn connected_website(&self) -> Option<&str> {
        match self {
            Message::ConnectedWebsite(message) => Some(&message.website),
            _ => None,
        }
    }

    #[must_use]
    pub const fn write_access_allowed(&self) -> Option<&types::WriteAccessAllowed> {
        match self {
            Message::WriteAccessAllowed(message) => Some(&message.allowed),
            _ => None,
        }
    }

    #[must_use]
    pub const fn passport_data(&self) -> Option<&types::PassportData> {
        match self {
            Message::PassportData(message) => Some(&message.data),
            _ => None,
        }
    }

    #[must_use]
    pub const fn proximity_alert_triggered(&self) -> Option<&types::ProximityAlertTriggered> {
        match self {
            Message::ProximityAlertTriggered(message) => Some(&message.triggered),
            _ => None,
        }
    }

    #[must_use]
    pub const fn chat_boost_added(&self) -> Option<&types::ChatBoostAdded> {
        match self {
            Message::ChatBoostAdded(message) => Some(&message.added),
            _ => None,
        }
    }

    #[must_use]
    pub const fn forum_topic_created(&self) -> Option<&types::ForumTopicCreated> {
        match self {
            Message::ForumTopicCreated(message) => Some(&message.created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn forum_topic_edited(&self) -> Option<&types::ForumTopicEdited> {
        match self {
            Message::ForumTopicEdited(message) => Some(&message.edited),
            _ => None,
        }
    }

    #[must_use]
    pub const fn forum_topic_closed(&self) -> Option<&types::ForumTopicClosed> {
        match self {
            Message::ForumTopicClosed(message) => Some(&message.closed),
            _ => None,
        }
    }

    #[must_use]
    pub const fn forum_topic_reopened(&self) -> Option<&types::ForumTopicReopened> {
        match self {
            Message::ForumTopicReopened(message) => Some(&message.reopened),
            _ => None,
        }
    }

    #[must_use]
    pub const fn general_forum_topic_hidden(&self) -> Option<&types::GeneralForumTopicHidden> {
        match self {
            Message::GeneralForumTopicHidden(message) => Some(&message.hidden),
            _ => None,
        }
    }

    #[must_use]
    pub const fn general_forum_topic_unhidden(&self) -> Option<&types::GeneralForumTopicUnhidden> {
        match self {
            Message::GeneralForumTopicUnhidden(message) => Some(&message.unhidden),
            _ => None,
        }
    }

    #[must_use]
    pub const fn giveaway_created(&self) -> Option<&types::GiveawayCreated> {
        match self {
            Message::GiveawayCreated(message) => Some(&message.created),
            _ => None,
        }
    }

    #[must_use]
    pub const fn giveaway(&self) -> Option<&types::Giveaway> {
        match self {
            Message::Giveaway(message) => Some(&message.giveaway),
            _ => None,
        }
    }

    #[must_use]
    pub const fn giveaway_winners(&self) -> Option<&types::GiveawayWinners> {
        match self {
            Message::GiveawayWinners(message) => Some(&message.winners),
            _ => None,
        }
    }

    #[must_use]
    pub const fn giveaway_completed(&self) -> Option<&types::GiveawayCompleted> {
        match self {
            Message::GiveawayCompleted(message) => Some(&message.completed),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_chat_scheduled(&self) -> Option<&types::VideoChatScheduled> {
        match self {
            Message::VideoChatScheduled(message) => Some(&message.scheduled),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_chat_started(&self) -> Option<&types::VideoChatStarted> {
        match self {
            Message::VideoChatStarted(message) => Some(&message.started),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_chat_ended(&self) -> Option<&types::VideoChatEnded> {
        match self {
            Message::VideoChatEnded(message) => Some(&message.ended),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_chat_participants_invited(
        &self,
    ) -> Option<&types::VideoChatParticipantsInvited> {
        match self {
            Message::VideoChatParticipantsInvited(message) => Some(&message.invited),
            _ => None,
        }
    }

    #[must_use]
    pub const fn web_app_data(&self) -> Option<&types::WebAppData> {
        match self {
            Message::WebAppData(message) => Some(&message.data),
            _ => None,
        }
    }

    #[must_use]
    pub const fn photo(&self) -> Option<&[PhotoSize]> {
        match self {
            Message::Photo(message) => Some(&message.photo),
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
            Message::Story(message) => Some(&message.story),
            _ => None,
        }
    }

    #[must_use]
    pub const fn sticker(&self) -> Option<&types::Sticker> {
        match self {
            Message::Sticker(message) => Some(&message.sticker),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video(&self) -> Option<&types::Video> {
        match self {
            Message::Video(message) => Some(&message.video),
            _ => None,
        }
    }

    #[must_use]
    pub const fn video_note(&self) -> Option<&types::VideoNote> {
        match self {
            Message::VideoNote(message) => Some(&message.video_note),
            _ => None,
        }
    }

    #[must_use]
    pub const fn voice(&self) -> Option<&types::Voice> {
        match self {
            Message::Voice(message) => Some(&message.voice),
            _ => None,
        }
    }

    #[must_use]
    pub const fn migrate_to_chat_id(&self) -> Option<i64> {
        match self {
            Message::MigrateToChat(message) => Some(message.to_chat_id),
            _ => None,
        }
    }

    #[must_use]
    pub const fn migrate_from_chat_id(&self) -> Option<i64> {
        match self {
            Message::MigrateFromChat(message) => Some(message.from_chat_id),
            _ => None,
        }
    }
}

impl Default for Message {
    #[must_use]
    fn default() -> Self {
        Message::Text(Box::default())
    }
}

macro_rules! impl_try_from_message {
    ($variant:ident, $ty:ty) => {
        impl TryFrom<Message> for $ty {
            type Error = ConvertToTypeError;

            fn try_from(value: Message) -> Result<Self, Self::Error> {
                if let Message::$variant(val) = value {
                    Ok(*val)
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
impl_try_from_message!(ChatBoostAdded, ChatBoostAdded);
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
impl_try_from_update!(ChatBoostAdded);
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
                Message::Text(message) => assert_eq!(*message, message_text),
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
                Message::Text(message) => assert_eq!(*message, message_kind),
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
                Message::Animation(message) => assert_eq!(*message, message_kind),
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
                Message::Audio(message) => assert_eq!(*message, message_kind),
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
                Message::Document(message) => assert_eq!(*message, message_kind),
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
                Message::Photo(message) => assert_eq!(*message, message_kind),
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
                Message::Sticker(message) => assert_eq!(*message, message_kind),
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
            "story": {
                "chat": {
                    "id": -1,
                    "title": "test",
                    "type": "channel",
                },
                "id": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::Story(message) => assert_eq!(*message, message_kind),
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
                Message::Video(message) => assert_eq!(*message, message_kind),
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
                Message::VideoNote(message) => assert_eq!(*message, message_kind),
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
                Message::Voice(message) => assert_eq!(*message, message_kind),
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
                Message::Contact(message) => assert_eq!(*message, message_kind),
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
                Message::Dice(message) => assert_eq!(*message, message_kind),
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
                Message::Game(message) => assert_eq!(*message, message_kind),
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
                Message::Poll(message) => assert_eq!(*message, message_kind),
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
                Message::Venue(message) => assert_eq!(*message, message_kind),
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
                Message::Location(message) => assert_eq!(*message, message_kind),
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
                Message::NewChatMembers(message) => assert_eq!(*message, message_kind),
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
                Message::LeftChatMember(message) => assert_eq!(*message, message_kind),
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
                Message::NewChatTitle(message) => assert_eq!(*message, message_kind),
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
                Message::NewChatPhoto(message) => assert_eq!(*message, message_kind),
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
                Message::DeleteChatPhoto(message) => assert_eq!(*message, message_kind),
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
                Message::Pinned(message) => assert_eq!(*message, message_kind),
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
                Message::Invoice(message) => assert_eq!(*message, message_kind),
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
                Message::UsersShared(message) => assert_eq!(*message, message_kind),
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
                Message::ChatShared(message) => assert_eq!(*message, message_kind),
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
                Message::PassportData(message) => assert_eq!(*message, message_kind),
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
    fn deserialize_chat_boost_added() {
        let jsons = [serde_json::json!({
            "message_id": 1,
            "date": 0,
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
            "boost_added": {
                "boost_count": 1,
            },
        })];

        for json in jsons {
            let message_kind = serde_json::from_value(json.clone()).unwrap();
            let message: Message = serde_json::from_value(json).unwrap();

            match message {
                Message::ChatBoostAdded(message) => {
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
                Message::Giveaway(message) => assert_eq!(*message, message_kind),
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
                Message::WebAppData(message) => assert_eq!(*message, message_kind),
                _ => panic!("Unexpected message type: {message:?}"),
            }
        }
    }
}
