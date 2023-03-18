use std::fmt::{self, Debug};

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
    VideoChatSheduled,
    VoiceChatStarted,
    VoiceChatEnded,
    VoiceChatParticipantsInvited,
    WebAppData,
    Unknown,
}

impl Debug for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ContentType {
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
            ContentType::VideoChatSheduled => "video_chat_sheduled",
            ContentType::VoiceChatStarted => "voice_chat_started",
            ContentType::VoiceChatEnded => "voice_chat_ended",
            ContentType::VoiceChatParticipantsInvited => "voice_chat_participants_invited",
            ContentType::WebAppData => "web_app_data",
            ContentType::Unknown => "unknown",
        }
    }
}

impl From<ContentType> for String {
    fn from(content_type: ContentType) -> Self {
        content_type.as_str().to_string()
    }
}

impl<'a> From<&'a ContentType> for String {
    fn from(content_type: &'a ContentType) -> Self {
        content_type.as_str().to_string()
    }
}
