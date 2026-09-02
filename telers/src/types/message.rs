use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Animation(crate::types::MessageAnimation),
    LivePhoto(crate::types::MessageLivePhoto),
    Venue(crate::types::MessageVenue),
    Audio(crate::types::MessageAudio),
    BoostAdded(crate::types::MessageBoostAdded),
    ChannelChatCreated(crate::types::MessageChannelChatCreated),
    ChatBackgroundSet(crate::types::MessageChatBackgroundSet),
    ChatOwnerChanged(crate::types::MessageChatOwnerChanged),
    ChatOwnerLeft(crate::types::MessageChatOwnerLeft),
    ChatShared(crate::types::MessageChatShared),
    Checklist(crate::types::MessageChecklist),
    ChecklistTasksAdded(crate::types::MessageChecklistTasksAdded),
    ChecklistTasksDone(crate::types::MessageChecklistTasksDone),
    CommunityChatAdded(crate::types::MessageCommunityChatAdded),
    CommunityChatJoined(crate::types::MessageCommunityChatJoined),
    CommunityChatRemoved(crate::types::MessageCommunityChatRemoved),
    ConnectedWebsite(crate::types::MessageConnectedWebsite),
    Contact(crate::types::MessageContact),
    DeleteChatPhoto(crate::types::MessageDeleteChatPhoto),
    Dice(crate::types::MessageDice),
    DirectMessagePriceChanged(crate::types::MessageDirectMessagePriceChanged),
    Document(crate::types::MessageDocument),
    ForumTopicClosed(crate::types::MessageForumTopicClosed),
    ForumTopicCreated(crate::types::MessageForumTopicCreated),
    ForumTopicEdited(crate::types::MessageForumTopicEdited),
    ForumTopicReopened(crate::types::MessageForumTopicReopened),
    Game(crate::types::MessageGame),
    GeneralForumTopicHidden(crate::types::MessageGeneralForumTopicHidden),
    GeneralForumTopicUnhidden(crate::types::MessageGeneralForumTopicUnhidden),
    Gift(crate::types::MessageGift),
    GiftUpgradeSent(crate::types::MessageGiftUpgradeSent),
    Giveaway(crate::types::MessageGiveaway),
    GiveawayCompleted(crate::types::MessageGiveawayCompleted),
    GiveawayCreated(crate::types::MessageGiveawayCreated),
    GiveawayWinners(crate::types::MessageGiveawayWinners),
    GroupChatCreated(crate::types::MessageGroupChatCreated),
    Invoice(crate::types::MessageInvoice),
    LeftChatMember(crate::types::MessageLeftChatMember),
    Location(crate::types::MessageLocation),
    ManagedBotCreated(crate::types::MessageManagedBotCreated),
    MessageAutoDeleteTimerChanged(crate::types::MessageMessageAutoDeleteTimerChanged),
    MigrateFromChatId(crate::types::MessageMigrateFromChatId),
    MigrateToChatId(crate::types::MessageMigrateToChatId),
    NewChatMembers(crate::types::MessageNewChatMembers),
    NewChatPhoto(crate::types::MessageNewChatPhoto),
    NewChatTitle(crate::types::MessageNewChatTitle),
    PaidMedia(crate::types::MessagePaidMedia),
    PaidMessagePriceChanged(crate::types::MessagePaidMessagePriceChanged),
    PassportData(crate::types::MessagePassportData),
    Photo(crate::types::MessagePhoto),
    PinnedMessage(crate::types::MessagePinnedMessage),
    Poll(crate::types::MessagePoll),
    PollOptionAdded(crate::types::MessagePollOptionAdded),
    PollOptionDeleted(crate::types::MessagePollOptionDeleted),
    ProximityAlertTriggered(crate::types::MessageProximityAlertTriggered),
    RefundedPayment(crate::types::MessageRefundedPayment),
    RichMessage(crate::types::MessageRichMessage),
    Sticker(crate::types::MessageSticker),
    Story(crate::types::MessageStory),
    SuccessfulPayment(crate::types::MessageSuccessfulPayment),
    SuggestedPostApprovalFailed(crate::types::MessageSuggestedPostApprovalFailed),
    SuggestedPostApproved(crate::types::MessageSuggestedPostApproved),
    SuggestedPostDeclined(crate::types::MessageSuggestedPostDeclined),
    SuggestedPostPaid(crate::types::MessageSuggestedPostPaid),
    SuggestedPostRefunded(crate::types::MessageSuggestedPostRefunded),
    SupergroupChatCreated(crate::types::MessageSupergroupChatCreated),
    Text(crate::types::MessageText),
    UniqueGift(crate::types::MessageUniqueGift),
    UsersShared(crate::types::MessageUsersShared),
    Video(crate::types::MessageVideo),
    VideoChatEnded(crate::types::MessageVideoChatEnded),
    VideoChatParticipantsInvited(crate::types::MessageVideoChatParticipantsInvited),
    VideoChatScheduled(crate::types::MessageVideoChatScheduled),
    VideoChatStarted(crate::types::MessageVideoChatStarted),
    VideoNote(crate::types::MessageVideoNote),
    Voice(crate::types::MessageVoice),
    WebAppData(crate::types::MessageWebAppData),
    WriteAccessAllowed(crate::types::MessageWriteAccessAllowed),
    Unknown(crate::types::MessageUnknown),
}
impl Message {
    /// Helper method for field `animation`.
    ///
    /// Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set.
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::Animation> {
        match self {
            Self::Animation(val) => Some(val.animation.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `audio`.
    ///
    /// Message is an audio file, information about the file
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::Audio(val) => Some(val.audio.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `author_signature`.
    ///
    /// Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[must_use]
    pub fn author_signature(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.author_signature.as_deref(),
            Self::LivePhoto(val) => val.author_signature.as_deref(),
            Self::Venue(val) => val.author_signature.as_deref(),
            Self::Audio(val) => val.author_signature.as_deref(),
            Self::BoostAdded(val) => val.author_signature.as_deref(),
            Self::ChannelChatCreated(val) => val.author_signature.as_deref(),
            Self::ChatBackgroundSet(val) => val.author_signature.as_deref(),
            Self::ChatOwnerChanged(val) => val.author_signature.as_deref(),
            Self::ChatOwnerLeft(val) => val.author_signature.as_deref(),
            Self::ChatShared(val) => val.author_signature.as_deref(),
            Self::Checklist(val) => val.author_signature.as_deref(),
            Self::ChecklistTasksAdded(val) => val.author_signature.as_deref(),
            Self::ChecklistTasksDone(val) => val.author_signature.as_deref(),
            Self::CommunityChatAdded(val) => val.author_signature.as_deref(),
            Self::CommunityChatJoined(val) => val.author_signature.as_deref(),
            Self::CommunityChatRemoved(val) => val.author_signature.as_deref(),
            Self::ConnectedWebsite(val) => val.author_signature.as_deref(),
            Self::Contact(val) => val.author_signature.as_deref(),
            Self::DeleteChatPhoto(val) => val.author_signature.as_deref(),
            Self::Dice(val) => val.author_signature.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.author_signature.as_deref(),
            Self::Document(val) => val.author_signature.as_deref(),
            Self::ForumTopicClosed(val) => val.author_signature.as_deref(),
            Self::ForumTopicCreated(val) => val.author_signature.as_deref(),
            Self::ForumTopicEdited(val) => val.author_signature.as_deref(),
            Self::ForumTopicReopened(val) => val.author_signature.as_deref(),
            Self::Game(val) => val.author_signature.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.author_signature.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.author_signature.as_deref(),
            Self::Gift(val) => val.author_signature.as_deref(),
            Self::GiftUpgradeSent(val) => val.author_signature.as_deref(),
            Self::Giveaway(val) => val.author_signature.as_deref(),
            Self::GiveawayCompleted(val) => val.author_signature.as_deref(),
            Self::GiveawayCreated(val) => val.author_signature.as_deref(),
            Self::GiveawayWinners(val) => val.author_signature.as_deref(),
            Self::GroupChatCreated(val) => val.author_signature.as_deref(),
            Self::Invoice(val) => val.author_signature.as_deref(),
            Self::LeftChatMember(val) => val.author_signature.as_deref(),
            Self::Location(val) => val.author_signature.as_deref(),
            Self::ManagedBotCreated(val) => val.author_signature.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.author_signature.as_deref(),
            Self::MigrateFromChatId(val) => val.author_signature.as_deref(),
            Self::MigrateToChatId(val) => val.author_signature.as_deref(),
            Self::NewChatMembers(val) => val.author_signature.as_deref(),
            Self::NewChatPhoto(val) => val.author_signature.as_deref(),
            Self::NewChatTitle(val) => val.author_signature.as_deref(),
            Self::PaidMedia(val) => val.author_signature.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.author_signature.as_deref(),
            Self::PassportData(val) => val.author_signature.as_deref(),
            Self::Photo(val) => val.author_signature.as_deref(),
            Self::PinnedMessage(val) => val.author_signature.as_deref(),
            Self::Poll(val) => val.author_signature.as_deref(),
            Self::PollOptionAdded(val) => val.author_signature.as_deref(),
            Self::PollOptionDeleted(val) => val.author_signature.as_deref(),
            Self::ProximityAlertTriggered(val) => val.author_signature.as_deref(),
            Self::RefundedPayment(val) => val.author_signature.as_deref(),
            Self::RichMessage(val) => val.author_signature.as_deref(),
            Self::Sticker(val) => val.author_signature.as_deref(),
            Self::Story(val) => val.author_signature.as_deref(),
            Self::SuccessfulPayment(val) => val.author_signature.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.author_signature.as_deref(),
            Self::SuggestedPostApproved(val) => val.author_signature.as_deref(),
            Self::SuggestedPostDeclined(val) => val.author_signature.as_deref(),
            Self::SuggestedPostPaid(val) => val.author_signature.as_deref(),
            Self::SuggestedPostRefunded(val) => val.author_signature.as_deref(),
            Self::SupergroupChatCreated(val) => val.author_signature.as_deref(),
            Self::Text(val) => val.author_signature.as_deref(),
            Self::UniqueGift(val) => val.author_signature.as_deref(),
            Self::UsersShared(val) => val.author_signature.as_deref(),
            Self::Video(val) => val.author_signature.as_deref(),
            Self::VideoChatEnded(val) => val.author_signature.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.author_signature.as_deref(),
            Self::VideoChatScheduled(val) => val.author_signature.as_deref(),
            Self::VideoChatStarted(val) => val.author_signature.as_deref(),
            Self::VideoNote(val) => val.author_signature.as_deref(),
            Self::Voice(val) => val.author_signature.as_deref(),
            Self::WebAppData(val) => val.author_signature.as_deref(),
            Self::WriteAccessAllowed(val) => val.author_signature.as_deref(),
            Self::Unknown(val) => val.author_signature.as_deref(),
        }
    }

    /// Helper method for field `boost_added`.
    ///
    /// Service message: user boosted the chat
    #[must_use]
    pub fn boost_added(&self) -> Option<&crate::types::ChatBoostAdded> {
        match self {
            Self::BoostAdded(val) => Some(&val.boost_added),
            _ => None,
        }
    }

    /// Helper method for field `business_connection_id`.
    ///
    /// Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[must_use]
    pub fn business_connection_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.business_connection_id.as_deref(),
            Self::LivePhoto(val) => val.business_connection_id.as_deref(),
            Self::Venue(val) => val.business_connection_id.as_deref(),
            Self::Audio(val) => val.business_connection_id.as_deref(),
            Self::BoostAdded(val) => val.business_connection_id.as_deref(),
            Self::ChannelChatCreated(val) => val.business_connection_id.as_deref(),
            Self::ChatBackgroundSet(val) => val.business_connection_id.as_deref(),
            Self::ChatOwnerChanged(val) => val.business_connection_id.as_deref(),
            Self::ChatOwnerLeft(val) => val.business_connection_id.as_deref(),
            Self::ChatShared(val) => val.business_connection_id.as_deref(),
            Self::Checklist(val) => val.business_connection_id.as_deref(),
            Self::ChecklistTasksAdded(val) => val.business_connection_id.as_deref(),
            Self::ChecklistTasksDone(val) => val.business_connection_id.as_deref(),
            Self::CommunityChatAdded(val) => val.business_connection_id.as_deref(),
            Self::CommunityChatJoined(val) => val.business_connection_id.as_deref(),
            Self::CommunityChatRemoved(val) => val.business_connection_id.as_deref(),
            Self::ConnectedWebsite(val) => val.business_connection_id.as_deref(),
            Self::Contact(val) => val.business_connection_id.as_deref(),
            Self::DeleteChatPhoto(val) => val.business_connection_id.as_deref(),
            Self::Dice(val) => val.business_connection_id.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.business_connection_id.as_deref(),
            Self::Document(val) => val.business_connection_id.as_deref(),
            Self::ForumTopicClosed(val) => val.business_connection_id.as_deref(),
            Self::ForumTopicCreated(val) => val.business_connection_id.as_deref(),
            Self::ForumTopicEdited(val) => val.business_connection_id.as_deref(),
            Self::ForumTopicReopened(val) => val.business_connection_id.as_deref(),
            Self::Game(val) => val.business_connection_id.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.business_connection_id.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.business_connection_id.as_deref(),
            Self::Gift(val) => val.business_connection_id.as_deref(),
            Self::GiftUpgradeSent(val) => val.business_connection_id.as_deref(),
            Self::Giveaway(val) => val.business_connection_id.as_deref(),
            Self::GiveawayCompleted(val) => val.business_connection_id.as_deref(),
            Self::GiveawayCreated(val) => val.business_connection_id.as_deref(),
            Self::GiveawayWinners(val) => val.business_connection_id.as_deref(),
            Self::GroupChatCreated(val) => val.business_connection_id.as_deref(),
            Self::Invoice(val) => val.business_connection_id.as_deref(),
            Self::LeftChatMember(val) => val.business_connection_id.as_deref(),
            Self::Location(val) => val.business_connection_id.as_deref(),
            Self::ManagedBotCreated(val) => val.business_connection_id.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.business_connection_id.as_deref(),
            Self::MigrateFromChatId(val) => val.business_connection_id.as_deref(),
            Self::MigrateToChatId(val) => val.business_connection_id.as_deref(),
            Self::NewChatMembers(val) => val.business_connection_id.as_deref(),
            Self::NewChatPhoto(val) => val.business_connection_id.as_deref(),
            Self::NewChatTitle(val) => val.business_connection_id.as_deref(),
            Self::PaidMedia(val) => val.business_connection_id.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.business_connection_id.as_deref(),
            Self::PassportData(val) => val.business_connection_id.as_deref(),
            Self::Photo(val) => val.business_connection_id.as_deref(),
            Self::PinnedMessage(val) => val.business_connection_id.as_deref(),
            Self::Poll(val) => val.business_connection_id.as_deref(),
            Self::PollOptionAdded(val) => val.business_connection_id.as_deref(),
            Self::PollOptionDeleted(val) => val.business_connection_id.as_deref(),
            Self::ProximityAlertTriggered(val) => val.business_connection_id.as_deref(),
            Self::RefundedPayment(val) => val.business_connection_id.as_deref(),
            Self::RichMessage(val) => val.business_connection_id.as_deref(),
            Self::Sticker(val) => val.business_connection_id.as_deref(),
            Self::Story(val) => val.business_connection_id.as_deref(),
            Self::SuccessfulPayment(val) => val.business_connection_id.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.business_connection_id.as_deref(),
            Self::SuggestedPostApproved(val) => val.business_connection_id.as_deref(),
            Self::SuggestedPostDeclined(val) => val.business_connection_id.as_deref(),
            Self::SuggestedPostPaid(val) => val.business_connection_id.as_deref(),
            Self::SuggestedPostRefunded(val) => val.business_connection_id.as_deref(),
            Self::SupergroupChatCreated(val) => val.business_connection_id.as_deref(),
            Self::Text(val) => val.business_connection_id.as_deref(),
            Self::UniqueGift(val) => val.business_connection_id.as_deref(),
            Self::UsersShared(val) => val.business_connection_id.as_deref(),
            Self::Video(val) => val.business_connection_id.as_deref(),
            Self::VideoChatEnded(val) => val.business_connection_id.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.business_connection_id.as_deref(),
            Self::VideoChatScheduled(val) => val.business_connection_id.as_deref(),
            Self::VideoChatStarted(val) => val.business_connection_id.as_deref(),
            Self::VideoNote(val) => val.business_connection_id.as_deref(),
            Self::Voice(val) => val.business_connection_id.as_deref(),
            Self::WebAppData(val) => val.business_connection_id.as_deref(),
            Self::WriteAccessAllowed(val) => val.business_connection_id.as_deref(),
            Self::Unknown(val) => val.business_connection_id.as_deref(),
        }
    }

    /// Helper method for field `caption`.
    ///
    /// Caption for the animation, audio, document, paid media, photo, video or voice
    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.caption.as_deref(),
            Self::LivePhoto(val) => val.caption.as_deref(),
            Self::Venue(val) => val.caption.as_deref(),
            Self::Audio(val) => val.caption.as_deref(),
            Self::BoostAdded(val) => val.caption.as_deref(),
            Self::ChannelChatCreated(val) => val.caption.as_deref(),
            Self::ChatBackgroundSet(val) => val.caption.as_deref(),
            Self::ChatOwnerChanged(val) => val.caption.as_deref(),
            Self::ChatOwnerLeft(val) => val.caption.as_deref(),
            Self::ChatShared(val) => val.caption.as_deref(),
            Self::Checklist(val) => val.caption.as_deref(),
            Self::ChecklistTasksAdded(val) => val.caption.as_deref(),
            Self::ChecklistTasksDone(val) => val.caption.as_deref(),
            Self::CommunityChatAdded(val) => val.caption.as_deref(),
            Self::CommunityChatJoined(val) => val.caption.as_deref(),
            Self::CommunityChatRemoved(val) => val.caption.as_deref(),
            Self::ConnectedWebsite(val) => val.caption.as_deref(),
            Self::Contact(val) => val.caption.as_deref(),
            Self::DeleteChatPhoto(val) => val.caption.as_deref(),
            Self::Dice(val) => val.caption.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.caption.as_deref(),
            Self::Document(val) => val.caption.as_deref(),
            Self::ForumTopicClosed(val) => val.caption.as_deref(),
            Self::ForumTopicCreated(val) => val.caption.as_deref(),
            Self::ForumTopicEdited(val) => val.caption.as_deref(),
            Self::ForumTopicReopened(val) => val.caption.as_deref(),
            Self::Game(val) => val.caption.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.caption.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.caption.as_deref(),
            Self::Gift(val) => val.caption.as_deref(),
            Self::GiftUpgradeSent(val) => val.caption.as_deref(),
            Self::Giveaway(val) => val.caption.as_deref(),
            Self::GiveawayCompleted(val) => val.caption.as_deref(),
            Self::GiveawayCreated(val) => val.caption.as_deref(),
            Self::GiveawayWinners(val) => val.caption.as_deref(),
            Self::GroupChatCreated(val) => val.caption.as_deref(),
            Self::Invoice(val) => val.caption.as_deref(),
            Self::LeftChatMember(val) => val.caption.as_deref(),
            Self::Location(val) => val.caption.as_deref(),
            Self::ManagedBotCreated(val) => val.caption.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.caption.as_deref(),
            Self::MigrateFromChatId(val) => val.caption.as_deref(),
            Self::MigrateToChatId(val) => val.caption.as_deref(),
            Self::NewChatMembers(val) => val.caption.as_deref(),
            Self::NewChatPhoto(val) => val.caption.as_deref(),
            Self::NewChatTitle(val) => val.caption.as_deref(),
            Self::PaidMedia(val) => val.caption.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.caption.as_deref(),
            Self::PassportData(val) => val.caption.as_deref(),
            Self::Photo(val) => val.caption.as_deref(),
            Self::PinnedMessage(val) => val.caption.as_deref(),
            Self::Poll(val) => val.caption.as_deref(),
            Self::PollOptionAdded(val) => val.caption.as_deref(),
            Self::PollOptionDeleted(val) => val.caption.as_deref(),
            Self::ProximityAlertTriggered(val) => val.caption.as_deref(),
            Self::RefundedPayment(val) => val.caption.as_deref(),
            Self::RichMessage(val) => val.caption.as_deref(),
            Self::Sticker(val) => val.caption.as_deref(),
            Self::Story(val) => val.caption.as_deref(),
            Self::SuccessfulPayment(val) => val.caption.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.caption.as_deref(),
            Self::SuggestedPostApproved(val) => val.caption.as_deref(),
            Self::SuggestedPostDeclined(val) => val.caption.as_deref(),
            Self::SuggestedPostPaid(val) => val.caption.as_deref(),
            Self::SuggestedPostRefunded(val) => val.caption.as_deref(),
            Self::SupergroupChatCreated(val) => val.caption.as_deref(),
            Self::Text(val) => val.caption.as_deref(),
            Self::UniqueGift(val) => val.caption.as_deref(),
            Self::UsersShared(val) => val.caption.as_deref(),
            Self::Video(val) => val.caption.as_deref(),
            Self::VideoChatEnded(val) => val.caption.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.caption.as_deref(),
            Self::VideoChatScheduled(val) => val.caption.as_deref(),
            Self::VideoChatStarted(val) => val.caption.as_deref(),
            Self::VideoNote(val) => val.caption.as_deref(),
            Self::Voice(val) => val.caption.as_deref(),
            Self::WebAppData(val) => val.caption.as_deref(),
            Self::WriteAccessAllowed(val) => val.caption.as_deref(),
            Self::Unknown(val) => val.caption.as_deref(),
        }
    }

    /// Helper method for field `caption_entities`.
    ///
    /// For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Animation(val) => val.caption_entities.as_deref(),
            Self::LivePhoto(val) => val.caption_entities.as_deref(),
            Self::Venue(val) => val.caption_entities.as_deref(),
            Self::Audio(val) => val.caption_entities.as_deref(),
            Self::BoostAdded(val) => val.caption_entities.as_deref(),
            Self::ChannelChatCreated(val) => val.caption_entities.as_deref(),
            Self::ChatBackgroundSet(val) => val.caption_entities.as_deref(),
            Self::ChatOwnerChanged(val) => val.caption_entities.as_deref(),
            Self::ChatOwnerLeft(val) => val.caption_entities.as_deref(),
            Self::ChatShared(val) => val.caption_entities.as_deref(),
            Self::Checklist(val) => val.caption_entities.as_deref(),
            Self::ChecklistTasksAdded(val) => val.caption_entities.as_deref(),
            Self::ChecklistTasksDone(val) => val.caption_entities.as_deref(),
            Self::CommunityChatAdded(val) => val.caption_entities.as_deref(),
            Self::CommunityChatJoined(val) => val.caption_entities.as_deref(),
            Self::CommunityChatRemoved(val) => val.caption_entities.as_deref(),
            Self::ConnectedWebsite(val) => val.caption_entities.as_deref(),
            Self::Contact(val) => val.caption_entities.as_deref(),
            Self::DeleteChatPhoto(val) => val.caption_entities.as_deref(),
            Self::Dice(val) => val.caption_entities.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.caption_entities.as_deref(),
            Self::Document(val) => val.caption_entities.as_deref(),
            Self::ForumTopicClosed(val) => val.caption_entities.as_deref(),
            Self::ForumTopicCreated(val) => val.caption_entities.as_deref(),
            Self::ForumTopicEdited(val) => val.caption_entities.as_deref(),
            Self::ForumTopicReopened(val) => val.caption_entities.as_deref(),
            Self::Game(val) => val.caption_entities.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.caption_entities.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.caption_entities.as_deref(),
            Self::Gift(val) => val.caption_entities.as_deref(),
            Self::GiftUpgradeSent(val) => val.caption_entities.as_deref(),
            Self::Giveaway(val) => val.caption_entities.as_deref(),
            Self::GiveawayCompleted(val) => val.caption_entities.as_deref(),
            Self::GiveawayCreated(val) => val.caption_entities.as_deref(),
            Self::GiveawayWinners(val) => val.caption_entities.as_deref(),
            Self::GroupChatCreated(val) => val.caption_entities.as_deref(),
            Self::Invoice(val) => val.caption_entities.as_deref(),
            Self::LeftChatMember(val) => val.caption_entities.as_deref(),
            Self::Location(val) => val.caption_entities.as_deref(),
            Self::ManagedBotCreated(val) => val.caption_entities.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.caption_entities.as_deref(),
            Self::MigrateFromChatId(val) => val.caption_entities.as_deref(),
            Self::MigrateToChatId(val) => val.caption_entities.as_deref(),
            Self::NewChatMembers(val) => val.caption_entities.as_deref(),
            Self::NewChatPhoto(val) => val.caption_entities.as_deref(),
            Self::NewChatTitle(val) => val.caption_entities.as_deref(),
            Self::PaidMedia(val) => val.caption_entities.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.caption_entities.as_deref(),
            Self::PassportData(val) => val.caption_entities.as_deref(),
            Self::Photo(val) => val.caption_entities.as_deref(),
            Self::PinnedMessage(val) => val.caption_entities.as_deref(),
            Self::Poll(val) => val.caption_entities.as_deref(),
            Self::PollOptionAdded(val) => val.caption_entities.as_deref(),
            Self::PollOptionDeleted(val) => val.caption_entities.as_deref(),
            Self::ProximityAlertTriggered(val) => val.caption_entities.as_deref(),
            Self::RefundedPayment(val) => val.caption_entities.as_deref(),
            Self::RichMessage(val) => val.caption_entities.as_deref(),
            Self::Sticker(val) => val.caption_entities.as_deref(),
            Self::Story(val) => val.caption_entities.as_deref(),
            Self::SuccessfulPayment(val) => val.caption_entities.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.caption_entities.as_deref(),
            Self::SuggestedPostApproved(val) => val.caption_entities.as_deref(),
            Self::SuggestedPostDeclined(val) => val.caption_entities.as_deref(),
            Self::SuggestedPostPaid(val) => val.caption_entities.as_deref(),
            Self::SuggestedPostRefunded(val) => val.caption_entities.as_deref(),
            Self::SupergroupChatCreated(val) => val.caption_entities.as_deref(),
            Self::Text(val) => val.caption_entities.as_deref(),
            Self::UniqueGift(val) => val.caption_entities.as_deref(),
            Self::UsersShared(val) => val.caption_entities.as_deref(),
            Self::Video(val) => val.caption_entities.as_deref(),
            Self::VideoChatEnded(val) => val.caption_entities.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.caption_entities.as_deref(),
            Self::VideoChatScheduled(val) => val.caption_entities.as_deref(),
            Self::VideoChatStarted(val) => val.caption_entities.as_deref(),
            Self::VideoNote(val) => val.caption_entities.as_deref(),
            Self::Voice(val) => val.caption_entities.as_deref(),
            Self::WebAppData(val) => val.caption_entities.as_deref(),
            Self::WriteAccessAllowed(val) => val.caption_entities.as_deref(),
            Self::Unknown(val) => val.caption_entities.as_deref(),
        }
    }

    /// Helper method for field `channel_chat_created`.
    ///
    /// Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in `reply_to_message` if someone replies to a very first message in a channel.
    #[must_use]
    pub fn channel_chat_created(&self) -> Option<bool> {
        match self {
            Self::ChannelChatCreated(val) => Some(val.channel_chat_created),
            _ => None,
        }
    }

    /// Helper method for field `chat`.
    ///
    /// Chat the message belongs to
    #[must_use]
    pub fn chat(&self) -> &crate::types::Chat {
        match self {
            Self::Animation(val) => val.chat.as_ref(),
            Self::LivePhoto(val) => val.chat.as_ref(),
            Self::Venue(val) => val.chat.as_ref(),
            Self::Audio(val) => val.chat.as_ref(),
            Self::BoostAdded(val) => val.chat.as_ref(),
            Self::ChannelChatCreated(val) => val.chat.as_ref(),
            Self::ChatBackgroundSet(val) => val.chat.as_ref(),
            Self::ChatOwnerChanged(val) => val.chat.as_ref(),
            Self::ChatOwnerLeft(val) => val.chat.as_ref(),
            Self::ChatShared(val) => val.chat.as_ref(),
            Self::Checklist(val) => val.chat.as_ref(),
            Self::ChecklistTasksAdded(val) => val.chat.as_ref(),
            Self::ChecklistTasksDone(val) => val.chat.as_ref(),
            Self::CommunityChatAdded(val) => val.chat.as_ref(),
            Self::CommunityChatJoined(val) => val.chat.as_ref(),
            Self::CommunityChatRemoved(val) => val.chat.as_ref(),
            Self::ConnectedWebsite(val) => val.chat.as_ref(),
            Self::Contact(val) => val.chat.as_ref(),
            Self::DeleteChatPhoto(val) => val.chat.as_ref(),
            Self::Dice(val) => val.chat.as_ref(),
            Self::DirectMessagePriceChanged(val) => val.chat.as_ref(),
            Self::Document(val) => val.chat.as_ref(),
            Self::ForumTopicClosed(val) => val.chat.as_ref(),
            Self::ForumTopicCreated(val) => val.chat.as_ref(),
            Self::ForumTopicEdited(val) => val.chat.as_ref(),
            Self::ForumTopicReopened(val) => val.chat.as_ref(),
            Self::Game(val) => val.chat.as_ref(),
            Self::GeneralForumTopicHidden(val) => val.chat.as_ref(),
            Self::GeneralForumTopicUnhidden(val) => val.chat.as_ref(),
            Self::Gift(val) => val.chat.as_ref(),
            Self::GiftUpgradeSent(val) => val.chat.as_ref(),
            Self::Giveaway(val) => val.chat.as_ref(),
            Self::GiveawayCompleted(val) => val.chat.as_ref(),
            Self::GiveawayCreated(val) => val.chat.as_ref(),
            Self::GiveawayWinners(val) => val.chat.as_ref(),
            Self::GroupChatCreated(val) => val.chat.as_ref(),
            Self::Invoice(val) => val.chat.as_ref(),
            Self::LeftChatMember(val) => val.chat.as_ref(),
            Self::Location(val) => val.chat.as_ref(),
            Self::ManagedBotCreated(val) => val.chat.as_ref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.chat.as_ref(),
            Self::MigrateFromChatId(val) => val.chat.as_ref(),
            Self::MigrateToChatId(val) => val.chat.as_ref(),
            Self::NewChatMembers(val) => val.chat.as_ref(),
            Self::NewChatPhoto(val) => val.chat.as_ref(),
            Self::NewChatTitle(val) => val.chat.as_ref(),
            Self::PaidMedia(val) => val.chat.as_ref(),
            Self::PaidMessagePriceChanged(val) => val.chat.as_ref(),
            Self::PassportData(val) => val.chat.as_ref(),
            Self::Photo(val) => val.chat.as_ref(),
            Self::PinnedMessage(val) => val.chat.as_ref(),
            Self::Poll(val) => val.chat.as_ref(),
            Self::PollOptionAdded(val) => val.chat.as_ref(),
            Self::PollOptionDeleted(val) => val.chat.as_ref(),
            Self::ProximityAlertTriggered(val) => val.chat.as_ref(),
            Self::RefundedPayment(val) => val.chat.as_ref(),
            Self::RichMessage(val) => val.chat.as_ref(),
            Self::Sticker(val) => val.chat.as_ref(),
            Self::Story(val) => val.chat.as_ref(),
            Self::SuccessfulPayment(val) => val.chat.as_ref(),
            Self::SuggestedPostApprovalFailed(val) => val.chat.as_ref(),
            Self::SuggestedPostApproved(val) => val.chat.as_ref(),
            Self::SuggestedPostDeclined(val) => val.chat.as_ref(),
            Self::SuggestedPostPaid(val) => val.chat.as_ref(),
            Self::SuggestedPostRefunded(val) => val.chat.as_ref(),
            Self::SupergroupChatCreated(val) => val.chat.as_ref(),
            Self::Text(val) => val.chat.as_ref(),
            Self::UniqueGift(val) => val.chat.as_ref(),
            Self::UsersShared(val) => val.chat.as_ref(),
            Self::Video(val) => val.chat.as_ref(),
            Self::VideoChatEnded(val) => val.chat.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.chat.as_ref(),
            Self::VideoChatScheduled(val) => val.chat.as_ref(),
            Self::VideoChatStarted(val) => val.chat.as_ref(),
            Self::VideoNote(val) => val.chat.as_ref(),
            Self::Voice(val) => val.chat.as_ref(),
            Self::WebAppData(val) => val.chat.as_ref(),
            Self::WriteAccessAllowed(val) => val.chat.as_ref(),
            Self::Unknown(val) => val.chat.as_ref(),
        }
    }

    /// Helper method for field `chat_background_set`.
    ///
    /// Service message: chat background set
    #[must_use]
    pub fn chat_background_set(&self) -> Option<&crate::types::ChatBackground> {
        match self {
            Self::ChatBackgroundSet(val) => Some(&val.chat_background_set),
            _ => None,
        }
    }

    /// Helper method for field `chat_owner_changed`.
    ///
    /// Service message: chat owner has changed
    #[must_use]
    pub fn chat_owner_changed(&self) -> Option<&crate::types::ChatOwnerChanged> {
        match self {
            Self::ChatOwnerChanged(val) => Some(&val.chat_owner_changed),
            _ => None,
        }
    }

    /// Helper method for field `chat_owner_left`.
    ///
    /// Service message: chat owner has left
    #[must_use]
    pub fn chat_owner_left(&self) -> Option<&crate::types::ChatOwnerLeft> {
        match self {
            Self::ChatOwnerLeft(val) => Some(&val.chat_owner_left),
            _ => None,
        }
    }

    /// Helper method for field `chat_shared`.
    ///
    /// Service message: a chat was shared with the bot
    #[must_use]
    pub fn chat_shared(&self) -> Option<&crate::types::ChatShared> {
        match self {
            Self::ChatShared(val) => Some(&val.chat_shared),
            _ => None,
        }
    }

    /// Helper method for field `checklist`.
    ///
    /// Message is a checklist
    #[must_use]
    pub fn checklist(&self) -> Option<&crate::types::Checklist> {
        match self {
            Self::Checklist(val) => Some(&val.checklist),
            _ => None,
        }
    }

    /// Helper method for field `checklist_tasks_added`.
    ///
    /// Service message: tasks were added to a checklist
    #[must_use]
    pub fn checklist_tasks_added(&self) -> Option<&crate::types::ChecklistTasksAdded> {
        match self {
            Self::ChecklistTasksAdded(val) => Some(&val.checklist_tasks_added),
            _ => None,
        }
    }

    /// Helper method for field `checklist_tasks_done`.
    ///
    /// Service message: some tasks in a checklist were marked as done or not done
    #[must_use]
    pub fn checklist_tasks_done(&self) -> Option<&crate::types::ChecklistTasksDone> {
        match self {
            Self::ChecklistTasksDone(val) => Some(&val.checklist_tasks_done),
            _ => None,
        }
    }

    /// Helper method for field `community_chat_added`.
    ///
    /// Service message: chat or bot added to a Community
    #[must_use]
    pub fn community_chat_added(&self) -> Option<&crate::types::CommunityChatAdded> {
        match self {
            Self::CommunityChatAdded(val) => Some(&val.community_chat_added),
            _ => None,
        }
    }

    /// Helper method for field `community_chat_joined`.
    ///
    /// Service message: chat was joined by a user from a Community
    #[must_use]
    pub fn community_chat_joined(&self) -> Option<&crate::types::CommunityChatJoined> {
        match self {
            Self::CommunityChatJoined(val) => Some(&val.community_chat_joined),
            _ => None,
        }
    }

    /// Helper method for field `community_chat_removed`.
    ///
    /// Service message: chat or bot removed from a Community
    #[must_use]
    pub fn community_chat_removed(&self) -> Option<&crate::types::CommunityChatRemoved> {
        match self {
            Self::CommunityChatRemoved(val) => Some(&val.community_chat_removed),
            _ => None,
        }
    }

    /// Helper method for field `connected_website`.
    ///
    /// The domain name of the website on which the user has logged in. More about Telegram Login: <https://core.telegram.org/widgets/login>
    #[must_use]
    pub fn connected_website(&self) -> Option<&str> {
        match self {
            Self::ConnectedWebsite(val) => Some(val.connected_website.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `contact`.
    ///
    /// Message is a shared contact, information about the contact
    #[must_use]
    pub fn contact(&self) -> Option<&crate::types::Contact> {
        match self {
            Self::Contact(val) => Some(&val.contact),
            _ => None,
        }
    }

    /// Helper method for field `date`.
    ///
    /// Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    #[must_use]
    pub fn date(&self) -> i64 {
        match self {
            Self::Animation(val) => val.date,
            Self::LivePhoto(val) => val.date,
            Self::Venue(val) => val.date,
            Self::Audio(val) => val.date,
            Self::BoostAdded(val) => val.date,
            Self::ChannelChatCreated(val) => val.date,
            Self::ChatBackgroundSet(val) => val.date,
            Self::ChatOwnerChanged(val) => val.date,
            Self::ChatOwnerLeft(val) => val.date,
            Self::ChatShared(val) => val.date,
            Self::Checklist(val) => val.date,
            Self::ChecklistTasksAdded(val) => val.date,
            Self::ChecklistTasksDone(val) => val.date,
            Self::CommunityChatAdded(val) => val.date,
            Self::CommunityChatJoined(val) => val.date,
            Self::CommunityChatRemoved(val) => val.date,
            Self::ConnectedWebsite(val) => val.date,
            Self::Contact(val) => val.date,
            Self::DeleteChatPhoto(val) => val.date,
            Self::Dice(val) => val.date,
            Self::DirectMessagePriceChanged(val) => val.date,
            Self::Document(val) => val.date,
            Self::ForumTopicClosed(val) => val.date,
            Self::ForumTopicCreated(val) => val.date,
            Self::ForumTopicEdited(val) => val.date,
            Self::ForumTopicReopened(val) => val.date,
            Self::Game(val) => val.date,
            Self::GeneralForumTopicHidden(val) => val.date,
            Self::GeneralForumTopicUnhidden(val) => val.date,
            Self::Gift(val) => val.date,
            Self::GiftUpgradeSent(val) => val.date,
            Self::Giveaway(val) => val.date,
            Self::GiveawayCompleted(val) => val.date,
            Self::GiveawayCreated(val) => val.date,
            Self::GiveawayWinners(val) => val.date,
            Self::GroupChatCreated(val) => val.date,
            Self::Invoice(val) => val.date,
            Self::LeftChatMember(val) => val.date,
            Self::Location(val) => val.date,
            Self::ManagedBotCreated(val) => val.date,
            Self::MessageAutoDeleteTimerChanged(val) => val.date,
            Self::MigrateFromChatId(val) => val.date,
            Self::MigrateToChatId(val) => val.date,
            Self::NewChatMembers(val) => val.date,
            Self::NewChatPhoto(val) => val.date,
            Self::NewChatTitle(val) => val.date,
            Self::PaidMedia(val) => val.date,
            Self::PaidMessagePriceChanged(val) => val.date,
            Self::PassportData(val) => val.date,
            Self::Photo(val) => val.date,
            Self::PinnedMessage(val) => val.date,
            Self::Poll(val) => val.date,
            Self::PollOptionAdded(val) => val.date,
            Self::PollOptionDeleted(val) => val.date,
            Self::ProximityAlertTriggered(val) => val.date,
            Self::RefundedPayment(val) => val.date,
            Self::RichMessage(val) => val.date,
            Self::Sticker(val) => val.date,
            Self::Story(val) => val.date,
            Self::SuccessfulPayment(val) => val.date,
            Self::SuggestedPostApprovalFailed(val) => val.date,
            Self::SuggestedPostApproved(val) => val.date,
            Self::SuggestedPostDeclined(val) => val.date,
            Self::SuggestedPostPaid(val) => val.date,
            Self::SuggestedPostRefunded(val) => val.date,
            Self::SupergroupChatCreated(val) => val.date,
            Self::Text(val) => val.date,
            Self::UniqueGift(val) => val.date,
            Self::UsersShared(val) => val.date,
            Self::Video(val) => val.date,
            Self::VideoChatEnded(val) => val.date,
            Self::VideoChatParticipantsInvited(val) => val.date,
            Self::VideoChatScheduled(val) => val.date,
            Self::VideoChatStarted(val) => val.date,
            Self::VideoNote(val) => val.date,
            Self::Voice(val) => val.date,
            Self::WebAppData(val) => val.date,
            Self::WriteAccessAllowed(val) => val.date,
            Self::Unknown(val) => val.date,
        }
    }

    /// Helper method for field `delete_chat_photo`.
    ///
    /// Service message: the chat photo was deleted
    #[must_use]
    pub fn delete_chat_photo(&self) -> Option<bool> {
        match self {
            Self::DeleteChatPhoto(val) => Some(val.delete_chat_photo),
            _ => None,
        }
    }

    /// Helper method for field `dice`.
    ///
    /// Message is a dice with random value
    #[must_use]
    pub fn dice(&self) -> Option<&crate::types::Dice> {
        match self {
            Self::Dice(val) => Some(&val.dice),
            _ => None,
        }
    }

    /// Helper method for field `direct_message_price_changed`.
    ///
    /// Service message: the price for paid messages in the corresponding direct messages chat of a channel has changed
    #[must_use]
    pub fn direct_message_price_changed(&self) -> Option<&crate::types::DirectMessagePriceChanged> {
        match self {
            Self::DirectMessagePriceChanged(val) => Some(&val.direct_message_price_changed),
            _ => None,
        }
    }

    /// Helper method for field `direct_messages_topic`.
    ///
    /// Information about the direct messages chat topic that contains the message
    #[must_use]
    pub fn direct_messages_topic(&self) -> Option<&crate::types::DirectMessagesTopic> {
        match self {
            Self::Animation(val) => val.direct_messages_topic.as_ref(),
            Self::LivePhoto(val) => val.direct_messages_topic.as_ref(),
            Self::Venue(val) => val.direct_messages_topic.as_ref(),
            Self::Audio(val) => val.direct_messages_topic.as_ref(),
            Self::BoostAdded(val) => val.direct_messages_topic.as_ref(),
            Self::ChannelChatCreated(val) => val.direct_messages_topic.as_ref(),
            Self::ChatBackgroundSet(val) => val.direct_messages_topic.as_ref(),
            Self::ChatOwnerChanged(val) => val.direct_messages_topic.as_ref(),
            Self::ChatOwnerLeft(val) => val.direct_messages_topic.as_ref(),
            Self::ChatShared(val) => val.direct_messages_topic.as_ref(),
            Self::Checklist(val) => val.direct_messages_topic.as_ref(),
            Self::ChecklistTasksAdded(val) => val.direct_messages_topic.as_ref(),
            Self::ChecklistTasksDone(val) => val.direct_messages_topic.as_ref(),
            Self::CommunityChatAdded(val) => val.direct_messages_topic.as_ref(),
            Self::CommunityChatJoined(val) => val.direct_messages_topic.as_ref(),
            Self::CommunityChatRemoved(val) => val.direct_messages_topic.as_ref(),
            Self::ConnectedWebsite(val) => val.direct_messages_topic.as_ref(),
            Self::Contact(val) => val.direct_messages_topic.as_ref(),
            Self::DeleteChatPhoto(val) => val.direct_messages_topic.as_ref(),
            Self::Dice(val) => val.direct_messages_topic.as_ref(),
            Self::DirectMessagePriceChanged(val) => val.direct_messages_topic.as_ref(),
            Self::Document(val) => val.direct_messages_topic.as_ref(),
            Self::ForumTopicClosed(val) => val.direct_messages_topic.as_ref(),
            Self::ForumTopicCreated(val) => val.direct_messages_topic.as_ref(),
            Self::ForumTopicEdited(val) => val.direct_messages_topic.as_ref(),
            Self::ForumTopicReopened(val) => val.direct_messages_topic.as_ref(),
            Self::Game(val) => val.direct_messages_topic.as_ref(),
            Self::GeneralForumTopicHidden(val) => val.direct_messages_topic.as_ref(),
            Self::GeneralForumTopicUnhidden(val) => val.direct_messages_topic.as_ref(),
            Self::Gift(val) => val.direct_messages_topic.as_ref(),
            Self::GiftUpgradeSent(val) => val.direct_messages_topic.as_ref(),
            Self::Giveaway(val) => val.direct_messages_topic.as_ref(),
            Self::GiveawayCompleted(val) => val.direct_messages_topic.as_ref(),
            Self::GiveawayCreated(val) => val.direct_messages_topic.as_ref(),
            Self::GiveawayWinners(val) => val.direct_messages_topic.as_ref(),
            Self::GroupChatCreated(val) => val.direct_messages_topic.as_ref(),
            Self::Invoice(val) => val.direct_messages_topic.as_ref(),
            Self::LeftChatMember(val) => val.direct_messages_topic.as_ref(),
            Self::Location(val) => val.direct_messages_topic.as_ref(),
            Self::ManagedBotCreated(val) => val.direct_messages_topic.as_ref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.direct_messages_topic.as_ref(),
            Self::MigrateFromChatId(val) => val.direct_messages_topic.as_ref(),
            Self::MigrateToChatId(val) => val.direct_messages_topic.as_ref(),
            Self::NewChatMembers(val) => val.direct_messages_topic.as_ref(),
            Self::NewChatPhoto(val) => val.direct_messages_topic.as_ref(),
            Self::NewChatTitle(val) => val.direct_messages_topic.as_ref(),
            Self::PaidMedia(val) => val.direct_messages_topic.as_ref(),
            Self::PaidMessagePriceChanged(val) => val.direct_messages_topic.as_ref(),
            Self::PassportData(val) => val.direct_messages_topic.as_ref(),
            Self::Photo(val) => val.direct_messages_topic.as_ref(),
            Self::PinnedMessage(val) => val.direct_messages_topic.as_ref(),
            Self::Poll(val) => val.direct_messages_topic.as_ref(),
            Self::PollOptionAdded(val) => val.direct_messages_topic.as_ref(),
            Self::PollOptionDeleted(val) => val.direct_messages_topic.as_ref(),
            Self::ProximityAlertTriggered(val) => val.direct_messages_topic.as_ref(),
            Self::RefundedPayment(val) => val.direct_messages_topic.as_ref(),
            Self::RichMessage(val) => val.direct_messages_topic.as_ref(),
            Self::Sticker(val) => val.direct_messages_topic.as_ref(),
            Self::Story(val) => val.direct_messages_topic.as_ref(),
            Self::SuccessfulPayment(val) => val.direct_messages_topic.as_ref(),
            Self::SuggestedPostApprovalFailed(val) => val.direct_messages_topic.as_ref(),
            Self::SuggestedPostApproved(val) => val.direct_messages_topic.as_ref(),
            Self::SuggestedPostDeclined(val) => val.direct_messages_topic.as_ref(),
            Self::SuggestedPostPaid(val) => val.direct_messages_topic.as_ref(),
            Self::SuggestedPostRefunded(val) => val.direct_messages_topic.as_ref(),
            Self::SupergroupChatCreated(val) => val.direct_messages_topic.as_ref(),
            Self::Text(val) => val.direct_messages_topic.as_ref(),
            Self::UniqueGift(val) => val.direct_messages_topic.as_ref(),
            Self::UsersShared(val) => val.direct_messages_topic.as_ref(),
            Self::Video(val) => val.direct_messages_topic.as_ref(),
            Self::VideoChatEnded(val) => val.direct_messages_topic.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.direct_messages_topic.as_ref(),
            Self::VideoChatScheduled(val) => val.direct_messages_topic.as_ref(),
            Self::VideoChatStarted(val) => val.direct_messages_topic.as_ref(),
            Self::VideoNote(val) => val.direct_messages_topic.as_ref(),
            Self::Voice(val) => val.direct_messages_topic.as_ref(),
            Self::WebAppData(val) => val.direct_messages_topic.as_ref(),
            Self::WriteAccessAllowed(val) => val.direct_messages_topic.as_ref(),
            Self::Unknown(val) => val.direct_messages_topic.as_ref(),
        }
    }

    /// Helper method for field `document`.
    ///
    /// Message is a general file, information about the file
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::Document> {
        match self {
            Self::Document(val) => Some(val.document.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `edit_date`.
    ///
    /// Date the message was last edited in Unix time
    #[must_use]
    pub fn edit_date(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.edit_date,
            Self::LivePhoto(val) => val.edit_date,
            Self::Venue(val) => val.edit_date,
            Self::Audio(val) => val.edit_date,
            Self::BoostAdded(val) => val.edit_date,
            Self::ChannelChatCreated(val) => val.edit_date,
            Self::ChatBackgroundSet(val) => val.edit_date,
            Self::ChatOwnerChanged(val) => val.edit_date,
            Self::ChatOwnerLeft(val) => val.edit_date,
            Self::ChatShared(val) => val.edit_date,
            Self::Checklist(val) => val.edit_date,
            Self::ChecklistTasksAdded(val) => val.edit_date,
            Self::ChecklistTasksDone(val) => val.edit_date,
            Self::CommunityChatAdded(val) => val.edit_date,
            Self::CommunityChatJoined(val) => val.edit_date,
            Self::CommunityChatRemoved(val) => val.edit_date,
            Self::ConnectedWebsite(val) => val.edit_date,
            Self::Contact(val) => val.edit_date,
            Self::DeleteChatPhoto(val) => val.edit_date,
            Self::Dice(val) => val.edit_date,
            Self::DirectMessagePriceChanged(val) => val.edit_date,
            Self::Document(val) => val.edit_date,
            Self::ForumTopicClosed(val) => val.edit_date,
            Self::ForumTopicCreated(val) => val.edit_date,
            Self::ForumTopicEdited(val) => val.edit_date,
            Self::ForumTopicReopened(val) => val.edit_date,
            Self::Game(val) => val.edit_date,
            Self::GeneralForumTopicHidden(val) => val.edit_date,
            Self::GeneralForumTopicUnhidden(val) => val.edit_date,
            Self::Gift(val) => val.edit_date,
            Self::GiftUpgradeSent(val) => val.edit_date,
            Self::Giveaway(val) => val.edit_date,
            Self::GiveawayCompleted(val) => val.edit_date,
            Self::GiveawayCreated(val) => val.edit_date,
            Self::GiveawayWinners(val) => val.edit_date,
            Self::GroupChatCreated(val) => val.edit_date,
            Self::Invoice(val) => val.edit_date,
            Self::LeftChatMember(val) => val.edit_date,
            Self::Location(val) => val.edit_date,
            Self::ManagedBotCreated(val) => val.edit_date,
            Self::MessageAutoDeleteTimerChanged(val) => val.edit_date,
            Self::MigrateFromChatId(val) => val.edit_date,
            Self::MigrateToChatId(val) => val.edit_date,
            Self::NewChatMembers(val) => val.edit_date,
            Self::NewChatPhoto(val) => val.edit_date,
            Self::NewChatTitle(val) => val.edit_date,
            Self::PaidMedia(val) => val.edit_date,
            Self::PaidMessagePriceChanged(val) => val.edit_date,
            Self::PassportData(val) => val.edit_date,
            Self::Photo(val) => val.edit_date,
            Self::PinnedMessage(val) => val.edit_date,
            Self::Poll(val) => val.edit_date,
            Self::PollOptionAdded(val) => val.edit_date,
            Self::PollOptionDeleted(val) => val.edit_date,
            Self::ProximityAlertTriggered(val) => val.edit_date,
            Self::RefundedPayment(val) => val.edit_date,
            Self::RichMessage(val) => val.edit_date,
            Self::Sticker(val) => val.edit_date,
            Self::Story(val) => val.edit_date,
            Self::SuccessfulPayment(val) => val.edit_date,
            Self::SuggestedPostApprovalFailed(val) => val.edit_date,
            Self::SuggestedPostApproved(val) => val.edit_date,
            Self::SuggestedPostDeclined(val) => val.edit_date,
            Self::SuggestedPostPaid(val) => val.edit_date,
            Self::SuggestedPostRefunded(val) => val.edit_date,
            Self::SupergroupChatCreated(val) => val.edit_date,
            Self::Text(val) => val.edit_date,
            Self::UniqueGift(val) => val.edit_date,
            Self::UsersShared(val) => val.edit_date,
            Self::Video(val) => val.edit_date,
            Self::VideoChatEnded(val) => val.edit_date,
            Self::VideoChatParticipantsInvited(val) => val.edit_date,
            Self::VideoChatScheduled(val) => val.edit_date,
            Self::VideoChatStarted(val) => val.edit_date,
            Self::VideoNote(val) => val.edit_date,
            Self::Voice(val) => val.edit_date,
            Self::WebAppData(val) => val.edit_date,
            Self::WriteAccessAllowed(val) => val.edit_date,
            Self::Unknown(val) => val.edit_date,
        }
    }

    /// Helper method for field `effect_id`.
    ///
    /// Unique identifier of the message effect added to the message
    #[must_use]
    pub fn effect_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.effect_id.as_deref(),
            Self::LivePhoto(val) => val.effect_id.as_deref(),
            Self::Venue(val) => val.effect_id.as_deref(),
            Self::Audio(val) => val.effect_id.as_deref(),
            Self::BoostAdded(val) => val.effect_id.as_deref(),
            Self::ChannelChatCreated(val) => val.effect_id.as_deref(),
            Self::ChatBackgroundSet(val) => val.effect_id.as_deref(),
            Self::ChatOwnerChanged(val) => val.effect_id.as_deref(),
            Self::ChatOwnerLeft(val) => val.effect_id.as_deref(),
            Self::ChatShared(val) => val.effect_id.as_deref(),
            Self::Checklist(val) => val.effect_id.as_deref(),
            Self::ChecklistTasksAdded(val) => val.effect_id.as_deref(),
            Self::ChecklistTasksDone(val) => val.effect_id.as_deref(),
            Self::CommunityChatAdded(val) => val.effect_id.as_deref(),
            Self::CommunityChatJoined(val) => val.effect_id.as_deref(),
            Self::CommunityChatRemoved(val) => val.effect_id.as_deref(),
            Self::ConnectedWebsite(val) => val.effect_id.as_deref(),
            Self::Contact(val) => val.effect_id.as_deref(),
            Self::DeleteChatPhoto(val) => val.effect_id.as_deref(),
            Self::Dice(val) => val.effect_id.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.effect_id.as_deref(),
            Self::Document(val) => val.effect_id.as_deref(),
            Self::ForumTopicClosed(val) => val.effect_id.as_deref(),
            Self::ForumTopicCreated(val) => val.effect_id.as_deref(),
            Self::ForumTopicEdited(val) => val.effect_id.as_deref(),
            Self::ForumTopicReopened(val) => val.effect_id.as_deref(),
            Self::Game(val) => val.effect_id.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.effect_id.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.effect_id.as_deref(),
            Self::Gift(val) => val.effect_id.as_deref(),
            Self::GiftUpgradeSent(val) => val.effect_id.as_deref(),
            Self::Giveaway(val) => val.effect_id.as_deref(),
            Self::GiveawayCompleted(val) => val.effect_id.as_deref(),
            Self::GiveawayCreated(val) => val.effect_id.as_deref(),
            Self::GiveawayWinners(val) => val.effect_id.as_deref(),
            Self::GroupChatCreated(val) => val.effect_id.as_deref(),
            Self::Invoice(val) => val.effect_id.as_deref(),
            Self::LeftChatMember(val) => val.effect_id.as_deref(),
            Self::Location(val) => val.effect_id.as_deref(),
            Self::ManagedBotCreated(val) => val.effect_id.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.effect_id.as_deref(),
            Self::MigrateFromChatId(val) => val.effect_id.as_deref(),
            Self::MigrateToChatId(val) => val.effect_id.as_deref(),
            Self::NewChatMembers(val) => val.effect_id.as_deref(),
            Self::NewChatPhoto(val) => val.effect_id.as_deref(),
            Self::NewChatTitle(val) => val.effect_id.as_deref(),
            Self::PaidMedia(val) => val.effect_id.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.effect_id.as_deref(),
            Self::PassportData(val) => val.effect_id.as_deref(),
            Self::Photo(val) => val.effect_id.as_deref(),
            Self::PinnedMessage(val) => val.effect_id.as_deref(),
            Self::Poll(val) => val.effect_id.as_deref(),
            Self::PollOptionAdded(val) => val.effect_id.as_deref(),
            Self::PollOptionDeleted(val) => val.effect_id.as_deref(),
            Self::ProximityAlertTriggered(val) => val.effect_id.as_deref(),
            Self::RefundedPayment(val) => val.effect_id.as_deref(),
            Self::RichMessage(val) => val.effect_id.as_deref(),
            Self::Sticker(val) => val.effect_id.as_deref(),
            Self::Story(val) => val.effect_id.as_deref(),
            Self::SuccessfulPayment(val) => val.effect_id.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.effect_id.as_deref(),
            Self::SuggestedPostApproved(val) => val.effect_id.as_deref(),
            Self::SuggestedPostDeclined(val) => val.effect_id.as_deref(),
            Self::SuggestedPostPaid(val) => val.effect_id.as_deref(),
            Self::SuggestedPostRefunded(val) => val.effect_id.as_deref(),
            Self::SupergroupChatCreated(val) => val.effect_id.as_deref(),
            Self::Text(val) => val.effect_id.as_deref(),
            Self::UniqueGift(val) => val.effect_id.as_deref(),
            Self::UsersShared(val) => val.effect_id.as_deref(),
            Self::Video(val) => val.effect_id.as_deref(),
            Self::VideoChatEnded(val) => val.effect_id.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.effect_id.as_deref(),
            Self::VideoChatScheduled(val) => val.effect_id.as_deref(),
            Self::VideoChatStarted(val) => val.effect_id.as_deref(),
            Self::VideoNote(val) => val.effect_id.as_deref(),
            Self::Voice(val) => val.effect_id.as_deref(),
            Self::WebAppData(val) => val.effect_id.as_deref(),
            Self::WriteAccessAllowed(val) => val.effect_id.as_deref(),
            Self::Unknown(val) => val.effect_id.as_deref(),
        }
    }

    /// Helper method for field `entities`.
    ///
    /// For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Animation(val) => val.entities.as_deref(),
            Self::LivePhoto(val) => val.entities.as_deref(),
            Self::Venue(val) => val.entities.as_deref(),
            Self::Audio(val) => val.entities.as_deref(),
            Self::BoostAdded(val) => val.entities.as_deref(),
            Self::ChannelChatCreated(val) => val.entities.as_deref(),
            Self::ChatBackgroundSet(val) => val.entities.as_deref(),
            Self::ChatOwnerChanged(val) => val.entities.as_deref(),
            Self::ChatOwnerLeft(val) => val.entities.as_deref(),
            Self::ChatShared(val) => val.entities.as_deref(),
            Self::Checklist(val) => val.entities.as_deref(),
            Self::ChecklistTasksAdded(val) => val.entities.as_deref(),
            Self::ChecklistTasksDone(val) => val.entities.as_deref(),
            Self::CommunityChatAdded(val) => val.entities.as_deref(),
            Self::CommunityChatJoined(val) => val.entities.as_deref(),
            Self::CommunityChatRemoved(val) => val.entities.as_deref(),
            Self::ConnectedWebsite(val) => val.entities.as_deref(),
            Self::Contact(val) => val.entities.as_deref(),
            Self::DeleteChatPhoto(val) => val.entities.as_deref(),
            Self::Dice(val) => val.entities.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.entities.as_deref(),
            Self::Document(val) => val.entities.as_deref(),
            Self::ForumTopicClosed(val) => val.entities.as_deref(),
            Self::ForumTopicCreated(val) => val.entities.as_deref(),
            Self::ForumTopicEdited(val) => val.entities.as_deref(),
            Self::ForumTopicReopened(val) => val.entities.as_deref(),
            Self::Game(val) => val.entities.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.entities.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.entities.as_deref(),
            Self::Gift(val) => val.entities.as_deref(),
            Self::GiftUpgradeSent(val) => val.entities.as_deref(),
            Self::Giveaway(val) => val.entities.as_deref(),
            Self::GiveawayCompleted(val) => val.entities.as_deref(),
            Self::GiveawayCreated(val) => val.entities.as_deref(),
            Self::GiveawayWinners(val) => val.entities.as_deref(),
            Self::GroupChatCreated(val) => val.entities.as_deref(),
            Self::Invoice(val) => val.entities.as_deref(),
            Self::LeftChatMember(val) => val.entities.as_deref(),
            Self::Location(val) => val.entities.as_deref(),
            Self::ManagedBotCreated(val) => val.entities.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.entities.as_deref(),
            Self::MigrateFromChatId(val) => val.entities.as_deref(),
            Self::MigrateToChatId(val) => val.entities.as_deref(),
            Self::NewChatMembers(val) => val.entities.as_deref(),
            Self::NewChatPhoto(val) => val.entities.as_deref(),
            Self::NewChatTitle(val) => val.entities.as_deref(),
            Self::PaidMedia(val) => val.entities.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.entities.as_deref(),
            Self::PassportData(val) => val.entities.as_deref(),
            Self::Photo(val) => val.entities.as_deref(),
            Self::PinnedMessage(val) => val.entities.as_deref(),
            Self::Poll(val) => val.entities.as_deref(),
            Self::PollOptionAdded(val) => val.entities.as_deref(),
            Self::PollOptionDeleted(val) => val.entities.as_deref(),
            Self::ProximityAlertTriggered(val) => val.entities.as_deref(),
            Self::RefundedPayment(val) => val.entities.as_deref(),
            Self::RichMessage(val) => val.entities.as_deref(),
            Self::Sticker(val) => val.entities.as_deref(),
            Self::Story(val) => val.entities.as_deref(),
            Self::SuccessfulPayment(val) => val.entities.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.entities.as_deref(),
            Self::SuggestedPostApproved(val) => val.entities.as_deref(),
            Self::SuggestedPostDeclined(val) => val.entities.as_deref(),
            Self::SuggestedPostPaid(val) => val.entities.as_deref(),
            Self::SuggestedPostRefunded(val) => val.entities.as_deref(),
            Self::SupergroupChatCreated(val) => val.entities.as_deref(),
            Self::Text(val) => val.entities.as_deref(),
            Self::UniqueGift(val) => val.entities.as_deref(),
            Self::UsersShared(val) => val.entities.as_deref(),
            Self::Video(val) => val.entities.as_deref(),
            Self::VideoChatEnded(val) => val.entities.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.entities.as_deref(),
            Self::VideoChatScheduled(val) => val.entities.as_deref(),
            Self::VideoChatStarted(val) => val.entities.as_deref(),
            Self::VideoNote(val) => val.entities.as_deref(),
            Self::Voice(val) => val.entities.as_deref(),
            Self::WebAppData(val) => val.entities.as_deref(),
            Self::WriteAccessAllowed(val) => val.entities.as_deref(),
            Self::Unknown(val) => val.entities.as_deref(),
        }
    }

    /// Helper method for field `ephemeral_message_id`.
    ///
    /// For ephemeral messages, identifier of the ephemeral message inside this chat. The identifier may be reused for another ephemeral message after the message is deleted or expires.
    #[must_use]
    pub fn ephemeral_message_id(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.ephemeral_message_id,
            Self::LivePhoto(val) => val.ephemeral_message_id,
            Self::Venue(val) => val.ephemeral_message_id,
            Self::Audio(val) => val.ephemeral_message_id,
            Self::BoostAdded(val) => val.ephemeral_message_id,
            Self::ChannelChatCreated(val) => val.ephemeral_message_id,
            Self::ChatBackgroundSet(val) => val.ephemeral_message_id,
            Self::ChatOwnerChanged(val) => val.ephemeral_message_id,
            Self::ChatOwnerLeft(val) => val.ephemeral_message_id,
            Self::ChatShared(val) => val.ephemeral_message_id,
            Self::Checklist(val) => val.ephemeral_message_id,
            Self::ChecklistTasksAdded(val) => val.ephemeral_message_id,
            Self::ChecklistTasksDone(val) => val.ephemeral_message_id,
            Self::CommunityChatAdded(val) => val.ephemeral_message_id,
            Self::CommunityChatJoined(val) => val.ephemeral_message_id,
            Self::CommunityChatRemoved(val) => val.ephemeral_message_id,
            Self::ConnectedWebsite(val) => val.ephemeral_message_id,
            Self::Contact(val) => val.ephemeral_message_id,
            Self::DeleteChatPhoto(val) => val.ephemeral_message_id,
            Self::Dice(val) => val.ephemeral_message_id,
            Self::DirectMessagePriceChanged(val) => val.ephemeral_message_id,
            Self::Document(val) => val.ephemeral_message_id,
            Self::ForumTopicClosed(val) => val.ephemeral_message_id,
            Self::ForumTopicCreated(val) => val.ephemeral_message_id,
            Self::ForumTopicEdited(val) => val.ephemeral_message_id,
            Self::ForumTopicReopened(val) => val.ephemeral_message_id,
            Self::Game(val) => val.ephemeral_message_id,
            Self::GeneralForumTopicHidden(val) => val.ephemeral_message_id,
            Self::GeneralForumTopicUnhidden(val) => val.ephemeral_message_id,
            Self::Gift(val) => val.ephemeral_message_id,
            Self::GiftUpgradeSent(val) => val.ephemeral_message_id,
            Self::Giveaway(val) => val.ephemeral_message_id,
            Self::GiveawayCompleted(val) => val.ephemeral_message_id,
            Self::GiveawayCreated(val) => val.ephemeral_message_id,
            Self::GiveawayWinners(val) => val.ephemeral_message_id,
            Self::GroupChatCreated(val) => val.ephemeral_message_id,
            Self::Invoice(val) => val.ephemeral_message_id,
            Self::LeftChatMember(val) => val.ephemeral_message_id,
            Self::Location(val) => val.ephemeral_message_id,
            Self::ManagedBotCreated(val) => val.ephemeral_message_id,
            Self::MessageAutoDeleteTimerChanged(val) => val.ephemeral_message_id,
            Self::MigrateFromChatId(val) => val.ephemeral_message_id,
            Self::MigrateToChatId(val) => val.ephemeral_message_id,
            Self::NewChatMembers(val) => val.ephemeral_message_id,
            Self::NewChatPhoto(val) => val.ephemeral_message_id,
            Self::NewChatTitle(val) => val.ephemeral_message_id,
            Self::PaidMedia(val) => val.ephemeral_message_id,
            Self::PaidMessagePriceChanged(val) => val.ephemeral_message_id,
            Self::PassportData(val) => val.ephemeral_message_id,
            Self::Photo(val) => val.ephemeral_message_id,
            Self::PinnedMessage(val) => val.ephemeral_message_id,
            Self::Poll(val) => val.ephemeral_message_id,
            Self::PollOptionAdded(val) => val.ephemeral_message_id,
            Self::PollOptionDeleted(val) => val.ephemeral_message_id,
            Self::ProximityAlertTriggered(val) => val.ephemeral_message_id,
            Self::RefundedPayment(val) => val.ephemeral_message_id,
            Self::RichMessage(val) => val.ephemeral_message_id,
            Self::Sticker(val) => val.ephemeral_message_id,
            Self::Story(val) => val.ephemeral_message_id,
            Self::SuccessfulPayment(val) => val.ephemeral_message_id,
            Self::SuggestedPostApprovalFailed(val) => val.ephemeral_message_id,
            Self::SuggestedPostApproved(val) => val.ephemeral_message_id,
            Self::SuggestedPostDeclined(val) => val.ephemeral_message_id,
            Self::SuggestedPostPaid(val) => val.ephemeral_message_id,
            Self::SuggestedPostRefunded(val) => val.ephemeral_message_id,
            Self::SupergroupChatCreated(val) => val.ephemeral_message_id,
            Self::Text(val) => val.ephemeral_message_id,
            Self::UniqueGift(val) => val.ephemeral_message_id,
            Self::UsersShared(val) => val.ephemeral_message_id,
            Self::Video(val) => val.ephemeral_message_id,
            Self::VideoChatEnded(val) => val.ephemeral_message_id,
            Self::VideoChatParticipantsInvited(val) => val.ephemeral_message_id,
            Self::VideoChatScheduled(val) => val.ephemeral_message_id,
            Self::VideoChatStarted(val) => val.ephemeral_message_id,
            Self::VideoNote(val) => val.ephemeral_message_id,
            Self::Voice(val) => val.ephemeral_message_id,
            Self::WebAppData(val) => val.ephemeral_message_id,
            Self::WriteAccessAllowed(val) => val.ephemeral_message_id,
            Self::Unknown(val) => val.ephemeral_message_id,
        }
    }

    /// Helper method for field `external_reply`.
    ///
    /// Information about the message that is being replied to, which may come from another chat or forum topic
    #[must_use]
    pub fn external_reply(&self) -> Option<&crate::types::ExternalReplyInfo> {
        match self {
            Self::Animation(val) => val.external_reply.as_deref(),
            Self::LivePhoto(val) => val.external_reply.as_deref(),
            Self::Venue(val) => val.external_reply.as_deref(),
            Self::Audio(val) => val.external_reply.as_deref(),
            Self::BoostAdded(val) => val.external_reply.as_deref(),
            Self::ChannelChatCreated(val) => val.external_reply.as_deref(),
            Self::ChatBackgroundSet(val) => val.external_reply.as_deref(),
            Self::ChatOwnerChanged(val) => val.external_reply.as_deref(),
            Self::ChatOwnerLeft(val) => val.external_reply.as_deref(),
            Self::ChatShared(val) => val.external_reply.as_deref(),
            Self::Checklist(val) => val.external_reply.as_deref(),
            Self::ChecklistTasksAdded(val) => val.external_reply.as_deref(),
            Self::ChecklistTasksDone(val) => val.external_reply.as_deref(),
            Self::CommunityChatAdded(val) => val.external_reply.as_deref(),
            Self::CommunityChatJoined(val) => val.external_reply.as_deref(),
            Self::CommunityChatRemoved(val) => val.external_reply.as_deref(),
            Self::ConnectedWebsite(val) => val.external_reply.as_deref(),
            Self::Contact(val) => val.external_reply.as_deref(),
            Self::DeleteChatPhoto(val) => val.external_reply.as_deref(),
            Self::Dice(val) => val.external_reply.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.external_reply.as_deref(),
            Self::Document(val) => val.external_reply.as_deref(),
            Self::ForumTopicClosed(val) => val.external_reply.as_deref(),
            Self::ForumTopicCreated(val) => val.external_reply.as_deref(),
            Self::ForumTopicEdited(val) => val.external_reply.as_deref(),
            Self::ForumTopicReopened(val) => val.external_reply.as_deref(),
            Self::Game(val) => val.external_reply.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.external_reply.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.external_reply.as_deref(),
            Self::Gift(val) => val.external_reply.as_deref(),
            Self::GiftUpgradeSent(val) => val.external_reply.as_deref(),
            Self::Giveaway(val) => val.external_reply.as_deref(),
            Self::GiveawayCompleted(val) => val.external_reply.as_deref(),
            Self::GiveawayCreated(val) => val.external_reply.as_deref(),
            Self::GiveawayWinners(val) => val.external_reply.as_deref(),
            Self::GroupChatCreated(val) => val.external_reply.as_deref(),
            Self::Invoice(val) => val.external_reply.as_deref(),
            Self::LeftChatMember(val) => val.external_reply.as_deref(),
            Self::Location(val) => val.external_reply.as_deref(),
            Self::ManagedBotCreated(val) => val.external_reply.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.external_reply.as_deref(),
            Self::MigrateFromChatId(val) => val.external_reply.as_deref(),
            Self::MigrateToChatId(val) => val.external_reply.as_deref(),
            Self::NewChatMembers(val) => val.external_reply.as_deref(),
            Self::NewChatPhoto(val) => val.external_reply.as_deref(),
            Self::NewChatTitle(val) => val.external_reply.as_deref(),
            Self::PaidMedia(val) => val.external_reply.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.external_reply.as_deref(),
            Self::PassportData(val) => val.external_reply.as_deref(),
            Self::Photo(val) => val.external_reply.as_deref(),
            Self::PinnedMessage(val) => val.external_reply.as_deref(),
            Self::Poll(val) => val.external_reply.as_deref(),
            Self::PollOptionAdded(val) => val.external_reply.as_deref(),
            Self::PollOptionDeleted(val) => val.external_reply.as_deref(),
            Self::ProximityAlertTriggered(val) => val.external_reply.as_deref(),
            Self::RefundedPayment(val) => val.external_reply.as_deref(),
            Self::RichMessage(val) => val.external_reply.as_deref(),
            Self::Sticker(val) => val.external_reply.as_deref(),
            Self::Story(val) => val.external_reply.as_deref(),
            Self::SuccessfulPayment(val) => val.external_reply.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.external_reply.as_deref(),
            Self::SuggestedPostApproved(val) => val.external_reply.as_deref(),
            Self::SuggestedPostDeclined(val) => val.external_reply.as_deref(),
            Self::SuggestedPostPaid(val) => val.external_reply.as_deref(),
            Self::SuggestedPostRefunded(val) => val.external_reply.as_deref(),
            Self::SupergroupChatCreated(val) => val.external_reply.as_deref(),
            Self::Text(val) => val.external_reply.as_deref(),
            Self::UniqueGift(val) => val.external_reply.as_deref(),
            Self::UsersShared(val) => val.external_reply.as_deref(),
            Self::Video(val) => val.external_reply.as_deref(),
            Self::VideoChatEnded(val) => val.external_reply.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.external_reply.as_deref(),
            Self::VideoChatScheduled(val) => val.external_reply.as_deref(),
            Self::VideoChatStarted(val) => val.external_reply.as_deref(),
            Self::VideoNote(val) => val.external_reply.as_deref(),
            Self::Voice(val) => val.external_reply.as_deref(),
            Self::WebAppData(val) => val.external_reply.as_deref(),
            Self::WriteAccessAllowed(val) => val.external_reply.as_deref(),
            Self::Unknown(val) => val.external_reply.as_deref(),
        }
    }

    /// Helper method for field `forum_topic_closed`.
    ///
    /// Service message: forum topic closed
    #[must_use]
    pub fn forum_topic_closed(&self) -> Option<&crate::types::ForumTopicClosed> {
        match self {
            Self::ForumTopicClosed(val) => Some(&val.forum_topic_closed),
            _ => None,
        }
    }

    /// Helper method for field `forum_topic_created`.
    ///
    /// Service message: forum topic created
    #[must_use]
    pub fn forum_topic_created(&self) -> Option<&crate::types::ForumTopicCreated> {
        match self {
            Self::ForumTopicCreated(val) => Some(&val.forum_topic_created),
            _ => None,
        }
    }

    /// Helper method for field `forum_topic_edited`.
    ///
    /// Service message: forum topic edited
    #[must_use]
    pub fn forum_topic_edited(&self) -> Option<&crate::types::ForumTopicEdited> {
        match self {
            Self::ForumTopicEdited(val) => Some(&val.forum_topic_edited),
            _ => None,
        }
    }

    /// Helper method for field `forum_topic_reopened`.
    ///
    /// Service message: forum topic reopened
    #[must_use]
    pub fn forum_topic_reopened(&self) -> Option<&crate::types::ForumTopicReopened> {
        match self {
            Self::ForumTopicReopened(val) => Some(&val.forum_topic_reopened),
            _ => None,
        }
    }

    /// Helper method for field `forward_origin`.
    ///
    /// Information about the original message for forwarded messages
    #[must_use]
    pub fn forward_origin(&self) -> Option<&crate::types::MessageOrigin> {
        match self {
            Self::Animation(val) => val.forward_origin.as_ref(),
            Self::LivePhoto(val) => val.forward_origin.as_ref(),
            Self::Venue(val) => val.forward_origin.as_ref(),
            Self::Audio(val) => val.forward_origin.as_ref(),
            Self::BoostAdded(val) => val.forward_origin.as_ref(),
            Self::ChannelChatCreated(val) => val.forward_origin.as_ref(),
            Self::ChatBackgroundSet(val) => val.forward_origin.as_ref(),
            Self::ChatOwnerChanged(val) => val.forward_origin.as_ref(),
            Self::ChatOwnerLeft(val) => val.forward_origin.as_ref(),
            Self::ChatShared(val) => val.forward_origin.as_ref(),
            Self::Checklist(val) => val.forward_origin.as_ref(),
            Self::ChecklistTasksAdded(val) => val.forward_origin.as_ref(),
            Self::ChecklistTasksDone(val) => val.forward_origin.as_ref(),
            Self::CommunityChatAdded(val) => val.forward_origin.as_ref(),
            Self::CommunityChatJoined(val) => val.forward_origin.as_ref(),
            Self::CommunityChatRemoved(val) => val.forward_origin.as_ref(),
            Self::ConnectedWebsite(val) => val.forward_origin.as_ref(),
            Self::Contact(val) => val.forward_origin.as_ref(),
            Self::DeleteChatPhoto(val) => val.forward_origin.as_ref(),
            Self::Dice(val) => val.forward_origin.as_ref(),
            Self::DirectMessagePriceChanged(val) => val.forward_origin.as_ref(),
            Self::Document(val) => val.forward_origin.as_ref(),
            Self::ForumTopicClosed(val) => val.forward_origin.as_ref(),
            Self::ForumTopicCreated(val) => val.forward_origin.as_ref(),
            Self::ForumTopicEdited(val) => val.forward_origin.as_ref(),
            Self::ForumTopicReopened(val) => val.forward_origin.as_ref(),
            Self::Game(val) => val.forward_origin.as_ref(),
            Self::GeneralForumTopicHidden(val) => val.forward_origin.as_ref(),
            Self::GeneralForumTopicUnhidden(val) => val.forward_origin.as_ref(),
            Self::Gift(val) => val.forward_origin.as_ref(),
            Self::GiftUpgradeSent(val) => val.forward_origin.as_ref(),
            Self::Giveaway(val) => val.forward_origin.as_ref(),
            Self::GiveawayCompleted(val) => val.forward_origin.as_ref(),
            Self::GiveawayCreated(val) => val.forward_origin.as_ref(),
            Self::GiveawayWinners(val) => val.forward_origin.as_ref(),
            Self::GroupChatCreated(val) => val.forward_origin.as_ref(),
            Self::Invoice(val) => val.forward_origin.as_ref(),
            Self::LeftChatMember(val) => val.forward_origin.as_ref(),
            Self::Location(val) => val.forward_origin.as_ref(),
            Self::ManagedBotCreated(val) => val.forward_origin.as_ref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.forward_origin.as_ref(),
            Self::MigrateFromChatId(val) => val.forward_origin.as_ref(),
            Self::MigrateToChatId(val) => val.forward_origin.as_ref(),
            Self::NewChatMembers(val) => val.forward_origin.as_ref(),
            Self::NewChatPhoto(val) => val.forward_origin.as_ref(),
            Self::NewChatTitle(val) => val.forward_origin.as_ref(),
            Self::PaidMedia(val) => val.forward_origin.as_ref(),
            Self::PaidMessagePriceChanged(val) => val.forward_origin.as_ref(),
            Self::PassportData(val) => val.forward_origin.as_ref(),
            Self::Photo(val) => val.forward_origin.as_ref(),
            Self::PinnedMessage(val) => val.forward_origin.as_ref(),
            Self::Poll(val) => val.forward_origin.as_ref(),
            Self::PollOptionAdded(val) => val.forward_origin.as_ref(),
            Self::PollOptionDeleted(val) => val.forward_origin.as_ref(),
            Self::ProximityAlertTriggered(val) => val.forward_origin.as_ref(),
            Self::RefundedPayment(val) => val.forward_origin.as_ref(),
            Self::RichMessage(val) => val.forward_origin.as_ref(),
            Self::Sticker(val) => val.forward_origin.as_ref(),
            Self::Story(val) => val.forward_origin.as_ref(),
            Self::SuccessfulPayment(val) => val.forward_origin.as_ref(),
            Self::SuggestedPostApprovalFailed(val) => val.forward_origin.as_ref(),
            Self::SuggestedPostApproved(val) => val.forward_origin.as_ref(),
            Self::SuggestedPostDeclined(val) => val.forward_origin.as_ref(),
            Self::SuggestedPostPaid(val) => val.forward_origin.as_ref(),
            Self::SuggestedPostRefunded(val) => val.forward_origin.as_ref(),
            Self::SupergroupChatCreated(val) => val.forward_origin.as_ref(),
            Self::Text(val) => val.forward_origin.as_ref(),
            Self::UniqueGift(val) => val.forward_origin.as_ref(),
            Self::UsersShared(val) => val.forward_origin.as_ref(),
            Self::Video(val) => val.forward_origin.as_ref(),
            Self::VideoChatEnded(val) => val.forward_origin.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.forward_origin.as_ref(),
            Self::VideoChatScheduled(val) => val.forward_origin.as_ref(),
            Self::VideoChatStarted(val) => val.forward_origin.as_ref(),
            Self::VideoNote(val) => val.forward_origin.as_ref(),
            Self::Voice(val) => val.forward_origin.as_ref(),
            Self::WebAppData(val) => val.forward_origin.as_ref(),
            Self::WriteAccessAllowed(val) => val.forward_origin.as_ref(),
            Self::Unknown(val) => val.forward_origin.as_ref(),
        }
    }

    /// Helper method for field `from`.
    ///
    /// Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn from(&self) -> Option<&crate::types::User> {
        match self {
            Self::Animation(val) => val.from.as_deref(),
            Self::LivePhoto(val) => val.from.as_deref(),
            Self::Venue(val) => val.from.as_deref(),
            Self::Audio(val) => val.from.as_deref(),
            Self::BoostAdded(val) => val.from.as_deref(),
            Self::ChannelChatCreated(val) => val.from.as_deref(),
            Self::ChatBackgroundSet(val) => val.from.as_deref(),
            Self::ChatOwnerChanged(val) => val.from.as_deref(),
            Self::ChatOwnerLeft(val) => val.from.as_deref(),
            Self::ChatShared(val) => val.from.as_deref(),
            Self::Checklist(val) => val.from.as_deref(),
            Self::ChecklistTasksAdded(val) => val.from.as_deref(),
            Self::ChecklistTasksDone(val) => val.from.as_deref(),
            Self::CommunityChatAdded(val) => val.from.as_deref(),
            Self::CommunityChatJoined(val) => val.from.as_deref(),
            Self::CommunityChatRemoved(val) => val.from.as_deref(),
            Self::ConnectedWebsite(val) => val.from.as_deref(),
            Self::Contact(val) => val.from.as_deref(),
            Self::DeleteChatPhoto(val) => val.from.as_deref(),
            Self::Dice(val) => val.from.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.from.as_deref(),
            Self::Document(val) => val.from.as_deref(),
            Self::ForumTopicClosed(val) => val.from.as_deref(),
            Self::ForumTopicCreated(val) => val.from.as_deref(),
            Self::ForumTopicEdited(val) => val.from.as_deref(),
            Self::ForumTopicReopened(val) => val.from.as_deref(),
            Self::Game(val) => val.from.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.from.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.from.as_deref(),
            Self::Gift(val) => val.from.as_deref(),
            Self::GiftUpgradeSent(val) => val.from.as_deref(),
            Self::Giveaway(val) => val.from.as_deref(),
            Self::GiveawayCompleted(val) => val.from.as_deref(),
            Self::GiveawayCreated(val) => val.from.as_deref(),
            Self::GiveawayWinners(val) => val.from.as_deref(),
            Self::GroupChatCreated(val) => val.from.as_deref(),
            Self::Invoice(val) => val.from.as_deref(),
            Self::LeftChatMember(val) => val.from.as_deref(),
            Self::Location(val) => val.from.as_deref(),
            Self::ManagedBotCreated(val) => val.from.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.from.as_deref(),
            Self::MigrateFromChatId(val) => val.from.as_deref(),
            Self::MigrateToChatId(val) => val.from.as_deref(),
            Self::NewChatMembers(val) => val.from.as_deref(),
            Self::NewChatPhoto(val) => val.from.as_deref(),
            Self::NewChatTitle(val) => val.from.as_deref(),
            Self::PaidMedia(val) => val.from.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.from.as_deref(),
            Self::PassportData(val) => val.from.as_deref(),
            Self::Photo(val) => val.from.as_deref(),
            Self::PinnedMessage(val) => val.from.as_deref(),
            Self::Poll(val) => val.from.as_deref(),
            Self::PollOptionAdded(val) => val.from.as_deref(),
            Self::PollOptionDeleted(val) => val.from.as_deref(),
            Self::ProximityAlertTriggered(val) => val.from.as_deref(),
            Self::RefundedPayment(val) => val.from.as_deref(),
            Self::RichMessage(val) => val.from.as_deref(),
            Self::Sticker(val) => val.from.as_deref(),
            Self::Story(val) => val.from.as_deref(),
            Self::SuccessfulPayment(val) => val.from.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.from.as_deref(),
            Self::SuggestedPostApproved(val) => val.from.as_deref(),
            Self::SuggestedPostDeclined(val) => val.from.as_deref(),
            Self::SuggestedPostPaid(val) => val.from.as_deref(),
            Self::SuggestedPostRefunded(val) => val.from.as_deref(),
            Self::SupergroupChatCreated(val) => val.from.as_deref(),
            Self::Text(val) => val.from.as_deref(),
            Self::UniqueGift(val) => val.from.as_deref(),
            Self::UsersShared(val) => val.from.as_deref(),
            Self::Video(val) => val.from.as_deref(),
            Self::VideoChatEnded(val) => val.from.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.from.as_deref(),
            Self::VideoChatScheduled(val) => val.from.as_deref(),
            Self::VideoChatStarted(val) => val.from.as_deref(),
            Self::VideoNote(val) => val.from.as_deref(),
            Self::Voice(val) => val.from.as_deref(),
            Self::WebAppData(val) => val.from.as_deref(),
            Self::WriteAccessAllowed(val) => val.from.as_deref(),
            Self::Unknown(val) => val.from.as_deref(),
        }
    }

    /// Helper method for field `game`.
    ///
    /// Message is a game, information about the game. More about games: <https://core.telegram.org/bots/api#games>
    #[must_use]
    pub fn game(&self) -> Option<&crate::types::Game> {
        match self {
            Self::Game(val) => Some(&val.game),
            _ => None,
        }
    }

    /// Helper method for field `general_forum_topic_hidden`.
    ///
    /// Service message: the 'General' forum topic hidden
    #[must_use]
    pub fn general_forum_topic_hidden(&self) -> Option<&crate::types::GeneralForumTopicHidden> {
        match self {
            Self::GeneralForumTopicHidden(val) => Some(&val.general_forum_topic_hidden),
            _ => None,
        }
    }

    /// Helper method for field `general_forum_topic_unhidden`.
    ///
    /// Service message: the 'General' forum topic unhidden
    #[must_use]
    pub fn general_forum_topic_unhidden(&self) -> Option<&crate::types::GeneralForumTopicUnhidden> {
        match self {
            Self::GeneralForumTopicUnhidden(val) => Some(&val.general_forum_topic_unhidden),
            _ => None,
        }
    }

    /// Helper method for field `gift`.
    ///
    /// Service message: a regular gift was sent or received
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::Gift(val) => Some(&val.gift),
            _ => None,
        }
    }

    /// Helper method for field `gift_upgrade_sent`.
    ///
    /// Service message: upgrade of a gift was purchased after the gift was sent
    #[must_use]
    pub fn gift_upgrade_sent(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::GiftUpgradeSent(val) => Some(&val.gift_upgrade_sent),
            _ => None,
        }
    }

    /// Helper method for field `giveaway`.
    ///
    /// The message is a scheduled giveaway message
    #[must_use]
    pub fn giveaway(&self) -> Option<&crate::types::Giveaway> {
        match self {
            Self::Giveaway(val) => Some(&val.giveaway),
            _ => None,
        }
    }

    /// Helper method for field `giveaway_completed`.
    ///
    /// Service message: a giveaway without public winners was completed
    #[must_use]
    pub fn giveaway_completed(&self) -> Option<&crate::types::GiveawayCompleted> {
        match self {
            Self::GiveawayCompleted(val) => Some(&val.giveaway_completed),
            _ => None,
        }
    }

    /// Helper method for field `giveaway_created`.
    ///
    /// Service message: a scheduled giveaway was created
    #[must_use]
    pub fn giveaway_created(&self) -> Option<&crate::types::GiveawayCreated> {
        match self {
            Self::GiveawayCreated(val) => Some(&val.giveaway_created),
            _ => None,
        }
    }

    /// Helper method for field `giveaway_winners`.
    ///
    /// A giveaway with public winners was completed
    #[must_use]
    pub fn giveaway_winners(&self) -> Option<&crate::types::GiveawayWinners> {
        match self {
            Self::GiveawayWinners(val) => Some(&val.giveaway_winners),
            _ => None,
        }
    }

    /// Helper method for field `group_chat_created`.
    ///
    /// Service message: the group has been created
    #[must_use]
    pub fn group_chat_created(&self) -> Option<bool> {
        match self {
            Self::GroupChatCreated(val) => Some(val.group_chat_created),
            _ => None,
        }
    }

    /// Helper method for field `guest_bot_caller_chat`.
    ///
    /// For a message sent by a guest bot, this is the chat whose original message triggered the bot's response
    #[must_use]
    pub fn guest_bot_caller_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Animation(val) => val.guest_bot_caller_chat.as_deref(),
            Self::LivePhoto(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Venue(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Audio(val) => val.guest_bot_caller_chat.as_deref(),
            Self::BoostAdded(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ChannelChatCreated(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ChatBackgroundSet(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ChatOwnerChanged(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ChatOwnerLeft(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ChatShared(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Checklist(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ChecklistTasksAdded(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ChecklistTasksDone(val) => val.guest_bot_caller_chat.as_deref(),
            Self::CommunityChatAdded(val) => val.guest_bot_caller_chat.as_deref(),
            Self::CommunityChatJoined(val) => val.guest_bot_caller_chat.as_deref(),
            Self::CommunityChatRemoved(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ConnectedWebsite(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Contact(val) => val.guest_bot_caller_chat.as_deref(),
            Self::DeleteChatPhoto(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Dice(val) => val.guest_bot_caller_chat.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Document(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ForumTopicClosed(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ForumTopicCreated(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ForumTopicEdited(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ForumTopicReopened(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Game(val) => val.guest_bot_caller_chat.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.guest_bot_caller_chat.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Gift(val) => val.guest_bot_caller_chat.as_deref(),
            Self::GiftUpgradeSent(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Giveaway(val) => val.guest_bot_caller_chat.as_deref(),
            Self::GiveawayCompleted(val) => val.guest_bot_caller_chat.as_deref(),
            Self::GiveawayCreated(val) => val.guest_bot_caller_chat.as_deref(),
            Self::GiveawayWinners(val) => val.guest_bot_caller_chat.as_deref(),
            Self::GroupChatCreated(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Invoice(val) => val.guest_bot_caller_chat.as_deref(),
            Self::LeftChatMember(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Location(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ManagedBotCreated(val) => val.guest_bot_caller_chat.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.guest_bot_caller_chat.as_deref(),
            Self::MigrateFromChatId(val) => val.guest_bot_caller_chat.as_deref(),
            Self::MigrateToChatId(val) => val.guest_bot_caller_chat.as_deref(),
            Self::NewChatMembers(val) => val.guest_bot_caller_chat.as_deref(),
            Self::NewChatPhoto(val) => val.guest_bot_caller_chat.as_deref(),
            Self::NewChatTitle(val) => val.guest_bot_caller_chat.as_deref(),
            Self::PaidMedia(val) => val.guest_bot_caller_chat.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.guest_bot_caller_chat.as_deref(),
            Self::PassportData(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Photo(val) => val.guest_bot_caller_chat.as_deref(),
            Self::PinnedMessage(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Poll(val) => val.guest_bot_caller_chat.as_deref(),
            Self::PollOptionAdded(val) => val.guest_bot_caller_chat.as_deref(),
            Self::PollOptionDeleted(val) => val.guest_bot_caller_chat.as_deref(),
            Self::ProximityAlertTriggered(val) => val.guest_bot_caller_chat.as_deref(),
            Self::RefundedPayment(val) => val.guest_bot_caller_chat.as_deref(),
            Self::RichMessage(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Sticker(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Story(val) => val.guest_bot_caller_chat.as_deref(),
            Self::SuccessfulPayment(val) => val.guest_bot_caller_chat.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.guest_bot_caller_chat.as_deref(),
            Self::SuggestedPostApproved(val) => val.guest_bot_caller_chat.as_deref(),
            Self::SuggestedPostDeclined(val) => val.guest_bot_caller_chat.as_deref(),
            Self::SuggestedPostPaid(val) => val.guest_bot_caller_chat.as_deref(),
            Self::SuggestedPostRefunded(val) => val.guest_bot_caller_chat.as_deref(),
            Self::SupergroupChatCreated(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Text(val) => val.guest_bot_caller_chat.as_deref(),
            Self::UniqueGift(val) => val.guest_bot_caller_chat.as_deref(),
            Self::UsersShared(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Video(val) => val.guest_bot_caller_chat.as_deref(),
            Self::VideoChatEnded(val) => val.guest_bot_caller_chat.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.guest_bot_caller_chat.as_deref(),
            Self::VideoChatScheduled(val) => val.guest_bot_caller_chat.as_deref(),
            Self::VideoChatStarted(val) => val.guest_bot_caller_chat.as_deref(),
            Self::VideoNote(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Voice(val) => val.guest_bot_caller_chat.as_deref(),
            Self::WebAppData(val) => val.guest_bot_caller_chat.as_deref(),
            Self::WriteAccessAllowed(val) => val.guest_bot_caller_chat.as_deref(),
            Self::Unknown(val) => val.guest_bot_caller_chat.as_deref(),
        }
    }

    /// Helper method for field `guest_bot_caller_user`.
    ///
    /// For a message sent by a guest bot, this is the user whose original message triggered the bot's response
    #[must_use]
    pub fn guest_bot_caller_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::Animation(val) => val.guest_bot_caller_user.as_deref(),
            Self::LivePhoto(val) => val.guest_bot_caller_user.as_deref(),
            Self::Venue(val) => val.guest_bot_caller_user.as_deref(),
            Self::Audio(val) => val.guest_bot_caller_user.as_deref(),
            Self::BoostAdded(val) => val.guest_bot_caller_user.as_deref(),
            Self::ChannelChatCreated(val) => val.guest_bot_caller_user.as_deref(),
            Self::ChatBackgroundSet(val) => val.guest_bot_caller_user.as_deref(),
            Self::ChatOwnerChanged(val) => val.guest_bot_caller_user.as_deref(),
            Self::ChatOwnerLeft(val) => val.guest_bot_caller_user.as_deref(),
            Self::ChatShared(val) => val.guest_bot_caller_user.as_deref(),
            Self::Checklist(val) => val.guest_bot_caller_user.as_deref(),
            Self::ChecklistTasksAdded(val) => val.guest_bot_caller_user.as_deref(),
            Self::ChecklistTasksDone(val) => val.guest_bot_caller_user.as_deref(),
            Self::CommunityChatAdded(val) => val.guest_bot_caller_user.as_deref(),
            Self::CommunityChatJoined(val) => val.guest_bot_caller_user.as_deref(),
            Self::CommunityChatRemoved(val) => val.guest_bot_caller_user.as_deref(),
            Self::ConnectedWebsite(val) => val.guest_bot_caller_user.as_deref(),
            Self::Contact(val) => val.guest_bot_caller_user.as_deref(),
            Self::DeleteChatPhoto(val) => val.guest_bot_caller_user.as_deref(),
            Self::Dice(val) => val.guest_bot_caller_user.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.guest_bot_caller_user.as_deref(),
            Self::Document(val) => val.guest_bot_caller_user.as_deref(),
            Self::ForumTopicClosed(val) => val.guest_bot_caller_user.as_deref(),
            Self::ForumTopicCreated(val) => val.guest_bot_caller_user.as_deref(),
            Self::ForumTopicEdited(val) => val.guest_bot_caller_user.as_deref(),
            Self::ForumTopicReopened(val) => val.guest_bot_caller_user.as_deref(),
            Self::Game(val) => val.guest_bot_caller_user.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.guest_bot_caller_user.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.guest_bot_caller_user.as_deref(),
            Self::Gift(val) => val.guest_bot_caller_user.as_deref(),
            Self::GiftUpgradeSent(val) => val.guest_bot_caller_user.as_deref(),
            Self::Giveaway(val) => val.guest_bot_caller_user.as_deref(),
            Self::GiveawayCompleted(val) => val.guest_bot_caller_user.as_deref(),
            Self::GiveawayCreated(val) => val.guest_bot_caller_user.as_deref(),
            Self::GiveawayWinners(val) => val.guest_bot_caller_user.as_deref(),
            Self::GroupChatCreated(val) => val.guest_bot_caller_user.as_deref(),
            Self::Invoice(val) => val.guest_bot_caller_user.as_deref(),
            Self::LeftChatMember(val) => val.guest_bot_caller_user.as_deref(),
            Self::Location(val) => val.guest_bot_caller_user.as_deref(),
            Self::ManagedBotCreated(val) => val.guest_bot_caller_user.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.guest_bot_caller_user.as_deref(),
            Self::MigrateFromChatId(val) => val.guest_bot_caller_user.as_deref(),
            Self::MigrateToChatId(val) => val.guest_bot_caller_user.as_deref(),
            Self::NewChatMembers(val) => val.guest_bot_caller_user.as_deref(),
            Self::NewChatPhoto(val) => val.guest_bot_caller_user.as_deref(),
            Self::NewChatTitle(val) => val.guest_bot_caller_user.as_deref(),
            Self::PaidMedia(val) => val.guest_bot_caller_user.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.guest_bot_caller_user.as_deref(),
            Self::PassportData(val) => val.guest_bot_caller_user.as_deref(),
            Self::Photo(val) => val.guest_bot_caller_user.as_deref(),
            Self::PinnedMessage(val) => val.guest_bot_caller_user.as_deref(),
            Self::Poll(val) => val.guest_bot_caller_user.as_deref(),
            Self::PollOptionAdded(val) => val.guest_bot_caller_user.as_deref(),
            Self::PollOptionDeleted(val) => val.guest_bot_caller_user.as_deref(),
            Self::ProximityAlertTriggered(val) => val.guest_bot_caller_user.as_deref(),
            Self::RefundedPayment(val) => val.guest_bot_caller_user.as_deref(),
            Self::RichMessage(val) => val.guest_bot_caller_user.as_deref(),
            Self::Sticker(val) => val.guest_bot_caller_user.as_deref(),
            Self::Story(val) => val.guest_bot_caller_user.as_deref(),
            Self::SuccessfulPayment(val) => val.guest_bot_caller_user.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.guest_bot_caller_user.as_deref(),
            Self::SuggestedPostApproved(val) => val.guest_bot_caller_user.as_deref(),
            Self::SuggestedPostDeclined(val) => val.guest_bot_caller_user.as_deref(),
            Self::SuggestedPostPaid(val) => val.guest_bot_caller_user.as_deref(),
            Self::SuggestedPostRefunded(val) => val.guest_bot_caller_user.as_deref(),
            Self::SupergroupChatCreated(val) => val.guest_bot_caller_user.as_deref(),
            Self::Text(val) => val.guest_bot_caller_user.as_deref(),
            Self::UniqueGift(val) => val.guest_bot_caller_user.as_deref(),
            Self::UsersShared(val) => val.guest_bot_caller_user.as_deref(),
            Self::Video(val) => val.guest_bot_caller_user.as_deref(),
            Self::VideoChatEnded(val) => val.guest_bot_caller_user.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.guest_bot_caller_user.as_deref(),
            Self::VideoChatScheduled(val) => val.guest_bot_caller_user.as_deref(),
            Self::VideoChatStarted(val) => val.guest_bot_caller_user.as_deref(),
            Self::VideoNote(val) => val.guest_bot_caller_user.as_deref(),
            Self::Voice(val) => val.guest_bot_caller_user.as_deref(),
            Self::WebAppData(val) => val.guest_bot_caller_user.as_deref(),
            Self::WriteAccessAllowed(val) => val.guest_bot_caller_user.as_deref(),
            Self::Unknown(val) => val.guest_bot_caller_user.as_deref(),
        }
    }

    /// Helper method for field `guest_query_id`.
    ///
    /// The unique identifier for the guest query. Use this identifier with the method [`crate::methods::AnswerGuestQuery`] to send a response message. If non-empty, the message belongs to the chat where the guest bot was summoned, which may not coincide with other existing bot chats sharing the same identifier.
    #[must_use]
    pub fn guest_query_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.guest_query_id.as_deref(),
            Self::LivePhoto(val) => val.guest_query_id.as_deref(),
            Self::Venue(val) => val.guest_query_id.as_deref(),
            Self::Audio(val) => val.guest_query_id.as_deref(),
            Self::BoostAdded(val) => val.guest_query_id.as_deref(),
            Self::ChannelChatCreated(val) => val.guest_query_id.as_deref(),
            Self::ChatBackgroundSet(val) => val.guest_query_id.as_deref(),
            Self::ChatOwnerChanged(val) => val.guest_query_id.as_deref(),
            Self::ChatOwnerLeft(val) => val.guest_query_id.as_deref(),
            Self::ChatShared(val) => val.guest_query_id.as_deref(),
            Self::Checklist(val) => val.guest_query_id.as_deref(),
            Self::ChecklistTasksAdded(val) => val.guest_query_id.as_deref(),
            Self::ChecklistTasksDone(val) => val.guest_query_id.as_deref(),
            Self::CommunityChatAdded(val) => val.guest_query_id.as_deref(),
            Self::CommunityChatJoined(val) => val.guest_query_id.as_deref(),
            Self::CommunityChatRemoved(val) => val.guest_query_id.as_deref(),
            Self::ConnectedWebsite(val) => val.guest_query_id.as_deref(),
            Self::Contact(val) => val.guest_query_id.as_deref(),
            Self::DeleteChatPhoto(val) => val.guest_query_id.as_deref(),
            Self::Dice(val) => val.guest_query_id.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.guest_query_id.as_deref(),
            Self::Document(val) => val.guest_query_id.as_deref(),
            Self::ForumTopicClosed(val) => val.guest_query_id.as_deref(),
            Self::ForumTopicCreated(val) => val.guest_query_id.as_deref(),
            Self::ForumTopicEdited(val) => val.guest_query_id.as_deref(),
            Self::ForumTopicReopened(val) => val.guest_query_id.as_deref(),
            Self::Game(val) => val.guest_query_id.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.guest_query_id.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.guest_query_id.as_deref(),
            Self::Gift(val) => val.guest_query_id.as_deref(),
            Self::GiftUpgradeSent(val) => val.guest_query_id.as_deref(),
            Self::Giveaway(val) => val.guest_query_id.as_deref(),
            Self::GiveawayCompleted(val) => val.guest_query_id.as_deref(),
            Self::GiveawayCreated(val) => val.guest_query_id.as_deref(),
            Self::GiveawayWinners(val) => val.guest_query_id.as_deref(),
            Self::GroupChatCreated(val) => val.guest_query_id.as_deref(),
            Self::Invoice(val) => val.guest_query_id.as_deref(),
            Self::LeftChatMember(val) => val.guest_query_id.as_deref(),
            Self::Location(val) => val.guest_query_id.as_deref(),
            Self::ManagedBotCreated(val) => val.guest_query_id.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.guest_query_id.as_deref(),
            Self::MigrateFromChatId(val) => val.guest_query_id.as_deref(),
            Self::MigrateToChatId(val) => val.guest_query_id.as_deref(),
            Self::NewChatMembers(val) => val.guest_query_id.as_deref(),
            Self::NewChatPhoto(val) => val.guest_query_id.as_deref(),
            Self::NewChatTitle(val) => val.guest_query_id.as_deref(),
            Self::PaidMedia(val) => val.guest_query_id.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.guest_query_id.as_deref(),
            Self::PassportData(val) => val.guest_query_id.as_deref(),
            Self::Photo(val) => val.guest_query_id.as_deref(),
            Self::PinnedMessage(val) => val.guest_query_id.as_deref(),
            Self::Poll(val) => val.guest_query_id.as_deref(),
            Self::PollOptionAdded(val) => val.guest_query_id.as_deref(),
            Self::PollOptionDeleted(val) => val.guest_query_id.as_deref(),
            Self::ProximityAlertTriggered(val) => val.guest_query_id.as_deref(),
            Self::RefundedPayment(val) => val.guest_query_id.as_deref(),
            Self::RichMessage(val) => val.guest_query_id.as_deref(),
            Self::Sticker(val) => val.guest_query_id.as_deref(),
            Self::Story(val) => val.guest_query_id.as_deref(),
            Self::SuccessfulPayment(val) => val.guest_query_id.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.guest_query_id.as_deref(),
            Self::SuggestedPostApproved(val) => val.guest_query_id.as_deref(),
            Self::SuggestedPostDeclined(val) => val.guest_query_id.as_deref(),
            Self::SuggestedPostPaid(val) => val.guest_query_id.as_deref(),
            Self::SuggestedPostRefunded(val) => val.guest_query_id.as_deref(),
            Self::SupergroupChatCreated(val) => val.guest_query_id.as_deref(),
            Self::Text(val) => val.guest_query_id.as_deref(),
            Self::UniqueGift(val) => val.guest_query_id.as_deref(),
            Self::UsersShared(val) => val.guest_query_id.as_deref(),
            Self::Video(val) => val.guest_query_id.as_deref(),
            Self::VideoChatEnded(val) => val.guest_query_id.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.guest_query_id.as_deref(),
            Self::VideoChatScheduled(val) => val.guest_query_id.as_deref(),
            Self::VideoChatStarted(val) => val.guest_query_id.as_deref(),
            Self::VideoNote(val) => val.guest_query_id.as_deref(),
            Self::Voice(val) => val.guest_query_id.as_deref(),
            Self::WebAppData(val) => val.guest_query_id.as_deref(),
            Self::WriteAccessAllowed(val) => val.guest_query_id.as_deref(),
            Self::Unknown(val) => val.guest_query_id.as_deref(),
        }
    }

    /// Helper method for field `has_media_spoiler`.
    ///
    /// `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.has_media_spoiler,
            Self::LivePhoto(val) => val.has_media_spoiler,
            Self::Venue(val) => val.has_media_spoiler,
            Self::Audio(val) => val.has_media_spoiler,
            Self::BoostAdded(val) => val.has_media_spoiler,
            Self::ChannelChatCreated(val) => val.has_media_spoiler,
            Self::ChatBackgroundSet(val) => val.has_media_spoiler,
            Self::ChatOwnerChanged(val) => val.has_media_spoiler,
            Self::ChatOwnerLeft(val) => val.has_media_spoiler,
            Self::ChatShared(val) => val.has_media_spoiler,
            Self::Checklist(val) => val.has_media_spoiler,
            Self::ChecklistTasksAdded(val) => val.has_media_spoiler,
            Self::ChecklistTasksDone(val) => val.has_media_spoiler,
            Self::CommunityChatAdded(val) => val.has_media_spoiler,
            Self::CommunityChatJoined(val) => val.has_media_spoiler,
            Self::CommunityChatRemoved(val) => val.has_media_spoiler,
            Self::ConnectedWebsite(val) => val.has_media_spoiler,
            Self::Contact(val) => val.has_media_spoiler,
            Self::DeleteChatPhoto(val) => val.has_media_spoiler,
            Self::Dice(val) => val.has_media_spoiler,
            Self::DirectMessagePriceChanged(val) => val.has_media_spoiler,
            Self::Document(val) => val.has_media_spoiler,
            Self::ForumTopicClosed(val) => val.has_media_spoiler,
            Self::ForumTopicCreated(val) => val.has_media_spoiler,
            Self::ForumTopicEdited(val) => val.has_media_spoiler,
            Self::ForumTopicReopened(val) => val.has_media_spoiler,
            Self::Game(val) => val.has_media_spoiler,
            Self::GeneralForumTopicHidden(val) => val.has_media_spoiler,
            Self::GeneralForumTopicUnhidden(val) => val.has_media_spoiler,
            Self::Gift(val) => val.has_media_spoiler,
            Self::GiftUpgradeSent(val) => val.has_media_spoiler,
            Self::Giveaway(val) => val.has_media_spoiler,
            Self::GiveawayCompleted(val) => val.has_media_spoiler,
            Self::GiveawayCreated(val) => val.has_media_spoiler,
            Self::GiveawayWinners(val) => val.has_media_spoiler,
            Self::GroupChatCreated(val) => val.has_media_spoiler,
            Self::Invoice(val) => val.has_media_spoiler,
            Self::LeftChatMember(val) => val.has_media_spoiler,
            Self::Location(val) => val.has_media_spoiler,
            Self::ManagedBotCreated(val) => val.has_media_spoiler,
            Self::MessageAutoDeleteTimerChanged(val) => val.has_media_spoiler,
            Self::MigrateFromChatId(val) => val.has_media_spoiler,
            Self::MigrateToChatId(val) => val.has_media_spoiler,
            Self::NewChatMembers(val) => val.has_media_spoiler,
            Self::NewChatPhoto(val) => val.has_media_spoiler,
            Self::NewChatTitle(val) => val.has_media_spoiler,
            Self::PaidMedia(val) => val.has_media_spoiler,
            Self::PaidMessagePriceChanged(val) => val.has_media_spoiler,
            Self::PassportData(val) => val.has_media_spoiler,
            Self::Photo(val) => val.has_media_spoiler,
            Self::PinnedMessage(val) => val.has_media_spoiler,
            Self::Poll(val) => val.has_media_spoiler,
            Self::PollOptionAdded(val) => val.has_media_spoiler,
            Self::PollOptionDeleted(val) => val.has_media_spoiler,
            Self::ProximityAlertTriggered(val) => val.has_media_spoiler,
            Self::RefundedPayment(val) => val.has_media_spoiler,
            Self::RichMessage(val) => val.has_media_spoiler,
            Self::Sticker(val) => val.has_media_spoiler,
            Self::Story(val) => val.has_media_spoiler,
            Self::SuccessfulPayment(val) => val.has_media_spoiler,
            Self::SuggestedPostApprovalFailed(val) => val.has_media_spoiler,
            Self::SuggestedPostApproved(val) => val.has_media_spoiler,
            Self::SuggestedPostDeclined(val) => val.has_media_spoiler,
            Self::SuggestedPostPaid(val) => val.has_media_spoiler,
            Self::SuggestedPostRefunded(val) => val.has_media_spoiler,
            Self::SupergroupChatCreated(val) => val.has_media_spoiler,
            Self::Text(val) => val.has_media_spoiler,
            Self::UniqueGift(val) => val.has_media_spoiler,
            Self::UsersShared(val) => val.has_media_spoiler,
            Self::Video(val) => val.has_media_spoiler,
            Self::VideoChatEnded(val) => val.has_media_spoiler,
            Self::VideoChatParticipantsInvited(val) => val.has_media_spoiler,
            Self::VideoChatScheduled(val) => val.has_media_spoiler,
            Self::VideoChatStarted(val) => val.has_media_spoiler,
            Self::VideoNote(val) => val.has_media_spoiler,
            Self::Voice(val) => val.has_media_spoiler,
            Self::WebAppData(val) => val.has_media_spoiler,
            Self::WriteAccessAllowed(val) => val.has_media_spoiler,
            Self::Unknown(val) => val.has_media_spoiler,
        }
    }

    /// Helper method for field `has_protected_content`.
    ///
    /// `true`, if the message can't be forwarded
    #[must_use]
    pub fn has_protected_content(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.has_protected_content,
            Self::LivePhoto(val) => val.has_protected_content,
            Self::Venue(val) => val.has_protected_content,
            Self::Audio(val) => val.has_protected_content,
            Self::BoostAdded(val) => val.has_protected_content,
            Self::ChannelChatCreated(val) => val.has_protected_content,
            Self::ChatBackgroundSet(val) => val.has_protected_content,
            Self::ChatOwnerChanged(val) => val.has_protected_content,
            Self::ChatOwnerLeft(val) => val.has_protected_content,
            Self::ChatShared(val) => val.has_protected_content,
            Self::Checklist(val) => val.has_protected_content,
            Self::ChecklistTasksAdded(val) => val.has_protected_content,
            Self::ChecklistTasksDone(val) => val.has_protected_content,
            Self::CommunityChatAdded(val) => val.has_protected_content,
            Self::CommunityChatJoined(val) => val.has_protected_content,
            Self::CommunityChatRemoved(val) => val.has_protected_content,
            Self::ConnectedWebsite(val) => val.has_protected_content,
            Self::Contact(val) => val.has_protected_content,
            Self::DeleteChatPhoto(val) => val.has_protected_content,
            Self::Dice(val) => val.has_protected_content,
            Self::DirectMessagePriceChanged(val) => val.has_protected_content,
            Self::Document(val) => val.has_protected_content,
            Self::ForumTopicClosed(val) => val.has_protected_content,
            Self::ForumTopicCreated(val) => val.has_protected_content,
            Self::ForumTopicEdited(val) => val.has_protected_content,
            Self::ForumTopicReopened(val) => val.has_protected_content,
            Self::Game(val) => val.has_protected_content,
            Self::GeneralForumTopicHidden(val) => val.has_protected_content,
            Self::GeneralForumTopicUnhidden(val) => val.has_protected_content,
            Self::Gift(val) => val.has_protected_content,
            Self::GiftUpgradeSent(val) => val.has_protected_content,
            Self::Giveaway(val) => val.has_protected_content,
            Self::GiveawayCompleted(val) => val.has_protected_content,
            Self::GiveawayCreated(val) => val.has_protected_content,
            Self::GiveawayWinners(val) => val.has_protected_content,
            Self::GroupChatCreated(val) => val.has_protected_content,
            Self::Invoice(val) => val.has_protected_content,
            Self::LeftChatMember(val) => val.has_protected_content,
            Self::Location(val) => val.has_protected_content,
            Self::ManagedBotCreated(val) => val.has_protected_content,
            Self::MessageAutoDeleteTimerChanged(val) => val.has_protected_content,
            Self::MigrateFromChatId(val) => val.has_protected_content,
            Self::MigrateToChatId(val) => val.has_protected_content,
            Self::NewChatMembers(val) => val.has_protected_content,
            Self::NewChatPhoto(val) => val.has_protected_content,
            Self::NewChatTitle(val) => val.has_protected_content,
            Self::PaidMedia(val) => val.has_protected_content,
            Self::PaidMessagePriceChanged(val) => val.has_protected_content,
            Self::PassportData(val) => val.has_protected_content,
            Self::Photo(val) => val.has_protected_content,
            Self::PinnedMessage(val) => val.has_protected_content,
            Self::Poll(val) => val.has_protected_content,
            Self::PollOptionAdded(val) => val.has_protected_content,
            Self::PollOptionDeleted(val) => val.has_protected_content,
            Self::ProximityAlertTriggered(val) => val.has_protected_content,
            Self::RefundedPayment(val) => val.has_protected_content,
            Self::RichMessage(val) => val.has_protected_content,
            Self::Sticker(val) => val.has_protected_content,
            Self::Story(val) => val.has_protected_content,
            Self::SuccessfulPayment(val) => val.has_protected_content,
            Self::SuggestedPostApprovalFailed(val) => val.has_protected_content,
            Self::SuggestedPostApproved(val) => val.has_protected_content,
            Self::SuggestedPostDeclined(val) => val.has_protected_content,
            Self::SuggestedPostPaid(val) => val.has_protected_content,
            Self::SuggestedPostRefunded(val) => val.has_protected_content,
            Self::SupergroupChatCreated(val) => val.has_protected_content,
            Self::Text(val) => val.has_protected_content,
            Self::UniqueGift(val) => val.has_protected_content,
            Self::UsersShared(val) => val.has_protected_content,
            Self::Video(val) => val.has_protected_content,
            Self::VideoChatEnded(val) => val.has_protected_content,
            Self::VideoChatParticipantsInvited(val) => val.has_protected_content,
            Self::VideoChatScheduled(val) => val.has_protected_content,
            Self::VideoChatStarted(val) => val.has_protected_content,
            Self::VideoNote(val) => val.has_protected_content,
            Self::Voice(val) => val.has_protected_content,
            Self::WebAppData(val) => val.has_protected_content,
            Self::WriteAccessAllowed(val) => val.has_protected_content,
            Self::Unknown(val) => val.has_protected_content,
        }
    }

    /// Helper method for field `invoice`.
    ///
    /// Message is an invoice for a payment, information about the invoice. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn invoice(&self) -> Option<&crate::types::Invoice> {
        match self {
            Self::Invoice(val) => Some(&val.invoice),
            _ => None,
        }
    }

    /// Helper method for field `is_automatic_forward`.
    ///
    /// `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[must_use]
    pub fn is_automatic_forward(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.is_automatic_forward,
            Self::LivePhoto(val) => val.is_automatic_forward,
            Self::Venue(val) => val.is_automatic_forward,
            Self::Audio(val) => val.is_automatic_forward,
            Self::BoostAdded(val) => val.is_automatic_forward,
            Self::ChannelChatCreated(val) => val.is_automatic_forward,
            Self::ChatBackgroundSet(val) => val.is_automatic_forward,
            Self::ChatOwnerChanged(val) => val.is_automatic_forward,
            Self::ChatOwnerLeft(val) => val.is_automatic_forward,
            Self::ChatShared(val) => val.is_automatic_forward,
            Self::Checklist(val) => val.is_automatic_forward,
            Self::ChecklistTasksAdded(val) => val.is_automatic_forward,
            Self::ChecklistTasksDone(val) => val.is_automatic_forward,
            Self::CommunityChatAdded(val) => val.is_automatic_forward,
            Self::CommunityChatJoined(val) => val.is_automatic_forward,
            Self::CommunityChatRemoved(val) => val.is_automatic_forward,
            Self::ConnectedWebsite(val) => val.is_automatic_forward,
            Self::Contact(val) => val.is_automatic_forward,
            Self::DeleteChatPhoto(val) => val.is_automatic_forward,
            Self::Dice(val) => val.is_automatic_forward,
            Self::DirectMessagePriceChanged(val) => val.is_automatic_forward,
            Self::Document(val) => val.is_automatic_forward,
            Self::ForumTopicClosed(val) => val.is_automatic_forward,
            Self::ForumTopicCreated(val) => val.is_automatic_forward,
            Self::ForumTopicEdited(val) => val.is_automatic_forward,
            Self::ForumTopicReopened(val) => val.is_automatic_forward,
            Self::Game(val) => val.is_automatic_forward,
            Self::GeneralForumTopicHidden(val) => val.is_automatic_forward,
            Self::GeneralForumTopicUnhidden(val) => val.is_automatic_forward,
            Self::Gift(val) => val.is_automatic_forward,
            Self::GiftUpgradeSent(val) => val.is_automatic_forward,
            Self::Giveaway(val) => val.is_automatic_forward,
            Self::GiveawayCompleted(val) => val.is_automatic_forward,
            Self::GiveawayCreated(val) => val.is_automatic_forward,
            Self::GiveawayWinners(val) => val.is_automatic_forward,
            Self::GroupChatCreated(val) => val.is_automatic_forward,
            Self::Invoice(val) => val.is_automatic_forward,
            Self::LeftChatMember(val) => val.is_automatic_forward,
            Self::Location(val) => val.is_automatic_forward,
            Self::ManagedBotCreated(val) => val.is_automatic_forward,
            Self::MessageAutoDeleteTimerChanged(val) => val.is_automatic_forward,
            Self::MigrateFromChatId(val) => val.is_automatic_forward,
            Self::MigrateToChatId(val) => val.is_automatic_forward,
            Self::NewChatMembers(val) => val.is_automatic_forward,
            Self::NewChatPhoto(val) => val.is_automatic_forward,
            Self::NewChatTitle(val) => val.is_automatic_forward,
            Self::PaidMedia(val) => val.is_automatic_forward,
            Self::PaidMessagePriceChanged(val) => val.is_automatic_forward,
            Self::PassportData(val) => val.is_automatic_forward,
            Self::Photo(val) => val.is_automatic_forward,
            Self::PinnedMessage(val) => val.is_automatic_forward,
            Self::Poll(val) => val.is_automatic_forward,
            Self::PollOptionAdded(val) => val.is_automatic_forward,
            Self::PollOptionDeleted(val) => val.is_automatic_forward,
            Self::ProximityAlertTriggered(val) => val.is_automatic_forward,
            Self::RefundedPayment(val) => val.is_automatic_forward,
            Self::RichMessage(val) => val.is_automatic_forward,
            Self::Sticker(val) => val.is_automatic_forward,
            Self::Story(val) => val.is_automatic_forward,
            Self::SuccessfulPayment(val) => val.is_automatic_forward,
            Self::SuggestedPostApprovalFailed(val) => val.is_automatic_forward,
            Self::SuggestedPostApproved(val) => val.is_automatic_forward,
            Self::SuggestedPostDeclined(val) => val.is_automatic_forward,
            Self::SuggestedPostPaid(val) => val.is_automatic_forward,
            Self::SuggestedPostRefunded(val) => val.is_automatic_forward,
            Self::SupergroupChatCreated(val) => val.is_automatic_forward,
            Self::Text(val) => val.is_automatic_forward,
            Self::UniqueGift(val) => val.is_automatic_forward,
            Self::UsersShared(val) => val.is_automatic_forward,
            Self::Video(val) => val.is_automatic_forward,
            Self::VideoChatEnded(val) => val.is_automatic_forward,
            Self::VideoChatParticipantsInvited(val) => val.is_automatic_forward,
            Self::VideoChatScheduled(val) => val.is_automatic_forward,
            Self::VideoChatStarted(val) => val.is_automatic_forward,
            Self::VideoNote(val) => val.is_automatic_forward,
            Self::Voice(val) => val.is_automatic_forward,
            Self::WebAppData(val) => val.is_automatic_forward,
            Self::WriteAccessAllowed(val) => val.is_automatic_forward,
            Self::Unknown(val) => val.is_automatic_forward,
        }
    }

    /// Helper method for field `is_from_offline`.
    ///
    /// `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[must_use]
    pub fn is_from_offline(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.is_from_offline,
            Self::LivePhoto(val) => val.is_from_offline,
            Self::Venue(val) => val.is_from_offline,
            Self::Audio(val) => val.is_from_offline,
            Self::BoostAdded(val) => val.is_from_offline,
            Self::ChannelChatCreated(val) => val.is_from_offline,
            Self::ChatBackgroundSet(val) => val.is_from_offline,
            Self::ChatOwnerChanged(val) => val.is_from_offline,
            Self::ChatOwnerLeft(val) => val.is_from_offline,
            Self::ChatShared(val) => val.is_from_offline,
            Self::Checklist(val) => val.is_from_offline,
            Self::ChecklistTasksAdded(val) => val.is_from_offline,
            Self::ChecklistTasksDone(val) => val.is_from_offline,
            Self::CommunityChatAdded(val) => val.is_from_offline,
            Self::CommunityChatJoined(val) => val.is_from_offline,
            Self::CommunityChatRemoved(val) => val.is_from_offline,
            Self::ConnectedWebsite(val) => val.is_from_offline,
            Self::Contact(val) => val.is_from_offline,
            Self::DeleteChatPhoto(val) => val.is_from_offline,
            Self::Dice(val) => val.is_from_offline,
            Self::DirectMessagePriceChanged(val) => val.is_from_offline,
            Self::Document(val) => val.is_from_offline,
            Self::ForumTopicClosed(val) => val.is_from_offline,
            Self::ForumTopicCreated(val) => val.is_from_offline,
            Self::ForumTopicEdited(val) => val.is_from_offline,
            Self::ForumTopicReopened(val) => val.is_from_offline,
            Self::Game(val) => val.is_from_offline,
            Self::GeneralForumTopicHidden(val) => val.is_from_offline,
            Self::GeneralForumTopicUnhidden(val) => val.is_from_offline,
            Self::Gift(val) => val.is_from_offline,
            Self::GiftUpgradeSent(val) => val.is_from_offline,
            Self::Giveaway(val) => val.is_from_offline,
            Self::GiveawayCompleted(val) => val.is_from_offline,
            Self::GiveawayCreated(val) => val.is_from_offline,
            Self::GiveawayWinners(val) => val.is_from_offline,
            Self::GroupChatCreated(val) => val.is_from_offline,
            Self::Invoice(val) => val.is_from_offline,
            Self::LeftChatMember(val) => val.is_from_offline,
            Self::Location(val) => val.is_from_offline,
            Self::ManagedBotCreated(val) => val.is_from_offline,
            Self::MessageAutoDeleteTimerChanged(val) => val.is_from_offline,
            Self::MigrateFromChatId(val) => val.is_from_offline,
            Self::MigrateToChatId(val) => val.is_from_offline,
            Self::NewChatMembers(val) => val.is_from_offline,
            Self::NewChatPhoto(val) => val.is_from_offline,
            Self::NewChatTitle(val) => val.is_from_offline,
            Self::PaidMedia(val) => val.is_from_offline,
            Self::PaidMessagePriceChanged(val) => val.is_from_offline,
            Self::PassportData(val) => val.is_from_offline,
            Self::Photo(val) => val.is_from_offline,
            Self::PinnedMessage(val) => val.is_from_offline,
            Self::Poll(val) => val.is_from_offline,
            Self::PollOptionAdded(val) => val.is_from_offline,
            Self::PollOptionDeleted(val) => val.is_from_offline,
            Self::ProximityAlertTriggered(val) => val.is_from_offline,
            Self::RefundedPayment(val) => val.is_from_offline,
            Self::RichMessage(val) => val.is_from_offline,
            Self::Sticker(val) => val.is_from_offline,
            Self::Story(val) => val.is_from_offline,
            Self::SuccessfulPayment(val) => val.is_from_offline,
            Self::SuggestedPostApprovalFailed(val) => val.is_from_offline,
            Self::SuggestedPostApproved(val) => val.is_from_offline,
            Self::SuggestedPostDeclined(val) => val.is_from_offline,
            Self::SuggestedPostPaid(val) => val.is_from_offline,
            Self::SuggestedPostRefunded(val) => val.is_from_offline,
            Self::SupergroupChatCreated(val) => val.is_from_offline,
            Self::Text(val) => val.is_from_offline,
            Self::UniqueGift(val) => val.is_from_offline,
            Self::UsersShared(val) => val.is_from_offline,
            Self::Video(val) => val.is_from_offline,
            Self::VideoChatEnded(val) => val.is_from_offline,
            Self::VideoChatParticipantsInvited(val) => val.is_from_offline,
            Self::VideoChatScheduled(val) => val.is_from_offline,
            Self::VideoChatStarted(val) => val.is_from_offline,
            Self::VideoNote(val) => val.is_from_offline,
            Self::Voice(val) => val.is_from_offline,
            Self::WebAppData(val) => val.is_from_offline,
            Self::WriteAccessAllowed(val) => val.is_from_offline,
            Self::Unknown(val) => val.is_from_offline,
        }
    }

    /// Helper method for field `is_paid_post`.
    ///
    /// `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    #[must_use]
    pub fn is_paid_post(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.is_paid_post,
            Self::LivePhoto(val) => val.is_paid_post,
            Self::Venue(val) => val.is_paid_post,
            Self::Audio(val) => val.is_paid_post,
            Self::BoostAdded(val) => val.is_paid_post,
            Self::ChannelChatCreated(val) => val.is_paid_post,
            Self::ChatBackgroundSet(val) => val.is_paid_post,
            Self::ChatOwnerChanged(val) => val.is_paid_post,
            Self::ChatOwnerLeft(val) => val.is_paid_post,
            Self::ChatShared(val) => val.is_paid_post,
            Self::Checklist(val) => val.is_paid_post,
            Self::ChecklistTasksAdded(val) => val.is_paid_post,
            Self::ChecklistTasksDone(val) => val.is_paid_post,
            Self::CommunityChatAdded(val) => val.is_paid_post,
            Self::CommunityChatJoined(val) => val.is_paid_post,
            Self::CommunityChatRemoved(val) => val.is_paid_post,
            Self::ConnectedWebsite(val) => val.is_paid_post,
            Self::Contact(val) => val.is_paid_post,
            Self::DeleteChatPhoto(val) => val.is_paid_post,
            Self::Dice(val) => val.is_paid_post,
            Self::DirectMessagePriceChanged(val) => val.is_paid_post,
            Self::Document(val) => val.is_paid_post,
            Self::ForumTopicClosed(val) => val.is_paid_post,
            Self::ForumTopicCreated(val) => val.is_paid_post,
            Self::ForumTopicEdited(val) => val.is_paid_post,
            Self::ForumTopicReopened(val) => val.is_paid_post,
            Self::Game(val) => val.is_paid_post,
            Self::GeneralForumTopicHidden(val) => val.is_paid_post,
            Self::GeneralForumTopicUnhidden(val) => val.is_paid_post,
            Self::Gift(val) => val.is_paid_post,
            Self::GiftUpgradeSent(val) => val.is_paid_post,
            Self::Giveaway(val) => val.is_paid_post,
            Self::GiveawayCompleted(val) => val.is_paid_post,
            Self::GiveawayCreated(val) => val.is_paid_post,
            Self::GiveawayWinners(val) => val.is_paid_post,
            Self::GroupChatCreated(val) => val.is_paid_post,
            Self::Invoice(val) => val.is_paid_post,
            Self::LeftChatMember(val) => val.is_paid_post,
            Self::Location(val) => val.is_paid_post,
            Self::ManagedBotCreated(val) => val.is_paid_post,
            Self::MessageAutoDeleteTimerChanged(val) => val.is_paid_post,
            Self::MigrateFromChatId(val) => val.is_paid_post,
            Self::MigrateToChatId(val) => val.is_paid_post,
            Self::NewChatMembers(val) => val.is_paid_post,
            Self::NewChatPhoto(val) => val.is_paid_post,
            Self::NewChatTitle(val) => val.is_paid_post,
            Self::PaidMedia(val) => val.is_paid_post,
            Self::PaidMessagePriceChanged(val) => val.is_paid_post,
            Self::PassportData(val) => val.is_paid_post,
            Self::Photo(val) => val.is_paid_post,
            Self::PinnedMessage(val) => val.is_paid_post,
            Self::Poll(val) => val.is_paid_post,
            Self::PollOptionAdded(val) => val.is_paid_post,
            Self::PollOptionDeleted(val) => val.is_paid_post,
            Self::ProximityAlertTriggered(val) => val.is_paid_post,
            Self::RefundedPayment(val) => val.is_paid_post,
            Self::RichMessage(val) => val.is_paid_post,
            Self::Sticker(val) => val.is_paid_post,
            Self::Story(val) => val.is_paid_post,
            Self::SuccessfulPayment(val) => val.is_paid_post,
            Self::SuggestedPostApprovalFailed(val) => val.is_paid_post,
            Self::SuggestedPostApproved(val) => val.is_paid_post,
            Self::SuggestedPostDeclined(val) => val.is_paid_post,
            Self::SuggestedPostPaid(val) => val.is_paid_post,
            Self::SuggestedPostRefunded(val) => val.is_paid_post,
            Self::SupergroupChatCreated(val) => val.is_paid_post,
            Self::Text(val) => val.is_paid_post,
            Self::UniqueGift(val) => val.is_paid_post,
            Self::UsersShared(val) => val.is_paid_post,
            Self::Video(val) => val.is_paid_post,
            Self::VideoChatEnded(val) => val.is_paid_post,
            Self::VideoChatParticipantsInvited(val) => val.is_paid_post,
            Self::VideoChatScheduled(val) => val.is_paid_post,
            Self::VideoChatStarted(val) => val.is_paid_post,
            Self::VideoNote(val) => val.is_paid_post,
            Self::Voice(val) => val.is_paid_post,
            Self::WebAppData(val) => val.is_paid_post,
            Self::WriteAccessAllowed(val) => val.is_paid_post,
            Self::Unknown(val) => val.is_paid_post,
        }
    }

    /// Helper method for field `is_topic_message`.
    ///
    /// `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    #[must_use]
    pub fn is_topic_message(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.is_topic_message,
            Self::LivePhoto(val) => val.is_topic_message,
            Self::Venue(val) => val.is_topic_message,
            Self::Audio(val) => val.is_topic_message,
            Self::BoostAdded(val) => val.is_topic_message,
            Self::ChannelChatCreated(val) => val.is_topic_message,
            Self::ChatBackgroundSet(val) => val.is_topic_message,
            Self::ChatOwnerChanged(val) => val.is_topic_message,
            Self::ChatOwnerLeft(val) => val.is_topic_message,
            Self::ChatShared(val) => val.is_topic_message,
            Self::Checklist(val) => val.is_topic_message,
            Self::ChecklistTasksAdded(val) => val.is_topic_message,
            Self::ChecklistTasksDone(val) => val.is_topic_message,
            Self::CommunityChatAdded(val) => val.is_topic_message,
            Self::CommunityChatJoined(val) => val.is_topic_message,
            Self::CommunityChatRemoved(val) => val.is_topic_message,
            Self::ConnectedWebsite(val) => val.is_topic_message,
            Self::Contact(val) => val.is_topic_message,
            Self::DeleteChatPhoto(val) => val.is_topic_message,
            Self::Dice(val) => val.is_topic_message,
            Self::DirectMessagePriceChanged(val) => val.is_topic_message,
            Self::Document(val) => val.is_topic_message,
            Self::ForumTopicClosed(val) => val.is_topic_message,
            Self::ForumTopicCreated(val) => val.is_topic_message,
            Self::ForumTopicEdited(val) => val.is_topic_message,
            Self::ForumTopicReopened(val) => val.is_topic_message,
            Self::Game(val) => val.is_topic_message,
            Self::GeneralForumTopicHidden(val) => val.is_topic_message,
            Self::GeneralForumTopicUnhidden(val) => val.is_topic_message,
            Self::Gift(val) => val.is_topic_message,
            Self::GiftUpgradeSent(val) => val.is_topic_message,
            Self::Giveaway(val) => val.is_topic_message,
            Self::GiveawayCompleted(val) => val.is_topic_message,
            Self::GiveawayCreated(val) => val.is_topic_message,
            Self::GiveawayWinners(val) => val.is_topic_message,
            Self::GroupChatCreated(val) => val.is_topic_message,
            Self::Invoice(val) => val.is_topic_message,
            Self::LeftChatMember(val) => val.is_topic_message,
            Self::Location(val) => val.is_topic_message,
            Self::ManagedBotCreated(val) => val.is_topic_message,
            Self::MessageAutoDeleteTimerChanged(val) => val.is_topic_message,
            Self::MigrateFromChatId(val) => val.is_topic_message,
            Self::MigrateToChatId(val) => val.is_topic_message,
            Self::NewChatMembers(val) => val.is_topic_message,
            Self::NewChatPhoto(val) => val.is_topic_message,
            Self::NewChatTitle(val) => val.is_topic_message,
            Self::PaidMedia(val) => val.is_topic_message,
            Self::PaidMessagePriceChanged(val) => val.is_topic_message,
            Self::PassportData(val) => val.is_topic_message,
            Self::Photo(val) => val.is_topic_message,
            Self::PinnedMessage(val) => val.is_topic_message,
            Self::Poll(val) => val.is_topic_message,
            Self::PollOptionAdded(val) => val.is_topic_message,
            Self::PollOptionDeleted(val) => val.is_topic_message,
            Self::ProximityAlertTriggered(val) => val.is_topic_message,
            Self::RefundedPayment(val) => val.is_topic_message,
            Self::RichMessage(val) => val.is_topic_message,
            Self::Sticker(val) => val.is_topic_message,
            Self::Story(val) => val.is_topic_message,
            Self::SuccessfulPayment(val) => val.is_topic_message,
            Self::SuggestedPostApprovalFailed(val) => val.is_topic_message,
            Self::SuggestedPostApproved(val) => val.is_topic_message,
            Self::SuggestedPostDeclined(val) => val.is_topic_message,
            Self::SuggestedPostPaid(val) => val.is_topic_message,
            Self::SuggestedPostRefunded(val) => val.is_topic_message,
            Self::SupergroupChatCreated(val) => val.is_topic_message,
            Self::Text(val) => val.is_topic_message,
            Self::UniqueGift(val) => val.is_topic_message,
            Self::UsersShared(val) => val.is_topic_message,
            Self::Video(val) => val.is_topic_message,
            Self::VideoChatEnded(val) => val.is_topic_message,
            Self::VideoChatParticipantsInvited(val) => val.is_topic_message,
            Self::VideoChatScheduled(val) => val.is_topic_message,
            Self::VideoChatStarted(val) => val.is_topic_message,
            Self::VideoNote(val) => val.is_topic_message,
            Self::Voice(val) => val.is_topic_message,
            Self::WebAppData(val) => val.is_topic_message,
            Self::WriteAccessAllowed(val) => val.is_topic_message,
            Self::Unknown(val) => val.is_topic_message,
        }
    }

    /// Helper method for field `left_chat_member`.
    ///
    /// A member was removed from the group, information about them (this member may be the bot itself)
    #[must_use]
    pub fn left_chat_member(&self) -> Option<&crate::types::User> {
        match self {
            Self::LeftChatMember(val) => Some(val.left_chat_member.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `link_preview_options`.
    ///
    /// Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        match self {
            Self::Animation(val) => val.link_preview_options.as_ref(),
            Self::LivePhoto(val) => val.link_preview_options.as_ref(),
            Self::Venue(val) => val.link_preview_options.as_ref(),
            Self::Audio(val) => val.link_preview_options.as_ref(),
            Self::BoostAdded(val) => val.link_preview_options.as_ref(),
            Self::ChannelChatCreated(val) => val.link_preview_options.as_ref(),
            Self::ChatBackgroundSet(val) => val.link_preview_options.as_ref(),
            Self::ChatOwnerChanged(val) => val.link_preview_options.as_ref(),
            Self::ChatOwnerLeft(val) => val.link_preview_options.as_ref(),
            Self::ChatShared(val) => val.link_preview_options.as_ref(),
            Self::Checklist(val) => val.link_preview_options.as_ref(),
            Self::ChecklistTasksAdded(val) => val.link_preview_options.as_ref(),
            Self::ChecklistTasksDone(val) => val.link_preview_options.as_ref(),
            Self::CommunityChatAdded(val) => val.link_preview_options.as_ref(),
            Self::CommunityChatJoined(val) => val.link_preview_options.as_ref(),
            Self::CommunityChatRemoved(val) => val.link_preview_options.as_ref(),
            Self::ConnectedWebsite(val) => val.link_preview_options.as_ref(),
            Self::Contact(val) => val.link_preview_options.as_ref(),
            Self::DeleteChatPhoto(val) => val.link_preview_options.as_ref(),
            Self::Dice(val) => val.link_preview_options.as_ref(),
            Self::DirectMessagePriceChanged(val) => val.link_preview_options.as_ref(),
            Self::Document(val) => val.link_preview_options.as_ref(),
            Self::ForumTopicClosed(val) => val.link_preview_options.as_ref(),
            Self::ForumTopicCreated(val) => val.link_preview_options.as_ref(),
            Self::ForumTopicEdited(val) => val.link_preview_options.as_ref(),
            Self::ForumTopicReopened(val) => val.link_preview_options.as_ref(),
            Self::Game(val) => val.link_preview_options.as_ref(),
            Self::GeneralForumTopicHidden(val) => val.link_preview_options.as_ref(),
            Self::GeneralForumTopicUnhidden(val) => val.link_preview_options.as_ref(),
            Self::Gift(val) => val.link_preview_options.as_ref(),
            Self::GiftUpgradeSent(val) => val.link_preview_options.as_ref(),
            Self::Giveaway(val) => val.link_preview_options.as_ref(),
            Self::GiveawayCompleted(val) => val.link_preview_options.as_ref(),
            Self::GiveawayCreated(val) => val.link_preview_options.as_ref(),
            Self::GiveawayWinners(val) => val.link_preview_options.as_ref(),
            Self::GroupChatCreated(val) => val.link_preview_options.as_ref(),
            Self::Invoice(val) => val.link_preview_options.as_ref(),
            Self::LeftChatMember(val) => val.link_preview_options.as_ref(),
            Self::Location(val) => val.link_preview_options.as_ref(),
            Self::ManagedBotCreated(val) => val.link_preview_options.as_ref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.link_preview_options.as_ref(),
            Self::MigrateFromChatId(val) => val.link_preview_options.as_ref(),
            Self::MigrateToChatId(val) => val.link_preview_options.as_ref(),
            Self::NewChatMembers(val) => val.link_preview_options.as_ref(),
            Self::NewChatPhoto(val) => val.link_preview_options.as_ref(),
            Self::NewChatTitle(val) => val.link_preview_options.as_ref(),
            Self::PaidMedia(val) => val.link_preview_options.as_ref(),
            Self::PaidMessagePriceChanged(val) => val.link_preview_options.as_ref(),
            Self::PassportData(val) => val.link_preview_options.as_ref(),
            Self::Photo(val) => val.link_preview_options.as_ref(),
            Self::PinnedMessage(val) => val.link_preview_options.as_ref(),
            Self::Poll(val) => val.link_preview_options.as_ref(),
            Self::PollOptionAdded(val) => val.link_preview_options.as_ref(),
            Self::PollOptionDeleted(val) => val.link_preview_options.as_ref(),
            Self::ProximityAlertTriggered(val) => val.link_preview_options.as_ref(),
            Self::RefundedPayment(val) => val.link_preview_options.as_ref(),
            Self::RichMessage(val) => val.link_preview_options.as_ref(),
            Self::Sticker(val) => val.link_preview_options.as_ref(),
            Self::Story(val) => val.link_preview_options.as_ref(),
            Self::SuccessfulPayment(val) => val.link_preview_options.as_ref(),
            Self::SuggestedPostApprovalFailed(val) => val.link_preview_options.as_ref(),
            Self::SuggestedPostApproved(val) => val.link_preview_options.as_ref(),
            Self::SuggestedPostDeclined(val) => val.link_preview_options.as_ref(),
            Self::SuggestedPostPaid(val) => val.link_preview_options.as_ref(),
            Self::SuggestedPostRefunded(val) => val.link_preview_options.as_ref(),
            Self::SupergroupChatCreated(val) => val.link_preview_options.as_ref(),
            Self::Text(val) => val.link_preview_options.as_ref(),
            Self::UniqueGift(val) => val.link_preview_options.as_ref(),
            Self::UsersShared(val) => val.link_preview_options.as_ref(),
            Self::Video(val) => val.link_preview_options.as_ref(),
            Self::VideoChatEnded(val) => val.link_preview_options.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.link_preview_options.as_ref(),
            Self::VideoChatScheduled(val) => val.link_preview_options.as_ref(),
            Self::VideoChatStarted(val) => val.link_preview_options.as_ref(),
            Self::VideoNote(val) => val.link_preview_options.as_ref(),
            Self::Voice(val) => val.link_preview_options.as_ref(),
            Self::WebAppData(val) => val.link_preview_options.as_ref(),
            Self::WriteAccessAllowed(val) => val.link_preview_options.as_ref(),
            Self::Unknown(val) => val.link_preview_options.as_ref(),
        }
    }

    /// Helper method for field `live_photo`.
    ///
    /// Message is a live photo, information about the live photo. For backward compatibility, when this field is set, the photo field will also be set.
    #[must_use]
    pub fn live_photo(&self) -> Option<&crate::types::LivePhoto> {
        match self {
            Self::LivePhoto(val) => Some(&val.live_photo),
            _ => None,
        }
    }

    /// Helper method for field `location`.
    ///
    /// Message is a shared location, information about the location
    #[must_use]
    pub fn location(&self) -> Option<&crate::types::Location> {
        match self {
            Self::Location(val) => Some(&val.location),
            _ => None,
        }
    }

    /// Helper method for field `managed_bot_created`.
    ///
    /// Service message: user created a bot that will be managed by the current bot
    #[must_use]
    pub fn managed_bot_created(&self) -> Option<&crate::types::ManagedBotCreated> {
        match self {
            Self::ManagedBotCreated(val) => Some(&val.managed_bot_created),
            _ => None,
        }
    }

    /// Helper method for field `media_group_id`.
    ///
    /// The unique identifier inside this chat of a media message group this message belongs to
    #[must_use]
    pub fn media_group_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.media_group_id.as_deref(),
            Self::LivePhoto(val) => val.media_group_id.as_deref(),
            Self::Venue(val) => val.media_group_id.as_deref(),
            Self::Audio(val) => val.media_group_id.as_deref(),
            Self::BoostAdded(val) => val.media_group_id.as_deref(),
            Self::ChannelChatCreated(val) => val.media_group_id.as_deref(),
            Self::ChatBackgroundSet(val) => val.media_group_id.as_deref(),
            Self::ChatOwnerChanged(val) => val.media_group_id.as_deref(),
            Self::ChatOwnerLeft(val) => val.media_group_id.as_deref(),
            Self::ChatShared(val) => val.media_group_id.as_deref(),
            Self::Checklist(val) => val.media_group_id.as_deref(),
            Self::ChecklistTasksAdded(val) => val.media_group_id.as_deref(),
            Self::ChecklistTasksDone(val) => val.media_group_id.as_deref(),
            Self::CommunityChatAdded(val) => val.media_group_id.as_deref(),
            Self::CommunityChatJoined(val) => val.media_group_id.as_deref(),
            Self::CommunityChatRemoved(val) => val.media_group_id.as_deref(),
            Self::ConnectedWebsite(val) => val.media_group_id.as_deref(),
            Self::Contact(val) => val.media_group_id.as_deref(),
            Self::DeleteChatPhoto(val) => val.media_group_id.as_deref(),
            Self::Dice(val) => val.media_group_id.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.media_group_id.as_deref(),
            Self::Document(val) => val.media_group_id.as_deref(),
            Self::ForumTopicClosed(val) => val.media_group_id.as_deref(),
            Self::ForumTopicCreated(val) => val.media_group_id.as_deref(),
            Self::ForumTopicEdited(val) => val.media_group_id.as_deref(),
            Self::ForumTopicReopened(val) => val.media_group_id.as_deref(),
            Self::Game(val) => val.media_group_id.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.media_group_id.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.media_group_id.as_deref(),
            Self::Gift(val) => val.media_group_id.as_deref(),
            Self::GiftUpgradeSent(val) => val.media_group_id.as_deref(),
            Self::Giveaway(val) => val.media_group_id.as_deref(),
            Self::GiveawayCompleted(val) => val.media_group_id.as_deref(),
            Self::GiveawayCreated(val) => val.media_group_id.as_deref(),
            Self::GiveawayWinners(val) => val.media_group_id.as_deref(),
            Self::GroupChatCreated(val) => val.media_group_id.as_deref(),
            Self::Invoice(val) => val.media_group_id.as_deref(),
            Self::LeftChatMember(val) => val.media_group_id.as_deref(),
            Self::Location(val) => val.media_group_id.as_deref(),
            Self::ManagedBotCreated(val) => val.media_group_id.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.media_group_id.as_deref(),
            Self::MigrateFromChatId(val) => val.media_group_id.as_deref(),
            Self::MigrateToChatId(val) => val.media_group_id.as_deref(),
            Self::NewChatMembers(val) => val.media_group_id.as_deref(),
            Self::NewChatPhoto(val) => val.media_group_id.as_deref(),
            Self::NewChatTitle(val) => val.media_group_id.as_deref(),
            Self::PaidMedia(val) => val.media_group_id.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.media_group_id.as_deref(),
            Self::PassportData(val) => val.media_group_id.as_deref(),
            Self::Photo(val) => val.media_group_id.as_deref(),
            Self::PinnedMessage(val) => val.media_group_id.as_deref(),
            Self::Poll(val) => val.media_group_id.as_deref(),
            Self::PollOptionAdded(val) => val.media_group_id.as_deref(),
            Self::PollOptionDeleted(val) => val.media_group_id.as_deref(),
            Self::ProximityAlertTriggered(val) => val.media_group_id.as_deref(),
            Self::RefundedPayment(val) => val.media_group_id.as_deref(),
            Self::RichMessage(val) => val.media_group_id.as_deref(),
            Self::Sticker(val) => val.media_group_id.as_deref(),
            Self::Story(val) => val.media_group_id.as_deref(),
            Self::SuccessfulPayment(val) => val.media_group_id.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.media_group_id.as_deref(),
            Self::SuggestedPostApproved(val) => val.media_group_id.as_deref(),
            Self::SuggestedPostDeclined(val) => val.media_group_id.as_deref(),
            Self::SuggestedPostPaid(val) => val.media_group_id.as_deref(),
            Self::SuggestedPostRefunded(val) => val.media_group_id.as_deref(),
            Self::SupergroupChatCreated(val) => val.media_group_id.as_deref(),
            Self::Text(val) => val.media_group_id.as_deref(),
            Self::UniqueGift(val) => val.media_group_id.as_deref(),
            Self::UsersShared(val) => val.media_group_id.as_deref(),
            Self::Video(val) => val.media_group_id.as_deref(),
            Self::VideoChatEnded(val) => val.media_group_id.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.media_group_id.as_deref(),
            Self::VideoChatScheduled(val) => val.media_group_id.as_deref(),
            Self::VideoChatStarted(val) => val.media_group_id.as_deref(),
            Self::VideoNote(val) => val.media_group_id.as_deref(),
            Self::Voice(val) => val.media_group_id.as_deref(),
            Self::WebAppData(val) => val.media_group_id.as_deref(),
            Self::WriteAccessAllowed(val) => val.media_group_id.as_deref(),
            Self::Unknown(val) => val.media_group_id.as_deref(),
        }
    }

    /// Helper method for field `message_auto_delete_timer_changed`.
    ///
    /// Service message: auto-delete timer settings changed in the chat
    #[must_use]
    pub fn message_auto_delete_timer_changed(
        &self,
    ) -> Option<&crate::types::MessageAutoDeleteTimerChanged> {
        match self {
            Self::MessageAutoDeleteTimerChanged(val) => {
                Some(&val.message_auto_delete_timer_changed)
            }
            _ => None,
        }
    }

    /// Helper method for field `message_id`.
    ///
    /// Unique message identifier inside this chat; 0 for ephemeral messages. In specific instances (e.g., a message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent.
    #[must_use]
    pub fn message_id(&self) -> i64 {
        match self {
            Self::Animation(val) => val.message_id,
            Self::LivePhoto(val) => val.message_id,
            Self::Venue(val) => val.message_id,
            Self::Audio(val) => val.message_id,
            Self::BoostAdded(val) => val.message_id,
            Self::ChannelChatCreated(val) => val.message_id,
            Self::ChatBackgroundSet(val) => val.message_id,
            Self::ChatOwnerChanged(val) => val.message_id,
            Self::ChatOwnerLeft(val) => val.message_id,
            Self::ChatShared(val) => val.message_id,
            Self::Checklist(val) => val.message_id,
            Self::ChecklistTasksAdded(val) => val.message_id,
            Self::ChecklistTasksDone(val) => val.message_id,
            Self::CommunityChatAdded(val) => val.message_id,
            Self::CommunityChatJoined(val) => val.message_id,
            Self::CommunityChatRemoved(val) => val.message_id,
            Self::ConnectedWebsite(val) => val.message_id,
            Self::Contact(val) => val.message_id,
            Self::DeleteChatPhoto(val) => val.message_id,
            Self::Dice(val) => val.message_id,
            Self::DirectMessagePriceChanged(val) => val.message_id,
            Self::Document(val) => val.message_id,
            Self::ForumTopicClosed(val) => val.message_id,
            Self::ForumTopicCreated(val) => val.message_id,
            Self::ForumTopicEdited(val) => val.message_id,
            Self::ForumTopicReopened(val) => val.message_id,
            Self::Game(val) => val.message_id,
            Self::GeneralForumTopicHidden(val) => val.message_id,
            Self::GeneralForumTopicUnhidden(val) => val.message_id,
            Self::Gift(val) => val.message_id,
            Self::GiftUpgradeSent(val) => val.message_id,
            Self::Giveaway(val) => val.message_id,
            Self::GiveawayCompleted(val) => val.message_id,
            Self::GiveawayCreated(val) => val.message_id,
            Self::GiveawayWinners(val) => val.message_id,
            Self::GroupChatCreated(val) => val.message_id,
            Self::Invoice(val) => val.message_id,
            Self::LeftChatMember(val) => val.message_id,
            Self::Location(val) => val.message_id,
            Self::ManagedBotCreated(val) => val.message_id,
            Self::MessageAutoDeleteTimerChanged(val) => val.message_id,
            Self::MigrateFromChatId(val) => val.message_id,
            Self::MigrateToChatId(val) => val.message_id,
            Self::NewChatMembers(val) => val.message_id,
            Self::NewChatPhoto(val) => val.message_id,
            Self::NewChatTitle(val) => val.message_id,
            Self::PaidMedia(val) => val.message_id,
            Self::PaidMessagePriceChanged(val) => val.message_id,
            Self::PassportData(val) => val.message_id,
            Self::Photo(val) => val.message_id,
            Self::PinnedMessage(val) => val.message_id,
            Self::Poll(val) => val.message_id,
            Self::PollOptionAdded(val) => val.message_id,
            Self::PollOptionDeleted(val) => val.message_id,
            Self::ProximityAlertTriggered(val) => val.message_id,
            Self::RefundedPayment(val) => val.message_id,
            Self::RichMessage(val) => val.message_id,
            Self::Sticker(val) => val.message_id,
            Self::Story(val) => val.message_id,
            Self::SuccessfulPayment(val) => val.message_id,
            Self::SuggestedPostApprovalFailed(val) => val.message_id,
            Self::SuggestedPostApproved(val) => val.message_id,
            Self::SuggestedPostDeclined(val) => val.message_id,
            Self::SuggestedPostPaid(val) => val.message_id,
            Self::SuggestedPostRefunded(val) => val.message_id,
            Self::SupergroupChatCreated(val) => val.message_id,
            Self::Text(val) => val.message_id,
            Self::UniqueGift(val) => val.message_id,
            Self::UsersShared(val) => val.message_id,
            Self::Video(val) => val.message_id,
            Self::VideoChatEnded(val) => val.message_id,
            Self::VideoChatParticipantsInvited(val) => val.message_id,
            Self::VideoChatScheduled(val) => val.message_id,
            Self::VideoChatStarted(val) => val.message_id,
            Self::VideoNote(val) => val.message_id,
            Self::Voice(val) => val.message_id,
            Self::WebAppData(val) => val.message_id,
            Self::WriteAccessAllowed(val) => val.message_id,
            Self::Unknown(val) => val.message_id,
        }
    }

    /// Helper method for field `message_thread_id`.
    ///
    /// Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[must_use]
    pub fn message_thread_id(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.message_thread_id,
            Self::LivePhoto(val) => val.message_thread_id,
            Self::Venue(val) => val.message_thread_id,
            Self::Audio(val) => val.message_thread_id,
            Self::BoostAdded(val) => val.message_thread_id,
            Self::ChannelChatCreated(val) => val.message_thread_id,
            Self::ChatBackgroundSet(val) => val.message_thread_id,
            Self::ChatOwnerChanged(val) => val.message_thread_id,
            Self::ChatOwnerLeft(val) => val.message_thread_id,
            Self::ChatShared(val) => val.message_thread_id,
            Self::Checklist(val) => val.message_thread_id,
            Self::ChecklistTasksAdded(val) => val.message_thread_id,
            Self::ChecklistTasksDone(val) => val.message_thread_id,
            Self::CommunityChatAdded(val) => val.message_thread_id,
            Self::CommunityChatJoined(val) => val.message_thread_id,
            Self::CommunityChatRemoved(val) => val.message_thread_id,
            Self::ConnectedWebsite(val) => val.message_thread_id,
            Self::Contact(val) => val.message_thread_id,
            Self::DeleteChatPhoto(val) => val.message_thread_id,
            Self::Dice(val) => val.message_thread_id,
            Self::DirectMessagePriceChanged(val) => val.message_thread_id,
            Self::Document(val) => val.message_thread_id,
            Self::ForumTopicClosed(val) => val.message_thread_id,
            Self::ForumTopicCreated(val) => val.message_thread_id,
            Self::ForumTopicEdited(val) => val.message_thread_id,
            Self::ForumTopicReopened(val) => val.message_thread_id,
            Self::Game(val) => val.message_thread_id,
            Self::GeneralForumTopicHidden(val) => val.message_thread_id,
            Self::GeneralForumTopicUnhidden(val) => val.message_thread_id,
            Self::Gift(val) => val.message_thread_id,
            Self::GiftUpgradeSent(val) => val.message_thread_id,
            Self::Giveaway(val) => val.message_thread_id,
            Self::GiveawayCompleted(val) => val.message_thread_id,
            Self::GiveawayCreated(val) => val.message_thread_id,
            Self::GiveawayWinners(val) => val.message_thread_id,
            Self::GroupChatCreated(val) => val.message_thread_id,
            Self::Invoice(val) => val.message_thread_id,
            Self::LeftChatMember(val) => val.message_thread_id,
            Self::Location(val) => val.message_thread_id,
            Self::ManagedBotCreated(val) => val.message_thread_id,
            Self::MessageAutoDeleteTimerChanged(val) => val.message_thread_id,
            Self::MigrateFromChatId(val) => val.message_thread_id,
            Self::MigrateToChatId(val) => val.message_thread_id,
            Self::NewChatMembers(val) => val.message_thread_id,
            Self::NewChatPhoto(val) => val.message_thread_id,
            Self::NewChatTitle(val) => val.message_thread_id,
            Self::PaidMedia(val) => val.message_thread_id,
            Self::PaidMessagePriceChanged(val) => val.message_thread_id,
            Self::PassportData(val) => val.message_thread_id,
            Self::Photo(val) => val.message_thread_id,
            Self::PinnedMessage(val) => val.message_thread_id,
            Self::Poll(val) => val.message_thread_id,
            Self::PollOptionAdded(val) => val.message_thread_id,
            Self::PollOptionDeleted(val) => val.message_thread_id,
            Self::ProximityAlertTriggered(val) => val.message_thread_id,
            Self::RefundedPayment(val) => val.message_thread_id,
            Self::RichMessage(val) => val.message_thread_id,
            Self::Sticker(val) => val.message_thread_id,
            Self::Story(val) => val.message_thread_id,
            Self::SuccessfulPayment(val) => val.message_thread_id,
            Self::SuggestedPostApprovalFailed(val) => val.message_thread_id,
            Self::SuggestedPostApproved(val) => val.message_thread_id,
            Self::SuggestedPostDeclined(val) => val.message_thread_id,
            Self::SuggestedPostPaid(val) => val.message_thread_id,
            Self::SuggestedPostRefunded(val) => val.message_thread_id,
            Self::SupergroupChatCreated(val) => val.message_thread_id,
            Self::Text(val) => val.message_thread_id,
            Self::UniqueGift(val) => val.message_thread_id,
            Self::UsersShared(val) => val.message_thread_id,
            Self::Video(val) => val.message_thread_id,
            Self::VideoChatEnded(val) => val.message_thread_id,
            Self::VideoChatParticipantsInvited(val) => val.message_thread_id,
            Self::VideoChatScheduled(val) => val.message_thread_id,
            Self::VideoChatStarted(val) => val.message_thread_id,
            Self::VideoNote(val) => val.message_thread_id,
            Self::Voice(val) => val.message_thread_id,
            Self::WebAppData(val) => val.message_thread_id,
            Self::WriteAccessAllowed(val) => val.message_thread_id,
            Self::Unknown(val) => val.message_thread_id,
        }
    }

    /// Helper method for field `migrate_from_chat_id`.
    ///
    /// The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn migrate_from_chat_id(&self) -> Option<i64> {
        match self {
            Self::MigrateFromChatId(val) => Some(val.migrate_from_chat_id),
            _ => None,
        }
    }

    /// Helper method for field `migrate_to_chat_id`.
    ///
    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        match self {
            Self::MigrateToChatId(val) => Some(val.migrate_to_chat_id),
            _ => None,
        }
    }

    /// Helper method for field `new_chat_members`.
    ///
    /// New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[must_use]
    pub fn new_chat_members(&self) -> Option<&[crate::types::User]> {
        match self {
            Self::NewChatMembers(val) => Some(val.new_chat_members.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `new_chat_photo`.
    ///
    /// A chat photo was change to this value
    #[must_use]
    pub fn new_chat_photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::NewChatPhoto(val) => Some(val.new_chat_photo.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `new_chat_title`.
    ///
    /// A chat title was changed to this value
    #[must_use]
    pub fn new_chat_title(&self) -> Option<&str> {
        match self {
            Self::NewChatTitle(val) => Some(val.new_chat_title.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `paid_media`.
    ///
    /// Message contains paid media; information about the paid media
    #[must_use]
    pub fn paid_media(&self) -> Option<&crate::types::PaidMediaInfo> {
        match self {
            Self::PaidMedia(val) => Some(&val.paid_media),
            _ => None,
        }
    }

    /// Helper method for field `paid_message_price_changed`.
    ///
    /// Service message: the price for paid messages has changed in the chat
    #[must_use]
    pub fn paid_message_price_changed(&self) -> Option<&crate::types::PaidMessagePriceChanged> {
        match self {
            Self::PaidMessagePriceChanged(val) => Some(&val.paid_message_price_changed),
            _ => None,
        }
    }

    /// Helper method for field `paid_star_count`.
    ///
    /// The number of Telegram Stars that were paid by the sender of the message to send it
    #[must_use]
    pub fn paid_star_count(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.paid_star_count,
            Self::LivePhoto(val) => val.paid_star_count,
            Self::Venue(val) => val.paid_star_count,
            Self::Audio(val) => val.paid_star_count,
            Self::BoostAdded(val) => val.paid_star_count,
            Self::ChannelChatCreated(val) => val.paid_star_count,
            Self::ChatBackgroundSet(val) => val.paid_star_count,
            Self::ChatOwnerChanged(val) => val.paid_star_count,
            Self::ChatOwnerLeft(val) => val.paid_star_count,
            Self::ChatShared(val) => val.paid_star_count,
            Self::Checklist(val) => val.paid_star_count,
            Self::ChecklistTasksAdded(val) => val.paid_star_count,
            Self::ChecklistTasksDone(val) => val.paid_star_count,
            Self::CommunityChatAdded(val) => val.paid_star_count,
            Self::CommunityChatJoined(val) => val.paid_star_count,
            Self::CommunityChatRemoved(val) => val.paid_star_count,
            Self::ConnectedWebsite(val) => val.paid_star_count,
            Self::Contact(val) => val.paid_star_count,
            Self::DeleteChatPhoto(val) => val.paid_star_count,
            Self::Dice(val) => val.paid_star_count,
            Self::DirectMessagePriceChanged(val) => val.paid_star_count,
            Self::Document(val) => val.paid_star_count,
            Self::ForumTopicClosed(val) => val.paid_star_count,
            Self::ForumTopicCreated(val) => val.paid_star_count,
            Self::ForumTopicEdited(val) => val.paid_star_count,
            Self::ForumTopicReopened(val) => val.paid_star_count,
            Self::Game(val) => val.paid_star_count,
            Self::GeneralForumTopicHidden(val) => val.paid_star_count,
            Self::GeneralForumTopicUnhidden(val) => val.paid_star_count,
            Self::Gift(val) => val.paid_star_count,
            Self::GiftUpgradeSent(val) => val.paid_star_count,
            Self::Giveaway(val) => val.paid_star_count,
            Self::GiveawayCompleted(val) => val.paid_star_count,
            Self::GiveawayCreated(val) => val.paid_star_count,
            Self::GiveawayWinners(val) => val.paid_star_count,
            Self::GroupChatCreated(val) => val.paid_star_count,
            Self::Invoice(val) => val.paid_star_count,
            Self::LeftChatMember(val) => val.paid_star_count,
            Self::Location(val) => val.paid_star_count,
            Self::ManagedBotCreated(val) => val.paid_star_count,
            Self::MessageAutoDeleteTimerChanged(val) => val.paid_star_count,
            Self::MigrateFromChatId(val) => val.paid_star_count,
            Self::MigrateToChatId(val) => val.paid_star_count,
            Self::NewChatMembers(val) => val.paid_star_count,
            Self::NewChatPhoto(val) => val.paid_star_count,
            Self::NewChatTitle(val) => val.paid_star_count,
            Self::PaidMedia(val) => val.paid_star_count,
            Self::PaidMessagePriceChanged(val) => val.paid_star_count,
            Self::PassportData(val) => val.paid_star_count,
            Self::Photo(val) => val.paid_star_count,
            Self::PinnedMessage(val) => val.paid_star_count,
            Self::Poll(val) => val.paid_star_count,
            Self::PollOptionAdded(val) => val.paid_star_count,
            Self::PollOptionDeleted(val) => val.paid_star_count,
            Self::ProximityAlertTriggered(val) => val.paid_star_count,
            Self::RefundedPayment(val) => val.paid_star_count,
            Self::RichMessage(val) => val.paid_star_count,
            Self::Sticker(val) => val.paid_star_count,
            Self::Story(val) => val.paid_star_count,
            Self::SuccessfulPayment(val) => val.paid_star_count,
            Self::SuggestedPostApprovalFailed(val) => val.paid_star_count,
            Self::SuggestedPostApproved(val) => val.paid_star_count,
            Self::SuggestedPostDeclined(val) => val.paid_star_count,
            Self::SuggestedPostPaid(val) => val.paid_star_count,
            Self::SuggestedPostRefunded(val) => val.paid_star_count,
            Self::SupergroupChatCreated(val) => val.paid_star_count,
            Self::Text(val) => val.paid_star_count,
            Self::UniqueGift(val) => val.paid_star_count,
            Self::UsersShared(val) => val.paid_star_count,
            Self::Video(val) => val.paid_star_count,
            Self::VideoChatEnded(val) => val.paid_star_count,
            Self::VideoChatParticipantsInvited(val) => val.paid_star_count,
            Self::VideoChatScheduled(val) => val.paid_star_count,
            Self::VideoChatStarted(val) => val.paid_star_count,
            Self::VideoNote(val) => val.paid_star_count,
            Self::Voice(val) => val.paid_star_count,
            Self::WebAppData(val) => val.paid_star_count,
            Self::WriteAccessAllowed(val) => val.paid_star_count,
            Self::Unknown(val) => val.paid_star_count,
        }
    }

    /// Helper method for field `passport_data`.
    ///
    /// Telegram Passport data
    #[must_use]
    pub fn passport_data(&self) -> Option<&crate::types::PassportData> {
        match self {
            Self::PassportData(val) => Some(&val.passport_data),
            _ => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// Message is a photo, available sizes of the photo
    #[must_use]
    pub fn photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Photo(val) => Some(val.photo.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `pinned_message`.
    ///
    /// Specified message was pinned. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[must_use]
    pub fn pinned_message(&self) -> Option<&crate::types::MaybeInaccessibleMessage> {
        match self {
            Self::PinnedMessage(val) => Some(val.pinned_message.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `poll`.
    ///
    /// Message is a native poll, information about the poll
    #[must_use]
    pub fn poll(&self) -> Option<&crate::types::Poll> {
        match self {
            Self::Poll(val) => Some(val.poll.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `poll_option_added`.
    ///
    /// Service message: answer option was added to a poll
    #[must_use]
    pub fn poll_option_added(&self) -> Option<&crate::types::PollOptionAdded> {
        match self {
            Self::PollOptionAdded(val) => Some(&val.poll_option_added),
            _ => None,
        }
    }

    /// Helper method for field `poll_option_deleted`.
    ///
    /// Service message: answer option was deleted from a poll
    #[must_use]
    pub fn poll_option_deleted(&self) -> Option<&crate::types::PollOptionDeleted> {
        match self {
            Self::PollOptionDeleted(val) => Some(&val.poll_option_deleted),
            _ => None,
        }
    }

    /// Helper method for field `proximity_alert_triggered`.
    ///
    /// Service message: a user in the chat triggered another user's proximity alert while sharing Live Location
    #[must_use]
    pub fn proximity_alert_triggered(&self) -> Option<&crate::types::ProximityAlertTriggered> {
        match self {
            Self::ProximityAlertTriggered(val) => Some(&val.proximity_alert_triggered),
            _ => None,
        }
    }

    /// Helper method for field `quote`.
    ///
    /// For replies that quote part of the original message, the quoted part of the message
    #[must_use]
    pub fn quote(&self) -> Option<&crate::types::TextQuote> {
        match self {
            Self::Animation(val) => val.quote.as_ref(),
            Self::LivePhoto(val) => val.quote.as_ref(),
            Self::Venue(val) => val.quote.as_ref(),
            Self::Audio(val) => val.quote.as_ref(),
            Self::BoostAdded(val) => val.quote.as_ref(),
            Self::ChannelChatCreated(val) => val.quote.as_ref(),
            Self::ChatBackgroundSet(val) => val.quote.as_ref(),
            Self::ChatOwnerChanged(val) => val.quote.as_ref(),
            Self::ChatOwnerLeft(val) => val.quote.as_ref(),
            Self::ChatShared(val) => val.quote.as_ref(),
            Self::Checklist(val) => val.quote.as_ref(),
            Self::ChecklistTasksAdded(val) => val.quote.as_ref(),
            Self::ChecklistTasksDone(val) => val.quote.as_ref(),
            Self::CommunityChatAdded(val) => val.quote.as_ref(),
            Self::CommunityChatJoined(val) => val.quote.as_ref(),
            Self::CommunityChatRemoved(val) => val.quote.as_ref(),
            Self::ConnectedWebsite(val) => val.quote.as_ref(),
            Self::Contact(val) => val.quote.as_ref(),
            Self::DeleteChatPhoto(val) => val.quote.as_ref(),
            Self::Dice(val) => val.quote.as_ref(),
            Self::DirectMessagePriceChanged(val) => val.quote.as_ref(),
            Self::Document(val) => val.quote.as_ref(),
            Self::ForumTopicClosed(val) => val.quote.as_ref(),
            Self::ForumTopicCreated(val) => val.quote.as_ref(),
            Self::ForumTopicEdited(val) => val.quote.as_ref(),
            Self::ForumTopicReopened(val) => val.quote.as_ref(),
            Self::Game(val) => val.quote.as_ref(),
            Self::GeneralForumTopicHidden(val) => val.quote.as_ref(),
            Self::GeneralForumTopicUnhidden(val) => val.quote.as_ref(),
            Self::Gift(val) => val.quote.as_ref(),
            Self::GiftUpgradeSent(val) => val.quote.as_ref(),
            Self::Giveaway(val) => val.quote.as_ref(),
            Self::GiveawayCompleted(val) => val.quote.as_ref(),
            Self::GiveawayCreated(val) => val.quote.as_ref(),
            Self::GiveawayWinners(val) => val.quote.as_ref(),
            Self::GroupChatCreated(val) => val.quote.as_ref(),
            Self::Invoice(val) => val.quote.as_ref(),
            Self::LeftChatMember(val) => val.quote.as_ref(),
            Self::Location(val) => val.quote.as_ref(),
            Self::ManagedBotCreated(val) => val.quote.as_ref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.quote.as_ref(),
            Self::MigrateFromChatId(val) => val.quote.as_ref(),
            Self::MigrateToChatId(val) => val.quote.as_ref(),
            Self::NewChatMembers(val) => val.quote.as_ref(),
            Self::NewChatPhoto(val) => val.quote.as_ref(),
            Self::NewChatTitle(val) => val.quote.as_ref(),
            Self::PaidMedia(val) => val.quote.as_ref(),
            Self::PaidMessagePriceChanged(val) => val.quote.as_ref(),
            Self::PassportData(val) => val.quote.as_ref(),
            Self::Photo(val) => val.quote.as_ref(),
            Self::PinnedMessage(val) => val.quote.as_ref(),
            Self::Poll(val) => val.quote.as_ref(),
            Self::PollOptionAdded(val) => val.quote.as_ref(),
            Self::PollOptionDeleted(val) => val.quote.as_ref(),
            Self::ProximityAlertTriggered(val) => val.quote.as_ref(),
            Self::RefundedPayment(val) => val.quote.as_ref(),
            Self::RichMessage(val) => val.quote.as_ref(),
            Self::Sticker(val) => val.quote.as_ref(),
            Self::Story(val) => val.quote.as_ref(),
            Self::SuccessfulPayment(val) => val.quote.as_ref(),
            Self::SuggestedPostApprovalFailed(val) => val.quote.as_ref(),
            Self::SuggestedPostApproved(val) => val.quote.as_ref(),
            Self::SuggestedPostDeclined(val) => val.quote.as_ref(),
            Self::SuggestedPostPaid(val) => val.quote.as_ref(),
            Self::SuggestedPostRefunded(val) => val.quote.as_ref(),
            Self::SupergroupChatCreated(val) => val.quote.as_ref(),
            Self::Text(val) => val.quote.as_ref(),
            Self::UniqueGift(val) => val.quote.as_ref(),
            Self::UsersShared(val) => val.quote.as_ref(),
            Self::Video(val) => val.quote.as_ref(),
            Self::VideoChatEnded(val) => val.quote.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.quote.as_ref(),
            Self::VideoChatScheduled(val) => val.quote.as_ref(),
            Self::VideoChatStarted(val) => val.quote.as_ref(),
            Self::VideoNote(val) => val.quote.as_ref(),
            Self::Voice(val) => val.quote.as_ref(),
            Self::WebAppData(val) => val.quote.as_ref(),
            Self::WriteAccessAllowed(val) => val.quote.as_ref(),
            Self::Unknown(val) => val.quote.as_ref(),
        }
    }

    /// Helper method for field `receiver_user`.
    ///
    /// For ephemeral messages, the user who received the message
    #[must_use]
    pub fn receiver_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::Animation(val) => val.receiver_user.as_deref(),
            Self::LivePhoto(val) => val.receiver_user.as_deref(),
            Self::Venue(val) => val.receiver_user.as_deref(),
            Self::Audio(val) => val.receiver_user.as_deref(),
            Self::BoostAdded(val) => val.receiver_user.as_deref(),
            Self::ChannelChatCreated(val) => val.receiver_user.as_deref(),
            Self::ChatBackgroundSet(val) => val.receiver_user.as_deref(),
            Self::ChatOwnerChanged(val) => val.receiver_user.as_deref(),
            Self::ChatOwnerLeft(val) => val.receiver_user.as_deref(),
            Self::ChatShared(val) => val.receiver_user.as_deref(),
            Self::Checklist(val) => val.receiver_user.as_deref(),
            Self::ChecklistTasksAdded(val) => val.receiver_user.as_deref(),
            Self::ChecklistTasksDone(val) => val.receiver_user.as_deref(),
            Self::CommunityChatAdded(val) => val.receiver_user.as_deref(),
            Self::CommunityChatJoined(val) => val.receiver_user.as_deref(),
            Self::CommunityChatRemoved(val) => val.receiver_user.as_deref(),
            Self::ConnectedWebsite(val) => val.receiver_user.as_deref(),
            Self::Contact(val) => val.receiver_user.as_deref(),
            Self::DeleteChatPhoto(val) => val.receiver_user.as_deref(),
            Self::Dice(val) => val.receiver_user.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.receiver_user.as_deref(),
            Self::Document(val) => val.receiver_user.as_deref(),
            Self::ForumTopicClosed(val) => val.receiver_user.as_deref(),
            Self::ForumTopicCreated(val) => val.receiver_user.as_deref(),
            Self::ForumTopicEdited(val) => val.receiver_user.as_deref(),
            Self::ForumTopicReopened(val) => val.receiver_user.as_deref(),
            Self::Game(val) => val.receiver_user.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.receiver_user.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.receiver_user.as_deref(),
            Self::Gift(val) => val.receiver_user.as_deref(),
            Self::GiftUpgradeSent(val) => val.receiver_user.as_deref(),
            Self::Giveaway(val) => val.receiver_user.as_deref(),
            Self::GiveawayCompleted(val) => val.receiver_user.as_deref(),
            Self::GiveawayCreated(val) => val.receiver_user.as_deref(),
            Self::GiveawayWinners(val) => val.receiver_user.as_deref(),
            Self::GroupChatCreated(val) => val.receiver_user.as_deref(),
            Self::Invoice(val) => val.receiver_user.as_deref(),
            Self::LeftChatMember(val) => val.receiver_user.as_deref(),
            Self::Location(val) => val.receiver_user.as_deref(),
            Self::ManagedBotCreated(val) => val.receiver_user.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.receiver_user.as_deref(),
            Self::MigrateFromChatId(val) => val.receiver_user.as_deref(),
            Self::MigrateToChatId(val) => val.receiver_user.as_deref(),
            Self::NewChatMembers(val) => val.receiver_user.as_deref(),
            Self::NewChatPhoto(val) => val.receiver_user.as_deref(),
            Self::NewChatTitle(val) => val.receiver_user.as_deref(),
            Self::PaidMedia(val) => val.receiver_user.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.receiver_user.as_deref(),
            Self::PassportData(val) => val.receiver_user.as_deref(),
            Self::Photo(val) => val.receiver_user.as_deref(),
            Self::PinnedMessage(val) => val.receiver_user.as_deref(),
            Self::Poll(val) => val.receiver_user.as_deref(),
            Self::PollOptionAdded(val) => val.receiver_user.as_deref(),
            Self::PollOptionDeleted(val) => val.receiver_user.as_deref(),
            Self::ProximityAlertTriggered(val) => val.receiver_user.as_deref(),
            Self::RefundedPayment(val) => val.receiver_user.as_deref(),
            Self::RichMessage(val) => val.receiver_user.as_deref(),
            Self::Sticker(val) => val.receiver_user.as_deref(),
            Self::Story(val) => val.receiver_user.as_deref(),
            Self::SuccessfulPayment(val) => val.receiver_user.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.receiver_user.as_deref(),
            Self::SuggestedPostApproved(val) => val.receiver_user.as_deref(),
            Self::SuggestedPostDeclined(val) => val.receiver_user.as_deref(),
            Self::SuggestedPostPaid(val) => val.receiver_user.as_deref(),
            Self::SuggestedPostRefunded(val) => val.receiver_user.as_deref(),
            Self::SupergroupChatCreated(val) => val.receiver_user.as_deref(),
            Self::Text(val) => val.receiver_user.as_deref(),
            Self::UniqueGift(val) => val.receiver_user.as_deref(),
            Self::UsersShared(val) => val.receiver_user.as_deref(),
            Self::Video(val) => val.receiver_user.as_deref(),
            Self::VideoChatEnded(val) => val.receiver_user.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.receiver_user.as_deref(),
            Self::VideoChatScheduled(val) => val.receiver_user.as_deref(),
            Self::VideoChatStarted(val) => val.receiver_user.as_deref(),
            Self::VideoNote(val) => val.receiver_user.as_deref(),
            Self::Voice(val) => val.receiver_user.as_deref(),
            Self::WebAppData(val) => val.receiver_user.as_deref(),
            Self::WriteAccessAllowed(val) => val.receiver_user.as_deref(),
            Self::Unknown(val) => val.receiver_user.as_deref(),
        }
    }

    /// Helper method for field `refunded_payment`.
    ///
    /// Message is a service message about a refunded payment, information about the payment. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn refunded_payment(&self) -> Option<&crate::types::RefundedPayment> {
        match self {
            Self::RefundedPayment(val) => Some(&val.refunded_payment),
            _ => None,
        }
    }

    /// Helper method for field `reply_markup`.
    ///
    /// Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    #[must_use]
    pub fn reply_markup(&self) -> Option<&crate::types::InlineKeyboardMarkup> {
        match self {
            Self::Animation(val) => val.reply_markup.as_ref(),
            Self::LivePhoto(val) => val.reply_markup.as_ref(),
            Self::Venue(val) => val.reply_markup.as_ref(),
            Self::Audio(val) => val.reply_markup.as_ref(),
            Self::BoostAdded(val) => val.reply_markup.as_ref(),
            Self::ChannelChatCreated(val) => val.reply_markup.as_ref(),
            Self::ChatBackgroundSet(val) => val.reply_markup.as_ref(),
            Self::ChatOwnerChanged(val) => val.reply_markup.as_ref(),
            Self::ChatOwnerLeft(val) => val.reply_markup.as_ref(),
            Self::ChatShared(val) => val.reply_markup.as_ref(),
            Self::Checklist(val) => val.reply_markup.as_ref(),
            Self::ChecklistTasksAdded(val) => val.reply_markup.as_ref(),
            Self::ChecklistTasksDone(val) => val.reply_markup.as_ref(),
            Self::CommunityChatAdded(val) => val.reply_markup.as_ref(),
            Self::CommunityChatJoined(val) => val.reply_markup.as_ref(),
            Self::CommunityChatRemoved(val) => val.reply_markup.as_ref(),
            Self::ConnectedWebsite(val) => val.reply_markup.as_ref(),
            Self::Contact(val) => val.reply_markup.as_ref(),
            Self::DeleteChatPhoto(val) => val.reply_markup.as_ref(),
            Self::Dice(val) => val.reply_markup.as_ref(),
            Self::DirectMessagePriceChanged(val) => val.reply_markup.as_ref(),
            Self::Document(val) => val.reply_markup.as_ref(),
            Self::ForumTopicClosed(val) => val.reply_markup.as_ref(),
            Self::ForumTopicCreated(val) => val.reply_markup.as_ref(),
            Self::ForumTopicEdited(val) => val.reply_markup.as_ref(),
            Self::ForumTopicReopened(val) => val.reply_markup.as_ref(),
            Self::Game(val) => val.reply_markup.as_ref(),
            Self::GeneralForumTopicHidden(val) => val.reply_markup.as_ref(),
            Self::GeneralForumTopicUnhidden(val) => val.reply_markup.as_ref(),
            Self::Gift(val) => val.reply_markup.as_ref(),
            Self::GiftUpgradeSent(val) => val.reply_markup.as_ref(),
            Self::Giveaway(val) => val.reply_markup.as_ref(),
            Self::GiveawayCompleted(val) => val.reply_markup.as_ref(),
            Self::GiveawayCreated(val) => val.reply_markup.as_ref(),
            Self::GiveawayWinners(val) => val.reply_markup.as_ref(),
            Self::GroupChatCreated(val) => val.reply_markup.as_ref(),
            Self::Invoice(val) => val.reply_markup.as_ref(),
            Self::LeftChatMember(val) => val.reply_markup.as_ref(),
            Self::Location(val) => val.reply_markup.as_ref(),
            Self::ManagedBotCreated(val) => val.reply_markup.as_ref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.reply_markup.as_ref(),
            Self::MigrateFromChatId(val) => val.reply_markup.as_ref(),
            Self::MigrateToChatId(val) => val.reply_markup.as_ref(),
            Self::NewChatMembers(val) => val.reply_markup.as_ref(),
            Self::NewChatPhoto(val) => val.reply_markup.as_ref(),
            Self::NewChatTitle(val) => val.reply_markup.as_ref(),
            Self::PaidMedia(val) => val.reply_markup.as_ref(),
            Self::PaidMessagePriceChanged(val) => val.reply_markup.as_ref(),
            Self::PassportData(val) => val.reply_markup.as_ref(),
            Self::Photo(val) => val.reply_markup.as_ref(),
            Self::PinnedMessage(val) => val.reply_markup.as_ref(),
            Self::Poll(val) => val.reply_markup.as_ref(),
            Self::PollOptionAdded(val) => val.reply_markup.as_ref(),
            Self::PollOptionDeleted(val) => val.reply_markup.as_ref(),
            Self::ProximityAlertTriggered(val) => val.reply_markup.as_ref(),
            Self::RefundedPayment(val) => val.reply_markup.as_ref(),
            Self::RichMessage(val) => val.reply_markup.as_ref(),
            Self::Sticker(val) => val.reply_markup.as_ref(),
            Self::Story(val) => val.reply_markup.as_ref(),
            Self::SuccessfulPayment(val) => val.reply_markup.as_ref(),
            Self::SuggestedPostApprovalFailed(val) => val.reply_markup.as_ref(),
            Self::SuggestedPostApproved(val) => val.reply_markup.as_ref(),
            Self::SuggestedPostDeclined(val) => val.reply_markup.as_ref(),
            Self::SuggestedPostPaid(val) => val.reply_markup.as_ref(),
            Self::SuggestedPostRefunded(val) => val.reply_markup.as_ref(),
            Self::SupergroupChatCreated(val) => val.reply_markup.as_ref(),
            Self::Text(val) => val.reply_markup.as_ref(),
            Self::UniqueGift(val) => val.reply_markup.as_ref(),
            Self::UsersShared(val) => val.reply_markup.as_ref(),
            Self::Video(val) => val.reply_markup.as_ref(),
            Self::VideoChatEnded(val) => val.reply_markup.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.reply_markup.as_ref(),
            Self::VideoChatScheduled(val) => val.reply_markup.as_ref(),
            Self::VideoChatStarted(val) => val.reply_markup.as_ref(),
            Self::VideoNote(val) => val.reply_markup.as_ref(),
            Self::Voice(val) => val.reply_markup.as_ref(),
            Self::WebAppData(val) => val.reply_markup.as_ref(),
            Self::WriteAccessAllowed(val) => val.reply_markup.as_ref(),
            Self::Unknown(val) => val.reply_markup.as_ref(),
        }
    }

    /// Helper method for field `reply_to_checklist_task_id`.
    ///
    /// Identifier of the specific checklist task that is being replied to
    #[must_use]
    pub fn reply_to_checklist_task_id(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.reply_to_checklist_task_id,
            Self::LivePhoto(val) => val.reply_to_checklist_task_id,
            Self::Venue(val) => val.reply_to_checklist_task_id,
            Self::Audio(val) => val.reply_to_checklist_task_id,
            Self::BoostAdded(val) => val.reply_to_checklist_task_id,
            Self::ChannelChatCreated(val) => val.reply_to_checklist_task_id,
            Self::ChatBackgroundSet(val) => val.reply_to_checklist_task_id,
            Self::ChatOwnerChanged(val) => val.reply_to_checklist_task_id,
            Self::ChatOwnerLeft(val) => val.reply_to_checklist_task_id,
            Self::ChatShared(val) => val.reply_to_checklist_task_id,
            Self::Checklist(val) => val.reply_to_checklist_task_id,
            Self::ChecklistTasksAdded(val) => val.reply_to_checklist_task_id,
            Self::ChecklistTasksDone(val) => val.reply_to_checklist_task_id,
            Self::CommunityChatAdded(val) => val.reply_to_checklist_task_id,
            Self::CommunityChatJoined(val) => val.reply_to_checklist_task_id,
            Self::CommunityChatRemoved(val) => val.reply_to_checklist_task_id,
            Self::ConnectedWebsite(val) => val.reply_to_checklist_task_id,
            Self::Contact(val) => val.reply_to_checklist_task_id,
            Self::DeleteChatPhoto(val) => val.reply_to_checklist_task_id,
            Self::Dice(val) => val.reply_to_checklist_task_id,
            Self::DirectMessagePriceChanged(val) => val.reply_to_checklist_task_id,
            Self::Document(val) => val.reply_to_checklist_task_id,
            Self::ForumTopicClosed(val) => val.reply_to_checklist_task_id,
            Self::ForumTopicCreated(val) => val.reply_to_checklist_task_id,
            Self::ForumTopicEdited(val) => val.reply_to_checklist_task_id,
            Self::ForumTopicReopened(val) => val.reply_to_checklist_task_id,
            Self::Game(val) => val.reply_to_checklist_task_id,
            Self::GeneralForumTopicHidden(val) => val.reply_to_checklist_task_id,
            Self::GeneralForumTopicUnhidden(val) => val.reply_to_checklist_task_id,
            Self::Gift(val) => val.reply_to_checklist_task_id,
            Self::GiftUpgradeSent(val) => val.reply_to_checklist_task_id,
            Self::Giveaway(val) => val.reply_to_checklist_task_id,
            Self::GiveawayCompleted(val) => val.reply_to_checklist_task_id,
            Self::GiveawayCreated(val) => val.reply_to_checklist_task_id,
            Self::GiveawayWinners(val) => val.reply_to_checklist_task_id,
            Self::GroupChatCreated(val) => val.reply_to_checklist_task_id,
            Self::Invoice(val) => val.reply_to_checklist_task_id,
            Self::LeftChatMember(val) => val.reply_to_checklist_task_id,
            Self::Location(val) => val.reply_to_checklist_task_id,
            Self::ManagedBotCreated(val) => val.reply_to_checklist_task_id,
            Self::MessageAutoDeleteTimerChanged(val) => val.reply_to_checklist_task_id,
            Self::MigrateFromChatId(val) => val.reply_to_checklist_task_id,
            Self::MigrateToChatId(val) => val.reply_to_checklist_task_id,
            Self::NewChatMembers(val) => val.reply_to_checklist_task_id,
            Self::NewChatPhoto(val) => val.reply_to_checklist_task_id,
            Self::NewChatTitle(val) => val.reply_to_checklist_task_id,
            Self::PaidMedia(val) => val.reply_to_checklist_task_id,
            Self::PaidMessagePriceChanged(val) => val.reply_to_checklist_task_id,
            Self::PassportData(val) => val.reply_to_checklist_task_id,
            Self::Photo(val) => val.reply_to_checklist_task_id,
            Self::PinnedMessage(val) => val.reply_to_checklist_task_id,
            Self::Poll(val) => val.reply_to_checklist_task_id,
            Self::PollOptionAdded(val) => val.reply_to_checklist_task_id,
            Self::PollOptionDeleted(val) => val.reply_to_checklist_task_id,
            Self::ProximityAlertTriggered(val) => val.reply_to_checklist_task_id,
            Self::RefundedPayment(val) => val.reply_to_checklist_task_id,
            Self::RichMessage(val) => val.reply_to_checklist_task_id,
            Self::Sticker(val) => val.reply_to_checklist_task_id,
            Self::Story(val) => val.reply_to_checklist_task_id,
            Self::SuccessfulPayment(val) => val.reply_to_checklist_task_id,
            Self::SuggestedPostApprovalFailed(val) => val.reply_to_checklist_task_id,
            Self::SuggestedPostApproved(val) => val.reply_to_checklist_task_id,
            Self::SuggestedPostDeclined(val) => val.reply_to_checklist_task_id,
            Self::SuggestedPostPaid(val) => val.reply_to_checklist_task_id,
            Self::SuggestedPostRefunded(val) => val.reply_to_checklist_task_id,
            Self::SupergroupChatCreated(val) => val.reply_to_checklist_task_id,
            Self::Text(val) => val.reply_to_checklist_task_id,
            Self::UniqueGift(val) => val.reply_to_checklist_task_id,
            Self::UsersShared(val) => val.reply_to_checklist_task_id,
            Self::Video(val) => val.reply_to_checklist_task_id,
            Self::VideoChatEnded(val) => val.reply_to_checklist_task_id,
            Self::VideoChatParticipantsInvited(val) => val.reply_to_checklist_task_id,
            Self::VideoChatScheduled(val) => val.reply_to_checklist_task_id,
            Self::VideoChatStarted(val) => val.reply_to_checklist_task_id,
            Self::VideoNote(val) => val.reply_to_checklist_task_id,
            Self::Voice(val) => val.reply_to_checklist_task_id,
            Self::WebAppData(val) => val.reply_to_checklist_task_id,
            Self::WriteAccessAllowed(val) => val.reply_to_checklist_task_id,
            Self::Unknown(val) => val.reply_to_checklist_task_id,
        }
    }

    /// Helper method for field `reply_to_message`.
    ///
    /// For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply. If the message is a reply to an ephemeral message, then this field may be omitted.
    #[must_use]
    pub fn reply_to_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::Animation(val) => val.reply_to_message.as_deref(),
            Self::LivePhoto(val) => val.reply_to_message.as_deref(),
            Self::Venue(val) => val.reply_to_message.as_deref(),
            Self::Audio(val) => val.reply_to_message.as_deref(),
            Self::BoostAdded(val) => val.reply_to_message.as_deref(),
            Self::ChannelChatCreated(val) => val.reply_to_message.as_deref(),
            Self::ChatBackgroundSet(val) => val.reply_to_message.as_deref(),
            Self::ChatOwnerChanged(val) => val.reply_to_message.as_deref(),
            Self::ChatOwnerLeft(val) => val.reply_to_message.as_deref(),
            Self::ChatShared(val) => val.reply_to_message.as_deref(),
            Self::Checklist(val) => val.reply_to_message.as_deref(),
            Self::ChecklistTasksAdded(val) => val.reply_to_message.as_deref(),
            Self::ChecklistTasksDone(val) => val.reply_to_message.as_deref(),
            Self::CommunityChatAdded(val) => val.reply_to_message.as_deref(),
            Self::CommunityChatJoined(val) => val.reply_to_message.as_deref(),
            Self::CommunityChatRemoved(val) => val.reply_to_message.as_deref(),
            Self::ConnectedWebsite(val) => val.reply_to_message.as_deref(),
            Self::Contact(val) => val.reply_to_message.as_deref(),
            Self::DeleteChatPhoto(val) => val.reply_to_message.as_deref(),
            Self::Dice(val) => val.reply_to_message.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.reply_to_message.as_deref(),
            Self::Document(val) => val.reply_to_message.as_deref(),
            Self::ForumTopicClosed(val) => val.reply_to_message.as_deref(),
            Self::ForumTopicCreated(val) => val.reply_to_message.as_deref(),
            Self::ForumTopicEdited(val) => val.reply_to_message.as_deref(),
            Self::ForumTopicReopened(val) => val.reply_to_message.as_deref(),
            Self::Game(val) => val.reply_to_message.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.reply_to_message.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.reply_to_message.as_deref(),
            Self::Gift(val) => val.reply_to_message.as_deref(),
            Self::GiftUpgradeSent(val) => val.reply_to_message.as_deref(),
            Self::Giveaway(val) => val.reply_to_message.as_deref(),
            Self::GiveawayCompleted(val) => val.reply_to_message.as_deref(),
            Self::GiveawayCreated(val) => val.reply_to_message.as_deref(),
            Self::GiveawayWinners(val) => val.reply_to_message.as_deref(),
            Self::GroupChatCreated(val) => val.reply_to_message.as_deref(),
            Self::Invoice(val) => val.reply_to_message.as_deref(),
            Self::LeftChatMember(val) => val.reply_to_message.as_deref(),
            Self::Location(val) => val.reply_to_message.as_deref(),
            Self::ManagedBotCreated(val) => val.reply_to_message.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.reply_to_message.as_deref(),
            Self::MigrateFromChatId(val) => val.reply_to_message.as_deref(),
            Self::MigrateToChatId(val) => val.reply_to_message.as_deref(),
            Self::NewChatMembers(val) => val.reply_to_message.as_deref(),
            Self::NewChatPhoto(val) => val.reply_to_message.as_deref(),
            Self::NewChatTitle(val) => val.reply_to_message.as_deref(),
            Self::PaidMedia(val) => val.reply_to_message.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.reply_to_message.as_deref(),
            Self::PassportData(val) => val.reply_to_message.as_deref(),
            Self::Photo(val) => val.reply_to_message.as_deref(),
            Self::PinnedMessage(val) => val.reply_to_message.as_deref(),
            Self::Poll(val) => val.reply_to_message.as_deref(),
            Self::PollOptionAdded(val) => val.reply_to_message.as_deref(),
            Self::PollOptionDeleted(val) => val.reply_to_message.as_deref(),
            Self::ProximityAlertTriggered(val) => val.reply_to_message.as_deref(),
            Self::RefundedPayment(val) => val.reply_to_message.as_deref(),
            Self::RichMessage(val) => val.reply_to_message.as_deref(),
            Self::Sticker(val) => val.reply_to_message.as_deref(),
            Self::Story(val) => val.reply_to_message.as_deref(),
            Self::SuccessfulPayment(val) => val.reply_to_message.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.reply_to_message.as_deref(),
            Self::SuggestedPostApproved(val) => val.reply_to_message.as_deref(),
            Self::SuggestedPostDeclined(val) => val.reply_to_message.as_deref(),
            Self::SuggestedPostPaid(val) => val.reply_to_message.as_deref(),
            Self::SuggestedPostRefunded(val) => val.reply_to_message.as_deref(),
            Self::SupergroupChatCreated(val) => val.reply_to_message.as_deref(),
            Self::Text(val) => val.reply_to_message.as_deref(),
            Self::UniqueGift(val) => val.reply_to_message.as_deref(),
            Self::UsersShared(val) => val.reply_to_message.as_deref(),
            Self::Video(val) => val.reply_to_message.as_deref(),
            Self::VideoChatEnded(val) => val.reply_to_message.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.reply_to_message.as_deref(),
            Self::VideoChatScheduled(val) => val.reply_to_message.as_deref(),
            Self::VideoChatStarted(val) => val.reply_to_message.as_deref(),
            Self::VideoNote(val) => val.reply_to_message.as_deref(),
            Self::Voice(val) => val.reply_to_message.as_deref(),
            Self::WebAppData(val) => val.reply_to_message.as_deref(),
            Self::WriteAccessAllowed(val) => val.reply_to_message.as_deref(),
            Self::Unknown(val) => val.reply_to_message.as_deref(),
        }
    }

    /// Helper method for field `reply_to_poll_option_id`.
    ///
    /// Persistent identifier of the specific poll option that is being replied to
    #[must_use]
    pub fn reply_to_poll_option_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.reply_to_poll_option_id.as_deref(),
            Self::LivePhoto(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Venue(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Audio(val) => val.reply_to_poll_option_id.as_deref(),
            Self::BoostAdded(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ChannelChatCreated(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ChatBackgroundSet(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ChatOwnerChanged(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ChatOwnerLeft(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ChatShared(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Checklist(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ChecklistTasksAdded(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ChecklistTasksDone(val) => val.reply_to_poll_option_id.as_deref(),
            Self::CommunityChatAdded(val) => val.reply_to_poll_option_id.as_deref(),
            Self::CommunityChatJoined(val) => val.reply_to_poll_option_id.as_deref(),
            Self::CommunityChatRemoved(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ConnectedWebsite(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Contact(val) => val.reply_to_poll_option_id.as_deref(),
            Self::DeleteChatPhoto(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Dice(val) => val.reply_to_poll_option_id.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Document(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ForumTopicClosed(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ForumTopicCreated(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ForumTopicEdited(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ForumTopicReopened(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Game(val) => val.reply_to_poll_option_id.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.reply_to_poll_option_id.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Gift(val) => val.reply_to_poll_option_id.as_deref(),
            Self::GiftUpgradeSent(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Giveaway(val) => val.reply_to_poll_option_id.as_deref(),
            Self::GiveawayCompleted(val) => val.reply_to_poll_option_id.as_deref(),
            Self::GiveawayCreated(val) => val.reply_to_poll_option_id.as_deref(),
            Self::GiveawayWinners(val) => val.reply_to_poll_option_id.as_deref(),
            Self::GroupChatCreated(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Invoice(val) => val.reply_to_poll_option_id.as_deref(),
            Self::LeftChatMember(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Location(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ManagedBotCreated(val) => val.reply_to_poll_option_id.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.reply_to_poll_option_id.as_deref(),
            Self::MigrateFromChatId(val) => val.reply_to_poll_option_id.as_deref(),
            Self::MigrateToChatId(val) => val.reply_to_poll_option_id.as_deref(),
            Self::NewChatMembers(val) => val.reply_to_poll_option_id.as_deref(),
            Self::NewChatPhoto(val) => val.reply_to_poll_option_id.as_deref(),
            Self::NewChatTitle(val) => val.reply_to_poll_option_id.as_deref(),
            Self::PaidMedia(val) => val.reply_to_poll_option_id.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.reply_to_poll_option_id.as_deref(),
            Self::PassportData(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Photo(val) => val.reply_to_poll_option_id.as_deref(),
            Self::PinnedMessage(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Poll(val) => val.reply_to_poll_option_id.as_deref(),
            Self::PollOptionAdded(val) => val.reply_to_poll_option_id.as_deref(),
            Self::PollOptionDeleted(val) => val.reply_to_poll_option_id.as_deref(),
            Self::ProximityAlertTriggered(val) => val.reply_to_poll_option_id.as_deref(),
            Self::RefundedPayment(val) => val.reply_to_poll_option_id.as_deref(),
            Self::RichMessage(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Sticker(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Story(val) => val.reply_to_poll_option_id.as_deref(),
            Self::SuccessfulPayment(val) => val.reply_to_poll_option_id.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.reply_to_poll_option_id.as_deref(),
            Self::SuggestedPostApproved(val) => val.reply_to_poll_option_id.as_deref(),
            Self::SuggestedPostDeclined(val) => val.reply_to_poll_option_id.as_deref(),
            Self::SuggestedPostPaid(val) => val.reply_to_poll_option_id.as_deref(),
            Self::SuggestedPostRefunded(val) => val.reply_to_poll_option_id.as_deref(),
            Self::SupergroupChatCreated(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Text(val) => val.reply_to_poll_option_id.as_deref(),
            Self::UniqueGift(val) => val.reply_to_poll_option_id.as_deref(),
            Self::UsersShared(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Video(val) => val.reply_to_poll_option_id.as_deref(),
            Self::VideoChatEnded(val) => val.reply_to_poll_option_id.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.reply_to_poll_option_id.as_deref(),
            Self::VideoChatScheduled(val) => val.reply_to_poll_option_id.as_deref(),
            Self::VideoChatStarted(val) => val.reply_to_poll_option_id.as_deref(),
            Self::VideoNote(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Voice(val) => val.reply_to_poll_option_id.as_deref(),
            Self::WebAppData(val) => val.reply_to_poll_option_id.as_deref(),
            Self::WriteAccessAllowed(val) => val.reply_to_poll_option_id.as_deref(),
            Self::Unknown(val) => val.reply_to_poll_option_id.as_deref(),
        }
    }

    /// Helper method for field `reply_to_story`.
    ///
    /// For replies to a story, the original story
    #[must_use]
    pub fn reply_to_story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::Animation(val) => val.reply_to_story.as_ref(),
            Self::LivePhoto(val) => val.reply_to_story.as_ref(),
            Self::Venue(val) => val.reply_to_story.as_ref(),
            Self::Audio(val) => val.reply_to_story.as_ref(),
            Self::BoostAdded(val) => val.reply_to_story.as_ref(),
            Self::ChannelChatCreated(val) => val.reply_to_story.as_ref(),
            Self::ChatBackgroundSet(val) => val.reply_to_story.as_ref(),
            Self::ChatOwnerChanged(val) => val.reply_to_story.as_ref(),
            Self::ChatOwnerLeft(val) => val.reply_to_story.as_ref(),
            Self::ChatShared(val) => val.reply_to_story.as_ref(),
            Self::Checklist(val) => val.reply_to_story.as_ref(),
            Self::ChecklistTasksAdded(val) => val.reply_to_story.as_ref(),
            Self::ChecklistTasksDone(val) => val.reply_to_story.as_ref(),
            Self::CommunityChatAdded(val) => val.reply_to_story.as_ref(),
            Self::CommunityChatJoined(val) => val.reply_to_story.as_ref(),
            Self::CommunityChatRemoved(val) => val.reply_to_story.as_ref(),
            Self::ConnectedWebsite(val) => val.reply_to_story.as_ref(),
            Self::Contact(val) => val.reply_to_story.as_ref(),
            Self::DeleteChatPhoto(val) => val.reply_to_story.as_ref(),
            Self::Dice(val) => val.reply_to_story.as_ref(),
            Self::DirectMessagePriceChanged(val) => val.reply_to_story.as_ref(),
            Self::Document(val) => val.reply_to_story.as_ref(),
            Self::ForumTopicClosed(val) => val.reply_to_story.as_ref(),
            Self::ForumTopicCreated(val) => val.reply_to_story.as_ref(),
            Self::ForumTopicEdited(val) => val.reply_to_story.as_ref(),
            Self::ForumTopicReopened(val) => val.reply_to_story.as_ref(),
            Self::Game(val) => val.reply_to_story.as_ref(),
            Self::GeneralForumTopicHidden(val) => val.reply_to_story.as_ref(),
            Self::GeneralForumTopicUnhidden(val) => val.reply_to_story.as_ref(),
            Self::Gift(val) => val.reply_to_story.as_ref(),
            Self::GiftUpgradeSent(val) => val.reply_to_story.as_ref(),
            Self::Giveaway(val) => val.reply_to_story.as_ref(),
            Self::GiveawayCompleted(val) => val.reply_to_story.as_ref(),
            Self::GiveawayCreated(val) => val.reply_to_story.as_ref(),
            Self::GiveawayWinners(val) => val.reply_to_story.as_ref(),
            Self::GroupChatCreated(val) => val.reply_to_story.as_ref(),
            Self::Invoice(val) => val.reply_to_story.as_ref(),
            Self::LeftChatMember(val) => val.reply_to_story.as_ref(),
            Self::Location(val) => val.reply_to_story.as_ref(),
            Self::ManagedBotCreated(val) => val.reply_to_story.as_ref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.reply_to_story.as_ref(),
            Self::MigrateFromChatId(val) => val.reply_to_story.as_ref(),
            Self::MigrateToChatId(val) => val.reply_to_story.as_ref(),
            Self::NewChatMembers(val) => val.reply_to_story.as_ref(),
            Self::NewChatPhoto(val) => val.reply_to_story.as_ref(),
            Self::NewChatTitle(val) => val.reply_to_story.as_ref(),
            Self::PaidMedia(val) => val.reply_to_story.as_ref(),
            Self::PaidMessagePriceChanged(val) => val.reply_to_story.as_ref(),
            Self::PassportData(val) => val.reply_to_story.as_ref(),
            Self::Photo(val) => val.reply_to_story.as_ref(),
            Self::PinnedMessage(val) => val.reply_to_story.as_ref(),
            Self::Poll(val) => val.reply_to_story.as_ref(),
            Self::PollOptionAdded(val) => val.reply_to_story.as_ref(),
            Self::PollOptionDeleted(val) => val.reply_to_story.as_ref(),
            Self::ProximityAlertTriggered(val) => val.reply_to_story.as_ref(),
            Self::RefundedPayment(val) => val.reply_to_story.as_ref(),
            Self::RichMessage(val) => val.reply_to_story.as_ref(),
            Self::Sticker(val) => val.reply_to_story.as_ref(),
            Self::Story(val) => val.reply_to_story.as_ref(),
            Self::SuccessfulPayment(val) => val.reply_to_story.as_ref(),
            Self::SuggestedPostApprovalFailed(val) => val.reply_to_story.as_ref(),
            Self::SuggestedPostApproved(val) => val.reply_to_story.as_ref(),
            Self::SuggestedPostDeclined(val) => val.reply_to_story.as_ref(),
            Self::SuggestedPostPaid(val) => val.reply_to_story.as_ref(),
            Self::SuggestedPostRefunded(val) => val.reply_to_story.as_ref(),
            Self::SupergroupChatCreated(val) => val.reply_to_story.as_ref(),
            Self::Text(val) => val.reply_to_story.as_ref(),
            Self::UniqueGift(val) => val.reply_to_story.as_ref(),
            Self::UsersShared(val) => val.reply_to_story.as_ref(),
            Self::Video(val) => val.reply_to_story.as_ref(),
            Self::VideoChatEnded(val) => val.reply_to_story.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.reply_to_story.as_ref(),
            Self::VideoChatScheduled(val) => val.reply_to_story.as_ref(),
            Self::VideoChatStarted(val) => val.reply_to_story.as_ref(),
            Self::VideoNote(val) => val.reply_to_story.as_ref(),
            Self::Voice(val) => val.reply_to_story.as_ref(),
            Self::WebAppData(val) => val.reply_to_story.as_ref(),
            Self::WriteAccessAllowed(val) => val.reply_to_story.as_ref(),
            Self::Unknown(val) => val.reply_to_story.as_ref(),
        }
    }

    /// Helper method for field `rich_message`.
    ///
    /// Message is a rich formatted message
    #[must_use]
    pub fn rich_message(&self) -> Option<&crate::types::RichMessage> {
        match self {
            Self::RichMessage(val) => Some(&val.rich_message),
            _ => None,
        }
    }

    /// Helper method for field `sender_boost_count`.
    ///
    /// If the sender of the message boosted the chat, the number of boosts added by the user
    #[must_use]
    pub fn sender_boost_count(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.sender_boost_count,
            Self::LivePhoto(val) => val.sender_boost_count,
            Self::Venue(val) => val.sender_boost_count,
            Self::Audio(val) => val.sender_boost_count,
            Self::BoostAdded(val) => val.sender_boost_count,
            Self::ChannelChatCreated(val) => val.sender_boost_count,
            Self::ChatBackgroundSet(val) => val.sender_boost_count,
            Self::ChatOwnerChanged(val) => val.sender_boost_count,
            Self::ChatOwnerLeft(val) => val.sender_boost_count,
            Self::ChatShared(val) => val.sender_boost_count,
            Self::Checklist(val) => val.sender_boost_count,
            Self::ChecklistTasksAdded(val) => val.sender_boost_count,
            Self::ChecklistTasksDone(val) => val.sender_boost_count,
            Self::CommunityChatAdded(val) => val.sender_boost_count,
            Self::CommunityChatJoined(val) => val.sender_boost_count,
            Self::CommunityChatRemoved(val) => val.sender_boost_count,
            Self::ConnectedWebsite(val) => val.sender_boost_count,
            Self::Contact(val) => val.sender_boost_count,
            Self::DeleteChatPhoto(val) => val.sender_boost_count,
            Self::Dice(val) => val.sender_boost_count,
            Self::DirectMessagePriceChanged(val) => val.sender_boost_count,
            Self::Document(val) => val.sender_boost_count,
            Self::ForumTopicClosed(val) => val.sender_boost_count,
            Self::ForumTopicCreated(val) => val.sender_boost_count,
            Self::ForumTopicEdited(val) => val.sender_boost_count,
            Self::ForumTopicReopened(val) => val.sender_boost_count,
            Self::Game(val) => val.sender_boost_count,
            Self::GeneralForumTopicHidden(val) => val.sender_boost_count,
            Self::GeneralForumTopicUnhidden(val) => val.sender_boost_count,
            Self::Gift(val) => val.sender_boost_count,
            Self::GiftUpgradeSent(val) => val.sender_boost_count,
            Self::Giveaway(val) => val.sender_boost_count,
            Self::GiveawayCompleted(val) => val.sender_boost_count,
            Self::GiveawayCreated(val) => val.sender_boost_count,
            Self::GiveawayWinners(val) => val.sender_boost_count,
            Self::GroupChatCreated(val) => val.sender_boost_count,
            Self::Invoice(val) => val.sender_boost_count,
            Self::LeftChatMember(val) => val.sender_boost_count,
            Self::Location(val) => val.sender_boost_count,
            Self::ManagedBotCreated(val) => val.sender_boost_count,
            Self::MessageAutoDeleteTimerChanged(val) => val.sender_boost_count,
            Self::MigrateFromChatId(val) => val.sender_boost_count,
            Self::MigrateToChatId(val) => val.sender_boost_count,
            Self::NewChatMembers(val) => val.sender_boost_count,
            Self::NewChatPhoto(val) => val.sender_boost_count,
            Self::NewChatTitle(val) => val.sender_boost_count,
            Self::PaidMedia(val) => val.sender_boost_count,
            Self::PaidMessagePriceChanged(val) => val.sender_boost_count,
            Self::PassportData(val) => val.sender_boost_count,
            Self::Photo(val) => val.sender_boost_count,
            Self::PinnedMessage(val) => val.sender_boost_count,
            Self::Poll(val) => val.sender_boost_count,
            Self::PollOptionAdded(val) => val.sender_boost_count,
            Self::PollOptionDeleted(val) => val.sender_boost_count,
            Self::ProximityAlertTriggered(val) => val.sender_boost_count,
            Self::RefundedPayment(val) => val.sender_boost_count,
            Self::RichMessage(val) => val.sender_boost_count,
            Self::Sticker(val) => val.sender_boost_count,
            Self::Story(val) => val.sender_boost_count,
            Self::SuccessfulPayment(val) => val.sender_boost_count,
            Self::SuggestedPostApprovalFailed(val) => val.sender_boost_count,
            Self::SuggestedPostApproved(val) => val.sender_boost_count,
            Self::SuggestedPostDeclined(val) => val.sender_boost_count,
            Self::SuggestedPostPaid(val) => val.sender_boost_count,
            Self::SuggestedPostRefunded(val) => val.sender_boost_count,
            Self::SupergroupChatCreated(val) => val.sender_boost_count,
            Self::Text(val) => val.sender_boost_count,
            Self::UniqueGift(val) => val.sender_boost_count,
            Self::UsersShared(val) => val.sender_boost_count,
            Self::Video(val) => val.sender_boost_count,
            Self::VideoChatEnded(val) => val.sender_boost_count,
            Self::VideoChatParticipantsInvited(val) => val.sender_boost_count,
            Self::VideoChatScheduled(val) => val.sender_boost_count,
            Self::VideoChatStarted(val) => val.sender_boost_count,
            Self::VideoNote(val) => val.sender_boost_count,
            Self::Voice(val) => val.sender_boost_count,
            Self::WebAppData(val) => val.sender_boost_count,
            Self::WriteAccessAllowed(val) => val.sender_boost_count,
            Self::Unknown(val) => val.sender_boost_count,
        }
    }

    /// Helper method for field `sender_business_bot`.
    ///
    /// The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[must_use]
    pub fn sender_business_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::Animation(val) => val.sender_business_bot.as_deref(),
            Self::LivePhoto(val) => val.sender_business_bot.as_deref(),
            Self::Venue(val) => val.sender_business_bot.as_deref(),
            Self::Audio(val) => val.sender_business_bot.as_deref(),
            Self::BoostAdded(val) => val.sender_business_bot.as_deref(),
            Self::ChannelChatCreated(val) => val.sender_business_bot.as_deref(),
            Self::ChatBackgroundSet(val) => val.sender_business_bot.as_deref(),
            Self::ChatOwnerChanged(val) => val.sender_business_bot.as_deref(),
            Self::ChatOwnerLeft(val) => val.sender_business_bot.as_deref(),
            Self::ChatShared(val) => val.sender_business_bot.as_deref(),
            Self::Checklist(val) => val.sender_business_bot.as_deref(),
            Self::ChecklistTasksAdded(val) => val.sender_business_bot.as_deref(),
            Self::ChecklistTasksDone(val) => val.sender_business_bot.as_deref(),
            Self::CommunityChatAdded(val) => val.sender_business_bot.as_deref(),
            Self::CommunityChatJoined(val) => val.sender_business_bot.as_deref(),
            Self::CommunityChatRemoved(val) => val.sender_business_bot.as_deref(),
            Self::ConnectedWebsite(val) => val.sender_business_bot.as_deref(),
            Self::Contact(val) => val.sender_business_bot.as_deref(),
            Self::DeleteChatPhoto(val) => val.sender_business_bot.as_deref(),
            Self::Dice(val) => val.sender_business_bot.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.sender_business_bot.as_deref(),
            Self::Document(val) => val.sender_business_bot.as_deref(),
            Self::ForumTopicClosed(val) => val.sender_business_bot.as_deref(),
            Self::ForumTopicCreated(val) => val.sender_business_bot.as_deref(),
            Self::ForumTopicEdited(val) => val.sender_business_bot.as_deref(),
            Self::ForumTopicReopened(val) => val.sender_business_bot.as_deref(),
            Self::Game(val) => val.sender_business_bot.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.sender_business_bot.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.sender_business_bot.as_deref(),
            Self::Gift(val) => val.sender_business_bot.as_deref(),
            Self::GiftUpgradeSent(val) => val.sender_business_bot.as_deref(),
            Self::Giveaway(val) => val.sender_business_bot.as_deref(),
            Self::GiveawayCompleted(val) => val.sender_business_bot.as_deref(),
            Self::GiveawayCreated(val) => val.sender_business_bot.as_deref(),
            Self::GiveawayWinners(val) => val.sender_business_bot.as_deref(),
            Self::GroupChatCreated(val) => val.sender_business_bot.as_deref(),
            Self::Invoice(val) => val.sender_business_bot.as_deref(),
            Self::LeftChatMember(val) => val.sender_business_bot.as_deref(),
            Self::Location(val) => val.sender_business_bot.as_deref(),
            Self::ManagedBotCreated(val) => val.sender_business_bot.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.sender_business_bot.as_deref(),
            Self::MigrateFromChatId(val) => val.sender_business_bot.as_deref(),
            Self::MigrateToChatId(val) => val.sender_business_bot.as_deref(),
            Self::NewChatMembers(val) => val.sender_business_bot.as_deref(),
            Self::NewChatPhoto(val) => val.sender_business_bot.as_deref(),
            Self::NewChatTitle(val) => val.sender_business_bot.as_deref(),
            Self::PaidMedia(val) => val.sender_business_bot.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.sender_business_bot.as_deref(),
            Self::PassportData(val) => val.sender_business_bot.as_deref(),
            Self::Photo(val) => val.sender_business_bot.as_deref(),
            Self::PinnedMessage(val) => val.sender_business_bot.as_deref(),
            Self::Poll(val) => val.sender_business_bot.as_deref(),
            Self::PollOptionAdded(val) => val.sender_business_bot.as_deref(),
            Self::PollOptionDeleted(val) => val.sender_business_bot.as_deref(),
            Self::ProximityAlertTriggered(val) => val.sender_business_bot.as_deref(),
            Self::RefundedPayment(val) => val.sender_business_bot.as_deref(),
            Self::RichMessage(val) => val.sender_business_bot.as_deref(),
            Self::Sticker(val) => val.sender_business_bot.as_deref(),
            Self::Story(val) => val.sender_business_bot.as_deref(),
            Self::SuccessfulPayment(val) => val.sender_business_bot.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.sender_business_bot.as_deref(),
            Self::SuggestedPostApproved(val) => val.sender_business_bot.as_deref(),
            Self::SuggestedPostDeclined(val) => val.sender_business_bot.as_deref(),
            Self::SuggestedPostPaid(val) => val.sender_business_bot.as_deref(),
            Self::SuggestedPostRefunded(val) => val.sender_business_bot.as_deref(),
            Self::SupergroupChatCreated(val) => val.sender_business_bot.as_deref(),
            Self::Text(val) => val.sender_business_bot.as_deref(),
            Self::UniqueGift(val) => val.sender_business_bot.as_deref(),
            Self::UsersShared(val) => val.sender_business_bot.as_deref(),
            Self::Video(val) => val.sender_business_bot.as_deref(),
            Self::VideoChatEnded(val) => val.sender_business_bot.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.sender_business_bot.as_deref(),
            Self::VideoChatScheduled(val) => val.sender_business_bot.as_deref(),
            Self::VideoChatStarted(val) => val.sender_business_bot.as_deref(),
            Self::VideoNote(val) => val.sender_business_bot.as_deref(),
            Self::Voice(val) => val.sender_business_bot.as_deref(),
            Self::WebAppData(val) => val.sender_business_bot.as_deref(),
            Self::WriteAccessAllowed(val) => val.sender_business_bot.as_deref(),
            Self::Unknown(val) => val.sender_business_bot.as_deref(),
        }
    }

    /// Helper method for field `sender_chat`.
    ///
    /// Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn sender_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Animation(val) => val.sender_chat.as_deref(),
            Self::LivePhoto(val) => val.sender_chat.as_deref(),
            Self::Venue(val) => val.sender_chat.as_deref(),
            Self::Audio(val) => val.sender_chat.as_deref(),
            Self::BoostAdded(val) => val.sender_chat.as_deref(),
            Self::ChannelChatCreated(val) => val.sender_chat.as_deref(),
            Self::ChatBackgroundSet(val) => val.sender_chat.as_deref(),
            Self::ChatOwnerChanged(val) => val.sender_chat.as_deref(),
            Self::ChatOwnerLeft(val) => val.sender_chat.as_deref(),
            Self::ChatShared(val) => val.sender_chat.as_deref(),
            Self::Checklist(val) => val.sender_chat.as_deref(),
            Self::ChecklistTasksAdded(val) => val.sender_chat.as_deref(),
            Self::ChecklistTasksDone(val) => val.sender_chat.as_deref(),
            Self::CommunityChatAdded(val) => val.sender_chat.as_deref(),
            Self::CommunityChatJoined(val) => val.sender_chat.as_deref(),
            Self::CommunityChatRemoved(val) => val.sender_chat.as_deref(),
            Self::ConnectedWebsite(val) => val.sender_chat.as_deref(),
            Self::Contact(val) => val.sender_chat.as_deref(),
            Self::DeleteChatPhoto(val) => val.sender_chat.as_deref(),
            Self::Dice(val) => val.sender_chat.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.sender_chat.as_deref(),
            Self::Document(val) => val.sender_chat.as_deref(),
            Self::ForumTopicClosed(val) => val.sender_chat.as_deref(),
            Self::ForumTopicCreated(val) => val.sender_chat.as_deref(),
            Self::ForumTopicEdited(val) => val.sender_chat.as_deref(),
            Self::ForumTopicReopened(val) => val.sender_chat.as_deref(),
            Self::Game(val) => val.sender_chat.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.sender_chat.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.sender_chat.as_deref(),
            Self::Gift(val) => val.sender_chat.as_deref(),
            Self::GiftUpgradeSent(val) => val.sender_chat.as_deref(),
            Self::Giveaway(val) => val.sender_chat.as_deref(),
            Self::GiveawayCompleted(val) => val.sender_chat.as_deref(),
            Self::GiveawayCreated(val) => val.sender_chat.as_deref(),
            Self::GiveawayWinners(val) => val.sender_chat.as_deref(),
            Self::GroupChatCreated(val) => val.sender_chat.as_deref(),
            Self::Invoice(val) => val.sender_chat.as_deref(),
            Self::LeftChatMember(val) => val.sender_chat.as_deref(),
            Self::Location(val) => val.sender_chat.as_deref(),
            Self::ManagedBotCreated(val) => val.sender_chat.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.sender_chat.as_deref(),
            Self::MigrateFromChatId(val) => val.sender_chat.as_deref(),
            Self::MigrateToChatId(val) => val.sender_chat.as_deref(),
            Self::NewChatMembers(val) => val.sender_chat.as_deref(),
            Self::NewChatPhoto(val) => val.sender_chat.as_deref(),
            Self::NewChatTitle(val) => val.sender_chat.as_deref(),
            Self::PaidMedia(val) => val.sender_chat.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.sender_chat.as_deref(),
            Self::PassportData(val) => val.sender_chat.as_deref(),
            Self::Photo(val) => val.sender_chat.as_deref(),
            Self::PinnedMessage(val) => val.sender_chat.as_deref(),
            Self::Poll(val) => val.sender_chat.as_deref(),
            Self::PollOptionAdded(val) => val.sender_chat.as_deref(),
            Self::PollOptionDeleted(val) => val.sender_chat.as_deref(),
            Self::ProximityAlertTriggered(val) => val.sender_chat.as_deref(),
            Self::RefundedPayment(val) => val.sender_chat.as_deref(),
            Self::RichMessage(val) => val.sender_chat.as_deref(),
            Self::Sticker(val) => val.sender_chat.as_deref(),
            Self::Story(val) => val.sender_chat.as_deref(),
            Self::SuccessfulPayment(val) => val.sender_chat.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.sender_chat.as_deref(),
            Self::SuggestedPostApproved(val) => val.sender_chat.as_deref(),
            Self::SuggestedPostDeclined(val) => val.sender_chat.as_deref(),
            Self::SuggestedPostPaid(val) => val.sender_chat.as_deref(),
            Self::SuggestedPostRefunded(val) => val.sender_chat.as_deref(),
            Self::SupergroupChatCreated(val) => val.sender_chat.as_deref(),
            Self::Text(val) => val.sender_chat.as_deref(),
            Self::UniqueGift(val) => val.sender_chat.as_deref(),
            Self::UsersShared(val) => val.sender_chat.as_deref(),
            Self::Video(val) => val.sender_chat.as_deref(),
            Self::VideoChatEnded(val) => val.sender_chat.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.sender_chat.as_deref(),
            Self::VideoChatScheduled(val) => val.sender_chat.as_deref(),
            Self::VideoChatStarted(val) => val.sender_chat.as_deref(),
            Self::VideoNote(val) => val.sender_chat.as_deref(),
            Self::Voice(val) => val.sender_chat.as_deref(),
            Self::WebAppData(val) => val.sender_chat.as_deref(),
            Self::WriteAccessAllowed(val) => val.sender_chat.as_deref(),
            Self::Unknown(val) => val.sender_chat.as_deref(),
        }
    }

    /// Helper method for field `sender_tag`.
    ///
    /// Tag or custom title of the sender of the message; for supergroups only
    #[must_use]
    pub fn sender_tag(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.sender_tag.as_deref(),
            Self::LivePhoto(val) => val.sender_tag.as_deref(),
            Self::Venue(val) => val.sender_tag.as_deref(),
            Self::Audio(val) => val.sender_tag.as_deref(),
            Self::BoostAdded(val) => val.sender_tag.as_deref(),
            Self::ChannelChatCreated(val) => val.sender_tag.as_deref(),
            Self::ChatBackgroundSet(val) => val.sender_tag.as_deref(),
            Self::ChatOwnerChanged(val) => val.sender_tag.as_deref(),
            Self::ChatOwnerLeft(val) => val.sender_tag.as_deref(),
            Self::ChatShared(val) => val.sender_tag.as_deref(),
            Self::Checklist(val) => val.sender_tag.as_deref(),
            Self::ChecklistTasksAdded(val) => val.sender_tag.as_deref(),
            Self::ChecklistTasksDone(val) => val.sender_tag.as_deref(),
            Self::CommunityChatAdded(val) => val.sender_tag.as_deref(),
            Self::CommunityChatJoined(val) => val.sender_tag.as_deref(),
            Self::CommunityChatRemoved(val) => val.sender_tag.as_deref(),
            Self::ConnectedWebsite(val) => val.sender_tag.as_deref(),
            Self::Contact(val) => val.sender_tag.as_deref(),
            Self::DeleteChatPhoto(val) => val.sender_tag.as_deref(),
            Self::Dice(val) => val.sender_tag.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.sender_tag.as_deref(),
            Self::Document(val) => val.sender_tag.as_deref(),
            Self::ForumTopicClosed(val) => val.sender_tag.as_deref(),
            Self::ForumTopicCreated(val) => val.sender_tag.as_deref(),
            Self::ForumTopicEdited(val) => val.sender_tag.as_deref(),
            Self::ForumTopicReopened(val) => val.sender_tag.as_deref(),
            Self::Game(val) => val.sender_tag.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.sender_tag.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.sender_tag.as_deref(),
            Self::Gift(val) => val.sender_tag.as_deref(),
            Self::GiftUpgradeSent(val) => val.sender_tag.as_deref(),
            Self::Giveaway(val) => val.sender_tag.as_deref(),
            Self::GiveawayCompleted(val) => val.sender_tag.as_deref(),
            Self::GiveawayCreated(val) => val.sender_tag.as_deref(),
            Self::GiveawayWinners(val) => val.sender_tag.as_deref(),
            Self::GroupChatCreated(val) => val.sender_tag.as_deref(),
            Self::Invoice(val) => val.sender_tag.as_deref(),
            Self::LeftChatMember(val) => val.sender_tag.as_deref(),
            Self::Location(val) => val.sender_tag.as_deref(),
            Self::ManagedBotCreated(val) => val.sender_tag.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.sender_tag.as_deref(),
            Self::MigrateFromChatId(val) => val.sender_tag.as_deref(),
            Self::MigrateToChatId(val) => val.sender_tag.as_deref(),
            Self::NewChatMembers(val) => val.sender_tag.as_deref(),
            Self::NewChatPhoto(val) => val.sender_tag.as_deref(),
            Self::NewChatTitle(val) => val.sender_tag.as_deref(),
            Self::PaidMedia(val) => val.sender_tag.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.sender_tag.as_deref(),
            Self::PassportData(val) => val.sender_tag.as_deref(),
            Self::Photo(val) => val.sender_tag.as_deref(),
            Self::PinnedMessage(val) => val.sender_tag.as_deref(),
            Self::Poll(val) => val.sender_tag.as_deref(),
            Self::PollOptionAdded(val) => val.sender_tag.as_deref(),
            Self::PollOptionDeleted(val) => val.sender_tag.as_deref(),
            Self::ProximityAlertTriggered(val) => val.sender_tag.as_deref(),
            Self::RefundedPayment(val) => val.sender_tag.as_deref(),
            Self::RichMessage(val) => val.sender_tag.as_deref(),
            Self::Sticker(val) => val.sender_tag.as_deref(),
            Self::Story(val) => val.sender_tag.as_deref(),
            Self::SuccessfulPayment(val) => val.sender_tag.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.sender_tag.as_deref(),
            Self::SuggestedPostApproved(val) => val.sender_tag.as_deref(),
            Self::SuggestedPostDeclined(val) => val.sender_tag.as_deref(),
            Self::SuggestedPostPaid(val) => val.sender_tag.as_deref(),
            Self::SuggestedPostRefunded(val) => val.sender_tag.as_deref(),
            Self::SupergroupChatCreated(val) => val.sender_tag.as_deref(),
            Self::Text(val) => val.sender_tag.as_deref(),
            Self::UniqueGift(val) => val.sender_tag.as_deref(),
            Self::UsersShared(val) => val.sender_tag.as_deref(),
            Self::Video(val) => val.sender_tag.as_deref(),
            Self::VideoChatEnded(val) => val.sender_tag.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.sender_tag.as_deref(),
            Self::VideoChatScheduled(val) => val.sender_tag.as_deref(),
            Self::VideoChatStarted(val) => val.sender_tag.as_deref(),
            Self::VideoNote(val) => val.sender_tag.as_deref(),
            Self::Voice(val) => val.sender_tag.as_deref(),
            Self::WebAppData(val) => val.sender_tag.as_deref(),
            Self::WriteAccessAllowed(val) => val.sender_tag.as_deref(),
            Self::Unknown(val) => val.sender_tag.as_deref(),
        }
    }

    /// Helper method for field `show_caption_above_media`.
    ///
    /// `true`, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.show_caption_above_media,
            Self::LivePhoto(val) => val.show_caption_above_media,
            Self::Venue(val) => val.show_caption_above_media,
            Self::Audio(val) => val.show_caption_above_media,
            Self::BoostAdded(val) => val.show_caption_above_media,
            Self::ChannelChatCreated(val) => val.show_caption_above_media,
            Self::ChatBackgroundSet(val) => val.show_caption_above_media,
            Self::ChatOwnerChanged(val) => val.show_caption_above_media,
            Self::ChatOwnerLeft(val) => val.show_caption_above_media,
            Self::ChatShared(val) => val.show_caption_above_media,
            Self::Checklist(val) => val.show_caption_above_media,
            Self::ChecklistTasksAdded(val) => val.show_caption_above_media,
            Self::ChecklistTasksDone(val) => val.show_caption_above_media,
            Self::CommunityChatAdded(val) => val.show_caption_above_media,
            Self::CommunityChatJoined(val) => val.show_caption_above_media,
            Self::CommunityChatRemoved(val) => val.show_caption_above_media,
            Self::ConnectedWebsite(val) => val.show_caption_above_media,
            Self::Contact(val) => val.show_caption_above_media,
            Self::DeleteChatPhoto(val) => val.show_caption_above_media,
            Self::Dice(val) => val.show_caption_above_media,
            Self::DirectMessagePriceChanged(val) => val.show_caption_above_media,
            Self::Document(val) => val.show_caption_above_media,
            Self::ForumTopicClosed(val) => val.show_caption_above_media,
            Self::ForumTopicCreated(val) => val.show_caption_above_media,
            Self::ForumTopicEdited(val) => val.show_caption_above_media,
            Self::ForumTopicReopened(val) => val.show_caption_above_media,
            Self::Game(val) => val.show_caption_above_media,
            Self::GeneralForumTopicHidden(val) => val.show_caption_above_media,
            Self::GeneralForumTopicUnhidden(val) => val.show_caption_above_media,
            Self::Gift(val) => val.show_caption_above_media,
            Self::GiftUpgradeSent(val) => val.show_caption_above_media,
            Self::Giveaway(val) => val.show_caption_above_media,
            Self::GiveawayCompleted(val) => val.show_caption_above_media,
            Self::GiveawayCreated(val) => val.show_caption_above_media,
            Self::GiveawayWinners(val) => val.show_caption_above_media,
            Self::GroupChatCreated(val) => val.show_caption_above_media,
            Self::Invoice(val) => val.show_caption_above_media,
            Self::LeftChatMember(val) => val.show_caption_above_media,
            Self::Location(val) => val.show_caption_above_media,
            Self::ManagedBotCreated(val) => val.show_caption_above_media,
            Self::MessageAutoDeleteTimerChanged(val) => val.show_caption_above_media,
            Self::MigrateFromChatId(val) => val.show_caption_above_media,
            Self::MigrateToChatId(val) => val.show_caption_above_media,
            Self::NewChatMembers(val) => val.show_caption_above_media,
            Self::NewChatPhoto(val) => val.show_caption_above_media,
            Self::NewChatTitle(val) => val.show_caption_above_media,
            Self::PaidMedia(val) => val.show_caption_above_media,
            Self::PaidMessagePriceChanged(val) => val.show_caption_above_media,
            Self::PassportData(val) => val.show_caption_above_media,
            Self::Photo(val) => val.show_caption_above_media,
            Self::PinnedMessage(val) => val.show_caption_above_media,
            Self::Poll(val) => val.show_caption_above_media,
            Self::PollOptionAdded(val) => val.show_caption_above_media,
            Self::PollOptionDeleted(val) => val.show_caption_above_media,
            Self::ProximityAlertTriggered(val) => val.show_caption_above_media,
            Self::RefundedPayment(val) => val.show_caption_above_media,
            Self::RichMessage(val) => val.show_caption_above_media,
            Self::Sticker(val) => val.show_caption_above_media,
            Self::Story(val) => val.show_caption_above_media,
            Self::SuccessfulPayment(val) => val.show_caption_above_media,
            Self::SuggestedPostApprovalFailed(val) => val.show_caption_above_media,
            Self::SuggestedPostApproved(val) => val.show_caption_above_media,
            Self::SuggestedPostDeclined(val) => val.show_caption_above_media,
            Self::SuggestedPostPaid(val) => val.show_caption_above_media,
            Self::SuggestedPostRefunded(val) => val.show_caption_above_media,
            Self::SupergroupChatCreated(val) => val.show_caption_above_media,
            Self::Text(val) => val.show_caption_above_media,
            Self::UniqueGift(val) => val.show_caption_above_media,
            Self::UsersShared(val) => val.show_caption_above_media,
            Self::Video(val) => val.show_caption_above_media,
            Self::VideoChatEnded(val) => val.show_caption_above_media,
            Self::VideoChatParticipantsInvited(val) => val.show_caption_above_media,
            Self::VideoChatScheduled(val) => val.show_caption_above_media,
            Self::VideoChatStarted(val) => val.show_caption_above_media,
            Self::VideoNote(val) => val.show_caption_above_media,
            Self::Voice(val) => val.show_caption_above_media,
            Self::WebAppData(val) => val.show_caption_above_media,
            Self::WriteAccessAllowed(val) => val.show_caption_above_media,
            Self::Unknown(val) => val.show_caption_above_media,
        }
    }

    /// Helper method for field `sticker`.
    ///
    /// Message is a sticker, information about the sticker
    #[must_use]
    pub fn sticker(&self) -> Option<&crate::types::Sticker> {
        match self {
            Self::Sticker(val) => Some(val.sticker.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `story`.
    ///
    /// Message is a forwarded story
    #[must_use]
    pub fn story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::Story(val) => Some(&val.story),
            _ => None,
        }
    }

    /// Helper method for field `successful_payment`.
    ///
    /// Message is a service message about a successful payment, information about the payment. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn successful_payment(&self) -> Option<&crate::types::SuccessfulPayment> {
        match self {
            Self::SuccessfulPayment(val) => Some(val.successful_payment.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `suggested_post_approval_failed`.
    ///
    /// Service message: approval of a suggested post has failed
    #[must_use]
    pub fn suggested_post_approval_failed(
        &self,
    ) -> Option<&crate::types::SuggestedPostApprovalFailed> {
        match self {
            Self::SuggestedPostApprovalFailed(val) => Some(&val.suggested_post_approval_failed),
            _ => None,
        }
    }

    /// Helper method for field `suggested_post_approved`.
    ///
    /// Service message: a suggested post was approved
    #[must_use]
    pub fn suggested_post_approved(&self) -> Option<&crate::types::SuggestedPostApproved> {
        match self {
            Self::SuggestedPostApproved(val) => Some(&val.suggested_post_approved),
            _ => None,
        }
    }

    /// Helper method for field `suggested_post_declined`.
    ///
    /// Service message: a suggested post was declined
    #[must_use]
    pub fn suggested_post_declined(&self) -> Option<&crate::types::SuggestedPostDeclined> {
        match self {
            Self::SuggestedPostDeclined(val) => Some(&val.suggested_post_declined),
            _ => None,
        }
    }

    /// Helper method for field `suggested_post_info`.
    ///
    /// Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    #[must_use]
    pub fn suggested_post_info(&self) -> Option<&crate::types::SuggestedPostInfo> {
        match self {
            Self::Animation(val) => val.suggested_post_info.as_ref(),
            Self::LivePhoto(val) => val.suggested_post_info.as_ref(),
            Self::Venue(val) => val.suggested_post_info.as_ref(),
            Self::Audio(val) => val.suggested_post_info.as_ref(),
            Self::BoostAdded(val) => val.suggested_post_info.as_ref(),
            Self::ChannelChatCreated(val) => val.suggested_post_info.as_ref(),
            Self::ChatBackgroundSet(val) => val.suggested_post_info.as_ref(),
            Self::ChatOwnerChanged(val) => val.suggested_post_info.as_ref(),
            Self::ChatOwnerLeft(val) => val.suggested_post_info.as_ref(),
            Self::ChatShared(val) => val.suggested_post_info.as_ref(),
            Self::Checklist(val) => val.suggested_post_info.as_ref(),
            Self::ChecklistTasksAdded(val) => val.suggested_post_info.as_ref(),
            Self::ChecklistTasksDone(val) => val.suggested_post_info.as_ref(),
            Self::CommunityChatAdded(val) => val.suggested_post_info.as_ref(),
            Self::CommunityChatJoined(val) => val.suggested_post_info.as_ref(),
            Self::CommunityChatRemoved(val) => val.suggested_post_info.as_ref(),
            Self::ConnectedWebsite(val) => val.suggested_post_info.as_ref(),
            Self::Contact(val) => val.suggested_post_info.as_ref(),
            Self::DeleteChatPhoto(val) => val.suggested_post_info.as_ref(),
            Self::Dice(val) => val.suggested_post_info.as_ref(),
            Self::DirectMessagePriceChanged(val) => val.suggested_post_info.as_ref(),
            Self::Document(val) => val.suggested_post_info.as_ref(),
            Self::ForumTopicClosed(val) => val.suggested_post_info.as_ref(),
            Self::ForumTopicCreated(val) => val.suggested_post_info.as_ref(),
            Self::ForumTopicEdited(val) => val.suggested_post_info.as_ref(),
            Self::ForumTopicReopened(val) => val.suggested_post_info.as_ref(),
            Self::Game(val) => val.suggested_post_info.as_ref(),
            Self::GeneralForumTopicHidden(val) => val.suggested_post_info.as_ref(),
            Self::GeneralForumTopicUnhidden(val) => val.suggested_post_info.as_ref(),
            Self::Gift(val) => val.suggested_post_info.as_ref(),
            Self::GiftUpgradeSent(val) => val.suggested_post_info.as_ref(),
            Self::Giveaway(val) => val.suggested_post_info.as_ref(),
            Self::GiveawayCompleted(val) => val.suggested_post_info.as_ref(),
            Self::GiveawayCreated(val) => val.suggested_post_info.as_ref(),
            Self::GiveawayWinners(val) => val.suggested_post_info.as_ref(),
            Self::GroupChatCreated(val) => val.suggested_post_info.as_ref(),
            Self::Invoice(val) => val.suggested_post_info.as_ref(),
            Self::LeftChatMember(val) => val.suggested_post_info.as_ref(),
            Self::Location(val) => val.suggested_post_info.as_ref(),
            Self::ManagedBotCreated(val) => val.suggested_post_info.as_ref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.suggested_post_info.as_ref(),
            Self::MigrateFromChatId(val) => val.suggested_post_info.as_ref(),
            Self::MigrateToChatId(val) => val.suggested_post_info.as_ref(),
            Self::NewChatMembers(val) => val.suggested_post_info.as_ref(),
            Self::NewChatPhoto(val) => val.suggested_post_info.as_ref(),
            Self::NewChatTitle(val) => val.suggested_post_info.as_ref(),
            Self::PaidMedia(val) => val.suggested_post_info.as_ref(),
            Self::PaidMessagePriceChanged(val) => val.suggested_post_info.as_ref(),
            Self::PassportData(val) => val.suggested_post_info.as_ref(),
            Self::Photo(val) => val.suggested_post_info.as_ref(),
            Self::PinnedMessage(val) => val.suggested_post_info.as_ref(),
            Self::Poll(val) => val.suggested_post_info.as_ref(),
            Self::PollOptionAdded(val) => val.suggested_post_info.as_ref(),
            Self::PollOptionDeleted(val) => val.suggested_post_info.as_ref(),
            Self::ProximityAlertTriggered(val) => val.suggested_post_info.as_ref(),
            Self::RefundedPayment(val) => val.suggested_post_info.as_ref(),
            Self::RichMessage(val) => val.suggested_post_info.as_ref(),
            Self::Sticker(val) => val.suggested_post_info.as_ref(),
            Self::Story(val) => val.suggested_post_info.as_ref(),
            Self::SuccessfulPayment(val) => val.suggested_post_info.as_ref(),
            Self::SuggestedPostApprovalFailed(val) => val.suggested_post_info.as_ref(),
            Self::SuggestedPostApproved(val) => val.suggested_post_info.as_ref(),
            Self::SuggestedPostDeclined(val) => val.suggested_post_info.as_ref(),
            Self::SuggestedPostPaid(val) => val.suggested_post_info.as_ref(),
            Self::SuggestedPostRefunded(val) => val.suggested_post_info.as_ref(),
            Self::SupergroupChatCreated(val) => val.suggested_post_info.as_ref(),
            Self::Text(val) => val.suggested_post_info.as_ref(),
            Self::UniqueGift(val) => val.suggested_post_info.as_ref(),
            Self::UsersShared(val) => val.suggested_post_info.as_ref(),
            Self::Video(val) => val.suggested_post_info.as_ref(),
            Self::VideoChatEnded(val) => val.suggested_post_info.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.suggested_post_info.as_ref(),
            Self::VideoChatScheduled(val) => val.suggested_post_info.as_ref(),
            Self::VideoChatStarted(val) => val.suggested_post_info.as_ref(),
            Self::VideoNote(val) => val.suggested_post_info.as_ref(),
            Self::Voice(val) => val.suggested_post_info.as_ref(),
            Self::WebAppData(val) => val.suggested_post_info.as_ref(),
            Self::WriteAccessAllowed(val) => val.suggested_post_info.as_ref(),
            Self::Unknown(val) => val.suggested_post_info.as_ref(),
        }
    }

    /// Helper method for field `suggested_post_paid`.
    ///
    /// Service message: payment for a suggested post was received
    #[must_use]
    pub fn suggested_post_paid(&self) -> Option<&crate::types::SuggestedPostPaid> {
        match self {
            Self::SuggestedPostPaid(val) => Some(&val.suggested_post_paid),
            _ => None,
        }
    }

    /// Helper method for field `suggested_post_refunded`.
    ///
    /// Service message: payment for a suggested post was refunded
    #[must_use]
    pub fn suggested_post_refunded(&self) -> Option<&crate::types::SuggestedPostRefunded> {
        match self {
            Self::SuggestedPostRefunded(val) => Some(&val.suggested_post_refunded),
            _ => None,
        }
    }

    /// Helper method for field `supergroup_chat_created`.
    ///
    /// Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in `reply_to_message` if someone replies to a very first message in a directly created supergroup.
    #[must_use]
    pub fn supergroup_chat_created(&self) -> Option<bool> {
        match self {
            Self::SupergroupChatCreated(val) => Some(val.supergroup_chat_created),
            _ => None,
        }
    }

    /// Helper method for field `text`.
    ///
    /// For text messages, the actual UTF-8 text of the message
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Text(val) => Some(val.text.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `unique_gift`.
    ///
    /// Service message: a unique gift was sent or received
    #[must_use]
    pub fn unique_gift(&self) -> Option<&crate::types::UniqueGiftInfo> {
        match self {
            Self::UniqueGift(val) => Some(&val.unique_gift),
            _ => None,
        }
    }

    /// Helper method for field `users_shared`.
    ///
    /// Service message: users were shared with the bot
    #[must_use]
    pub fn users_shared(&self) -> Option<&crate::types::UsersShared> {
        match self {
            Self::UsersShared(val) => Some(&val.users_shared),
            _ => None,
        }
    }

    /// Helper method for field `venue`.
    ///
    /// Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set.
    #[must_use]
    pub fn venue(&self) -> Option<&crate::types::Venue> {
        match self {
            Self::Venue(val) => Some(val.venue.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `via_bot`.
    ///
    /// Bot through which the message was sent
    #[must_use]
    pub fn via_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::Animation(val) => val.via_bot.as_deref(),
            Self::LivePhoto(val) => val.via_bot.as_deref(),
            Self::Venue(val) => val.via_bot.as_deref(),
            Self::Audio(val) => val.via_bot.as_deref(),
            Self::BoostAdded(val) => val.via_bot.as_deref(),
            Self::ChannelChatCreated(val) => val.via_bot.as_deref(),
            Self::ChatBackgroundSet(val) => val.via_bot.as_deref(),
            Self::ChatOwnerChanged(val) => val.via_bot.as_deref(),
            Self::ChatOwnerLeft(val) => val.via_bot.as_deref(),
            Self::ChatShared(val) => val.via_bot.as_deref(),
            Self::Checklist(val) => val.via_bot.as_deref(),
            Self::ChecklistTasksAdded(val) => val.via_bot.as_deref(),
            Self::ChecklistTasksDone(val) => val.via_bot.as_deref(),
            Self::CommunityChatAdded(val) => val.via_bot.as_deref(),
            Self::CommunityChatJoined(val) => val.via_bot.as_deref(),
            Self::CommunityChatRemoved(val) => val.via_bot.as_deref(),
            Self::ConnectedWebsite(val) => val.via_bot.as_deref(),
            Self::Contact(val) => val.via_bot.as_deref(),
            Self::DeleteChatPhoto(val) => val.via_bot.as_deref(),
            Self::Dice(val) => val.via_bot.as_deref(),
            Self::DirectMessagePriceChanged(val) => val.via_bot.as_deref(),
            Self::Document(val) => val.via_bot.as_deref(),
            Self::ForumTopicClosed(val) => val.via_bot.as_deref(),
            Self::ForumTopicCreated(val) => val.via_bot.as_deref(),
            Self::ForumTopicEdited(val) => val.via_bot.as_deref(),
            Self::ForumTopicReopened(val) => val.via_bot.as_deref(),
            Self::Game(val) => val.via_bot.as_deref(),
            Self::GeneralForumTopicHidden(val) => val.via_bot.as_deref(),
            Self::GeneralForumTopicUnhidden(val) => val.via_bot.as_deref(),
            Self::Gift(val) => val.via_bot.as_deref(),
            Self::GiftUpgradeSent(val) => val.via_bot.as_deref(),
            Self::Giveaway(val) => val.via_bot.as_deref(),
            Self::GiveawayCompleted(val) => val.via_bot.as_deref(),
            Self::GiveawayCreated(val) => val.via_bot.as_deref(),
            Self::GiveawayWinners(val) => val.via_bot.as_deref(),
            Self::GroupChatCreated(val) => val.via_bot.as_deref(),
            Self::Invoice(val) => val.via_bot.as_deref(),
            Self::LeftChatMember(val) => val.via_bot.as_deref(),
            Self::Location(val) => val.via_bot.as_deref(),
            Self::ManagedBotCreated(val) => val.via_bot.as_deref(),
            Self::MessageAutoDeleteTimerChanged(val) => val.via_bot.as_deref(),
            Self::MigrateFromChatId(val) => val.via_bot.as_deref(),
            Self::MigrateToChatId(val) => val.via_bot.as_deref(),
            Self::NewChatMembers(val) => val.via_bot.as_deref(),
            Self::NewChatPhoto(val) => val.via_bot.as_deref(),
            Self::NewChatTitle(val) => val.via_bot.as_deref(),
            Self::PaidMedia(val) => val.via_bot.as_deref(),
            Self::PaidMessagePriceChanged(val) => val.via_bot.as_deref(),
            Self::PassportData(val) => val.via_bot.as_deref(),
            Self::Photo(val) => val.via_bot.as_deref(),
            Self::PinnedMessage(val) => val.via_bot.as_deref(),
            Self::Poll(val) => val.via_bot.as_deref(),
            Self::PollOptionAdded(val) => val.via_bot.as_deref(),
            Self::PollOptionDeleted(val) => val.via_bot.as_deref(),
            Self::ProximityAlertTriggered(val) => val.via_bot.as_deref(),
            Self::RefundedPayment(val) => val.via_bot.as_deref(),
            Self::RichMessage(val) => val.via_bot.as_deref(),
            Self::Sticker(val) => val.via_bot.as_deref(),
            Self::Story(val) => val.via_bot.as_deref(),
            Self::SuccessfulPayment(val) => val.via_bot.as_deref(),
            Self::SuggestedPostApprovalFailed(val) => val.via_bot.as_deref(),
            Self::SuggestedPostApproved(val) => val.via_bot.as_deref(),
            Self::SuggestedPostDeclined(val) => val.via_bot.as_deref(),
            Self::SuggestedPostPaid(val) => val.via_bot.as_deref(),
            Self::SuggestedPostRefunded(val) => val.via_bot.as_deref(),
            Self::SupergroupChatCreated(val) => val.via_bot.as_deref(),
            Self::Text(val) => val.via_bot.as_deref(),
            Self::UniqueGift(val) => val.via_bot.as_deref(),
            Self::UsersShared(val) => val.via_bot.as_deref(),
            Self::Video(val) => val.via_bot.as_deref(),
            Self::VideoChatEnded(val) => val.via_bot.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.via_bot.as_deref(),
            Self::VideoChatScheduled(val) => val.via_bot.as_deref(),
            Self::VideoChatStarted(val) => val.via_bot.as_deref(),
            Self::VideoNote(val) => val.via_bot.as_deref(),
            Self::Voice(val) => val.via_bot.as_deref(),
            Self::WebAppData(val) => val.via_bot.as_deref(),
            Self::WriteAccessAllowed(val) => val.via_bot.as_deref(),
            Self::Unknown(val) => val.via_bot.as_deref(),
        }
    }

    /// Helper method for field `video`.
    ///
    /// Message is a video, information about the video
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::Video> {
        match self {
            Self::Video(val) => Some(val.video.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `video_chat_ended`.
    ///
    /// Service message: video chat ended
    #[must_use]
    pub fn video_chat_ended(&self) -> Option<&crate::types::VideoChatEnded> {
        match self {
            Self::VideoChatEnded(val) => Some(&val.video_chat_ended),
            _ => None,
        }
    }

    /// Helper method for field `video_chat_participants_invited`.
    ///
    /// Service message: new participants invited to a video chat
    #[must_use]
    pub fn video_chat_participants_invited(
        &self,
    ) -> Option<&crate::types::VideoChatParticipantsInvited> {
        match self {
            Self::VideoChatParticipantsInvited(val) => Some(&val.video_chat_participants_invited),
            _ => None,
        }
    }

    /// Helper method for field `video_chat_scheduled`.
    ///
    /// Service message: video chat scheduled
    #[must_use]
    pub fn video_chat_scheduled(&self) -> Option<&crate::types::VideoChatScheduled> {
        match self {
            Self::VideoChatScheduled(val) => Some(&val.video_chat_scheduled),
            _ => None,
        }
    }

    /// Helper method for field `video_chat_started`.
    ///
    /// Service message: video chat started
    #[must_use]
    pub fn video_chat_started(&self) -> Option<&crate::types::VideoChatStarted> {
        match self {
            Self::VideoChatStarted(val) => Some(&val.video_chat_started),
            _ => None,
        }
    }

    /// Helper method for field `video_note`.
    ///
    /// Message is a video note, information about the video message
    #[must_use]
    pub fn video_note(&self) -> Option<&crate::types::VideoNote> {
        match self {
            Self::VideoNote(val) => Some(val.video_note.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `voice`.
    ///
    /// Message is a voice message, information about the file
    #[must_use]
    pub fn voice(&self) -> Option<&crate::types::Voice> {
        match self {
            Self::Voice(val) => Some(&val.voice),
            _ => None,
        }
    }

    /// Helper method for field `web_app_data`.
    ///
    /// Service message: data sent by a Web App
    #[must_use]
    pub fn web_app_data(&self) -> Option<&crate::types::WebAppData> {
        match self {
            Self::WebAppData(val) => Some(&val.web_app_data),
            _ => None,
        }
    }

    /// Helper method for field `write_access_allowed`.
    ///
    /// Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method `requestWriteAccess`
    #[must_use]
    pub fn write_access_allowed(&self) -> Option<&crate::types::WriteAccessAllowed> {
        match self {
            Self::WriteAccessAllowed(val) => Some(&val.write_access_allowed),
            _ => None,
        }
    }

    /// Helper method for nested field `additional_chat_count`.
    #[must_use]
    pub fn additional_chat_count(&self) -> Option<i64> {
        match self {
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::additional_chat_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `address`.
    #[must_use]
    pub fn address(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                Some(inner.address.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `allows_multiple_answers`.
    #[must_use]
    pub fn allows_multiple_answers(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::allows_multiple_answers(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `allows_revoting`.
    #[must_use]
    pub fn allows_revoting(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::allows_revoting(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `amount`.
    #[must_use]
    pub fn amount(&self) -> Option<i64> {
        match self {
            Self::SuggestedPostPaid(val) => {
                let inner = &val.suggested_post_paid;
                inner.amount
            }
            _ => None,
        }
    }

    /// Helper method for nested field `are_direct_messages_enabled`.
    #[must_use]
    pub fn are_direct_messages_enabled(&self) -> Option<bool> {
        match self {
            Self::DirectMessagePriceChanged(val) => {
                let inner = &val.direct_message_price_changed;
                Some(inner.are_direct_messages_enabled)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `blocks`.
    #[must_use]
    pub fn blocks(&self) -> Option<&[crate::types::RichBlock]> {
        match self {
            Self::RichMessage(val) => {
                let inner = &val.rich_message;
                Some(inner.blocks.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `boost_count`.
    #[must_use]
    pub fn boost_count(&self) -> Option<i64> {
        match self {
            Self::BoostAdded(val) => {
                let inner = &val.boost_added;
                Some(inner.boost_count)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `bot`.
    #[must_use]
    pub fn bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::ManagedBotCreated(val) => {
                let inner = &val.managed_bot_created;
                Some(inner.bot.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `button_text`.
    #[must_use]
    pub fn button_text(&self) -> Option<&str> {
        match self {
            Self::WebAppData(val) => {
                let inner = &val.web_app_data;
                Some(inner.button_text.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `can_be_upgraded`.
    #[must_use]
    pub fn can_be_upgraded(&self) -> Option<bool> {
        match self {
            Self::Gift(val) => {
                let inner = &val.gift;
                inner.can_be_upgraded
            }
            Self::GiftUpgradeSent(val) => {
                let inner = &val.gift_upgrade_sent;
                inner.can_be_upgraded
            }
            _ => None,
        }
    }

    /// Helper method for nested field `chat_id`.
    #[must_use]
    pub fn chat_id(&self) -> Option<i64> {
        match self {
            Self::ChatShared(val) => {
                let inner = &val.chat_shared;
                Some(inner.chat_id)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `chats`.
    #[must_use]
    pub fn chats(&self) -> Option<&[crate::types::Chat]> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                Some(crate::types::Giveaway::chats(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `checklist_message`.
    #[must_use]
    pub fn checklist_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::ChecklistTasksAdded(val) => {
                let inner = &val.checklist_tasks_added;
                inner.checklist_message.as_deref()
            }
            Self::ChecklistTasksDone(val) => {
                let inner = &val.checklist_tasks_done;
                inner.checklist_message.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `close_date`.
    #[must_use]
    pub fn close_date(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::close_date(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `comment`.
    #[must_use]
    pub fn comment(&self) -> Option<&str> {
        match self {
            Self::SuggestedPostDeclined(val) => {
                let inner = &val.suggested_post_declined;
                inner.comment.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `community`.
    #[must_use]
    pub fn community(&self) -> Option<&crate::types::Community> {
        match self {
            Self::CommunityChatAdded(val) => {
                let inner = &val.community_chat_added;
                Some(&inner.community)
            }
            Self::CommunityChatJoined(val) => {
                let inner = &val.community_chat_joined;
                Some(&inner.community)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `convert_star_count`.
    #[must_use]
    pub fn convert_star_count(&self) -> Option<i64> {
        match self {
            Self::Gift(val) => {
                let inner = &val.gift;
                inner.convert_star_count
            }
            Self::GiftUpgradeSent(val) => {
                let inner = &val.gift_upgrade_sent;
                inner.convert_star_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `correct_option_ids`.
    #[must_use]
    pub fn correct_option_ids(&self) -> Option<&[i64]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::correct_option_ids(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `country_codes`.
    #[must_use]
    pub fn country_codes(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::country_codes(inner)
            }
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::country_codes(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `cover`.
    #[must_use]
    pub fn cover(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.cover.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `credentials`.
    #[must_use]
    pub fn credentials(&self) -> Option<&crate::types::EncryptedCredentials> {
        match self {
            Self::PassportData(val) => {
                let inner = &val.passport_data;
                Some(&inner.credentials)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `currency`.
    #[must_use]
    pub fn currency(&self) -> Option<&str> {
        match self {
            Self::Invoice(val) => {
                let inner = &val.invoice;
                Some(inner.currency.as_ref())
            }
            Self::RefundedPayment(val) => {
                let inner = &val.refunded_payment;
                Some(inner.currency.as_ref())
            }
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                Some(inner.currency.as_ref())
            }
            Self::SuggestedPostPaid(val) => {
                let inner = &val.suggested_post_paid;
                Some(inner.currency.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `custom_emoji_id`.
    #[must_use]
    pub fn custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::custom_emoji_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `description`.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Game(val) => {
                let inner = &val.game;
                Some(inner.description.as_ref())
            }
            Self::Invoice(val) => {
                let inner = &val.invoice;
                Some(inner.description.as_ref())
            }
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::description(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `description_entities`.
    #[must_use]
    pub fn description_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::description_entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `direct_message_star_count`.
    #[must_use]
    pub fn direct_message_star_count(&self) -> Option<i64> {
        match self {
            Self::DirectMessagePriceChanged(val) => {
                let inner = &val.direct_message_price_changed;
                inner.direct_message_star_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `distance`.
    #[must_use]
    pub fn distance(&self) -> Option<i64> {
        match self {
            Self::ProximityAlertTriggered(val) => {
                let inner = &val.proximity_alert_triggered;
                Some(inner.distance)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `duration`.
    #[must_use]
    pub fn duration(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.duration)
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.duration)
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.duration)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.duration)
            }
            Self::VideoChatEnded(val) => {
                let inner = &val.video_chat_ended;
                Some(inner.duration)
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                Some(inner.duration)
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                Some(inner.duration)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `emoji`.
    #[must_use]
    pub fn emoji(&self) -> Option<&str> {
        match self {
            Self::Dice(val) => {
                let inner = &val.dice;
                Some(inner.emoji.as_ref())
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::emoji(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `explanation`.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::explanation(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `explanation_entities`.
    #[must_use]
    pub fn explanation_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::explanation_entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `explanation_media`.
    #[must_use]
    pub fn explanation_media(&self) -> Option<&crate::types::PollMedia> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::explanation_media(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_id`.
    #[must_use]
    pub fn file_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.file_id.as_ref())
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::file_id(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                Some(inner.file_id.as_ref())
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                Some(inner.file_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_name`.
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.file_name.as_deref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.file_name.as_deref()
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.file_name.as_deref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.file_name.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_size`.
    #[must_use]
    pub fn file_size(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.file_size
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.file_size
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.file_size
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                inner.file_size
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::file_size(inner)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.file_size
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                inner.file_size
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                inner.file_size
            }
            _ => None,
        }
    }

    /// Helper method for nested field `file_unique_id`.
    #[must_use]
    pub fn file_unique_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.file_unique_id.as_ref())
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::file_unique_id(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                Some(inner.file_unique_id.as_ref())
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                Some(inner.file_unique_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `force_reply`.
    #[must_use]
    pub fn force_reply(&self) -> Option<bool> {
        self.reply_markup().and_then(|inner| inner.force_reply)
    }

    /// Helper method for nested field `foursquare_id`.
    #[must_use]
    pub fn foursquare_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.foursquare_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `foursquare_type`.
    #[must_use]
    pub fn foursquare_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.foursquare_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `from_attachment_menu`.
    #[must_use]
    pub fn from_attachment_menu(&self) -> Option<bool> {
        match self {
            Self::WriteAccessAllowed(val) => {
                let inner = &val.write_access_allowed;
                inner.from_attachment_menu
            }
            _ => None,
        }
    }

    /// Helper method for nested field `from_request`.
    #[must_use]
    pub fn from_request(&self) -> Option<bool> {
        match self {
            Self::WriteAccessAllowed(val) => {
                let inner = &val.write_access_allowed;
                inner.from_request
            }
            _ => None,
        }
    }

    /// Helper method for nested field `giveaway_message`.
    #[must_use]
    pub fn giveaway_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::GiveawayCompleted(val) => {
                let inner = &val.giveaway_completed;
                inner.giveaway_message.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `giveaway_message_id`.
    #[must_use]
    pub fn giveaway_message_id(&self) -> Option<i64> {
        match self {
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                Some(crate::types::GiveawayWinners::giveaway_message_id(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `google_place_id`.
    #[must_use]
    pub fn google_place_id(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.google_place_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `google_place_type`.
    #[must_use]
    pub fn google_place_type(&self) -> Option<&str> {
        match self {
            Self::Venue(val) => {
                let inner = val.venue.as_ref();
                inner.google_place_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `has_public_winners`.
    #[must_use]
    pub fn has_public_winners(&self) -> Option<bool> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::has_public_winners(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `heading`.
    #[must_use]
    pub fn heading(&self) -> Option<u16> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.heading
            }
            _ => None,
        }
    }

    /// Helper method for nested field `height`.
    #[must_use]
    pub fn height(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.height)
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.height)
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::height(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.height)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `horizontal_accuracy`.
    #[must_use]
    pub fn horizontal_accuracy(&self) -> Option<f64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.horizontal_accuracy
            }
            _ => None,
        }
    }

    /// Helper method for nested field `icon_color`.
    #[must_use]
    pub fn icon_color(&self) -> Option<i32> {
        match self {
            Self::ForumTopicCreated(val) => {
                let inner = &val.forum_topic_created;
                Some(inner.icon_color)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `icon_custom_emoji_id`.
    #[must_use]
    pub fn icon_custom_emoji_id(&self) -> Option<&str> {
        match self {
            Self::ForumTopicCreated(val) => {
                let inner = &val.forum_topic_created;
                inner.icon_custom_emoji_id.as_deref()
            }
            Self::ForumTopicEdited(val) => {
                let inner = &val.forum_topic_edited;
                inner.icon_custom_emoji_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `inline_keyboard`.
    #[must_use]
    pub fn inline_keyboard(&self) -> Option<&[Box<[crate::types::InlineKeyboardButton]>]> {
        self.reply_markup()
            .map(|inner| inner.inline_keyboard.as_ref())
    }

    /// Helper method for nested field `invoice_payload`.
    #[must_use]
    pub fn invoice_payload(&self) -> Option<&str> {
        match self {
            Self::RefundedPayment(val) => {
                let inner = &val.refunded_payment;
                Some(inner.invoice_payload.as_ref())
            }
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                Some(inner.invoice_payload.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_animated`.
    #[must_use]
    pub fn is_animated(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::is_animated(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_anonymous`.
    #[must_use]
    pub fn is_anonymous(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::is_anonymous(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_closed`.
    #[must_use]
    pub fn is_closed(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::is_closed(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_disabled`.
    #[must_use]
    pub fn is_disabled(&self) -> Option<bool> {
        self.link_preview_options()
            .and_then(|inner| inner.is_disabled)
    }

    /// Helper method for nested field `is_first_recurring`.
    #[must_use]
    pub fn is_first_recurring(&self) -> Option<bool> {
        match self {
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                inner.is_first_recurring
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_manual`.
    #[must_use]
    pub fn is_manual(&self) -> Option<bool> {
        self.quote().and_then(|inner| inner.is_manual)
    }

    /// Helper method for nested field `is_name_implicit`.
    #[must_use]
    pub fn is_name_implicit(&self) -> Option<bool> {
        match self {
            Self::ForumTopicCreated(val) => {
                let inner = &val.forum_topic_created;
                inner.is_name_implicit
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_private`.
    #[must_use]
    pub fn is_private(&self) -> Option<bool> {
        match self {
            Self::Gift(val) => {
                let inner = &val.gift;
                inner.is_private
            }
            Self::GiftUpgradeSent(val) => {
                let inner = &val.gift_upgrade_sent;
                inner.is_private
            }
            Self::UniqueGift(val) => {
                let inner = &val.unique_gift;
                inner.is_private
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_recurring`.
    #[must_use]
    pub fn is_recurring(&self) -> Option<bool> {
        match self {
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                inner.is_recurring
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_rtl`.
    #[must_use]
    pub fn is_rtl(&self) -> Option<bool> {
        match self {
            Self::RichMessage(val) => {
                let inner = &val.rich_message;
                inner.is_rtl
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_star_giveaway`.
    #[must_use]
    pub fn is_star_giveaway(&self) -> Option<bool> {
        match self {
            Self::GiveawayCompleted(val) => {
                let inner = &val.giveaway_completed;
                inner.is_star_giveaway
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_upgrade_separate`.
    #[must_use]
    pub fn is_upgrade_separate(&self) -> Option<bool> {
        match self {
            Self::Gift(val) => {
                let inner = &val.gift;
                inner.is_upgrade_separate
            }
            Self::GiftUpgradeSent(val) => {
                let inner = &val.gift_upgrade_sent;
                inner.is_upgrade_separate
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_video`.
    #[must_use]
    pub fn is_video(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::is_video(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `last_resale_amount`.
    #[must_use]
    pub fn last_resale_amount(&self) -> Option<i64> {
        match self {
            Self::UniqueGift(val) => {
                let inner = &val.unique_gift;
                inner.last_resale_amount
            }
            _ => None,
        }
    }

    /// Helper method for nested field `last_resale_currency`.
    #[must_use]
    pub fn last_resale_currency(&self) -> Option<&str> {
        match self {
            Self::UniqueGift(val) => {
                let inner = &val.unique_gift;
                inner.last_resale_currency.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `latitude`.
    #[must_use]
    pub fn latitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                Some(inner.latitude)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `length`.
    #[must_use]
    pub fn length(&self) -> Option<i64> {
        match self {
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                Some(inner.length)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `live_period`.
    #[must_use]
    pub fn live_period(&self) -> Option<i64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.live_period
            }
            _ => None,
        }
    }

    /// Helper method for nested field `longitude`.
    #[must_use]
    pub fn longitude(&self) -> Option<f64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                Some(inner.longitude)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `marked_as_done_task_ids`.
    #[must_use]
    pub fn marked_as_done_task_ids(&self) -> Option<&[i64]> {
        match self {
            Self::ChecklistTasksDone(val) => {
                let inner = &val.checklist_tasks_done;
                inner.marked_as_done_task_ids.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `marked_as_not_done_task_ids`.
    #[must_use]
    pub fn marked_as_not_done_task_ids(&self) -> Option<&[i64]> {
        match self {
            Self::ChecklistTasksDone(val) => {
                let inner = &val.checklist_tasks_done;
                inner.marked_as_not_done_task_ids.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `mask_position`.
    #[must_use]
    pub fn mask_position(&self) -> Option<&crate::types::MaskPosition> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::mask_position(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `media`.
    #[must_use]
    pub fn media(&self) -> Option<&crate::types::PollMedia> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::media(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `members_only`.
    #[must_use]
    pub fn members_only(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::members_only(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `message_auto_delete_time`.
    #[must_use]
    pub fn message_auto_delete_time(&self) -> Option<i64> {
        match self {
            Self::MessageAutoDeleteTimerChanged(val) => {
                let inner = &val.message_auto_delete_timer_changed;
                Some(inner.message_auto_delete_time)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `mime_type`.
    #[must_use]
    pub fn mime_type(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.mime_type.as_deref()
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                inner.mime_type.as_deref()
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.mime_type.as_deref()
            }
            Self::Voice(val) => {
                let inner = &val.voice;
                inner.mime_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `name`.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::ForumTopicCreated(val) => {
                let inner = &val.forum_topic_created;
                Some(inner.name.as_ref())
            }
            Self::ForumTopicEdited(val) => {
                let inner = &val.forum_topic_edited;
                inner.name.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `needs_repainting`.
    #[must_use]
    pub fn needs_repainting(&self) -> Option<bool> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::needs_repainting(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `new_owner`.
    #[must_use]
    pub fn new_owner(&self) -> Option<&crate::types::User> {
        match self {
            Self::ChatOwnerChanged(val) => {
                let inner = &val.chat_owner_changed;
                Some(inner.new_owner.as_ref())
            }
            Self::ChatOwnerLeft(val) => {
                let inner = &val.chat_owner_left;
                inner.new_owner.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `next_transfer_date`.
    #[must_use]
    pub fn next_transfer_date(&self) -> Option<i64> {
        match self {
            Self::UniqueGift(val) => {
                let inner = &val.unique_gift;
                inner.next_transfer_date
            }
            _ => None,
        }
    }

    /// Helper method for nested field `only_new_members`.
    #[must_use]
    pub fn only_new_members(&self) -> Option<bool> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::only_new_members(inner)
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::only_new_members(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `open_period`.
    #[must_use]
    pub fn open_period(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::open_period(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `option_persistent_id`.
    #[must_use]
    pub fn option_persistent_id(&self) -> Option<&str> {
        match self {
            Self::PollOptionAdded(val) => {
                let inner = &val.poll_option_added;
                Some(inner.option_persistent_id.as_ref())
            }
            Self::PollOptionDeleted(val) => {
                let inner = &val.poll_option_deleted;
                Some(inner.option_persistent_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `option_text`.
    #[must_use]
    pub fn option_text(&self) -> Option<&str> {
        match self {
            Self::PollOptionAdded(val) => {
                let inner = &val.poll_option_added;
                Some(inner.option_text.as_ref())
            }
            Self::PollOptionDeleted(val) => {
                let inner = &val.poll_option_deleted;
                Some(inner.option_text.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `option_text_entities`.
    #[must_use]
    pub fn option_text_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::PollOptionAdded(val) => {
                let inner = &val.poll_option_added;
                inner.option_text_entities.as_deref()
            }
            Self::PollOptionDeleted(val) => {
                let inner = &val.poll_option_deleted;
                inner.option_text_entities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `options`.
    #[must_use]
    pub fn options(&self) -> Option<&[crate::types::PollOption]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::options(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `order_info`.
    #[must_use]
    pub fn order_info(&self) -> Option<&crate::types::OrderInfo> {
        match self {
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                inner.order_info.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `others_can_add_tasks`.
    #[must_use]
    pub fn others_can_add_tasks(&self) -> Option<bool> {
        match self {
            Self::Checklist(val) => {
                let inner = &val.checklist;
                inner.others_can_add_tasks
            }
            _ => None,
        }
    }

    /// Helper method for nested field `others_can_mark_tasks_as_done`.
    #[must_use]
    pub fn others_can_mark_tasks_as_done(&self) -> Option<bool> {
        match self {
            Self::Checklist(val) => {
                let inner = &val.checklist;
                inner.others_can_mark_tasks_as_done
            }
            _ => None,
        }
    }

    /// Helper method for nested field `owned_gift_id`.
    #[must_use]
    pub fn owned_gift_id(&self) -> Option<&str> {
        match self {
            Self::Gift(val) => {
                let inner = &val.gift;
                inner.owned_gift_id.as_deref()
            }
            Self::GiftUpgradeSent(val) => {
                let inner = &val.gift_upgrade_sent;
                inner.owned_gift_id.as_deref()
            }
            Self::UniqueGift(val) => {
                let inner = &val.unique_gift;
                inner.owned_gift_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `paid_message_star_count`.
    #[must_use]
    pub fn paid_message_star_count(&self) -> Option<i64> {
        match self {
            Self::PaidMessagePriceChanged(val) => {
                let inner = &val.paid_message_price_changed;
                Some(inner.paid_message_star_count)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `performer`.
    #[must_use]
    pub fn performer(&self) -> Option<&str> {
        match self {
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.performer.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `phone_number`.
    #[must_use]
    pub fn phone_number(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => {
                let inner = &val.contact;
                Some(inner.phone_number.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `poll_message`.
    #[must_use]
    pub fn poll_message(&self) -> Option<&crate::types::MaybeInaccessibleMessage> {
        match self {
            Self::PollOptionAdded(val) => {
                let inner = &val.poll_option_added;
                inner.poll_message.as_deref()
            }
            Self::PollOptionDeleted(val) => {
                let inner = &val.poll_option_deleted;
                inner.poll_message.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `position`.
    #[must_use]
    pub fn position(&self) -> Option<i64> {
        self.quote().map(|inner| inner.position)
    }

    /// Helper method for nested field `prefer_large_media`.
    #[must_use]
    pub fn prefer_large_media(&self) -> Option<bool> {
        self.link_preview_options()
            .and_then(|inner| inner.prefer_large_media)
    }

    /// Helper method for nested field `prefer_small_media`.
    #[must_use]
    pub fn prefer_small_media(&self) -> Option<bool> {
        self.link_preview_options()
            .and_then(|inner| inner.prefer_small_media)
    }

    /// Helper method for nested field `premium_animation`.
    #[must_use]
    pub fn premium_animation(&self) -> Option<&crate::types::File> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::premium_animation(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `premium_subscription_month_count`.
    #[must_use]
    pub fn premium_subscription_month_count(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::premium_subscription_month_count(inner)
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::premium_subscription_month_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `prepaid_upgrade_star_count`.
    #[must_use]
    pub fn prepaid_upgrade_star_count(&self) -> Option<i64> {
        match self {
            Self::Gift(val) => {
                let inner = &val.gift;
                inner.prepaid_upgrade_star_count
            }
            Self::GiftUpgradeSent(val) => {
                let inner = &val.gift_upgrade_sent;
                inner.prepaid_upgrade_star_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `prize_description`.
    #[must_use]
    pub fn prize_description(&self) -> Option<&str> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::prize_description(inner)
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::prize_description(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `prize_star_count`.
    #[must_use]
    pub fn prize_star_count(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                crate::types::Giveaway::prize_star_count(inner)
            }
            Self::GiveawayCreated(val) => {
                let inner = &val.giveaway_created;
                inner.prize_star_count
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::prize_star_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `provider_payment_charge_id`.
    #[must_use]
    pub fn provider_payment_charge_id(&self) -> Option<&str> {
        match self {
            Self::RefundedPayment(val) => {
                let inner = &val.refunded_payment;
                inner.provider_payment_charge_id.as_deref()
            }
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                Some(inner.provider_payment_charge_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `proximity_alert_radius`.
    #[must_use]
    pub fn proximity_alert_radius(&self) -> Option<i64> {
        match self {
            Self::Location(val) => {
                let inner = &val.location;
                inner.proximity_alert_radius
            }
            _ => None,
        }
    }

    /// Helper method for nested field `qualities`.
    #[must_use]
    pub fn qualities(&self) -> Option<&[crate::types::VideoQuality]> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.qualities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `question`.
    #[must_use]
    pub fn question(&self) -> Option<&str> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::question(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `question_entities`.
    #[must_use]
    pub fn question_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::question_entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `reason`.
    #[must_use]
    pub fn reason(&self) -> Option<&str> {
        match self {
            Self::SuggestedPostRefunded(val) => {
                let inner = &val.suggested_post_refunded;
                Some(inner.reason.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `request_id`.
    #[must_use]
    pub fn request_id(&self) -> Option<i64> {
        match self {
            Self::ChatShared(val) => {
                let inner = &val.chat_shared;
                Some(inner.request_id)
            }
            Self::UsersShared(val) => {
                let inner = &val.users_shared;
                Some(inner.request_id)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `sender_user`.
    #[must_use]
    pub fn sender_user(&self) -> Option<&crate::types::User> {
        self.forward_origin()
            .and_then(crate::types::MessageOrigin::sender_user)
    }

    /// Helper method for nested field `sender_user_name`.
    #[must_use]
    pub fn sender_user_name(&self) -> Option<&str> {
        self.forward_origin()
            .and_then(crate::types::MessageOrigin::sender_user_name)
    }

    /// Helper method for nested field `set_name`.
    #[must_use]
    pub fn set_name(&self) -> Option<&str> {
        match self {
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::set_name(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `shipping_option_id`.
    #[must_use]
    pub fn shipping_option_id(&self) -> Option<&str> {
        match self {
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                inner.shipping_option_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `show_above_text`.
    #[must_use]
    pub fn show_above_text(&self) -> Option<bool> {
        self.link_preview_options()
            .and_then(|inner| inner.show_above_text)
    }

    /// Helper method for nested field `star_amount`.
    #[must_use]
    pub fn star_amount(&self) -> Option<&crate::types::StarAmount> {
        match self {
            Self::SuggestedPostPaid(val) => {
                let inner = &val.suggested_post_paid;
                inner.star_amount.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `star_count`.
    #[must_use]
    pub fn star_count(&self) -> Option<i64> {
        match self {
            Self::PaidMedia(val) => {
                let inner = &val.paid_media;
                Some(inner.star_count)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `start_date`.
    #[must_use]
    pub fn start_date(&self) -> Option<i64> {
        match self {
            Self::VideoChatScheduled(val) => {
                let inner = &val.video_chat_scheduled;
                Some(inner.start_date)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `start_parameter`.
    #[must_use]
    pub fn start_parameter(&self) -> Option<&str> {
        match self {
            Self::Invoice(val) => {
                let inner = &val.invoice;
                Some(inner.start_parameter.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `start_timestamp`.
    #[must_use]
    pub fn start_timestamp(&self) -> Option<i64> {
        match self {
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.start_timestamp
            }
            _ => None,
        }
    }

    /// Helper method for nested field `state`.
    #[must_use]
    pub fn state(&self) -> Option<&str> {
        self.suggested_post_info().map(|inner| inner.state.as_ref())
    }

    /// Helper method for nested field `subscription_expiration_date`.
    #[must_use]
    pub fn subscription_expiration_date(&self) -> Option<i64> {
        match self {
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                inner.subscription_expiration_date
            }
            _ => None,
        }
    }

    /// Helper method for nested field `suggested_post_message`.
    #[must_use]
    pub fn suggested_post_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::SuggestedPostApprovalFailed(val) => {
                let inner = &val.suggested_post_approval_failed;
                inner.suggested_post_message.as_deref()
            }
            Self::SuggestedPostApproved(val) => {
                let inner = &val.suggested_post_approved;
                inner.suggested_post_message.as_deref()
            }
            Self::SuggestedPostDeclined(val) => {
                let inner = &val.suggested_post_declined;
                inner.suggested_post_message.as_deref()
            }
            Self::SuggestedPostPaid(val) => {
                let inner = &val.suggested_post_paid;
                inner.suggested_post_message.as_deref()
            }
            Self::SuggestedPostRefunded(val) => {
                let inner = &val.suggested_post_refunded;
                inner.suggested_post_message.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `tasks`.
    #[must_use]
    pub fn tasks(&self) -> Option<&[crate::types::ChecklistTask]> {
        match self {
            Self::Checklist(val) => {
                let inner = &val.checklist;
                Some(inner.tasks.as_ref())
            }
            Self::ChecklistTasksAdded(val) => {
                let inner = &val.checklist_tasks_added;
                Some(inner.tasks.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `telegram_payment_charge_id`.
    #[must_use]
    pub fn telegram_payment_charge_id(&self) -> Option<&str> {
        match self {
            Self::RefundedPayment(val) => {
                let inner = &val.refunded_payment;
                Some(inner.telegram_payment_charge_id.as_ref())
            }
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                Some(inner.telegram_payment_charge_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `text_entities`.
    #[must_use]
    pub fn text_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Game(val) => {
                let inner = &val.game;
                inner.text_entities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `thumbnail`.
    #[must_use]
    pub fn thumbnail(&self) -> Option<&crate::types::PhotoSize> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Audio(val) => {
                let inner = val.audio.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Document(val) => {
                let inner = val.document.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                crate::types::Sticker::thumbnail(inner)
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                inner.thumbnail.as_ref()
            }
            Self::VideoNote(val) => {
                let inner = val.video_note.as_ref();
                inner.thumbnail.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `title_entities`.
    #[must_use]
    pub fn title_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Checklist(val) => {
                let inner = &val.checklist;
                inner.title_entities.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `topic_id`.
    #[must_use]
    pub fn topic_id(&self) -> Option<i64> {
        self.direct_messages_topic().map(|inner| inner.topic_id)
    }

    /// Helper method for nested field `total_amount`.
    #[must_use]
    pub fn total_amount(&self) -> Option<i64> {
        match self {
            Self::Invoice(val) => {
                let inner = &val.invoice;
                Some(inner.total_amount)
            }
            Self::RefundedPayment(val) => {
                let inner = &val.refunded_payment;
                Some(inner.total_amount)
            }
            Self::SuccessfulPayment(val) => {
                let inner = val.successful_payment.as_ref();
                Some(inner.total_amount)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `total_voter_count`.
    #[must_use]
    pub fn total_voter_count(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::total_voter_count(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `transfer_star_count`.
    #[must_use]
    pub fn transfer_star_count(&self) -> Option<i64> {
        match self {
            Self::UniqueGift(val) => {
                let inner = &val.unique_gift;
                inner.transfer_star_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `traveler`.
    #[must_use]
    pub fn traveler(&self) -> Option<&crate::types::User> {
        match self {
            Self::ProximityAlertTriggered(val) => {
                let inner = &val.proximity_alert_triggered;
                Some(inner.traveler.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `type`.
    #[must_use]
    pub fn r#type(&self) -> Option<&crate::types::BackgroundType> {
        match self {
            Self::ChatBackgroundSet(val) => {
                let inner = &val.chat_background_set;
                Some(inner.r#type.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `unclaimed_prize_count`.
    #[must_use]
    pub fn unclaimed_prize_count(&self) -> Option<i64> {
        match self {
            Self::GiveawayCompleted(val) => {
                let inner = &val.giveaway_completed;
                inner.unclaimed_prize_count
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::unclaimed_prize_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `unique_gift_number`.
    #[must_use]
    pub fn unique_gift_number(&self) -> Option<i64> {
        match self {
            Self::Gift(val) => {
                let inner = &val.gift;
                inner.unique_gift_number
            }
            Self::GiftUpgradeSent(val) => {
                let inner = &val.gift_upgrade_sent;
                inner.unique_gift_number
            }
            _ => None,
        }
    }

    /// Helper method for nested field `url`.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        self.link_preview_options()
            .and_then(|inner| inner.url.as_deref())
    }

    /// Helper method for nested field `user`.
    #[must_use]
    pub fn user(&self) -> Option<&crate::types::User> {
        self.direct_messages_topic()
            .and_then(|inner| inner.user.as_deref())
    }

    /// Helper method for nested field `user_id`.
    #[must_use]
    pub fn user_id(&self) -> Option<i64> {
        match self {
            Self::Contact(val) => {
                let inner = &val.contact;
                inner.user_id
            }
            _ => None,
        }
    }

    /// Helper method for nested field `value`.
    #[must_use]
    pub fn value(&self) -> Option<u8> {
        match self {
            Self::Dice(val) => {
                let inner = &val.dice;
                Some(inner.value)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `vcard`.
    #[must_use]
    pub fn vcard(&self) -> Option<&str> {
        match self {
            Self::Contact(val) => {
                let inner = &val.contact;
                inner.vcard.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `was_refunded`.
    #[must_use]
    pub fn was_refunded(&self) -> Option<bool> {
        match self {
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                crate::types::GiveawayWinners::was_refunded(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `watcher`.
    #[must_use]
    pub fn watcher(&self) -> Option<&crate::types::User> {
        match self {
            Self::ProximityAlertTriggered(val) => {
                let inner = &val.proximity_alert_triggered;
                Some(inner.watcher.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `web_app_name`.
    #[must_use]
    pub fn web_app_name(&self) -> Option<&str> {
        match self {
            Self::WriteAccessAllowed(val) => {
                let inner = &val.write_access_allowed;
                inner.web_app_name.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `width`.
    #[must_use]
    pub fn width(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => {
                let inner = val.animation.as_ref();
                Some(inner.width)
            }
            Self::LivePhoto(val) => {
                let inner = &val.live_photo;
                Some(inner.width)
            }
            Self::Sticker(val) => {
                let inner = val.sticker.as_ref();
                Some(crate::types::Sticker::width(inner))
            }
            Self::Video(val) => {
                let inner = val.video.as_ref();
                Some(inner.width)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `winner_count`.
    #[must_use]
    pub fn winner_count(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                Some(crate::types::Giveaway::winner_count(inner))
            }
            Self::GiveawayCompleted(val) => {
                let inner = &val.giveaway_completed;
                Some(inner.winner_count)
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                Some(crate::types::GiveawayWinners::winner_count(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `winners`.
    #[must_use]
    pub fn winners(&self) -> Option<&[crate::types::User]> {
        match self {
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                Some(crate::types::GiveawayWinners::winners(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `winners_selection_date`.
    #[must_use]
    pub fn winners_selection_date(&self) -> Option<i64> {
        match self {
            Self::Giveaway(val) => {
                let inner = &val.giveaway;
                Some(crate::types::Giveaway::winners_selection_date(inner))
            }
            Self::GiveawayWinners(val) => {
                let inner = &val.giveaway_winners;
                Some(crate::types::GiveawayWinners::winners_selection_date(inner))
            }
            _ => None,
        }
    }
}
impl From<crate::types::MessageAnimation> for Message {
    fn from(val: crate::types::MessageAnimation) -> Self {
        Self::Animation(val)
    }
}
impl TryFrom<Message> for crate::types::MessageAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Animation(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageAnimation),
            ))
        }
    }
}
impl From<crate::types::MessageLivePhoto> for Message {
    fn from(val: crate::types::MessageLivePhoto) -> Self {
        Self::LivePhoto(val)
    }
}
impl TryFrom<Message> for crate::types::MessageLivePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::LivePhoto(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageLivePhoto),
            ))
        }
    }
}
impl From<crate::types::MessageVenue> for Message {
    fn from(val: crate::types::MessageVenue) -> Self {
        Self::Venue(val)
    }
}
impl TryFrom<Message> for crate::types::MessageVenue {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Venue(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageVenue),
            ))
        }
    }
}
impl From<crate::types::MessageAudio> for Message {
    fn from(val: crate::types::MessageAudio) -> Self {
        Self::Audio(val)
    }
}
impl TryFrom<Message> for crate::types::MessageAudio {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Audio(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageAudio),
            ))
        }
    }
}
impl From<crate::types::MessageBoostAdded> for Message {
    fn from(val: crate::types::MessageBoostAdded) -> Self {
        Self::BoostAdded(val)
    }
}
impl TryFrom<Message> for crate::types::MessageBoostAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::BoostAdded(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageBoostAdded),
            ))
        }
    }
}
impl From<crate::types::MessageChannelChatCreated> for Message {
    fn from(val: crate::types::MessageChannelChatCreated) -> Self {
        Self::ChannelChatCreated(val)
    }
}
impl TryFrom<Message> for crate::types::MessageChannelChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ChannelChatCreated(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageChannelChatCreated),
            ))
        }
    }
}
impl From<crate::types::MessageChatBackgroundSet> for Message {
    fn from(val: crate::types::MessageChatBackgroundSet) -> Self {
        Self::ChatBackgroundSet(val)
    }
}
impl TryFrom<Message> for crate::types::MessageChatBackgroundSet {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ChatBackgroundSet(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageChatBackgroundSet),
            ))
        }
    }
}
impl From<crate::types::MessageChatOwnerChanged> for Message {
    fn from(val: crate::types::MessageChatOwnerChanged) -> Self {
        Self::ChatOwnerChanged(val)
    }
}
impl TryFrom<Message> for crate::types::MessageChatOwnerChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ChatOwnerChanged(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageChatOwnerChanged),
            ))
        }
    }
}
impl From<crate::types::MessageChatOwnerLeft> for Message {
    fn from(val: crate::types::MessageChatOwnerLeft) -> Self {
        Self::ChatOwnerLeft(val)
    }
}
impl TryFrom<Message> for crate::types::MessageChatOwnerLeft {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ChatOwnerLeft(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageChatOwnerLeft),
            ))
        }
    }
}
impl From<crate::types::MessageChatShared> for Message {
    fn from(val: crate::types::MessageChatShared) -> Self {
        Self::ChatShared(val)
    }
}
impl TryFrom<Message> for crate::types::MessageChatShared {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ChatShared(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageChatShared),
            ))
        }
    }
}
impl From<crate::types::MessageChecklist> for Message {
    fn from(val: crate::types::MessageChecklist) -> Self {
        Self::Checklist(val)
    }
}
impl TryFrom<Message> for crate::types::MessageChecklist {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Checklist(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageChecklist),
            ))
        }
    }
}
impl From<crate::types::MessageChecklistTasksAdded> for Message {
    fn from(val: crate::types::MessageChecklistTasksAdded) -> Self {
        Self::ChecklistTasksAdded(val)
    }
}
impl TryFrom<Message> for crate::types::MessageChecklistTasksAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ChecklistTasksAdded(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageChecklistTasksAdded),
            ))
        }
    }
}
impl From<crate::types::MessageChecklistTasksDone> for Message {
    fn from(val: crate::types::MessageChecklistTasksDone) -> Self {
        Self::ChecklistTasksDone(val)
    }
}
impl TryFrom<Message> for crate::types::MessageChecklistTasksDone {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ChecklistTasksDone(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageChecklistTasksDone),
            ))
        }
    }
}
impl From<crate::types::MessageCommunityChatAdded> for Message {
    fn from(val: crate::types::MessageCommunityChatAdded) -> Self {
        Self::CommunityChatAdded(val)
    }
}
impl TryFrom<Message> for crate::types::MessageCommunityChatAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::CommunityChatAdded(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageCommunityChatAdded),
            ))
        }
    }
}
impl From<crate::types::MessageCommunityChatJoined> for Message {
    fn from(val: crate::types::MessageCommunityChatJoined) -> Self {
        Self::CommunityChatJoined(val)
    }
}
impl TryFrom<Message> for crate::types::MessageCommunityChatJoined {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::CommunityChatJoined(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageCommunityChatJoined),
            ))
        }
    }
}
impl From<crate::types::MessageCommunityChatRemoved> for Message {
    fn from(val: crate::types::MessageCommunityChatRemoved) -> Self {
        Self::CommunityChatRemoved(val)
    }
}
impl TryFrom<Message> for crate::types::MessageCommunityChatRemoved {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::CommunityChatRemoved(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageCommunityChatRemoved),
            ))
        }
    }
}
impl From<crate::types::MessageConnectedWebsite> for Message {
    fn from(val: crate::types::MessageConnectedWebsite) -> Self {
        Self::ConnectedWebsite(val)
    }
}
impl TryFrom<Message> for crate::types::MessageConnectedWebsite {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ConnectedWebsite(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageConnectedWebsite),
            ))
        }
    }
}
impl From<crate::types::MessageContact> for Message {
    fn from(val: crate::types::MessageContact) -> Self {
        Self::Contact(val)
    }
}
impl TryFrom<Message> for crate::types::MessageContact {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Contact(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageContact),
            ))
        }
    }
}
impl From<crate::types::MessageDeleteChatPhoto> for Message {
    fn from(val: crate::types::MessageDeleteChatPhoto) -> Self {
        Self::DeleteChatPhoto(val)
    }
}
impl TryFrom<Message> for crate::types::MessageDeleteChatPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::DeleteChatPhoto(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageDeleteChatPhoto),
            ))
        }
    }
}
impl From<crate::types::MessageDice> for Message {
    fn from(val: crate::types::MessageDice) -> Self {
        Self::Dice(val)
    }
}
impl TryFrom<Message> for crate::types::MessageDice {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Dice(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageDice),
            ))
        }
    }
}
impl From<crate::types::MessageDirectMessagePriceChanged> for Message {
    fn from(val: crate::types::MessageDirectMessagePriceChanged) -> Self {
        Self::DirectMessagePriceChanged(val)
    }
}
impl TryFrom<Message> for crate::types::MessageDirectMessagePriceChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::DirectMessagePriceChanged(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageDirectMessagePriceChanged),
            ))
        }
    }
}
impl From<crate::types::MessageDocument> for Message {
    fn from(val: crate::types::MessageDocument) -> Self {
        Self::Document(val)
    }
}
impl TryFrom<Message> for crate::types::MessageDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Document(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageDocument),
            ))
        }
    }
}
impl From<crate::types::MessageForumTopicClosed> for Message {
    fn from(val: crate::types::MessageForumTopicClosed) -> Self {
        Self::ForumTopicClosed(val)
    }
}
impl TryFrom<Message> for crate::types::MessageForumTopicClosed {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ForumTopicClosed(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageForumTopicClosed),
            ))
        }
    }
}
impl From<crate::types::MessageForumTopicCreated> for Message {
    fn from(val: crate::types::MessageForumTopicCreated) -> Self {
        Self::ForumTopicCreated(val)
    }
}
impl TryFrom<Message> for crate::types::MessageForumTopicCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ForumTopicCreated(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageForumTopicCreated),
            ))
        }
    }
}
impl From<crate::types::MessageForumTopicEdited> for Message {
    fn from(val: crate::types::MessageForumTopicEdited) -> Self {
        Self::ForumTopicEdited(val)
    }
}
impl TryFrom<Message> for crate::types::MessageForumTopicEdited {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ForumTopicEdited(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageForumTopicEdited),
            ))
        }
    }
}
impl From<crate::types::MessageForumTopicReopened> for Message {
    fn from(val: crate::types::MessageForumTopicReopened) -> Self {
        Self::ForumTopicReopened(val)
    }
}
impl TryFrom<Message> for crate::types::MessageForumTopicReopened {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ForumTopicReopened(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageForumTopicReopened),
            ))
        }
    }
}
impl From<crate::types::MessageGame> for Message {
    fn from(val: crate::types::MessageGame) -> Self {
        Self::Game(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGame {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Game(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGame),
            ))
        }
    }
}
impl From<crate::types::MessageGeneralForumTopicHidden> for Message {
    fn from(val: crate::types::MessageGeneralForumTopicHidden) -> Self {
        Self::GeneralForumTopicHidden(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGeneralForumTopicHidden {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::GeneralForumTopicHidden(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGeneralForumTopicHidden),
            ))
        }
    }
}
impl From<crate::types::MessageGeneralForumTopicUnhidden> for Message {
    fn from(val: crate::types::MessageGeneralForumTopicUnhidden) -> Self {
        Self::GeneralForumTopicUnhidden(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGeneralForumTopicUnhidden {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::GeneralForumTopicUnhidden(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGeneralForumTopicUnhidden),
            ))
        }
    }
}
impl From<crate::types::MessageGift> for Message {
    fn from(val: crate::types::MessageGift) -> Self {
        Self::Gift(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGift {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Gift(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGift),
            ))
        }
    }
}
impl From<crate::types::MessageGiftUpgradeSent> for Message {
    fn from(val: crate::types::MessageGiftUpgradeSent) -> Self {
        Self::GiftUpgradeSent(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGiftUpgradeSent {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::GiftUpgradeSent(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGiftUpgradeSent),
            ))
        }
    }
}
impl From<crate::types::MessageGiveaway> for Message {
    fn from(val: crate::types::MessageGiveaway) -> Self {
        Self::Giveaway(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGiveaway {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Giveaway(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGiveaway),
            ))
        }
    }
}
impl From<crate::types::MessageGiveawayCompleted> for Message {
    fn from(val: crate::types::MessageGiveawayCompleted) -> Self {
        Self::GiveawayCompleted(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGiveawayCompleted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::GiveawayCompleted(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGiveawayCompleted),
            ))
        }
    }
}
impl From<crate::types::MessageGiveawayCreated> for Message {
    fn from(val: crate::types::MessageGiveawayCreated) -> Self {
        Self::GiveawayCreated(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGiveawayCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::GiveawayCreated(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGiveawayCreated),
            ))
        }
    }
}
impl From<crate::types::MessageGiveawayWinners> for Message {
    fn from(val: crate::types::MessageGiveawayWinners) -> Self {
        Self::GiveawayWinners(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGiveawayWinners {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::GiveawayWinners(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGiveawayWinners),
            ))
        }
    }
}
impl From<crate::types::MessageGroupChatCreated> for Message {
    fn from(val: crate::types::MessageGroupChatCreated) -> Self {
        Self::GroupChatCreated(val)
    }
}
impl TryFrom<Message> for crate::types::MessageGroupChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::GroupChatCreated(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageGroupChatCreated),
            ))
        }
    }
}
impl From<crate::types::MessageInvoice> for Message {
    fn from(val: crate::types::MessageInvoice) -> Self {
        Self::Invoice(val)
    }
}
impl TryFrom<Message> for crate::types::MessageInvoice {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Invoice(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageInvoice),
            ))
        }
    }
}
impl From<crate::types::MessageLeftChatMember> for Message {
    fn from(val: crate::types::MessageLeftChatMember) -> Self {
        Self::LeftChatMember(val)
    }
}
impl TryFrom<Message> for crate::types::MessageLeftChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::LeftChatMember(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageLeftChatMember),
            ))
        }
    }
}
impl From<crate::types::MessageLocation> for Message {
    fn from(val: crate::types::MessageLocation) -> Self {
        Self::Location(val)
    }
}
impl TryFrom<Message> for crate::types::MessageLocation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Location(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageLocation),
            ))
        }
    }
}
impl From<crate::types::MessageManagedBotCreated> for Message {
    fn from(val: crate::types::MessageManagedBotCreated) -> Self {
        Self::ManagedBotCreated(val)
    }
}
impl TryFrom<Message> for crate::types::MessageManagedBotCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ManagedBotCreated(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageManagedBotCreated),
            ))
        }
    }
}
impl From<crate::types::MessageMessageAutoDeleteTimerChanged> for Message {
    fn from(val: crate::types::MessageMessageAutoDeleteTimerChanged) -> Self {
        Self::MessageAutoDeleteTimerChanged(val)
    }
}
impl TryFrom<Message> for crate::types::MessageMessageAutoDeleteTimerChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::MessageAutoDeleteTimerChanged(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageMessageAutoDeleteTimerChanged),
            ))
        }
    }
}
impl From<crate::types::MessageMigrateFromChatId> for Message {
    fn from(val: crate::types::MessageMigrateFromChatId) -> Self {
        Self::MigrateFromChatId(val)
    }
}
impl TryFrom<Message> for crate::types::MessageMigrateFromChatId {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::MigrateFromChatId(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageMigrateFromChatId),
            ))
        }
    }
}
impl From<crate::types::MessageMigrateToChatId> for Message {
    fn from(val: crate::types::MessageMigrateToChatId) -> Self {
        Self::MigrateToChatId(val)
    }
}
impl TryFrom<Message> for crate::types::MessageMigrateToChatId {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::MigrateToChatId(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageMigrateToChatId),
            ))
        }
    }
}
impl From<crate::types::MessageNewChatMembers> for Message {
    fn from(val: crate::types::MessageNewChatMembers) -> Self {
        Self::NewChatMembers(val)
    }
}
impl TryFrom<Message> for crate::types::MessageNewChatMembers {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::NewChatMembers(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageNewChatMembers),
            ))
        }
    }
}
impl From<crate::types::MessageNewChatPhoto> for Message {
    fn from(val: crate::types::MessageNewChatPhoto) -> Self {
        Self::NewChatPhoto(val)
    }
}
impl TryFrom<Message> for crate::types::MessageNewChatPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::NewChatPhoto(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageNewChatPhoto),
            ))
        }
    }
}
impl From<crate::types::MessageNewChatTitle> for Message {
    fn from(val: crate::types::MessageNewChatTitle) -> Self {
        Self::NewChatTitle(val)
    }
}
impl TryFrom<Message> for crate::types::MessageNewChatTitle {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::NewChatTitle(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageNewChatTitle),
            ))
        }
    }
}
impl From<crate::types::MessagePaidMedia> for Message {
    fn from(val: crate::types::MessagePaidMedia) -> Self {
        Self::PaidMedia(val)
    }
}
impl TryFrom<Message> for crate::types::MessagePaidMedia {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::PaidMedia(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessagePaidMedia),
            ))
        }
    }
}
impl From<crate::types::MessagePaidMessagePriceChanged> for Message {
    fn from(val: crate::types::MessagePaidMessagePriceChanged) -> Self {
        Self::PaidMessagePriceChanged(val)
    }
}
impl TryFrom<Message> for crate::types::MessagePaidMessagePriceChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::PaidMessagePriceChanged(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessagePaidMessagePriceChanged),
            ))
        }
    }
}
impl From<crate::types::MessagePassportData> for Message {
    fn from(val: crate::types::MessagePassportData) -> Self {
        Self::PassportData(val)
    }
}
impl TryFrom<Message> for crate::types::MessagePassportData {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::PassportData(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessagePassportData),
            ))
        }
    }
}
impl From<crate::types::MessagePhoto> for Message {
    fn from(val: crate::types::MessagePhoto) -> Self {
        Self::Photo(val)
    }
}
impl TryFrom<Message> for crate::types::MessagePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Photo(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessagePhoto),
            ))
        }
    }
}
impl From<crate::types::MessagePinnedMessage> for Message {
    fn from(val: crate::types::MessagePinnedMessage) -> Self {
        Self::PinnedMessage(val)
    }
}
impl TryFrom<Message> for crate::types::MessagePinnedMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::PinnedMessage(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessagePinnedMessage),
            ))
        }
    }
}
impl From<crate::types::MessagePoll> for Message {
    fn from(val: crate::types::MessagePoll) -> Self {
        Self::Poll(val)
    }
}
impl TryFrom<Message> for crate::types::MessagePoll {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Poll(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessagePoll),
            ))
        }
    }
}
impl From<crate::types::MessagePollOptionAdded> for Message {
    fn from(val: crate::types::MessagePollOptionAdded) -> Self {
        Self::PollOptionAdded(val)
    }
}
impl TryFrom<Message> for crate::types::MessagePollOptionAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::PollOptionAdded(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessagePollOptionAdded),
            ))
        }
    }
}
impl From<crate::types::MessagePollOptionDeleted> for Message {
    fn from(val: crate::types::MessagePollOptionDeleted) -> Self {
        Self::PollOptionDeleted(val)
    }
}
impl TryFrom<Message> for crate::types::MessagePollOptionDeleted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::PollOptionDeleted(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessagePollOptionDeleted),
            ))
        }
    }
}
impl From<crate::types::MessageProximityAlertTriggered> for Message {
    fn from(val: crate::types::MessageProximityAlertTriggered) -> Self {
        Self::ProximityAlertTriggered(val)
    }
}
impl TryFrom<Message> for crate::types::MessageProximityAlertTriggered {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::ProximityAlertTriggered(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageProximityAlertTriggered),
            ))
        }
    }
}
impl From<crate::types::MessageRefundedPayment> for Message {
    fn from(val: crate::types::MessageRefundedPayment) -> Self {
        Self::RefundedPayment(val)
    }
}
impl TryFrom<Message> for crate::types::MessageRefundedPayment {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::RefundedPayment(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageRefundedPayment),
            ))
        }
    }
}
impl From<crate::types::MessageRichMessage> for Message {
    fn from(val: crate::types::MessageRichMessage) -> Self {
        Self::RichMessage(val)
    }
}
impl TryFrom<Message> for crate::types::MessageRichMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::RichMessage(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageRichMessage),
            ))
        }
    }
}
impl From<crate::types::MessageSticker> for Message {
    fn from(val: crate::types::MessageSticker) -> Self {
        Self::Sticker(val)
    }
}
impl TryFrom<Message> for crate::types::MessageSticker {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Sticker(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageSticker),
            ))
        }
    }
}
impl From<crate::types::MessageStory> for Message {
    fn from(val: crate::types::MessageStory) -> Self {
        Self::Story(val)
    }
}
impl TryFrom<Message> for crate::types::MessageStory {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Story(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageStory),
            ))
        }
    }
}
impl From<crate::types::MessageSuccessfulPayment> for Message {
    fn from(val: crate::types::MessageSuccessfulPayment) -> Self {
        Self::SuccessfulPayment(val)
    }
}
impl TryFrom<Message> for crate::types::MessageSuccessfulPayment {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::SuccessfulPayment(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageSuccessfulPayment),
            ))
        }
    }
}
impl From<crate::types::MessageSuggestedPostApprovalFailed> for Message {
    fn from(val: crate::types::MessageSuggestedPostApprovalFailed) -> Self {
        Self::SuggestedPostApprovalFailed(val)
    }
}
impl TryFrom<Message> for crate::types::MessageSuggestedPostApprovalFailed {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::SuggestedPostApprovalFailed(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageSuggestedPostApprovalFailed),
            ))
        }
    }
}
impl From<crate::types::MessageSuggestedPostApproved> for Message {
    fn from(val: crate::types::MessageSuggestedPostApproved) -> Self {
        Self::SuggestedPostApproved(val)
    }
}
impl TryFrom<Message> for crate::types::MessageSuggestedPostApproved {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::SuggestedPostApproved(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageSuggestedPostApproved),
            ))
        }
    }
}
impl From<crate::types::MessageSuggestedPostDeclined> for Message {
    fn from(val: crate::types::MessageSuggestedPostDeclined) -> Self {
        Self::SuggestedPostDeclined(val)
    }
}
impl TryFrom<Message> for crate::types::MessageSuggestedPostDeclined {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::SuggestedPostDeclined(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageSuggestedPostDeclined),
            ))
        }
    }
}
impl From<crate::types::MessageSuggestedPostPaid> for Message {
    fn from(val: crate::types::MessageSuggestedPostPaid) -> Self {
        Self::SuggestedPostPaid(val)
    }
}
impl TryFrom<Message> for crate::types::MessageSuggestedPostPaid {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::SuggestedPostPaid(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageSuggestedPostPaid),
            ))
        }
    }
}
impl From<crate::types::MessageSuggestedPostRefunded> for Message {
    fn from(val: crate::types::MessageSuggestedPostRefunded) -> Self {
        Self::SuggestedPostRefunded(val)
    }
}
impl TryFrom<Message> for crate::types::MessageSuggestedPostRefunded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::SuggestedPostRefunded(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageSuggestedPostRefunded),
            ))
        }
    }
}
impl From<crate::types::MessageSupergroupChatCreated> for Message {
    fn from(val: crate::types::MessageSupergroupChatCreated) -> Self {
        Self::SupergroupChatCreated(val)
    }
}
impl TryFrom<Message> for crate::types::MessageSupergroupChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::SupergroupChatCreated(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageSupergroupChatCreated),
            ))
        }
    }
}
impl From<crate::types::MessageText> for Message {
    fn from(val: crate::types::MessageText) -> Self {
        Self::Text(val)
    }
}
impl TryFrom<Message> for crate::types::MessageText {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Text(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageText),
            ))
        }
    }
}
impl From<crate::types::MessageUniqueGift> for Message {
    fn from(val: crate::types::MessageUniqueGift) -> Self {
        Self::UniqueGift(val)
    }
}
impl TryFrom<Message> for crate::types::MessageUniqueGift {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::UniqueGift(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageUniqueGift),
            ))
        }
    }
}
impl From<crate::types::MessageUsersShared> for Message {
    fn from(val: crate::types::MessageUsersShared) -> Self {
        Self::UsersShared(val)
    }
}
impl TryFrom<Message> for crate::types::MessageUsersShared {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::UsersShared(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageUsersShared),
            ))
        }
    }
}
impl From<crate::types::MessageVideo> for Message {
    fn from(val: crate::types::MessageVideo) -> Self {
        Self::Video(val)
    }
}
impl TryFrom<Message> for crate::types::MessageVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Video(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageVideo),
            ))
        }
    }
}
impl From<crate::types::MessageVideoChatEnded> for Message {
    fn from(val: crate::types::MessageVideoChatEnded) -> Self {
        Self::VideoChatEnded(val)
    }
}
impl TryFrom<Message> for crate::types::MessageVideoChatEnded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::VideoChatEnded(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageVideoChatEnded),
            ))
        }
    }
}
impl From<crate::types::MessageVideoChatParticipantsInvited> for Message {
    fn from(val: crate::types::MessageVideoChatParticipantsInvited) -> Self {
        Self::VideoChatParticipantsInvited(val)
    }
}
impl TryFrom<Message> for crate::types::MessageVideoChatParticipantsInvited {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::VideoChatParticipantsInvited(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageVideoChatParticipantsInvited),
            ))
        }
    }
}
impl From<crate::types::MessageVideoChatScheduled> for Message {
    fn from(val: crate::types::MessageVideoChatScheduled) -> Self {
        Self::VideoChatScheduled(val)
    }
}
impl TryFrom<Message> for crate::types::MessageVideoChatScheduled {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::VideoChatScheduled(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageVideoChatScheduled),
            ))
        }
    }
}
impl From<crate::types::MessageVideoChatStarted> for Message {
    fn from(val: crate::types::MessageVideoChatStarted) -> Self {
        Self::VideoChatStarted(val)
    }
}
impl TryFrom<Message> for crate::types::MessageVideoChatStarted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::VideoChatStarted(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageVideoChatStarted),
            ))
        }
    }
}
impl From<crate::types::MessageVideoNote> for Message {
    fn from(val: crate::types::MessageVideoNote) -> Self {
        Self::VideoNote(val)
    }
}
impl TryFrom<Message> for crate::types::MessageVideoNote {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::VideoNote(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageVideoNote),
            ))
        }
    }
}
impl From<crate::types::MessageVoice> for Message {
    fn from(val: crate::types::MessageVoice) -> Self {
        Self::Voice(val)
    }
}
impl TryFrom<Message> for crate::types::MessageVoice {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Voice(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageVoice),
            ))
        }
    }
}
impl From<crate::types::MessageWebAppData> for Message {
    fn from(val: crate::types::MessageWebAppData) -> Self {
        Self::WebAppData(val)
    }
}
impl TryFrom<Message> for crate::types::MessageWebAppData {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::WebAppData(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageWebAppData),
            ))
        }
    }
}
impl From<crate::types::MessageWriteAccessAllowed> for Message {
    fn from(val: crate::types::MessageWriteAccessAllowed) -> Self {
        Self::WriteAccessAllowed(val)
    }
}
impl TryFrom<Message> for crate::types::MessageWriteAccessAllowed {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::WriteAccessAllowed(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageWriteAccessAllowed),
            ))
        }
    }
}
impl From<crate::types::MessageUnknown> for Message {
    fn from(val: crate::types::MessageUnknown) -> Self {
        Self::Unknown(val)
    }
}
impl TryFrom<Message> for crate::types::MessageUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Message) -> Result<Self, Self::Error> {
        if let Message::Unknown(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Message),
                stringify!(MessageUnknown),
            ))
        }
    }
}
