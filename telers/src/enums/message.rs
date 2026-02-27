use crate::types::Message;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents a message.
/// Currently, it can be one of
/// - [`MessageAnimation`]
/// - [`MessageAudio`]
/// - [`MessageBoostAdded`]
/// - [`MessageChannelChatCreated`]
/// - [`MessageChatBackgroundSet`]
/// - [`MessageChatOwnerChanged`]
/// - [`MessageChatOwnerLeft`]
/// - [`MessageChatShared`]
/// - [`MessageChecklist`]
/// - [`MessageChecklistTasksAdded`]
/// - [`MessageChecklistTasksDone`]
/// - [`MessageConnectedWebsite`]
/// - [`MessageContact`]
/// - [`MessageDeleteChatPhoto`]
/// - [`MessageDice`]
/// - [`MessageDirectMessagePriceChanged`]
/// - [`MessageDocument`]
/// - [`MessageForumTopicClosed`]
/// - [`MessageForumTopicCreated`]
/// - [`MessageForumTopicEdited`]
/// - [`MessageForumTopicReopened`]
/// - [`MessageGame`]
/// - [`MessageGeneralForumTopicHidden`]
/// - [`MessageGeneralForumTopicUnhidden`]
/// - [`MessageGift`]
/// - [`MessageGiftUpgradeSent`]
/// - [`MessageGiveaway`]
/// - [`MessageGiveawayCompleted`]
/// - [`MessageGiveawayCreated`]
/// - [`MessageGiveawayWinners`]
/// - [`MessageGroupChatCreated`]
/// - [`MessageInvoice`]
/// - [`MessageLeftChatMember`]
/// - [`MessageLocation`]
/// - [`MessageMessageAutoDeleteTimerChanged`]
/// - [`MessageMigrateFromChatId`]
/// - [`MessageMigrateToChatId`]
/// - [`MessageNewChatMembers`]
/// - [`MessageNewChatPhoto`]
/// - [`MessageNewChatTitle`]
/// - [`MessagePaidMedia`]
/// - [`MessagePaidMessagePriceChanged`]
/// - [`MessagePassportData`]
/// - [`MessagePhoto`]
/// - [`MessagePinnedMessage`]
/// - [`MessagePoll`]
/// - [`MessageProximityAlertTriggered`]
/// - [`MessageRefundedPayment`]
/// - [`MessageSticker`]
/// - [`MessageStory`]
/// - [`MessageSuccessfulPayment`]
/// - [`MessageSuggestedPostApprovalFailed`]
/// - [`MessageSuggestedPostApproved`]
/// - [`MessageSuggestedPostDeclined`]
/// - [`MessageSuggestedPostPaid`]
/// - [`MessageSuggestedPostRefunded`]
/// - [`MessageSupergroupChatCreated`]
/// - [`MessageText`]
/// - [`MessageUniqueGift`]
/// - [`MessageUsersShared`]
/// - [`MessageVenue`]
/// - [`MessageVideo`]
/// - [`MessageVideoChatEnded`]
/// - [`MessageVideoChatParticipantsInvited`]
/// - [`MessageVideoChatScheduled`]
/// - [`MessageVideoChatStarted`]
/// - [`MessageVideoNote`]
/// - [`MessageVoice`]
/// - [`MessageWebAppData`]
/// - [`MessageWriteAccessAllowed`]
/// # Documentation
/// <https://core.telegram.org/bots/api#message>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum MessageType {
    #[strum(serialize = "animation")]
    Animation,
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
    #[strum(serialize = "proximity_alert_triggered")]
    ProximityAlertTriggered,
    #[strum(serialize = "refunded_payment")]
    RefundedPayment,
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
    #[strum(serialize = "venue")]
    Venue,
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
}
impl MessageType {
    #[must_use]
    pub const fn all() -> [MessageType; 70usize] {
        [
            MessageType::Animation,
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
            MessageType::ProximityAlertTriggered,
            MessageType::RefundedPayment,
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
            MessageType::Venue,
            MessageType::Video,
            MessageType::VideoChatEnded,
            MessageType::VideoChatParticipantsInvited,
            MessageType::VideoChatScheduled,
            MessageType::VideoChatStarted,
            MessageType::VideoNote,
            MessageType::Voice,
            MessageType::WebAppData,
            MessageType::WriteAccessAllowed,
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
            Message::ProximityAlertTriggered(_) => MessageType::ProximityAlertTriggered,
            Message::RefundedPayment(_) => MessageType::RefundedPayment,
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
            Message::Venue(_) => MessageType::Venue,
            Message::Video(_) => MessageType::Video,
            Message::VideoChatEnded(_) => MessageType::VideoChatEnded,
            Message::VideoChatParticipantsInvited(_) => MessageType::VideoChatParticipantsInvited,
            Message::VideoChatScheduled(_) => MessageType::VideoChatScheduled,
            Message::VideoChatStarted(_) => MessageType::VideoChatStarted,
            Message::VideoNote(_) => MessageType::VideoNote,
            Message::Voice(_) => MessageType::Voice,
            Message::WebAppData(_) => MessageType::WebAppData,
            Message::WriteAccessAllowed(_) => MessageType::WriteAccessAllowed,
        }
    }
}
impl From<Message> for MessageType {
    fn from(val: Message) -> Self {
        MessageType::from(&val)
    }
}
