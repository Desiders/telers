use crate::types::Message;

use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the content of the message
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum ContentType {
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "story")]
    Story,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "video_note")]
    VideoNote,
    #[strum(serialize = "voice")]
    Voice,
    #[strum(serialize = "contact")]
    Contact,
    #[strum(serialize = "dice")]
    Dice,
    #[strum(serialize = "game")]
    Game,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "new_chat_members")]
    NewChatMembers,
    #[strum(serialize = "left_chat_member")]
    LeftChatMember,
    #[strum(serialize = "new_chat_title")]
    NewChatTitle,
    #[strum(serialize = "new_chat_photo")]
    NewChatPhoto,
    #[strum(serialize = "delete_chat_photo")]
    DeleteChatPhoto,
    #[strum(serialize = "group_chat_created")]
    GroupChatCreated,
    #[strum(serialize = "supergroup_chat_created")]
    SupergroupChatCreated,
    #[strum(serialize = "channel_chat_created")]
    ChannelChatCreated,
    #[strum(serialize = "message_auto_delete_timer_changed")]
    MessageAutoDeleteTimerChanged,
    #[strum(serialize = "migrate_to_chat_id")]
    MigrateToChatId,
    #[strum(serialize = "migrate_from_chat_id")]
    MigrateFromChatId,
    #[strum(serialize = "pinned_message")]
    PinnedMessage,
    #[strum(serialize = "invoice")]
    Invoice,
    #[strum(serialize = "successful_payment")]
    SuccessfulPayment,
    #[strum(serialize = "user_shared")]
    UserShared,
    #[strum(serialize = "chat_shared")]
    ChatShared,
    #[strum(serialize = "connected_website")]
    ConnectedWebsite,
    #[strum(serialize = "write_access_allowed")]
    WriteAccessAllowed,
    #[strum(serialize = "passport_data")]
    PassportData,
    #[strum(serialize = "proximity_alert_triggered")]
    ProximityAlertTriggered,
    #[strum(serialize = "forum_topic_created")]
    ForumTopicCreated,
    #[strum(serialize = "forum_topic_edited")]
    ForumTopicEdited,
    #[strum(serialize = "forum_topic_closed")]
    ForumTopicClosed,
    #[strum(serialize = "forum_topic_reopened")]
    ForumTopicReopened,
    #[strum(serialize = "general_forum_topic_hidden")]
    GeneralForumTopicHidden,
    #[strum(serialize = "general_forum_topic_unhidden")]
    GeneralForumTopicUnhidden,
    #[strum(serialize = "giveaway_created")]
    GiveawayCreated,
    #[strum(serialize = "giveaway")]
    Giveaway,
    #[strum(serialize = "giveaway_winners")]
    GiveawayWinners,
    #[strum(serialize = "giveaway_completed")]
    GiveawayCompleted,
    #[strum(serialize = "video_chat_scheduled")]
    VideoChatScheduled,
    #[strum(serialize = "video_chat_started")]
    VideoChatStarted,
    #[strum(serialize = "video_chat_ended")]
    VideoChatEnded,
    #[strum(serialize = "video_chat_participants_invited")]
    VideoChatParticipantsInvited,
    #[strum(serialize = "web_app_data")]
    WebAppData,
    #[strum(serialize = "empty")]
    Empty,
}

impl ContentType {
    #[must_use]
    pub const fn all() -> [ContentType; 52] {
        [
            ContentType::Text,
            ContentType::Animation,
            ContentType::Audio,
            ContentType::Document,
            ContentType::Photo,
            ContentType::Sticker,
            ContentType::Story,
            ContentType::Video,
            ContentType::VideoNote,
            ContentType::Voice,
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
            ContentType::GiveawayCreated,
            ContentType::Giveaway,
            ContentType::GiveawayWinners,
            ContentType::GiveawayCompleted,
            ContentType::VideoChatScheduled,
            ContentType::VideoChatStarted,
            ContentType::VideoChatEnded,
            ContentType::VideoChatParticipantsInvited,
            ContentType::WebAppData,
            ContentType::Empty,
        ]
    }
}

impl From<ContentType> for Box<str> {
    fn from(content_type: ContentType) -> Self {
        Into::<&'static str>::into(content_type).into()
    }
}

impl From<ContentType> for String {
    fn from(content_type: ContentType) -> Self {
        content_type.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for ContentType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}

impl From<&Message> for ContentType {
    fn from(message: &Message) -> Self {
        match message {
            Message::Text(_) => ContentType::Text,
            Message::Animation(_) => ContentType::Animation,
            Message::Audio(_) => ContentType::Audio,
            Message::Document(_) => ContentType::Document,
            Message::Photo(_) => ContentType::Photo,
            Message::Sticker(_) => ContentType::Sticker,
            Message::Story(_) => ContentType::Story,
            Message::Video(_) => ContentType::Video,
            Message::VideoNote(_) => ContentType::VideoNote,
            Message::Voice(_) => ContentType::Voice,
            Message::Contact(_) => ContentType::Contact,
            Message::Dice(_) => ContentType::Dice,
            Message::Game(_) => ContentType::Game,
            Message::Poll(_) => ContentType::Poll,
            Message::Venue(_) => ContentType::Venue,
            Message::Location(_) => ContentType::Location,
            Message::NewChatMembers(_) => ContentType::NewChatMembers,
            Message::LeftChatMember(_) => ContentType::LeftChatMember,
            Message::NewChatTitle(_) => ContentType::NewChatTitle,
            Message::NewChatPhoto(_) => ContentType::NewChatPhoto,
            Message::DeleteChatPhoto(_) => ContentType::DeleteChatPhoto,
            Message::GroupChatCreated(_) => ContentType::GroupChatCreated,
            Message::SupergroupChatCreated(_) => ContentType::SupergroupChatCreated,
            Message::ChannelChatCreated(_) => ContentType::ChannelChatCreated,
            Message::MessageAutoDeleteTimerChanged(_) => ContentType::MessageAutoDeleteTimerChanged,
            Message::MigrateToChat(_) => ContentType::MigrateToChatId,
            Message::MigrateFromChat(_) => ContentType::MigrateFromChatId,
            Message::Pinned(_) => ContentType::PinnedMessage,
            Message::Invoice(_) => ContentType::Invoice,
            Message::SuccessfulPayment(_) => ContentType::SuccessfulPayment,
            Message::UsersShared(_) => ContentType::UserShared,
            Message::ChatShared(_) => ContentType::ChatShared,
            Message::ConnectedWebsite(_) => ContentType::ConnectedWebsite,
            Message::WriteAccessAllowed(_) => ContentType::WriteAccessAllowed,
            Message::PassportData(_) => ContentType::PassportData,
            Message::ProximityAlertTriggered(_) => ContentType::ProximityAlertTriggered,
            Message::ForumTopicCreated(_) => ContentType::ForumTopicCreated,
            Message::ForumTopicEdited(_) => ContentType::ForumTopicEdited,
            Message::ForumTopicClosed(_) => ContentType::ForumTopicClosed,
            Message::ForumTopicReopened(_) => ContentType::ForumTopicReopened,
            Message::GeneralForumTopicHidden(_) => ContentType::GeneralForumTopicHidden,
            Message::GeneralForumTopicUnhidden(_) => ContentType::GeneralForumTopicUnhidden,
            Message::GiveawayCreated(_) => ContentType::GiveawayCreated,
            Message::Giveaway(_) => ContentType::Giveaway,
            Message::GiveawayWinners(_) => ContentType::GiveawayWinners,
            Message::GiveawayCompleted(_) => ContentType::GiveawayCompleted,
            Message::VideoChatScheduled(_) => ContentType::VideoChatScheduled,
            Message::VideoChatStarted(_) => ContentType::VideoChatStarted,
            Message::VideoChatEnded(_) => ContentType::VideoChatEnded,
            Message::VideoChatParticipantsInvited(_) => ContentType::VideoChatParticipantsInvited,
            Message::WebAppData(_) => ContentType::WebAppData,
            Message::Empty(_) => ContentType::Empty,
        }
    }
}
