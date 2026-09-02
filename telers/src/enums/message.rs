use crate::types::Message;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents a message.
/// Currently, it can be one of
/// - [`crate::types::MessageAnimation`]
/// - [`crate::types::MessageAudio`]
/// - [`crate::types::MessageBoostAdded`]
/// - [`crate::types::MessageChannelChatCreated`]
/// - [`crate::types::MessageChatBackgroundSet`]
/// - [`crate::types::MessageChatOwnerChanged`]
/// - [`crate::types::MessageChatOwnerLeft`]
/// - [`crate::types::MessageChatShared`]
/// - [`crate::types::MessageChecklist`]
/// - [`crate::types::MessageChecklistTasksAdded`]
/// - [`crate::types::MessageChecklistTasksDone`]
/// - [`crate::types::MessageCommunityChatAdded`]
/// - [`crate::types::MessageCommunityChatJoined`]
/// - [`crate::types::MessageCommunityChatRemoved`]
/// - [`crate::types::MessageConnectedWebsite`]
/// - [`crate::types::MessageContact`]
/// - [`crate::types::MessageDeleteChatPhoto`]
/// - [`crate::types::MessageDice`]
/// - [`crate::types::MessageDirectMessagePriceChanged`]
/// - [`crate::types::MessageDocument`]
/// - [`crate::types::MessageForumTopicClosed`]
/// - [`crate::types::MessageForumTopicCreated`]
/// - [`crate::types::MessageForumTopicEdited`]
/// - [`crate::types::MessageForumTopicReopened`]
/// - [`crate::types::MessageGame`]
/// - [`crate::types::MessageGeneralForumTopicHidden`]
/// - [`crate::types::MessageGeneralForumTopicUnhidden`]
/// - [`crate::types::MessageGift`]
/// - [`crate::types::MessageGiftUpgradeSent`]
/// - [`crate::types::MessageGiveaway`]
/// - [`crate::types::MessageGiveawayCompleted`]
/// - [`crate::types::MessageGiveawayCreated`]
/// - [`crate::types::MessageGiveawayWinners`]
/// - [`crate::types::MessageGroupChatCreated`]
/// - [`crate::types::MessageInvoice`]
/// - [`crate::types::MessageLeftChatMember`]
/// - [`crate::types::MessageLivePhoto`]
/// - [`crate::types::MessageLocation`]
/// - [`crate::types::MessageManagedBotCreated`]
/// - [`crate::types::MessageMessageAutoDeleteTimerChanged`]
/// - [`crate::types::MessageMigrateFromChatId`]
/// - [`crate::types::MessageMigrateToChatId`]
/// - [`crate::types::MessageNewChatMembers`]
/// - [`crate::types::MessageNewChatPhoto`]
/// - [`crate::types::MessageNewChatTitle`]
/// - [`crate::types::MessagePaidMedia`]
/// - [`crate::types::MessagePaidMessagePriceChanged`]
/// - [`crate::types::MessagePassportData`]
/// - [`crate::types::MessagePhoto`]
/// - [`crate::types::MessagePinnedMessage`]
/// - [`crate::types::MessagePoll`]
/// - [`crate::types::MessagePollOptionAdded`]
/// - [`crate::types::MessagePollOptionDeleted`]
/// - [`crate::types::MessageProximityAlertTriggered`]
/// - [`crate::types::MessageRefundedPayment`]
/// - [`crate::types::MessageRichMessage`]
/// - [`crate::types::MessageSticker`]
/// - [`crate::types::MessageStory`]
/// - [`crate::types::MessageSuccessfulPayment`]
/// - [`crate::types::MessageSuggestedPostApprovalFailed`]
/// - [`crate::types::MessageSuggestedPostApproved`]
/// - [`crate::types::MessageSuggestedPostDeclined`]
/// - [`crate::types::MessageSuggestedPostPaid`]
/// - [`crate::types::MessageSuggestedPostRefunded`]
/// - [`crate::types::MessageSupergroupChatCreated`]
/// - [`crate::types::MessageText`]
/// - [`crate::types::MessageUniqueGift`]
/// - [`crate::types::MessageUsersShared`]
/// - [`crate::types::MessageVenue`]
/// - [`crate::types::MessageVideo`]
/// - [`crate::types::MessageVideoChatEnded`]
/// - [`crate::types::MessageVideoChatParticipantsInvited`]
/// - [`crate::types::MessageVideoChatScheduled`]
/// - [`crate::types::MessageVideoChatStarted`]
/// - [`crate::types::MessageVideoNote`]
/// - [`crate::types::MessageVoice`]
/// - [`crate::types::MessageWebAppData`]
/// - [`crate::types::MessageWriteAccessAllowed`]
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum MessageType {
    #[strum(serialize = "animation")]
    Animation,
    #[strum(serialize = "live_photo")]
    LivePhoto,
    #[strum(serialize = "venue")]
    Venue,
    #[strum(serialize = "audio")]
    Audio,
    #[strum(serialize = "boost_added")]
    BoostAdded,
    #[strum(serialize = "channel_chat_created")]
    ChannelChatCreated,
    #[strum(serialize = "chat_background_set")]
    ChatBackgroundSet,
    #[strum(serialize = "chat_owner_changed")]
    ChatOwnerChanged,
    #[strum(serialize = "chat_owner_left")]
    ChatOwnerLeft,
    #[strum(serialize = "chat_shared")]
    ChatShared,
    #[strum(serialize = "checklist")]
    Checklist,
    #[strum(serialize = "checklist_tasks_added")]
    ChecklistTasksAdded,
    #[strum(serialize = "checklist_tasks_done")]
    ChecklistTasksDone,
    #[strum(serialize = "community_chat_added")]
    CommunityChatAdded,
    #[strum(serialize = "community_chat_joined")]
    CommunityChatJoined,
    #[strum(serialize = "community_chat_removed")]
    CommunityChatRemoved,
    #[strum(serialize = "connected_website")]
    ConnectedWebsite,
    #[strum(serialize = "contact")]
    Contact,
    #[strum(serialize = "delete_chat_photo")]
    DeleteChatPhoto,
    #[strum(serialize = "dice")]
    Dice,
    #[strum(serialize = "direct_message_price_changed")]
    DirectMessagePriceChanged,
    #[strum(serialize = "document")]
    Document,
    #[strum(serialize = "forum_topic_closed")]
    ForumTopicClosed,
    #[strum(serialize = "forum_topic_created")]
    ForumTopicCreated,
    #[strum(serialize = "forum_topic_edited")]
    ForumTopicEdited,
    #[strum(serialize = "forum_topic_reopened")]
    ForumTopicReopened,
    #[strum(serialize = "game")]
    Game,
    #[strum(serialize = "general_forum_topic_hidden")]
    GeneralForumTopicHidden,
    #[strum(serialize = "general_forum_topic_unhidden")]
    GeneralForumTopicUnhidden,
    #[strum(serialize = "gift")]
    Gift,
    #[strum(serialize = "gift_upgrade_sent")]
    GiftUpgradeSent,
    #[strum(serialize = "giveaway")]
    Giveaway,
    #[strum(serialize = "giveaway_completed")]
    GiveawayCompleted,
    #[strum(serialize = "giveaway_created")]
    GiveawayCreated,
    #[strum(serialize = "giveaway_winners")]
    GiveawayWinners,
    #[strum(serialize = "group_chat_created")]
    GroupChatCreated,
    #[strum(serialize = "invoice")]
    Invoice,
    #[strum(serialize = "left_chat_member")]
    LeftChatMember,
    #[strum(serialize = "location")]
    Location,
    #[strum(serialize = "managed_bot_created")]
    ManagedBotCreated,
    #[strum(serialize = "message_auto_delete_timer_changed")]
    MessageAutoDeleteTimerChanged,
    #[strum(serialize = "migrate_from_chat_id")]
    MigrateFromChatId,
    #[strum(serialize = "migrate_to_chat_id")]
    MigrateToChatId,
    #[strum(serialize = "new_chat_members")]
    NewChatMembers,
    #[strum(serialize = "new_chat_photo")]
    NewChatPhoto,
    #[strum(serialize = "new_chat_title")]
    NewChatTitle,
    #[strum(serialize = "paid_media")]
    PaidMedia,
    #[strum(serialize = "paid_message_price_changed")]
    PaidMessagePriceChanged,
    #[strum(serialize = "passport_data")]
    PassportData,
    #[strum(serialize = "photo")]
    Photo,
    #[strum(serialize = "pinned_message")]
    PinnedMessage,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "poll_option_added")]
    PollOptionAdded,
    #[strum(serialize = "poll_option_deleted")]
    PollOptionDeleted,
    #[strum(serialize = "proximity_alert_triggered")]
    ProximityAlertTriggered,
    #[strum(serialize = "refunded_payment")]
    RefundedPayment,
    #[strum(serialize = "rich_message")]
    RichMessage,
    #[strum(serialize = "sticker")]
    Sticker,
    #[strum(serialize = "story")]
    Story,
    #[strum(serialize = "successful_payment")]
    SuccessfulPayment,
    #[strum(serialize = "suggested_post_approval_failed")]
    SuggestedPostApprovalFailed,
    #[strum(serialize = "suggested_post_approved")]
    SuggestedPostApproved,
    #[strum(serialize = "suggested_post_declined")]
    SuggestedPostDeclined,
    #[strum(serialize = "suggested_post_paid")]
    SuggestedPostPaid,
    #[strum(serialize = "suggested_post_refunded")]
    SuggestedPostRefunded,
    #[strum(serialize = "supergroup_chat_created")]
    SupergroupChatCreated,
    #[strum(serialize = "text")]
    Text,
    #[strum(serialize = "unique_gift")]
    UniqueGift,
    #[strum(serialize = "users_shared")]
    UsersShared,
    #[strum(serialize = "video")]
    Video,
    #[strum(serialize = "video_chat_ended")]
    VideoChatEnded,
    #[strum(serialize = "video_chat_participants_invited")]
    VideoChatParticipantsInvited,
    #[strum(serialize = "video_chat_scheduled")]
    VideoChatScheduled,
    #[strum(serialize = "video_chat_started")]
    VideoChatStarted,
    #[strum(serialize = "video_note")]
    VideoNote,
    #[strum(serialize = "voice")]
    Voice,
    #[strum(serialize = "web_app_data")]
    WebAppData,
    #[strum(serialize = "write_access_allowed")]
    WriteAccessAllowed,
    #[strum(serialize = "unknown")]
    Unknown,
}
impl MessageType {
    #[must_use]
    pub const fn all() -> [MessageType; 79usize] {
        [
            MessageType::Animation,
            MessageType::LivePhoto,
            MessageType::Venue,
            MessageType::Audio,
            MessageType::BoostAdded,
            MessageType::ChannelChatCreated,
            MessageType::ChatBackgroundSet,
            MessageType::ChatOwnerChanged,
            MessageType::ChatOwnerLeft,
            MessageType::ChatShared,
            MessageType::Checklist,
            MessageType::ChecklistTasksAdded,
            MessageType::ChecklistTasksDone,
            MessageType::CommunityChatAdded,
            MessageType::CommunityChatJoined,
            MessageType::CommunityChatRemoved,
            MessageType::ConnectedWebsite,
            MessageType::Contact,
            MessageType::DeleteChatPhoto,
            MessageType::Dice,
            MessageType::DirectMessagePriceChanged,
            MessageType::Document,
            MessageType::ForumTopicClosed,
            MessageType::ForumTopicCreated,
            MessageType::ForumTopicEdited,
            MessageType::ForumTopicReopened,
            MessageType::Game,
            MessageType::GeneralForumTopicHidden,
            MessageType::GeneralForumTopicUnhidden,
            MessageType::Gift,
            MessageType::GiftUpgradeSent,
            MessageType::Giveaway,
            MessageType::GiveawayCompleted,
            MessageType::GiveawayCreated,
            MessageType::GiveawayWinners,
            MessageType::GroupChatCreated,
            MessageType::Invoice,
            MessageType::LeftChatMember,
            MessageType::Location,
            MessageType::ManagedBotCreated,
            MessageType::MessageAutoDeleteTimerChanged,
            MessageType::MigrateFromChatId,
            MessageType::MigrateToChatId,
            MessageType::NewChatMembers,
            MessageType::NewChatPhoto,
            MessageType::NewChatTitle,
            MessageType::PaidMedia,
            MessageType::PaidMessagePriceChanged,
            MessageType::PassportData,
            MessageType::Photo,
            MessageType::PinnedMessage,
            MessageType::Poll,
            MessageType::PollOptionAdded,
            MessageType::PollOptionDeleted,
            MessageType::ProximityAlertTriggered,
            MessageType::RefundedPayment,
            MessageType::RichMessage,
            MessageType::Sticker,
            MessageType::Story,
            MessageType::SuccessfulPayment,
            MessageType::SuggestedPostApprovalFailed,
            MessageType::SuggestedPostApproved,
            MessageType::SuggestedPostDeclined,
            MessageType::SuggestedPostPaid,
            MessageType::SuggestedPostRefunded,
            MessageType::SupergroupChatCreated,
            MessageType::Text,
            MessageType::UniqueGift,
            MessageType::UsersShared,
            MessageType::Video,
            MessageType::VideoChatEnded,
            MessageType::VideoChatParticipantsInvited,
            MessageType::VideoChatScheduled,
            MessageType::VideoChatStarted,
            MessageType::VideoNote,
            MessageType::Voice,
            MessageType::WebAppData,
            MessageType::WriteAccessAllowed,
            MessageType::Unknown,
        ]
    }
}
impl From<MessageType> for Box<str> {
    fn from(val: MessageType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<MessageType> for String {
    fn from(val: MessageType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for MessageType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Message> for MessageType {
    fn from(val: &'a Message) -> Self {
        match val {
            Message::Animation(_) => MessageType::Animation,
            Message::LivePhoto(_) => MessageType::LivePhoto,
            Message::Venue(_) => MessageType::Venue,
            Message::Audio(_) => MessageType::Audio,
            Message::BoostAdded(_) => MessageType::BoostAdded,
            Message::ChannelChatCreated(_) => MessageType::ChannelChatCreated,
            Message::ChatBackgroundSet(_) => MessageType::ChatBackgroundSet,
            Message::ChatOwnerChanged(_) => MessageType::ChatOwnerChanged,
            Message::ChatOwnerLeft(_) => MessageType::ChatOwnerLeft,
            Message::ChatShared(_) => MessageType::ChatShared,
            Message::Checklist(_) => MessageType::Checklist,
            Message::ChecklistTasksAdded(_) => MessageType::ChecklistTasksAdded,
            Message::ChecklistTasksDone(_) => MessageType::ChecklistTasksDone,
            Message::CommunityChatAdded(_) => MessageType::CommunityChatAdded,
            Message::CommunityChatJoined(_) => MessageType::CommunityChatJoined,
            Message::CommunityChatRemoved(_) => MessageType::CommunityChatRemoved,
            Message::ConnectedWebsite(_) => MessageType::ConnectedWebsite,
            Message::Contact(_) => MessageType::Contact,
            Message::DeleteChatPhoto(_) => MessageType::DeleteChatPhoto,
            Message::Dice(_) => MessageType::Dice,
            Message::DirectMessagePriceChanged(_) => MessageType::DirectMessagePriceChanged,
            Message::Document(_) => MessageType::Document,
            Message::ForumTopicClosed(_) => MessageType::ForumTopicClosed,
            Message::ForumTopicCreated(_) => MessageType::ForumTopicCreated,
            Message::ForumTopicEdited(_) => MessageType::ForumTopicEdited,
            Message::ForumTopicReopened(_) => MessageType::ForumTopicReopened,
            Message::Game(_) => MessageType::Game,
            Message::GeneralForumTopicHidden(_) => MessageType::GeneralForumTopicHidden,
            Message::GeneralForumTopicUnhidden(_) => MessageType::GeneralForumTopicUnhidden,
            Message::Gift(_) => MessageType::Gift,
            Message::GiftUpgradeSent(_) => MessageType::GiftUpgradeSent,
            Message::Giveaway(_) => MessageType::Giveaway,
            Message::GiveawayCompleted(_) => MessageType::GiveawayCompleted,
            Message::GiveawayCreated(_) => MessageType::GiveawayCreated,
            Message::GiveawayWinners(_) => MessageType::GiveawayWinners,
            Message::GroupChatCreated(_) => MessageType::GroupChatCreated,
            Message::Invoice(_) => MessageType::Invoice,
            Message::LeftChatMember(_) => MessageType::LeftChatMember,
            Message::Location(_) => MessageType::Location,
            Message::ManagedBotCreated(_) => MessageType::ManagedBotCreated,
            Message::MessageAutoDeleteTimerChanged(_) => MessageType::MessageAutoDeleteTimerChanged,
            Message::MigrateFromChatId(_) => MessageType::MigrateFromChatId,
            Message::MigrateToChatId(_) => MessageType::MigrateToChatId,
            Message::NewChatMembers(_) => MessageType::NewChatMembers,
            Message::NewChatPhoto(_) => MessageType::NewChatPhoto,
            Message::NewChatTitle(_) => MessageType::NewChatTitle,
            Message::PaidMedia(_) => MessageType::PaidMedia,
            Message::PaidMessagePriceChanged(_) => MessageType::PaidMessagePriceChanged,
            Message::PassportData(_) => MessageType::PassportData,
            Message::Photo(_) => MessageType::Photo,
            Message::PinnedMessage(_) => MessageType::PinnedMessage,
            Message::Poll(_) => MessageType::Poll,
            Message::PollOptionAdded(_) => MessageType::PollOptionAdded,
            Message::PollOptionDeleted(_) => MessageType::PollOptionDeleted,
            Message::ProximityAlertTriggered(_) => MessageType::ProximityAlertTriggered,
            Message::RefundedPayment(_) => MessageType::RefundedPayment,
            Message::RichMessage(_) => MessageType::RichMessage,
            Message::Sticker(_) => MessageType::Sticker,
            Message::Story(_) => MessageType::Story,
            Message::SuccessfulPayment(_) => MessageType::SuccessfulPayment,
            Message::SuggestedPostApprovalFailed(_) => MessageType::SuggestedPostApprovalFailed,
            Message::SuggestedPostApproved(_) => MessageType::SuggestedPostApproved,
            Message::SuggestedPostDeclined(_) => MessageType::SuggestedPostDeclined,
            Message::SuggestedPostPaid(_) => MessageType::SuggestedPostPaid,
            Message::SuggestedPostRefunded(_) => MessageType::SuggestedPostRefunded,
            Message::SupergroupChatCreated(_) => MessageType::SupergroupChatCreated,
            Message::Text(_) => MessageType::Text,
            Message::UniqueGift(_) => MessageType::UniqueGift,
            Message::UsersShared(_) => MessageType::UsersShared,
            Message::Video(_) => MessageType::Video,
            Message::VideoChatEnded(_) => MessageType::VideoChatEnded,
            Message::VideoChatParticipantsInvited(_) => MessageType::VideoChatParticipantsInvited,
            Message::VideoChatScheduled(_) => MessageType::VideoChatScheduled,
            Message::VideoChatStarted(_) => MessageType::VideoChatStarted,
            Message::VideoNote(_) => MessageType::VideoNote,
            Message::Voice(_) => MessageType::Voice,
            Message::WebAppData(_) => MessageType::WebAppData,
            Message::WriteAccessAllowed(_) => MessageType::WriteAccessAllowed,
            Message::Unknown(_) => MessageType::Unknown,
        }
    }
}
impl From<Message> for MessageType {
    fn from(val: Message) -> Self {
        MessageType::from(&val)
    }
}
