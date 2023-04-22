use std::fmt::{self, Debug};

/// This enum represents all possible types of the content of the message
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum ContentType {
    Text,
    Animation,
    Audio,
    Document,
    Photo,
    Sticker,
    Video,
    VideoNote,
    Voice,
    HasMediaSpoiler,
    Contact,
    Dice,
    Game,
    Poll,
    Venue,
    Location,
    NewChatMembers,
    LeftChatMember,
    NewChatTitle,
    NewChatPhoto,
    DeleteChatPhoto,
    GroupChatCreated,
    SupergroupChatCreated,
    ChannelChatCreated,
    MessageAutoDeleteTimerChanged,
    MigrateToChatId,
    MigrateFromChatId,
    PinnedMessage,
    Invoice,
    SuccessfulPayment,
    UserShared,
    ChatShared,
    ConnectedWebsite,
    WriteAccessAllowed,
    PassportData,
    ProximityAlertTriggered,
    ForumTopicCreated,
    ForumTopicEdited,
    ForumTopicClosed,
    ForumTopicReopened,
    GeneralForumTopicHidden,
    GeneralForumTopicUnhidden,
    VideoChatScheduled,
    VideoChatStarted,
    VideoChatEnded,
    VideoChatParticipantsInvited,
    WebAppData,
    Unknown,
}

impl Debug for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ContentType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            ContentType::Text => "text",
            ContentType::Animation => "animation",
            ContentType::Audio => "audio",
            ContentType::Document => "document",
            ContentType::Photo => "photo",
            ContentType::Sticker => "sticker",
            ContentType::Video => "video",
            ContentType::VideoNote => "video_note",
            ContentType::Voice => "voice",
            ContentType::HasMediaSpoiler => "has_media_spoiler",
            ContentType::Contact => "contact",
            ContentType::Dice => "dice",
            ContentType::Game => "game",
            ContentType::Poll => "poll",
            ContentType::Venue => "venue",
            ContentType::Location => "location",
            ContentType::NewChatMembers => "new_chat_members",
            ContentType::LeftChatMember => "left_chat_member",
            ContentType::NewChatTitle => "new_chat_title",
            ContentType::NewChatPhoto => "new_chat_photo",
            ContentType::DeleteChatPhoto => "delete_chat_photo",
            ContentType::GroupChatCreated => "group_chat_created",
            ContentType::SupergroupChatCreated => "supergroup_chat_created",
            ContentType::ChannelChatCreated => "channel_chat_created",
            ContentType::MessageAutoDeleteTimerChanged => "message_auto_delete_timer_changed",
            ContentType::MigrateToChatId => "migrate_to_chat_id",
            ContentType::MigrateFromChatId => "migrate_from_chat_id",
            ContentType::PinnedMessage => "pinned_message",
            ContentType::Invoice => "invoice",
            ContentType::SuccessfulPayment => "successful_payment",
            ContentType::UserShared => "user_shared",
            ContentType::ChatShared => "chat_shared",
            ContentType::ConnectedWebsite => "connected_website",
            ContentType::WriteAccessAllowed => "write_access_allowed",
            ContentType::PassportData => "passport_data",
            ContentType::ProximityAlertTriggered => "proximity_alert_triggered",
            ContentType::ForumTopicCreated => "forum_topic_created",
            ContentType::ForumTopicEdited => "forum_topic_edited",
            ContentType::ForumTopicClosed => "forum_topic_closed",
            ContentType::ForumTopicReopened => "forum_topic_reopened",
            ContentType::GeneralForumTopicHidden => "general_forum_topic_hidden",
            ContentType::GeneralForumTopicUnhidden => "general_forum_topic_unhidden",
            ContentType::VideoChatScheduled => "video_chat_scheduled",
            ContentType::VideoChatStarted => "video_chat_started",
            ContentType::VideoChatEnded => "video_chat_ended",
            ContentType::VideoChatParticipantsInvited => "video_chat_participants_invited",
            ContentType::WebAppData => "web_app_data",
            ContentType::Unknown => "unknown",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [ContentType; 48] {
        &[
            ContentType::Text,
            ContentType::Animation,
            ContentType::Audio,
            ContentType::Document,
            ContentType::Photo,
            ContentType::Sticker,
            ContentType::Video,
            ContentType::VideoNote,
            ContentType::Voice,
            ContentType::HasMediaSpoiler,
            ContentType::Contact,
            ContentType::Dice,
            ContentType::Game,
            ContentType::Poll,
            ContentType::Venue,
            ContentType::Location,
            ContentType::NewChatMembers,
            ContentType::LeftChatMember,
            ContentType::NewChatTitle,
            ContentType::NewChatPhoto,
            ContentType::DeleteChatPhoto,
            ContentType::GroupChatCreated,
            ContentType::SupergroupChatCreated,
            ContentType::ChannelChatCreated,
            ContentType::MessageAutoDeleteTimerChanged,
            ContentType::MigrateToChatId,
            ContentType::MigrateFromChatId,
            ContentType::PinnedMessage,
            ContentType::Invoice,
            ContentType::SuccessfulPayment,
            ContentType::UserShared,
            ContentType::ChatShared,
            ContentType::ConnectedWebsite,
            ContentType::WriteAccessAllowed,
            ContentType::PassportData,
            ContentType::ProximityAlertTriggered,
            ContentType::ForumTopicCreated,
            ContentType::ForumTopicEdited,
            ContentType::ForumTopicClosed,
            ContentType::ForumTopicReopened,
            ContentType::GeneralForumTopicHidden,
            ContentType::GeneralForumTopicUnhidden,
            ContentType::VideoChatScheduled,
            ContentType::VideoChatStarted,
            ContentType::VideoChatEnded,
            ContentType::VideoChatParticipantsInvited,
            ContentType::WebAppData,
            ContentType::Unknown,
        ]
    }
}

impl<'a> PartialEq<&'a str> for ContentType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_str() == *other
    }
}

impl From<ContentType> for String {
    fn from(content_type: ContentType) -> Self {
        content_type.as_str().to_string()
    }
}

/// Never panics, because unknown content types are represented as [`ContentType::Unknown`].
impl<'a> From<&'a str> for ContentType {
    fn from(content_type: &'a str) -> Self {
        match content_type {
            "text" => ContentType::Text,
            "animation" => ContentType::Animation,
            "audio" => ContentType::Audio,
            "document" => ContentType::Document,
            "photo" => ContentType::Photo,
            "sticker" => ContentType::Sticker,
            "video" => ContentType::Video,
            "video_note" => ContentType::VideoNote,
            "voice" => ContentType::Voice,
            "has_media_spoiler" => ContentType::HasMediaSpoiler,
            "contact" => ContentType::Contact,
            "dice" => ContentType::Dice,
            "game" => ContentType::Game,
            "poll" => ContentType::Poll,
            "venue" => ContentType::Venue,
            "location" => ContentType::Location,
            "new_chat_members" => ContentType::NewChatMembers,
            "left_chat_member" => ContentType::LeftChatMember,
            "new_chat_title" => ContentType::NewChatTitle,
            "new_chat_photo" => ContentType::NewChatPhoto,
            "delete_chat_photo" => ContentType::DeleteChatPhoto,
            "group_chat_created" => ContentType::GroupChatCreated,
            "supergroup_chat_created" => ContentType::SupergroupChatCreated,
            "channel_chat_created" => ContentType::ChannelChatCreated,
            "message_auto_delete_timer_changed" => ContentType::MessageAutoDeleteTimerChanged,
            "migrate_to_chat_id" => ContentType::MigrateToChatId,
            "migrate_from_chat_id" => ContentType::MigrateFromChatId,
            "pinned_message" => ContentType::PinnedMessage,
            "invoice" => ContentType::Invoice,
            "successful_payment" => ContentType::SuccessfulPayment,
            "user_shared" => ContentType::UserShared,
            "chat_shared" => ContentType::ChatShared,
            "connected_website" => ContentType::ConnectedWebsite,
            "write_access_allowed" => ContentType::WriteAccessAllowed,
            "passport_data" => ContentType::PassportData,
            "proximity_alert_triggered" => ContentType::ProximityAlertTriggered,
            "forum_topic_created" => ContentType::ForumTopicCreated,
            "forum_topic_edited" => ContentType::ForumTopicEdited,
            "forum_topic_closed" => ContentType::ForumTopicClosed,
            "forum_topic_reopened" => ContentType::ForumTopicReopened,
            "general_forum_topic_hidden" => ContentType::GeneralForumTopicHidden,
            "general_forum_topic_unhidden" => ContentType::GeneralForumTopicUnhidden,
            "video_chat_scheduled" => ContentType::VideoChatScheduled,
            "video_chat_started" => ContentType::VideoChatStarted,
            "video_chat_ended" => ContentType::VideoChatEnded,
            "video_chat_participants_invited" => ContentType::VideoChatParticipantsInvited,
            "web_app_data" => ContentType::WebAppData,
            _ => ContentType::Unknown,
        }
    }
}
