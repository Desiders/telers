use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    Animation(crate::types::MessageAnimation),
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
    ProximityAlertTriggered(crate::types::MessageProximityAlertTriggered),
    RefundedPayment(crate::types::MessageRefundedPayment),
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
    Venue(crate::types::MessageVenue),
    Video(crate::types::MessageVideo),
    VideoChatEnded(crate::types::MessageVideoChatEnded),
    VideoChatParticipantsInvited(crate::types::MessageVideoChatParticipantsInvited),
    VideoChatScheduled(crate::types::MessageVideoChatScheduled),
    VideoChatStarted(crate::types::MessageVideoChatStarted),
    VideoNote(crate::types::MessageVideoNote),
    Voice(crate::types::MessageVoice),
    WebAppData(crate::types::MessageWebAppData),
    WriteAccessAllowed(crate::types::MessageWriteAccessAllowed),
}
impl Message {
    /// Helper method for field `animation`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Message is an animation, information about the animation. For backward compatibility, when this field is set, the document field will also be set
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::Animation> {
        match self {
            Self::Animation(val) => Some(val.animation.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `audio`.
    ///
    /// # Variants
    /// - `MessageAudio`. Message is an audio file, information about the file
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::Audio(val) => Some(val.audio.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `author_signature`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageAudio`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageBoostAdded`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageChannelChatCreated`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageChatBackgroundSet`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageChatOwnerChanged`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageChatOwnerLeft`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageChatShared`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageChecklist`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageChecklistTasksAdded`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageChecklistTasksDone`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageConnectedWebsite`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageContact`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageDeleteChatPhoto`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageDice`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageDirectMessagePriceChanged`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageDocument`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageForumTopicClosed`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageForumTopicCreated`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageForumTopicEdited`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageForumTopicReopened`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGame`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGeneralForumTopicHidden`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGeneralForumTopicUnhidden`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGift`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGiftUpgradeSent`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGiveaway`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGiveawayCompleted`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGiveawayCreated`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGiveawayWinners`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageGroupChatCreated`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageInvoice`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageLeftChatMember`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageLocation`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageMessageAutoDeleteTimerChanged`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageMigrateFromChatId`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageMigrateToChatId`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageNewChatMembers`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageNewChatPhoto`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageNewChatTitle`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessagePaidMedia`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessagePaidMessagePriceChanged`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessagePassportData`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessagePhoto`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessagePinnedMessage`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessagePoll`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageProximityAlertTriggered`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageRefundedPayment`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageSticker`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageStory`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageSuccessfulPayment`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageSuggestedPostApprovalFailed`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageSuggestedPostApproved`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageSuggestedPostDeclined`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageSuggestedPostPaid`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageSuggestedPostRefunded`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageSupergroupChatCreated`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageText`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageUniqueGift`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageUsersShared`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageVenue`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageVideo`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageVideoChatEnded`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageVideoChatParticipantsInvited`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageVideoChatScheduled`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageVideoChatStarted`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageVideoNote`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageVoice`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageWebAppData`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    /// - `MessageWriteAccessAllowed`. Signature of the post author for messages in channels, or the custom title of an anonymous group administrator
    #[must_use]
    pub fn author_signature(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.author_signature.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.author_signature.as_deref(),
            Self::RefundedPayment(val) => val.author_signature.as_deref(),
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
            Self::Venue(val) => val.author_signature.as_deref(),
            Self::Video(val) => val.author_signature.as_deref(),
            Self::VideoChatEnded(val) => val.author_signature.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.author_signature.as_deref(),
            Self::VideoChatScheduled(val) => val.author_signature.as_deref(),
            Self::VideoChatStarted(val) => val.author_signature.as_deref(),
            Self::VideoNote(val) => val.author_signature.as_deref(),
            Self::Voice(val) => val.author_signature.as_deref(),
            Self::WebAppData(val) => val.author_signature.as_deref(),
            Self::WriteAccessAllowed(val) => val.author_signature.as_deref(),
        }
    }

    /// Helper method for field `boost_added`.
    ///
    /// # Variants
    /// - `MessageBoostAdded`. Service message: user boosted the chat
    #[must_use]
    pub fn boost_added(&self) -> Option<&crate::types::ChatBoostAdded> {
        match self {
            Self::BoostAdded(val) => Some(&val.boost_added),
            _ => None,
        }
    }

    /// Helper method for field `business_connection_id`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageAudio`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageBoostAdded`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageChannelChatCreated`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageChatBackgroundSet`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageChatOwnerChanged`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageChatOwnerLeft`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageChatShared`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageChecklist`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageChecklistTasksAdded`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageChecklistTasksDone`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageConnectedWebsite`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageContact`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageDeleteChatPhoto`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageDice`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageDirectMessagePriceChanged`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageDocument`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageForumTopicClosed`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageForumTopicCreated`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageForumTopicEdited`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageForumTopicReopened`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGame`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGeneralForumTopicHidden`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGeneralForumTopicUnhidden`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGift`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGiftUpgradeSent`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGiveaway`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGiveawayCompleted`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGiveawayCreated`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGiveawayWinners`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageGroupChatCreated`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageInvoice`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageLeftChatMember`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageLocation`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageMessageAutoDeleteTimerChanged`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageMigrateFromChatId`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageMigrateToChatId`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageNewChatMembers`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageNewChatPhoto`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageNewChatTitle`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessagePaidMedia`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessagePaidMessagePriceChanged`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessagePassportData`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessagePhoto`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessagePinnedMessage`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessagePoll`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageProximityAlertTriggered`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageRefundedPayment`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageSticker`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageStory`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageSuccessfulPayment`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageSuggestedPostApprovalFailed`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageSuggestedPostApproved`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageSuggestedPostDeclined`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageSuggestedPostPaid`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageSuggestedPostRefunded`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageSupergroupChatCreated`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageText`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageUniqueGift`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageUsersShared`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageVenue`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageVideo`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageVideoChatEnded`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageVideoChatParticipantsInvited`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageVideoChatScheduled`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageVideoChatStarted`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageVideoNote`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageVoice`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageWebAppData`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    /// - `MessageWriteAccessAllowed`. Unique identifier of the business connection from which the message was received. If non-empty, the message belongs to a chat of the corresponding business account that is independent from any potential bot chat which might share the same identifier.
    #[must_use]
    pub fn business_connection_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.business_connection_id.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.business_connection_id.as_deref(),
            Self::RefundedPayment(val) => val.business_connection_id.as_deref(),
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
            Self::Venue(val) => val.business_connection_id.as_deref(),
            Self::Video(val) => val.business_connection_id.as_deref(),
            Self::VideoChatEnded(val) => val.business_connection_id.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.business_connection_id.as_deref(),
            Self::VideoChatScheduled(val) => val.business_connection_id.as_deref(),
            Self::VideoChatStarted(val) => val.business_connection_id.as_deref(),
            Self::VideoNote(val) => val.business_connection_id.as_deref(),
            Self::Voice(val) => val.business_connection_id.as_deref(),
            Self::WebAppData(val) => val.business_connection_id.as_deref(),
            Self::WriteAccessAllowed(val) => val.business_connection_id.as_deref(),
        }
    }

    /// Helper method for field `caption`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageAudio`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageBoostAdded`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageChannelChatCreated`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageChatBackgroundSet`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageChatOwnerChanged`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageChatOwnerLeft`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageChatShared`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageChecklist`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageChecklistTasksAdded`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageChecklistTasksDone`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageConnectedWebsite`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageContact`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageDeleteChatPhoto`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageDice`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageDirectMessagePriceChanged`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageDocument`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageForumTopicClosed`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageForumTopicCreated`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageForumTopicEdited`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageForumTopicReopened`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGame`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGeneralForumTopicHidden`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGeneralForumTopicUnhidden`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGift`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGiftUpgradeSent`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGiveaway`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGiveawayCompleted`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGiveawayCreated`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGiveawayWinners`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageGroupChatCreated`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageInvoice`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageLeftChatMember`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageLocation`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageMessageAutoDeleteTimerChanged`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageMigrateFromChatId`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageMigrateToChatId`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageNewChatMembers`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageNewChatPhoto`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageNewChatTitle`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessagePaidMedia`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessagePaidMessagePriceChanged`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessagePassportData`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessagePhoto`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessagePinnedMessage`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessagePoll`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageProximityAlertTriggered`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageRefundedPayment`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageSticker`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageStory`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageSuccessfulPayment`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageSuggestedPostApprovalFailed`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageSuggestedPostApproved`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageSuggestedPostDeclined`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageSuggestedPostPaid`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageSuggestedPostRefunded`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageSupergroupChatCreated`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageText`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageUniqueGift`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageUsersShared`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageVenue`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageVideo`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageVideoChatEnded`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageVideoChatParticipantsInvited`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageVideoChatScheduled`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageVideoChatStarted`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageVideoNote`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageVoice`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageWebAppData`. Caption for the animation, audio, document, paid media, photo, video or voice
    /// - `MessageWriteAccessAllowed`. Caption for the animation, audio, document, paid media, photo, video or voice
    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.caption.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.caption.as_deref(),
            Self::RefundedPayment(val) => val.caption.as_deref(),
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
            Self::Venue(val) => val.caption.as_deref(),
            Self::Video(val) => val.caption.as_deref(),
            Self::VideoChatEnded(val) => val.caption.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.caption.as_deref(),
            Self::VideoChatScheduled(val) => val.caption.as_deref(),
            Self::VideoChatStarted(val) => val.caption.as_deref(),
            Self::VideoNote(val) => val.caption.as_deref(),
            Self::Voice(val) => val.caption.as_deref(),
            Self::WebAppData(val) => val.caption.as_deref(),
            Self::WriteAccessAllowed(val) => val.caption.as_deref(),
        }
    }

    /// Helper method for field `caption_entities`.
    ///
    /// # Variants
    /// - `MessageAnimation`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageAudio`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageBoostAdded`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageChannelChatCreated`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageChatBackgroundSet`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageChatOwnerChanged`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageChatOwnerLeft`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageChatShared`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageChecklist`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageChecklistTasksAdded`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageChecklistTasksDone`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageConnectedWebsite`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageContact`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageDeleteChatPhoto`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageDice`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageDirectMessagePriceChanged`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageDocument`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageForumTopicClosed`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageForumTopicCreated`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageForumTopicEdited`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageForumTopicReopened`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGame`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGeneralForumTopicHidden`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGeneralForumTopicUnhidden`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGift`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGiftUpgradeSent`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGiveaway`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGiveawayCompleted`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGiveawayCreated`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGiveawayWinners`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageGroupChatCreated`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageInvoice`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageLeftChatMember`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageLocation`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageMessageAutoDeleteTimerChanged`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageMigrateFromChatId`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageMigrateToChatId`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageNewChatMembers`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageNewChatPhoto`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageNewChatTitle`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessagePaidMedia`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessagePaidMessagePriceChanged`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessagePassportData`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessagePhoto`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessagePinnedMessage`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessagePoll`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageProximityAlertTriggered`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageRefundedPayment`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageSticker`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageStory`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageSuccessfulPayment`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageSuggestedPostApprovalFailed`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageSuggestedPostApproved`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageSuggestedPostDeclined`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageSuggestedPostPaid`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageSuggestedPostRefunded`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageSupergroupChatCreated`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageText`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageUniqueGift`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageUsersShared`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageVenue`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageVideo`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageVideoChatEnded`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageVideoChatParticipantsInvited`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageVideoChatScheduled`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageVideoChatStarted`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageVideoNote`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageVoice`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageWebAppData`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    /// - `MessageWriteAccessAllowed`. For messages with a caption, special entities like usernames, URLs, bot commands, etc. that appear in the caption
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Animation(val) => val.caption_entities.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.caption_entities.as_deref(),
            Self::RefundedPayment(val) => val.caption_entities.as_deref(),
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
            Self::Venue(val) => val.caption_entities.as_deref(),
            Self::Video(val) => val.caption_entities.as_deref(),
            Self::VideoChatEnded(val) => val.caption_entities.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.caption_entities.as_deref(),
            Self::VideoChatScheduled(val) => val.caption_entities.as_deref(),
            Self::VideoChatStarted(val) => val.caption_entities.as_deref(),
            Self::VideoNote(val) => val.caption_entities.as_deref(),
            Self::Voice(val) => val.caption_entities.as_deref(),
            Self::WebAppData(val) => val.caption_entities.as_deref(),
            Self::WriteAccessAllowed(val) => val.caption_entities.as_deref(),
        }
    }

    /// Helper method for field `channel_chat_created`.
    ///
    /// # Variants
    /// - `MessageChannelChatCreated`. Service message: the channel has been created. This field can't be received in a message coming through updates, because bot can't be a member of a channel when it is created. It can only be found in `reply_to_message` if someone replies to a very first message in a channel.
    #[must_use]
    pub fn channel_chat_created(&self) -> Option<bool> {
        match self {
            Self::ChannelChatCreated(val) => Some(val.channel_chat_created),
            _ => None,
        }
    }

    /// Helper method for field `chat`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Chat the message belongs to
    /// - `MessageAudio`. Chat the message belongs to
    /// - `MessageBoostAdded`. Chat the message belongs to
    /// - `MessageChannelChatCreated`. Chat the message belongs to
    /// - `MessageChatBackgroundSet`. Chat the message belongs to
    /// - `MessageChatOwnerChanged`. Chat the message belongs to
    /// - `MessageChatOwnerLeft`. Chat the message belongs to
    /// - `MessageChatShared`. Chat the message belongs to
    /// - `MessageChecklist`. Chat the message belongs to
    /// - `MessageChecklistTasksAdded`. Chat the message belongs to
    /// - `MessageChecklistTasksDone`. Chat the message belongs to
    /// - `MessageConnectedWebsite`. Chat the message belongs to
    /// - `MessageContact`. Chat the message belongs to
    /// - `MessageDeleteChatPhoto`. Chat the message belongs to
    /// - `MessageDice`. Chat the message belongs to
    /// - `MessageDirectMessagePriceChanged`. Chat the message belongs to
    /// - `MessageDocument`. Chat the message belongs to
    /// - `MessageForumTopicClosed`. Chat the message belongs to
    /// - `MessageForumTopicCreated`. Chat the message belongs to
    /// - `MessageForumTopicEdited`. Chat the message belongs to
    /// - `MessageForumTopicReopened`. Chat the message belongs to
    /// - `MessageGame`. Chat the message belongs to
    /// - `MessageGeneralForumTopicHidden`. Chat the message belongs to
    /// - `MessageGeneralForumTopicUnhidden`. Chat the message belongs to
    /// - `MessageGift`. Chat the message belongs to
    /// - `MessageGiftUpgradeSent`. Chat the message belongs to
    /// - `MessageGiveaway`. Chat the message belongs to
    /// - `MessageGiveawayCompleted`. Chat the message belongs to
    /// - `MessageGiveawayCreated`. Chat the message belongs to
    /// - `MessageGiveawayWinners`. Chat the message belongs to
    /// - `MessageGroupChatCreated`. Chat the message belongs to
    /// - `MessageInvoice`. Chat the message belongs to
    /// - `MessageLeftChatMember`. Chat the message belongs to
    /// - `MessageLocation`. Chat the message belongs to
    /// - `MessageMessageAutoDeleteTimerChanged`. Chat the message belongs to
    /// - `MessageMigrateFromChatId`. Chat the message belongs to
    /// - `MessageMigrateToChatId`. Chat the message belongs to
    /// - `MessageNewChatMembers`. Chat the message belongs to
    /// - `MessageNewChatPhoto`. Chat the message belongs to
    /// - `MessageNewChatTitle`. Chat the message belongs to
    /// - `MessagePaidMedia`. Chat the message belongs to
    /// - `MessagePaidMessagePriceChanged`. Chat the message belongs to
    /// - `MessagePassportData`. Chat the message belongs to
    /// - `MessagePhoto`. Chat the message belongs to
    /// - `MessagePinnedMessage`. Chat the message belongs to
    /// - `MessagePoll`. Chat the message belongs to
    /// - `MessageProximityAlertTriggered`. Chat the message belongs to
    /// - `MessageRefundedPayment`. Chat the message belongs to
    /// - `MessageSticker`. Chat the message belongs to
    /// - `MessageStory`. Chat the message belongs to
    /// - `MessageSuccessfulPayment`. Chat the message belongs to
    /// - `MessageSuggestedPostApprovalFailed`. Chat the message belongs to
    /// - `MessageSuggestedPostApproved`. Chat the message belongs to
    /// - `MessageSuggestedPostDeclined`. Chat the message belongs to
    /// - `MessageSuggestedPostPaid`. Chat the message belongs to
    /// - `MessageSuggestedPostRefunded`. Chat the message belongs to
    /// - `MessageSupergroupChatCreated`. Chat the message belongs to
    /// - `MessageText`. Chat the message belongs to
    /// - `MessageUniqueGift`. Chat the message belongs to
    /// - `MessageUsersShared`. Chat the message belongs to
    /// - `MessageVenue`. Chat the message belongs to
    /// - `MessageVideo`. Chat the message belongs to
    /// - `MessageVideoChatEnded`. Chat the message belongs to
    /// - `MessageVideoChatParticipantsInvited`. Chat the message belongs to
    /// - `MessageVideoChatScheduled`. Chat the message belongs to
    /// - `MessageVideoChatStarted`. Chat the message belongs to
    /// - `MessageVideoNote`. Chat the message belongs to
    /// - `MessageVoice`. Chat the message belongs to
    /// - `MessageWebAppData`. Chat the message belongs to
    /// - `MessageWriteAccessAllowed`. Chat the message belongs to
    #[must_use]
    pub fn chat(&self) -> &crate::types::Chat {
        match self {
            Self::Animation(val) => val.chat.as_ref(),
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
            Self::ProximityAlertTriggered(val) => val.chat.as_ref(),
            Self::RefundedPayment(val) => val.chat.as_ref(),
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
            Self::Venue(val) => val.chat.as_ref(),
            Self::Video(val) => val.chat.as_ref(),
            Self::VideoChatEnded(val) => val.chat.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.chat.as_ref(),
            Self::VideoChatScheduled(val) => val.chat.as_ref(),
            Self::VideoChatStarted(val) => val.chat.as_ref(),
            Self::VideoNote(val) => val.chat.as_ref(),
            Self::Voice(val) => val.chat.as_ref(),
            Self::WebAppData(val) => val.chat.as_ref(),
            Self::WriteAccessAllowed(val) => val.chat.as_ref(),
        }
    }

    /// Helper method for field `chat_background_set`.
    ///
    /// # Variants
    /// - `MessageChatBackgroundSet`. Service message: chat background set
    #[must_use]
    pub fn chat_background_set(&self) -> Option<&crate::types::ChatBackground> {
        match self {
            Self::ChatBackgroundSet(val) => Some(&val.chat_background_set),
            _ => None,
        }
    }

    /// Helper method for field `chat_owner_changed`.
    ///
    /// # Variants
    /// - `MessageChatOwnerChanged`. Service message: chat owner has changed
    #[must_use]
    pub fn chat_owner_changed(&self) -> Option<&crate::types::ChatOwnerChanged> {
        match self {
            Self::ChatOwnerChanged(val) => Some(&val.chat_owner_changed),
            _ => None,
        }
    }

    /// Helper method for field `chat_owner_left`.
    ///
    /// # Variants
    /// - `MessageChatOwnerLeft`. Service message: chat owner has left
    #[must_use]
    pub fn chat_owner_left(&self) -> Option<&crate::types::ChatOwnerLeft> {
        match self {
            Self::ChatOwnerLeft(val) => Some(&val.chat_owner_left),
            _ => None,
        }
    }

    /// Helper method for field `chat_shared`.
    ///
    /// # Variants
    /// - `MessageChatShared`. Service message: a chat was shared with the bot
    #[must_use]
    pub fn chat_shared(&self) -> Option<&crate::types::ChatShared> {
        match self {
            Self::ChatShared(val) => Some(&val.chat_shared),
            _ => None,
        }
    }

    /// Helper method for field `checklist`.
    ///
    /// # Variants
    /// - `MessageChecklist`. Message is a checklist
    #[must_use]
    pub fn checklist(&self) -> Option<&crate::types::Checklist> {
        match self {
            Self::Checklist(val) => Some(&val.checklist),
            _ => None,
        }
    }

    /// Helper method for field `checklist_tasks_added`.
    ///
    /// # Variants
    /// - `MessageChecklistTasksAdded`. Service message: tasks were added to a checklist
    #[must_use]
    pub fn checklist_tasks_added(&self) -> Option<&crate::types::ChecklistTasksAdded> {
        match self {
            Self::ChecklistTasksAdded(val) => Some(&val.checklist_tasks_added),
            _ => None,
        }
    }

    /// Helper method for field `checklist_tasks_done`.
    ///
    /// # Variants
    /// - `MessageChecklistTasksDone`. Service message: some tasks in a checklist were marked as done or not done
    #[must_use]
    pub fn checklist_tasks_done(&self) -> Option<&crate::types::ChecklistTasksDone> {
        match self {
            Self::ChecklistTasksDone(val) => Some(&val.checklist_tasks_done),
            _ => None,
        }
    }

    /// Helper method for field `connected_website`.
    ///
    /// # Variants
    /// - `MessageConnectedWebsite`. The domain name of the website on which the user has logged in. More about Telegram Login: <https://core.telegram.org/widgets/login>
    #[must_use]
    pub fn connected_website(&self) -> Option<&str> {
        match self {
            Self::ConnectedWebsite(val) => Some(val.connected_website.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `contact`.
    ///
    /// # Variants
    /// - `MessageContact`. Message is a shared contact, information about the contact
    #[must_use]
    pub fn contact(&self) -> Option<&crate::types::Contact> {
        match self {
            Self::Contact(val) => Some(&val.contact),
            _ => None,
        }
    }

    /// Helper method for field `date`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageAudio`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageBoostAdded`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageChannelChatCreated`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageChatBackgroundSet`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageChatOwnerChanged`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageChatOwnerLeft`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageChatShared`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageChecklist`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageChecklistTasksAdded`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageChecklistTasksDone`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageConnectedWebsite`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageContact`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageDeleteChatPhoto`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageDice`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageDirectMessagePriceChanged`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageDocument`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageForumTopicClosed`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageForumTopicCreated`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageForumTopicEdited`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageForumTopicReopened`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGame`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGeneralForumTopicHidden`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGeneralForumTopicUnhidden`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGift`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGiftUpgradeSent`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGiveaway`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGiveawayCompleted`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGiveawayCreated`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGiveawayWinners`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageGroupChatCreated`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageInvoice`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageLeftChatMember`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageLocation`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageMessageAutoDeleteTimerChanged`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageMigrateFromChatId`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageMigrateToChatId`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageNewChatMembers`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageNewChatPhoto`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageNewChatTitle`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessagePaidMedia`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessagePaidMessagePriceChanged`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessagePassportData`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessagePhoto`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessagePinnedMessage`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessagePoll`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageProximityAlertTriggered`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageRefundedPayment`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageSticker`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageStory`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageSuccessfulPayment`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageSuggestedPostApprovalFailed`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageSuggestedPostApproved`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageSuggestedPostDeclined`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageSuggestedPostPaid`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageSuggestedPostRefunded`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageSupergroupChatCreated`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageText`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageUniqueGift`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageUsersShared`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageVenue`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageVideo`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageVideoChatEnded`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageVideoChatParticipantsInvited`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageVideoChatScheduled`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageVideoChatStarted`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageVideoNote`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageVoice`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageWebAppData`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    /// - `MessageWriteAccessAllowed`. Date the message was sent in Unix time. It is always a positive number, representing a valid date.
    #[must_use]
    pub fn date(&self) -> i64 {
        match self {
            Self::Animation(val) => val.date,
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
            Self::ProximityAlertTriggered(val) => val.date,
            Self::RefundedPayment(val) => val.date,
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
            Self::Venue(val) => val.date,
            Self::Video(val) => val.date,
            Self::VideoChatEnded(val) => val.date,
            Self::VideoChatParticipantsInvited(val) => val.date,
            Self::VideoChatScheduled(val) => val.date,
            Self::VideoChatStarted(val) => val.date,
            Self::VideoNote(val) => val.date,
            Self::Voice(val) => val.date,
            Self::WebAppData(val) => val.date,
            Self::WriteAccessAllowed(val) => val.date,
        }
    }

    /// Helper method for field `delete_chat_photo`.
    ///
    /// # Variants
    /// - `MessageDeleteChatPhoto`. Service message: the chat photo was deleted
    #[must_use]
    pub fn delete_chat_photo(&self) -> Option<bool> {
        match self {
            Self::DeleteChatPhoto(val) => Some(val.delete_chat_photo),
            _ => None,
        }
    }

    /// Helper method for field `dice`.
    ///
    /// # Variants
    /// - `MessageDice`. Message is a dice with random value
    #[must_use]
    pub fn dice(&self) -> Option<&crate::types::Dice> {
        match self {
            Self::Dice(val) => Some(&val.dice),
            _ => None,
        }
    }

    /// Helper method for field `direct_message_price_changed`.
    ///
    /// # Variants
    /// - `MessageDirectMessagePriceChanged`. Service message: the price for paid messages in the corresponding direct messages chat of a channel has changed
    #[must_use]
    pub fn direct_message_price_changed(&self) -> Option<&crate::types::DirectMessagePriceChanged> {
        match self {
            Self::DirectMessagePriceChanged(val) => Some(&val.direct_message_price_changed),
            _ => None,
        }
    }

    /// Helper method for field `direct_messages_topic`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Information about the direct messages chat topic that contains the message
    /// - `MessageAudio`. Information about the direct messages chat topic that contains the message
    /// - `MessageBoostAdded`. Information about the direct messages chat topic that contains the message
    /// - `MessageChannelChatCreated`. Information about the direct messages chat topic that contains the message
    /// - `MessageChatBackgroundSet`. Information about the direct messages chat topic that contains the message
    /// - `MessageChatOwnerChanged`. Information about the direct messages chat topic that contains the message
    /// - `MessageChatOwnerLeft`. Information about the direct messages chat topic that contains the message
    /// - `MessageChatShared`. Information about the direct messages chat topic that contains the message
    /// - `MessageChecklist`. Information about the direct messages chat topic that contains the message
    /// - `MessageChecklistTasksAdded`. Information about the direct messages chat topic that contains the message
    /// - `MessageChecklistTasksDone`. Information about the direct messages chat topic that contains the message
    /// - `MessageConnectedWebsite`. Information about the direct messages chat topic that contains the message
    /// - `MessageContact`. Information about the direct messages chat topic that contains the message
    /// - `MessageDeleteChatPhoto`. Information about the direct messages chat topic that contains the message
    /// - `MessageDice`. Information about the direct messages chat topic that contains the message
    /// - `MessageDirectMessagePriceChanged`. Information about the direct messages chat topic that contains the message
    /// - `MessageDocument`. Information about the direct messages chat topic that contains the message
    /// - `MessageForumTopicClosed`. Information about the direct messages chat topic that contains the message
    /// - `MessageForumTopicCreated`. Information about the direct messages chat topic that contains the message
    /// - `MessageForumTopicEdited`. Information about the direct messages chat topic that contains the message
    /// - `MessageForumTopicReopened`. Information about the direct messages chat topic that contains the message
    /// - `MessageGame`. Information about the direct messages chat topic that contains the message
    /// - `MessageGeneralForumTopicHidden`. Information about the direct messages chat topic that contains the message
    /// - `MessageGeneralForumTopicUnhidden`. Information about the direct messages chat topic that contains the message
    /// - `MessageGift`. Information about the direct messages chat topic that contains the message
    /// - `MessageGiftUpgradeSent`. Information about the direct messages chat topic that contains the message
    /// - `MessageGiveaway`. Information about the direct messages chat topic that contains the message
    /// - `MessageGiveawayCompleted`. Information about the direct messages chat topic that contains the message
    /// - `MessageGiveawayCreated`. Information about the direct messages chat topic that contains the message
    /// - `MessageGiveawayWinners`. Information about the direct messages chat topic that contains the message
    /// - `MessageGroupChatCreated`. Information about the direct messages chat topic that contains the message
    /// - `MessageInvoice`. Information about the direct messages chat topic that contains the message
    /// - `MessageLeftChatMember`. Information about the direct messages chat topic that contains the message
    /// - `MessageLocation`. Information about the direct messages chat topic that contains the message
    /// - `MessageMessageAutoDeleteTimerChanged`. Information about the direct messages chat topic that contains the message
    /// - `MessageMigrateFromChatId`. Information about the direct messages chat topic that contains the message
    /// - `MessageMigrateToChatId`. Information about the direct messages chat topic that contains the message
    /// - `MessageNewChatMembers`. Information about the direct messages chat topic that contains the message
    /// - `MessageNewChatPhoto`. Information about the direct messages chat topic that contains the message
    /// - `MessageNewChatTitle`. Information about the direct messages chat topic that contains the message
    /// - `MessagePaidMedia`. Information about the direct messages chat topic that contains the message
    /// - `MessagePaidMessagePriceChanged`. Information about the direct messages chat topic that contains the message
    /// - `MessagePassportData`. Information about the direct messages chat topic that contains the message
    /// - `MessagePhoto`. Information about the direct messages chat topic that contains the message
    /// - `MessagePinnedMessage`. Information about the direct messages chat topic that contains the message
    /// - `MessagePoll`. Information about the direct messages chat topic that contains the message
    /// - `MessageProximityAlertTriggered`. Information about the direct messages chat topic that contains the message
    /// - `MessageRefundedPayment`. Information about the direct messages chat topic that contains the message
    /// - `MessageSticker`. Information about the direct messages chat topic that contains the message
    /// - `MessageStory`. Information about the direct messages chat topic that contains the message
    /// - `MessageSuccessfulPayment`. Information about the direct messages chat topic that contains the message
    /// - `MessageSuggestedPostApprovalFailed`. Information about the direct messages chat topic that contains the message
    /// - `MessageSuggestedPostApproved`. Information about the direct messages chat topic that contains the message
    /// - `MessageSuggestedPostDeclined`. Information about the direct messages chat topic that contains the message
    /// - `MessageSuggestedPostPaid`. Information about the direct messages chat topic that contains the message
    /// - `MessageSuggestedPostRefunded`. Information about the direct messages chat topic that contains the message
    /// - `MessageSupergroupChatCreated`. Information about the direct messages chat topic that contains the message
    /// - `MessageText`. Information about the direct messages chat topic that contains the message
    /// - `MessageUniqueGift`. Information about the direct messages chat topic that contains the message
    /// - `MessageUsersShared`. Information about the direct messages chat topic that contains the message
    /// - `MessageVenue`. Information about the direct messages chat topic that contains the message
    /// - `MessageVideo`. Information about the direct messages chat topic that contains the message
    /// - `MessageVideoChatEnded`. Information about the direct messages chat topic that contains the message
    /// - `MessageVideoChatParticipantsInvited`. Information about the direct messages chat topic that contains the message
    /// - `MessageVideoChatScheduled`. Information about the direct messages chat topic that contains the message
    /// - `MessageVideoChatStarted`. Information about the direct messages chat topic that contains the message
    /// - `MessageVideoNote`. Information about the direct messages chat topic that contains the message
    /// - `MessageVoice`. Information about the direct messages chat topic that contains the message
    /// - `MessageWebAppData`. Information about the direct messages chat topic that contains the message
    /// - `MessageWriteAccessAllowed`. Information about the direct messages chat topic that contains the message
    #[must_use]
    pub fn direct_messages_topic(&self) -> Option<&crate::types::DirectMessagesTopic> {
        match self {
            Self::Animation(val) => val.direct_messages_topic.as_ref(),
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
            Self::ProximityAlertTriggered(val) => val.direct_messages_topic.as_ref(),
            Self::RefundedPayment(val) => val.direct_messages_topic.as_ref(),
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
            Self::Venue(val) => val.direct_messages_topic.as_ref(),
            Self::Video(val) => val.direct_messages_topic.as_ref(),
            Self::VideoChatEnded(val) => val.direct_messages_topic.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.direct_messages_topic.as_ref(),
            Self::VideoChatScheduled(val) => val.direct_messages_topic.as_ref(),
            Self::VideoChatStarted(val) => val.direct_messages_topic.as_ref(),
            Self::VideoNote(val) => val.direct_messages_topic.as_ref(),
            Self::Voice(val) => val.direct_messages_topic.as_ref(),
            Self::WebAppData(val) => val.direct_messages_topic.as_ref(),
            Self::WriteAccessAllowed(val) => val.direct_messages_topic.as_ref(),
        }
    }

    /// Helper method for field `document`.
    ///
    /// # Variants
    /// - `MessageDocument`. Message is a general file, information about the file
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::Document> {
        match self {
            Self::Document(val) => Some(val.document.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `edit_date`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Date the message was last edited in Unix time
    /// - `MessageAudio`. Date the message was last edited in Unix time
    /// - `MessageBoostAdded`. Date the message was last edited in Unix time
    /// - `MessageChannelChatCreated`. Date the message was last edited in Unix time
    /// - `MessageChatBackgroundSet`. Date the message was last edited in Unix time
    /// - `MessageChatOwnerChanged`. Date the message was last edited in Unix time
    /// - `MessageChatOwnerLeft`. Date the message was last edited in Unix time
    /// - `MessageChatShared`. Date the message was last edited in Unix time
    /// - `MessageChecklist`. Date the message was last edited in Unix time
    /// - `MessageChecklistTasksAdded`. Date the message was last edited in Unix time
    /// - `MessageChecklistTasksDone`. Date the message was last edited in Unix time
    /// - `MessageConnectedWebsite`. Date the message was last edited in Unix time
    /// - `MessageContact`. Date the message was last edited in Unix time
    /// - `MessageDeleteChatPhoto`. Date the message was last edited in Unix time
    /// - `MessageDice`. Date the message was last edited in Unix time
    /// - `MessageDirectMessagePriceChanged`. Date the message was last edited in Unix time
    /// - `MessageDocument`. Date the message was last edited in Unix time
    /// - `MessageForumTopicClosed`. Date the message was last edited in Unix time
    /// - `MessageForumTopicCreated`. Date the message was last edited in Unix time
    /// - `MessageForumTopicEdited`. Date the message was last edited in Unix time
    /// - `MessageForumTopicReopened`. Date the message was last edited in Unix time
    /// - `MessageGame`. Date the message was last edited in Unix time
    /// - `MessageGeneralForumTopicHidden`. Date the message was last edited in Unix time
    /// - `MessageGeneralForumTopicUnhidden`. Date the message was last edited in Unix time
    /// - `MessageGift`. Date the message was last edited in Unix time
    /// - `MessageGiftUpgradeSent`. Date the message was last edited in Unix time
    /// - `MessageGiveaway`. Date the message was last edited in Unix time
    /// - `MessageGiveawayCompleted`. Date the message was last edited in Unix time
    /// - `MessageGiveawayCreated`. Date the message was last edited in Unix time
    /// - `MessageGiveawayWinners`. Date the message was last edited in Unix time
    /// - `MessageGroupChatCreated`. Date the message was last edited in Unix time
    /// - `MessageInvoice`. Date the message was last edited in Unix time
    /// - `MessageLeftChatMember`. Date the message was last edited in Unix time
    /// - `MessageLocation`. Date the message was last edited in Unix time
    /// - `MessageMessageAutoDeleteTimerChanged`. Date the message was last edited in Unix time
    /// - `MessageMigrateFromChatId`. Date the message was last edited in Unix time
    /// - `MessageMigrateToChatId`. Date the message was last edited in Unix time
    /// - `MessageNewChatMembers`. Date the message was last edited in Unix time
    /// - `MessageNewChatPhoto`. Date the message was last edited in Unix time
    /// - `MessageNewChatTitle`. Date the message was last edited in Unix time
    /// - `MessagePaidMedia`. Date the message was last edited in Unix time
    /// - `MessagePaidMessagePriceChanged`. Date the message was last edited in Unix time
    /// - `MessagePassportData`. Date the message was last edited in Unix time
    /// - `MessagePhoto`. Date the message was last edited in Unix time
    /// - `MessagePinnedMessage`. Date the message was last edited in Unix time
    /// - `MessagePoll`. Date the message was last edited in Unix time
    /// - `MessageProximityAlertTriggered`. Date the message was last edited in Unix time
    /// - `MessageRefundedPayment`. Date the message was last edited in Unix time
    /// - `MessageSticker`. Date the message was last edited in Unix time
    /// - `MessageStory`. Date the message was last edited in Unix time
    /// - `MessageSuccessfulPayment`. Date the message was last edited in Unix time
    /// - `MessageSuggestedPostApprovalFailed`. Date the message was last edited in Unix time
    /// - `MessageSuggestedPostApproved`. Date the message was last edited in Unix time
    /// - `MessageSuggestedPostDeclined`. Date the message was last edited in Unix time
    /// - `MessageSuggestedPostPaid`. Date the message was last edited in Unix time
    /// - `MessageSuggestedPostRefunded`. Date the message was last edited in Unix time
    /// - `MessageSupergroupChatCreated`. Date the message was last edited in Unix time
    /// - `MessageText`. Date the message was last edited in Unix time
    /// - `MessageUniqueGift`. Date the message was last edited in Unix time
    /// - `MessageUsersShared`. Date the message was last edited in Unix time
    /// - `MessageVenue`. Date the message was last edited in Unix time
    /// - `MessageVideo`. Date the message was last edited in Unix time
    /// - `MessageVideoChatEnded`. Date the message was last edited in Unix time
    /// - `MessageVideoChatParticipantsInvited`. Date the message was last edited in Unix time
    /// - `MessageVideoChatScheduled`. Date the message was last edited in Unix time
    /// - `MessageVideoChatStarted`. Date the message was last edited in Unix time
    /// - `MessageVideoNote`. Date the message was last edited in Unix time
    /// - `MessageVoice`. Date the message was last edited in Unix time
    /// - `MessageWebAppData`. Date the message was last edited in Unix time
    /// - `MessageWriteAccessAllowed`. Date the message was last edited in Unix time
    #[must_use]
    pub fn edit_date(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.edit_date,
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
            Self::ProximityAlertTriggered(val) => val.edit_date,
            Self::RefundedPayment(val) => val.edit_date,
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
            Self::Venue(val) => val.edit_date,
            Self::Video(val) => val.edit_date,
            Self::VideoChatEnded(val) => val.edit_date,
            Self::VideoChatParticipantsInvited(val) => val.edit_date,
            Self::VideoChatScheduled(val) => val.edit_date,
            Self::VideoChatStarted(val) => val.edit_date,
            Self::VideoNote(val) => val.edit_date,
            Self::Voice(val) => val.edit_date,
            Self::WebAppData(val) => val.edit_date,
            Self::WriteAccessAllowed(val) => val.edit_date,
        }
    }

    /// Helper method for field `effect_id`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Unique identifier of the message effect added to the message
    /// - `MessageAudio`. Unique identifier of the message effect added to the message
    /// - `MessageBoostAdded`. Unique identifier of the message effect added to the message
    /// - `MessageChannelChatCreated`. Unique identifier of the message effect added to the message
    /// - `MessageChatBackgroundSet`. Unique identifier of the message effect added to the message
    /// - `MessageChatOwnerChanged`. Unique identifier of the message effect added to the message
    /// - `MessageChatOwnerLeft`. Unique identifier of the message effect added to the message
    /// - `MessageChatShared`. Unique identifier of the message effect added to the message
    /// - `MessageChecklist`. Unique identifier of the message effect added to the message
    /// - `MessageChecklistTasksAdded`. Unique identifier of the message effect added to the message
    /// - `MessageChecklistTasksDone`. Unique identifier of the message effect added to the message
    /// - `MessageConnectedWebsite`. Unique identifier of the message effect added to the message
    /// - `MessageContact`. Unique identifier of the message effect added to the message
    /// - `MessageDeleteChatPhoto`. Unique identifier of the message effect added to the message
    /// - `MessageDice`. Unique identifier of the message effect added to the message
    /// - `MessageDirectMessagePriceChanged`. Unique identifier of the message effect added to the message
    /// - `MessageDocument`. Unique identifier of the message effect added to the message
    /// - `MessageForumTopicClosed`. Unique identifier of the message effect added to the message
    /// - `MessageForumTopicCreated`. Unique identifier of the message effect added to the message
    /// - `MessageForumTopicEdited`. Unique identifier of the message effect added to the message
    /// - `MessageForumTopicReopened`. Unique identifier of the message effect added to the message
    /// - `MessageGame`. Unique identifier of the message effect added to the message
    /// - `MessageGeneralForumTopicHidden`. Unique identifier of the message effect added to the message
    /// - `MessageGeneralForumTopicUnhidden`. Unique identifier of the message effect added to the message
    /// - `MessageGift`. Unique identifier of the message effect added to the message
    /// - `MessageGiftUpgradeSent`. Unique identifier of the message effect added to the message
    /// - `MessageGiveaway`. Unique identifier of the message effect added to the message
    /// - `MessageGiveawayCompleted`. Unique identifier of the message effect added to the message
    /// - `MessageGiveawayCreated`. Unique identifier of the message effect added to the message
    /// - `MessageGiveawayWinners`. Unique identifier of the message effect added to the message
    /// - `MessageGroupChatCreated`. Unique identifier of the message effect added to the message
    /// - `MessageInvoice`. Unique identifier of the message effect added to the message
    /// - `MessageLeftChatMember`. Unique identifier of the message effect added to the message
    /// - `MessageLocation`. Unique identifier of the message effect added to the message
    /// - `MessageMessageAutoDeleteTimerChanged`. Unique identifier of the message effect added to the message
    /// - `MessageMigrateFromChatId`. Unique identifier of the message effect added to the message
    /// - `MessageMigrateToChatId`. Unique identifier of the message effect added to the message
    /// - `MessageNewChatMembers`. Unique identifier of the message effect added to the message
    /// - `MessageNewChatPhoto`. Unique identifier of the message effect added to the message
    /// - `MessageNewChatTitle`. Unique identifier of the message effect added to the message
    /// - `MessagePaidMedia`. Unique identifier of the message effect added to the message
    /// - `MessagePaidMessagePriceChanged`. Unique identifier of the message effect added to the message
    /// - `MessagePassportData`. Unique identifier of the message effect added to the message
    /// - `MessagePhoto`. Unique identifier of the message effect added to the message
    /// - `MessagePinnedMessage`. Unique identifier of the message effect added to the message
    /// - `MessagePoll`. Unique identifier of the message effect added to the message
    /// - `MessageProximityAlertTriggered`. Unique identifier of the message effect added to the message
    /// - `MessageRefundedPayment`. Unique identifier of the message effect added to the message
    /// - `MessageSticker`. Unique identifier of the message effect added to the message
    /// - `MessageStory`. Unique identifier of the message effect added to the message
    /// - `MessageSuccessfulPayment`. Unique identifier of the message effect added to the message
    /// - `MessageSuggestedPostApprovalFailed`. Unique identifier of the message effect added to the message
    /// - `MessageSuggestedPostApproved`. Unique identifier of the message effect added to the message
    /// - `MessageSuggestedPostDeclined`. Unique identifier of the message effect added to the message
    /// - `MessageSuggestedPostPaid`. Unique identifier of the message effect added to the message
    /// - `MessageSuggestedPostRefunded`. Unique identifier of the message effect added to the message
    /// - `MessageSupergroupChatCreated`. Unique identifier of the message effect added to the message
    /// - `MessageText`. Unique identifier of the message effect added to the message
    /// - `MessageUniqueGift`. Unique identifier of the message effect added to the message
    /// - `MessageUsersShared`. Unique identifier of the message effect added to the message
    /// - `MessageVenue`. Unique identifier of the message effect added to the message
    /// - `MessageVideo`. Unique identifier of the message effect added to the message
    /// - `MessageVideoChatEnded`. Unique identifier of the message effect added to the message
    /// - `MessageVideoChatParticipantsInvited`. Unique identifier of the message effect added to the message
    /// - `MessageVideoChatScheduled`. Unique identifier of the message effect added to the message
    /// - `MessageVideoChatStarted`. Unique identifier of the message effect added to the message
    /// - `MessageVideoNote`. Unique identifier of the message effect added to the message
    /// - `MessageVoice`. Unique identifier of the message effect added to the message
    /// - `MessageWebAppData`. Unique identifier of the message effect added to the message
    /// - `MessageWriteAccessAllowed`. Unique identifier of the message effect added to the message
    #[must_use]
    pub fn effect_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.effect_id.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.effect_id.as_deref(),
            Self::RefundedPayment(val) => val.effect_id.as_deref(),
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
            Self::Venue(val) => val.effect_id.as_deref(),
            Self::Video(val) => val.effect_id.as_deref(),
            Self::VideoChatEnded(val) => val.effect_id.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.effect_id.as_deref(),
            Self::VideoChatScheduled(val) => val.effect_id.as_deref(),
            Self::VideoChatStarted(val) => val.effect_id.as_deref(),
            Self::VideoNote(val) => val.effect_id.as_deref(),
            Self::Voice(val) => val.effect_id.as_deref(),
            Self::WebAppData(val) => val.effect_id.as_deref(),
            Self::WriteAccessAllowed(val) => val.effect_id.as_deref(),
        }
    }

    /// Helper method for field `entities`.
    ///
    /// # Variants
    /// - `MessageAnimation`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageAudio`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageBoostAdded`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageChannelChatCreated`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageChatBackgroundSet`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageChatOwnerChanged`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageChatOwnerLeft`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageChatShared`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageChecklist`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageChecklistTasksAdded`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageChecklistTasksDone`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageConnectedWebsite`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageContact`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageDeleteChatPhoto`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageDice`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageDirectMessagePriceChanged`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageDocument`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageForumTopicClosed`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageForumTopicCreated`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageForumTopicEdited`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageForumTopicReopened`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGame`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGeneralForumTopicHidden`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGeneralForumTopicUnhidden`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGift`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGiftUpgradeSent`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGiveaway`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGiveawayCompleted`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGiveawayCreated`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGiveawayWinners`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageGroupChatCreated`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageInvoice`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageLeftChatMember`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageLocation`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageMessageAutoDeleteTimerChanged`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageMigrateFromChatId`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageMigrateToChatId`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageNewChatMembers`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageNewChatPhoto`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageNewChatTitle`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessagePaidMedia`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessagePaidMessagePriceChanged`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessagePassportData`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessagePhoto`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessagePinnedMessage`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessagePoll`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageProximityAlertTriggered`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageRefundedPayment`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageSticker`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageStory`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageSuccessfulPayment`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageSuggestedPostApprovalFailed`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageSuggestedPostApproved`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageSuggestedPostDeclined`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageSuggestedPostPaid`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageSuggestedPostRefunded`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageSupergroupChatCreated`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageText`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageUniqueGift`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageUsersShared`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageVenue`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageVideo`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageVideoChatEnded`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageVideoChatParticipantsInvited`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageVideoChatScheduled`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageVideoChatStarted`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageVideoNote`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageVoice`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageWebAppData`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    /// - `MessageWriteAccessAllowed`. For text messages, special entities like usernames, URLs, bot commands, etc. that appear in the text
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Animation(val) => val.entities.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.entities.as_deref(),
            Self::RefundedPayment(val) => val.entities.as_deref(),
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
            Self::Venue(val) => val.entities.as_deref(),
            Self::Video(val) => val.entities.as_deref(),
            Self::VideoChatEnded(val) => val.entities.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.entities.as_deref(),
            Self::VideoChatScheduled(val) => val.entities.as_deref(),
            Self::VideoChatStarted(val) => val.entities.as_deref(),
            Self::VideoNote(val) => val.entities.as_deref(),
            Self::Voice(val) => val.entities.as_deref(),
            Self::WebAppData(val) => val.entities.as_deref(),
            Self::WriteAccessAllowed(val) => val.entities.as_deref(),
        }
    }

    /// Helper method for field `external_reply`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageAudio`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageBoostAdded`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageChannelChatCreated`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageChatBackgroundSet`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageChatOwnerChanged`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageChatOwnerLeft`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageChatShared`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageChecklist`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageChecklistTasksAdded`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageChecklistTasksDone`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageConnectedWebsite`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageContact`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageDeleteChatPhoto`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageDice`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageDirectMessagePriceChanged`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageDocument`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageForumTopicClosed`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageForumTopicCreated`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageForumTopicEdited`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageForumTopicReopened`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGame`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGeneralForumTopicHidden`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGeneralForumTopicUnhidden`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGift`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGiftUpgradeSent`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGiveaway`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGiveawayCompleted`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGiveawayCreated`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGiveawayWinners`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageGroupChatCreated`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageInvoice`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageLeftChatMember`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageLocation`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageMessageAutoDeleteTimerChanged`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageMigrateFromChatId`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageMigrateToChatId`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageNewChatMembers`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageNewChatPhoto`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageNewChatTitle`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessagePaidMedia`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessagePaidMessagePriceChanged`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessagePassportData`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessagePhoto`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessagePinnedMessage`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessagePoll`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageProximityAlertTriggered`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageRefundedPayment`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageSticker`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageStory`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageSuccessfulPayment`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageSuggestedPostApprovalFailed`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageSuggestedPostApproved`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageSuggestedPostDeclined`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageSuggestedPostPaid`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageSuggestedPostRefunded`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageSupergroupChatCreated`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageText`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageUniqueGift`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageUsersShared`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageVenue`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageVideo`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageVideoChatEnded`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageVideoChatParticipantsInvited`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageVideoChatScheduled`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageVideoChatStarted`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageVideoNote`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageVoice`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageWebAppData`. Information about the message that is being replied to, which may come from another chat or forum topic
    /// - `MessageWriteAccessAllowed`. Information about the message that is being replied to, which may come from another chat or forum topic
    #[must_use]
    pub fn external_reply(&self) -> Option<&crate::types::ExternalReplyInfo> {
        match self {
            Self::Animation(val) => val.external_reply.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.external_reply.as_deref(),
            Self::RefundedPayment(val) => val.external_reply.as_deref(),
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
            Self::Venue(val) => val.external_reply.as_deref(),
            Self::Video(val) => val.external_reply.as_deref(),
            Self::VideoChatEnded(val) => val.external_reply.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.external_reply.as_deref(),
            Self::VideoChatScheduled(val) => val.external_reply.as_deref(),
            Self::VideoChatStarted(val) => val.external_reply.as_deref(),
            Self::VideoNote(val) => val.external_reply.as_deref(),
            Self::Voice(val) => val.external_reply.as_deref(),
            Self::WebAppData(val) => val.external_reply.as_deref(),
            Self::WriteAccessAllowed(val) => val.external_reply.as_deref(),
        }
    }

    /// Helper method for field `forum_topic_closed`.
    ///
    /// # Variants
    /// - `MessageForumTopicClosed`. Service message: forum topic closed
    #[must_use]
    pub fn forum_topic_closed(&self) -> Option<&crate::types::ForumTopicClosed> {
        match self {
            Self::ForumTopicClosed(val) => Some(&val.forum_topic_closed),
            _ => None,
        }
    }

    /// Helper method for field `forum_topic_created`.
    ///
    /// # Variants
    /// - `MessageForumTopicCreated`. Service message: forum topic created
    #[must_use]
    pub fn forum_topic_created(&self) -> Option<&crate::types::ForumTopicCreated> {
        match self {
            Self::ForumTopicCreated(val) => Some(&val.forum_topic_created),
            _ => None,
        }
    }

    /// Helper method for field `forum_topic_edited`.
    ///
    /// # Variants
    /// - `MessageForumTopicEdited`. Service message: forum topic edited
    #[must_use]
    pub fn forum_topic_edited(&self) -> Option<&crate::types::ForumTopicEdited> {
        match self {
            Self::ForumTopicEdited(val) => Some(&val.forum_topic_edited),
            _ => None,
        }
    }

    /// Helper method for field `forum_topic_reopened`.
    ///
    /// # Variants
    /// - `MessageForumTopicReopened`. Service message: forum topic reopened
    #[must_use]
    pub fn forum_topic_reopened(&self) -> Option<&crate::types::ForumTopicReopened> {
        match self {
            Self::ForumTopicReopened(val) => Some(&val.forum_topic_reopened),
            _ => None,
        }
    }

    /// Helper method for field `forward_origin`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Information about the original message for forwarded messages
    /// - `MessageAudio`. Information about the original message for forwarded messages
    /// - `MessageBoostAdded`. Information about the original message for forwarded messages
    /// - `MessageChannelChatCreated`. Information about the original message for forwarded messages
    /// - `MessageChatBackgroundSet`. Information about the original message for forwarded messages
    /// - `MessageChatOwnerChanged`. Information about the original message for forwarded messages
    /// - `MessageChatOwnerLeft`. Information about the original message for forwarded messages
    /// - `MessageChatShared`. Information about the original message for forwarded messages
    /// - `MessageChecklist`. Information about the original message for forwarded messages
    /// - `MessageChecklistTasksAdded`. Information about the original message for forwarded messages
    /// - `MessageChecklistTasksDone`. Information about the original message for forwarded messages
    /// - `MessageConnectedWebsite`. Information about the original message for forwarded messages
    /// - `MessageContact`. Information about the original message for forwarded messages
    /// - `MessageDeleteChatPhoto`. Information about the original message for forwarded messages
    /// - `MessageDice`. Information about the original message for forwarded messages
    /// - `MessageDirectMessagePriceChanged`. Information about the original message for forwarded messages
    /// - `MessageDocument`. Information about the original message for forwarded messages
    /// - `MessageForumTopicClosed`. Information about the original message for forwarded messages
    /// - `MessageForumTopicCreated`. Information about the original message for forwarded messages
    /// - `MessageForumTopicEdited`. Information about the original message for forwarded messages
    /// - `MessageForumTopicReopened`. Information about the original message for forwarded messages
    /// - `MessageGame`. Information about the original message for forwarded messages
    /// - `MessageGeneralForumTopicHidden`. Information about the original message for forwarded messages
    /// - `MessageGeneralForumTopicUnhidden`. Information about the original message for forwarded messages
    /// - `MessageGift`. Information about the original message for forwarded messages
    /// - `MessageGiftUpgradeSent`. Information about the original message for forwarded messages
    /// - `MessageGiveaway`. Information about the original message for forwarded messages
    /// - `MessageGiveawayCompleted`. Information about the original message for forwarded messages
    /// - `MessageGiveawayCreated`. Information about the original message for forwarded messages
    /// - `MessageGiveawayWinners`. Information about the original message for forwarded messages
    /// - `MessageGroupChatCreated`. Information about the original message for forwarded messages
    /// - `MessageInvoice`. Information about the original message for forwarded messages
    /// - `MessageLeftChatMember`. Information about the original message for forwarded messages
    /// - `MessageLocation`. Information about the original message for forwarded messages
    /// - `MessageMessageAutoDeleteTimerChanged`. Information about the original message for forwarded messages
    /// - `MessageMigrateFromChatId`. Information about the original message for forwarded messages
    /// - `MessageMigrateToChatId`. Information about the original message for forwarded messages
    /// - `MessageNewChatMembers`. Information about the original message for forwarded messages
    /// - `MessageNewChatPhoto`. Information about the original message for forwarded messages
    /// - `MessageNewChatTitle`. Information about the original message for forwarded messages
    /// - `MessagePaidMedia`. Information about the original message for forwarded messages
    /// - `MessagePaidMessagePriceChanged`. Information about the original message for forwarded messages
    /// - `MessagePassportData`. Information about the original message for forwarded messages
    /// - `MessagePhoto`. Information about the original message for forwarded messages
    /// - `MessagePinnedMessage`. Information about the original message for forwarded messages
    /// - `MessagePoll`. Information about the original message for forwarded messages
    /// - `MessageProximityAlertTriggered`. Information about the original message for forwarded messages
    /// - `MessageRefundedPayment`. Information about the original message for forwarded messages
    /// - `MessageSticker`. Information about the original message for forwarded messages
    /// - `MessageStory`. Information about the original message for forwarded messages
    /// - `MessageSuccessfulPayment`. Information about the original message for forwarded messages
    /// - `MessageSuggestedPostApprovalFailed`. Information about the original message for forwarded messages
    /// - `MessageSuggestedPostApproved`. Information about the original message for forwarded messages
    /// - `MessageSuggestedPostDeclined`. Information about the original message for forwarded messages
    /// - `MessageSuggestedPostPaid`. Information about the original message for forwarded messages
    /// - `MessageSuggestedPostRefunded`. Information about the original message for forwarded messages
    /// - `MessageSupergroupChatCreated`. Information about the original message for forwarded messages
    /// - `MessageText`. Information about the original message for forwarded messages
    /// - `MessageUniqueGift`. Information about the original message for forwarded messages
    /// - `MessageUsersShared`. Information about the original message for forwarded messages
    /// - `MessageVenue`. Information about the original message for forwarded messages
    /// - `MessageVideo`. Information about the original message for forwarded messages
    /// - `MessageVideoChatEnded`. Information about the original message for forwarded messages
    /// - `MessageVideoChatParticipantsInvited`. Information about the original message for forwarded messages
    /// - `MessageVideoChatScheduled`. Information about the original message for forwarded messages
    /// - `MessageVideoChatStarted`. Information about the original message for forwarded messages
    /// - `MessageVideoNote`. Information about the original message for forwarded messages
    /// - `MessageVoice`. Information about the original message for forwarded messages
    /// - `MessageWebAppData`. Information about the original message for forwarded messages
    /// - `MessageWriteAccessAllowed`. Information about the original message for forwarded messages
    #[must_use]
    pub fn forward_origin(&self) -> Option<&crate::types::MessageOrigin> {
        match self {
            Self::Animation(val) => val.forward_origin.as_ref(),
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
            Self::ProximityAlertTriggered(val) => val.forward_origin.as_ref(),
            Self::RefundedPayment(val) => val.forward_origin.as_ref(),
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
            Self::Venue(val) => val.forward_origin.as_ref(),
            Self::Video(val) => val.forward_origin.as_ref(),
            Self::VideoChatEnded(val) => val.forward_origin.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.forward_origin.as_ref(),
            Self::VideoChatScheduled(val) => val.forward_origin.as_ref(),
            Self::VideoChatStarted(val) => val.forward_origin.as_ref(),
            Self::VideoNote(val) => val.forward_origin.as_ref(),
            Self::Voice(val) => val.forward_origin.as_ref(),
            Self::WebAppData(val) => val.forward_origin.as_ref(),
            Self::WriteAccessAllowed(val) => val.forward_origin.as_ref(),
        }
    }

    /// Helper method for field `from`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageAudio`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageBoostAdded`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageChannelChatCreated`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageChatBackgroundSet`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageChatOwnerChanged`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageChatOwnerLeft`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageChatShared`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageChecklist`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageChecklistTasksAdded`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageChecklistTasksDone`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageConnectedWebsite`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageContact`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageDeleteChatPhoto`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageDice`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageDirectMessagePriceChanged`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageDocument`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageForumTopicClosed`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageForumTopicCreated`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageForumTopicEdited`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageForumTopicReopened`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGame`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGeneralForumTopicHidden`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGeneralForumTopicUnhidden`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGift`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGiftUpgradeSent`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGiveaway`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGiveawayCompleted`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGiveawayCreated`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGiveawayWinners`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageGroupChatCreated`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageInvoice`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageLeftChatMember`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageLocation`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageMessageAutoDeleteTimerChanged`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageMigrateFromChatId`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageMigrateToChatId`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageNewChatMembers`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageNewChatPhoto`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageNewChatTitle`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessagePaidMedia`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessagePaidMessagePriceChanged`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessagePassportData`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessagePhoto`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessagePinnedMessage`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessagePoll`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageProximityAlertTriggered`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageRefundedPayment`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageSticker`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageStory`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageSuccessfulPayment`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageSuggestedPostApprovalFailed`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageSuggestedPostApproved`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageSuggestedPostDeclined`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageSuggestedPostPaid`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageSuggestedPostRefunded`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageSupergroupChatCreated`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageText`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageUniqueGift`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageUsersShared`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageVenue`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageVideo`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageVideoChatEnded`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageVideoChatParticipantsInvited`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageVideoChatScheduled`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageVideoChatStarted`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageVideoNote`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageVoice`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageWebAppData`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    /// - `MessageWriteAccessAllowed`. Sender of the message; may be empty for messages sent to channels. For backward compatibility, if the message was sent on behalf of a chat, the field contains a fake sender user in non-channel chats
    #[must_use]
    pub fn from(&self) -> Option<&crate::types::User> {
        match self {
            Self::Animation(val) => val.from.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.from.as_deref(),
            Self::RefundedPayment(val) => val.from.as_deref(),
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
            Self::Venue(val) => val.from.as_deref(),
            Self::Video(val) => val.from.as_deref(),
            Self::VideoChatEnded(val) => val.from.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.from.as_deref(),
            Self::VideoChatScheduled(val) => val.from.as_deref(),
            Self::VideoChatStarted(val) => val.from.as_deref(),
            Self::VideoNote(val) => val.from.as_deref(),
            Self::Voice(val) => val.from.as_deref(),
            Self::WebAppData(val) => val.from.as_deref(),
            Self::WriteAccessAllowed(val) => val.from.as_deref(),
        }
    }

    /// Helper method for field `game`.
    ///
    /// # Variants
    /// - `MessageGame`. Message is a game, information about the game. More about games: <https://core.telegram.org/bots/api#games>
    #[must_use]
    pub fn game(&self) -> Option<&crate::types::Game> {
        match self {
            Self::Game(val) => Some(&val.game),
            _ => None,
        }
    }

    /// Helper method for field `general_forum_topic_hidden`.
    ///
    /// # Variants
    /// - `MessageGeneralForumTopicHidden`. Service message: the 'General' forum topic hidden
    #[must_use]
    pub fn general_forum_topic_hidden(&self) -> Option<&crate::types::GeneralForumTopicHidden> {
        match self {
            Self::GeneralForumTopicHidden(val) => Some(&val.general_forum_topic_hidden),
            _ => None,
        }
    }

    /// Helper method for field `general_forum_topic_unhidden`.
    ///
    /// # Variants
    /// - `MessageGeneralForumTopicUnhidden`. Service message: the 'General' forum topic unhidden
    #[must_use]
    pub fn general_forum_topic_unhidden(&self) -> Option<&crate::types::GeneralForumTopicUnhidden> {
        match self {
            Self::GeneralForumTopicUnhidden(val) => Some(&val.general_forum_topic_unhidden),
            _ => None,
        }
    }

    /// Helper method for field `gift`.
    ///
    /// # Variants
    /// - `MessageGift`. Service message: a regular gift was sent or received
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::Gift(val) => Some(&val.gift),
            _ => None,
        }
    }

    /// Helper method for field `gift_upgrade_sent`.
    ///
    /// # Variants
    /// - `MessageGiftUpgradeSent`. Service message: upgrade of a gift was purchased after the gift was sent
    #[must_use]
    pub fn gift_upgrade_sent(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::GiftUpgradeSent(val) => Some(&val.gift_upgrade_sent),
            _ => None,
        }
    }

    /// Helper method for field `giveaway`.
    ///
    /// # Variants
    /// - `MessageGiveaway`. The message is a scheduled giveaway message
    #[must_use]
    pub fn giveaway(&self) -> Option<&crate::types::Giveaway> {
        match self {
            Self::Giveaway(val) => Some(&val.giveaway),
            _ => None,
        }
    }

    /// Helper method for field `giveaway_completed`.
    ///
    /// # Variants
    /// - `MessageGiveawayCompleted`. Service message: a giveaway without public winners was completed
    #[must_use]
    pub fn giveaway_completed(&self) -> Option<&crate::types::GiveawayCompleted> {
        match self {
            Self::GiveawayCompleted(val) => Some(&val.giveaway_completed),
            _ => None,
        }
    }

    /// Helper method for field `giveaway_created`.
    ///
    /// # Variants
    /// - `MessageGiveawayCreated`. Service message: a scheduled giveaway was created
    #[must_use]
    pub fn giveaway_created(&self) -> Option<&crate::types::GiveawayCreated> {
        match self {
            Self::GiveawayCreated(val) => Some(&val.giveaway_created),
            _ => None,
        }
    }

    /// Helper method for field `giveaway_winners`.
    ///
    /// # Variants
    /// - `MessageGiveawayWinners`. A giveaway with public winners was completed
    #[must_use]
    pub fn giveaway_winners(&self) -> Option<&crate::types::GiveawayWinners> {
        match self {
            Self::GiveawayWinners(val) => Some(&val.giveaway_winners),
            _ => None,
        }
    }

    /// Helper method for field `group_chat_created`.
    ///
    /// # Variants
    /// - `MessageGroupChatCreated`. Service message: the group has been created
    #[must_use]
    pub fn group_chat_created(&self) -> Option<bool> {
        match self {
            Self::GroupChatCreated(val) => Some(val.group_chat_created),
            _ => None,
        }
    }

    /// Helper method for field `has_media_spoiler`.
    ///
    /// # Variants
    /// - `MessageAnimation`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageAudio`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageBoostAdded`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageChannelChatCreated`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageChatBackgroundSet`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageChatOwnerChanged`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageChatOwnerLeft`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageChatShared`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageChecklist`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageChecklistTasksAdded`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageChecklistTasksDone`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageConnectedWebsite`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageContact`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageDeleteChatPhoto`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageDice`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageDirectMessagePriceChanged`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageDocument`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageForumTopicClosed`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageForumTopicCreated`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageForumTopicEdited`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageForumTopicReopened`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGame`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGeneralForumTopicHidden`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGeneralForumTopicUnhidden`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGift`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGiftUpgradeSent`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGiveaway`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGiveawayCompleted`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGiveawayCreated`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGiveawayWinners`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageGroupChatCreated`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageInvoice`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageLeftChatMember`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageLocation`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageMessageAutoDeleteTimerChanged`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageMigrateFromChatId`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageMigrateToChatId`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageNewChatMembers`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageNewChatPhoto`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageNewChatTitle`. `true`, if the message media is covered by a spoiler animation
    /// - `MessagePaidMedia`. `true`, if the message media is covered by a spoiler animation
    /// - `MessagePaidMessagePriceChanged`. `true`, if the message media is covered by a spoiler animation
    /// - `MessagePassportData`. `true`, if the message media is covered by a spoiler animation
    /// - `MessagePhoto`. `true`, if the message media is covered by a spoiler animation
    /// - `MessagePinnedMessage`. `true`, if the message media is covered by a spoiler animation
    /// - `MessagePoll`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageProximityAlertTriggered`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageRefundedPayment`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageSticker`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageStory`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageSuccessfulPayment`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageSuggestedPostApprovalFailed`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageSuggestedPostApproved`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageSuggestedPostDeclined`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageSuggestedPostPaid`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageSuggestedPostRefunded`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageSupergroupChatCreated`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageText`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageUniqueGift`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageUsersShared`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageVenue`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageVideo`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageVideoChatEnded`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageVideoChatParticipantsInvited`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageVideoChatScheduled`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageVideoChatStarted`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageVideoNote`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageVoice`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageWebAppData`. `true`, if the message media is covered by a spoiler animation
    /// - `MessageWriteAccessAllowed`. `true`, if the message media is covered by a spoiler animation
    #[must_use]
    pub fn has_media_spoiler(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.has_media_spoiler,
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
            Self::ProximityAlertTriggered(val) => val.has_media_spoiler,
            Self::RefundedPayment(val) => val.has_media_spoiler,
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
            Self::Venue(val) => val.has_media_spoiler,
            Self::Video(val) => val.has_media_spoiler,
            Self::VideoChatEnded(val) => val.has_media_spoiler,
            Self::VideoChatParticipantsInvited(val) => val.has_media_spoiler,
            Self::VideoChatScheduled(val) => val.has_media_spoiler,
            Self::VideoChatStarted(val) => val.has_media_spoiler,
            Self::VideoNote(val) => val.has_media_spoiler,
            Self::Voice(val) => val.has_media_spoiler,
            Self::WebAppData(val) => val.has_media_spoiler,
            Self::WriteAccessAllowed(val) => val.has_media_spoiler,
        }
    }

    /// Helper method for field `has_protected_content`.
    ///
    /// # Variants
    /// - `MessageAnimation`. `true`, if the message can't be forwarded
    /// - `MessageAudio`. `true`, if the message can't be forwarded
    /// - `MessageBoostAdded`. `true`, if the message can't be forwarded
    /// - `MessageChannelChatCreated`. `true`, if the message can't be forwarded
    /// - `MessageChatBackgroundSet`. `true`, if the message can't be forwarded
    /// - `MessageChatOwnerChanged`. `true`, if the message can't be forwarded
    /// - `MessageChatOwnerLeft`. `true`, if the message can't be forwarded
    /// - `MessageChatShared`. `true`, if the message can't be forwarded
    /// - `MessageChecklist`. `true`, if the message can't be forwarded
    /// - `MessageChecklistTasksAdded`. `true`, if the message can't be forwarded
    /// - `MessageChecklistTasksDone`. `true`, if the message can't be forwarded
    /// - `MessageConnectedWebsite`. `true`, if the message can't be forwarded
    /// - `MessageContact`. `true`, if the message can't be forwarded
    /// - `MessageDeleteChatPhoto`. `true`, if the message can't be forwarded
    /// - `MessageDice`. `true`, if the message can't be forwarded
    /// - `MessageDirectMessagePriceChanged`. `true`, if the message can't be forwarded
    /// - `MessageDocument`. `true`, if the message can't be forwarded
    /// - `MessageForumTopicClosed`. `true`, if the message can't be forwarded
    /// - `MessageForumTopicCreated`. `true`, if the message can't be forwarded
    /// - `MessageForumTopicEdited`. `true`, if the message can't be forwarded
    /// - `MessageForumTopicReopened`. `true`, if the message can't be forwarded
    /// - `MessageGame`. `true`, if the message can't be forwarded
    /// - `MessageGeneralForumTopicHidden`. `true`, if the message can't be forwarded
    /// - `MessageGeneralForumTopicUnhidden`. `true`, if the message can't be forwarded
    /// - `MessageGift`. `true`, if the message can't be forwarded
    /// - `MessageGiftUpgradeSent`. `true`, if the message can't be forwarded
    /// - `MessageGiveaway`. `true`, if the message can't be forwarded
    /// - `MessageGiveawayCompleted`. `true`, if the message can't be forwarded
    /// - `MessageGiveawayCreated`. `true`, if the message can't be forwarded
    /// - `MessageGiveawayWinners`. `true`, if the message can't be forwarded
    /// - `MessageGroupChatCreated`. `true`, if the message can't be forwarded
    /// - `MessageInvoice`. `true`, if the message can't be forwarded
    /// - `MessageLeftChatMember`. `true`, if the message can't be forwarded
    /// - `MessageLocation`. `true`, if the message can't be forwarded
    /// - `MessageMessageAutoDeleteTimerChanged`. `true`, if the message can't be forwarded
    /// - `MessageMigrateFromChatId`. `true`, if the message can't be forwarded
    /// - `MessageMigrateToChatId`. `true`, if the message can't be forwarded
    /// - `MessageNewChatMembers`. `true`, if the message can't be forwarded
    /// - `MessageNewChatPhoto`. `true`, if the message can't be forwarded
    /// - `MessageNewChatTitle`. `true`, if the message can't be forwarded
    /// - `MessagePaidMedia`. `true`, if the message can't be forwarded
    /// - `MessagePaidMessagePriceChanged`. `true`, if the message can't be forwarded
    /// - `MessagePassportData`. `true`, if the message can't be forwarded
    /// - `MessagePhoto`. `true`, if the message can't be forwarded
    /// - `MessagePinnedMessage`. `true`, if the message can't be forwarded
    /// - `MessagePoll`. `true`, if the message can't be forwarded
    /// - `MessageProximityAlertTriggered`. `true`, if the message can't be forwarded
    /// - `MessageRefundedPayment`. `true`, if the message can't be forwarded
    /// - `MessageSticker`. `true`, if the message can't be forwarded
    /// - `MessageStory`. `true`, if the message can't be forwarded
    /// - `MessageSuccessfulPayment`. `true`, if the message can't be forwarded
    /// - `MessageSuggestedPostApprovalFailed`. `true`, if the message can't be forwarded
    /// - `MessageSuggestedPostApproved`. `true`, if the message can't be forwarded
    /// - `MessageSuggestedPostDeclined`. `true`, if the message can't be forwarded
    /// - `MessageSuggestedPostPaid`. `true`, if the message can't be forwarded
    /// - `MessageSuggestedPostRefunded`. `true`, if the message can't be forwarded
    /// - `MessageSupergroupChatCreated`. `true`, if the message can't be forwarded
    /// - `MessageText`. `true`, if the message can't be forwarded
    /// - `MessageUniqueGift`. `true`, if the message can't be forwarded
    /// - `MessageUsersShared`. `true`, if the message can't be forwarded
    /// - `MessageVenue`. `true`, if the message can't be forwarded
    /// - `MessageVideo`. `true`, if the message can't be forwarded
    /// - `MessageVideoChatEnded`. `true`, if the message can't be forwarded
    /// - `MessageVideoChatParticipantsInvited`. `true`, if the message can't be forwarded
    /// - `MessageVideoChatScheduled`. `true`, if the message can't be forwarded
    /// - `MessageVideoChatStarted`. `true`, if the message can't be forwarded
    /// - `MessageVideoNote`. `true`, if the message can't be forwarded
    /// - `MessageVoice`. `true`, if the message can't be forwarded
    /// - `MessageWebAppData`. `true`, if the message can't be forwarded
    /// - `MessageWriteAccessAllowed`. `true`, if the message can't be forwarded
    #[must_use]
    pub fn has_protected_content(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.has_protected_content,
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
            Self::ProximityAlertTriggered(val) => val.has_protected_content,
            Self::RefundedPayment(val) => val.has_protected_content,
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
            Self::Venue(val) => val.has_protected_content,
            Self::Video(val) => val.has_protected_content,
            Self::VideoChatEnded(val) => val.has_protected_content,
            Self::VideoChatParticipantsInvited(val) => val.has_protected_content,
            Self::VideoChatScheduled(val) => val.has_protected_content,
            Self::VideoChatStarted(val) => val.has_protected_content,
            Self::VideoNote(val) => val.has_protected_content,
            Self::Voice(val) => val.has_protected_content,
            Self::WebAppData(val) => val.has_protected_content,
            Self::WriteAccessAllowed(val) => val.has_protected_content,
        }
    }

    /// Helper method for field `invoice`.
    ///
    /// # Variants
    /// - `MessageInvoice`. Message is an invoice for a payment, information about the invoice. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn invoice(&self) -> Option<&crate::types::Invoice> {
        match self {
            Self::Invoice(val) => Some(&val.invoice),
            _ => None,
        }
    }

    /// Helper method for field `is_automatic_forward`.
    ///
    /// # Variants
    /// - `MessageAnimation`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageAudio`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageBoostAdded`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageChannelChatCreated`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageChatBackgroundSet`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageChatOwnerChanged`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageChatOwnerLeft`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageChatShared`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageChecklist`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageChecklistTasksAdded`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageChecklistTasksDone`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageConnectedWebsite`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageContact`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageDeleteChatPhoto`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageDice`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageDirectMessagePriceChanged`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageDocument`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageForumTopicClosed`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageForumTopicCreated`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageForumTopicEdited`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageForumTopicReopened`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGame`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGeneralForumTopicHidden`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGeneralForumTopicUnhidden`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGift`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGiftUpgradeSent`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGiveaway`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGiveawayCompleted`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGiveawayCreated`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGiveawayWinners`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageGroupChatCreated`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageInvoice`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageLeftChatMember`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageLocation`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageMessageAutoDeleteTimerChanged`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageMigrateFromChatId`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageMigrateToChatId`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageNewChatMembers`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageNewChatPhoto`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageNewChatTitle`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessagePaidMedia`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessagePaidMessagePriceChanged`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessagePassportData`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessagePhoto`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessagePinnedMessage`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessagePoll`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageProximityAlertTriggered`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageRefundedPayment`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageSticker`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageStory`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageSuccessfulPayment`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageSuggestedPostApprovalFailed`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageSuggestedPostApproved`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageSuggestedPostDeclined`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageSuggestedPostPaid`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageSuggestedPostRefunded`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageSupergroupChatCreated`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageText`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageUniqueGift`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageUsersShared`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageVenue`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageVideo`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageVideoChatEnded`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageVideoChatParticipantsInvited`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageVideoChatScheduled`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageVideoChatStarted`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageVideoNote`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageVoice`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageWebAppData`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    /// - `MessageWriteAccessAllowed`. `true`, if the message is a channel post that was automatically forwarded to the connected discussion group
    #[must_use]
    pub fn is_automatic_forward(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.is_automatic_forward,
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
            Self::ProximityAlertTriggered(val) => val.is_automatic_forward,
            Self::RefundedPayment(val) => val.is_automatic_forward,
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
            Self::Venue(val) => val.is_automatic_forward,
            Self::Video(val) => val.is_automatic_forward,
            Self::VideoChatEnded(val) => val.is_automatic_forward,
            Self::VideoChatParticipantsInvited(val) => val.is_automatic_forward,
            Self::VideoChatScheduled(val) => val.is_automatic_forward,
            Self::VideoChatStarted(val) => val.is_automatic_forward,
            Self::VideoNote(val) => val.is_automatic_forward,
            Self::Voice(val) => val.is_automatic_forward,
            Self::WebAppData(val) => val.is_automatic_forward,
            Self::WriteAccessAllowed(val) => val.is_automatic_forward,
        }
    }

    /// Helper method for field `is_from_offline`.
    ///
    /// # Variants
    /// - `MessageAnimation`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageAudio`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageBoostAdded`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageChannelChatCreated`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageChatBackgroundSet`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageChatOwnerChanged`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageChatOwnerLeft`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageChatShared`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageChecklist`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageChecklistTasksAdded`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageChecklistTasksDone`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageConnectedWebsite`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageContact`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageDeleteChatPhoto`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageDice`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageDirectMessagePriceChanged`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageDocument`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageForumTopicClosed`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageForumTopicCreated`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageForumTopicEdited`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageForumTopicReopened`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGame`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGeneralForumTopicHidden`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGeneralForumTopicUnhidden`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGift`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGiftUpgradeSent`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGiveaway`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGiveawayCompleted`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGiveawayCreated`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGiveawayWinners`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageGroupChatCreated`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageInvoice`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageLeftChatMember`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageLocation`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageMessageAutoDeleteTimerChanged`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageMigrateFromChatId`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageMigrateToChatId`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageNewChatMembers`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageNewChatPhoto`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageNewChatTitle`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessagePaidMedia`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessagePaidMessagePriceChanged`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessagePassportData`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessagePhoto`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessagePinnedMessage`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessagePoll`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageProximityAlertTriggered`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageRefundedPayment`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageSticker`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageStory`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageSuccessfulPayment`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageSuggestedPostApprovalFailed`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageSuggestedPostApproved`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageSuggestedPostDeclined`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageSuggestedPostPaid`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageSuggestedPostRefunded`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageSupergroupChatCreated`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageText`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageUniqueGift`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageUsersShared`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageVenue`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageVideo`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageVideoChatEnded`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageVideoChatParticipantsInvited`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageVideoChatScheduled`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageVideoChatStarted`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageVideoNote`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageVoice`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageWebAppData`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    /// - `MessageWriteAccessAllowed`. `true`, if the message was sent by an implicit action, for example, as an away or a greeting business message, or as a scheduled message
    #[must_use]
    pub fn is_from_offline(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.is_from_offline,
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
            Self::ProximityAlertTriggered(val) => val.is_from_offline,
            Self::RefundedPayment(val) => val.is_from_offline,
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
            Self::Venue(val) => val.is_from_offline,
            Self::Video(val) => val.is_from_offline,
            Self::VideoChatEnded(val) => val.is_from_offline,
            Self::VideoChatParticipantsInvited(val) => val.is_from_offline,
            Self::VideoChatScheduled(val) => val.is_from_offline,
            Self::VideoChatStarted(val) => val.is_from_offline,
            Self::VideoNote(val) => val.is_from_offline,
            Self::Voice(val) => val.is_from_offline,
            Self::WebAppData(val) => val.is_from_offline,
            Self::WriteAccessAllowed(val) => val.is_from_offline,
        }
    }

    /// Helper method for field `is_paid_post`.
    ///
    /// # Variants
    /// - `MessageAnimation`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageAudio`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageBoostAdded`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageChannelChatCreated`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageChatBackgroundSet`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageChatOwnerChanged`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageChatOwnerLeft`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageChatShared`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageChecklist`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageChecklistTasksAdded`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageChecklistTasksDone`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageConnectedWebsite`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageContact`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageDeleteChatPhoto`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageDice`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageDirectMessagePriceChanged`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageDocument`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageForumTopicClosed`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageForumTopicCreated`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageForumTopicEdited`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageForumTopicReopened`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGame`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGeneralForumTopicHidden`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGeneralForumTopicUnhidden`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGift`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGiftUpgradeSent`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGiveaway`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGiveawayCompleted`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGiveawayCreated`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGiveawayWinners`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageGroupChatCreated`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageInvoice`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageLeftChatMember`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageLocation`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageMessageAutoDeleteTimerChanged`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageMigrateFromChatId`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageMigrateToChatId`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageNewChatMembers`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageNewChatPhoto`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageNewChatTitle`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessagePaidMedia`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessagePaidMessagePriceChanged`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessagePassportData`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessagePhoto`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessagePinnedMessage`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessagePoll`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageProximityAlertTriggered`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageRefundedPayment`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageSticker`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageStory`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageSuccessfulPayment`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageSuggestedPostApprovalFailed`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageSuggestedPostApproved`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageSuggestedPostDeclined`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageSuggestedPostPaid`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageSuggestedPostRefunded`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageSupergroupChatCreated`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageText`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageUniqueGift`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageUsersShared`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageVenue`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageVideo`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageVideoChatEnded`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageVideoChatParticipantsInvited`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageVideoChatScheduled`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageVideoChatStarted`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageVideoNote`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageVoice`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageWebAppData`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    /// - `MessageWriteAccessAllowed`. `true`, if the message is a paid post. Note that such posts must not be deleted for 24 hours to receive the payment and can't be edited.
    #[must_use]
    pub fn is_paid_post(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.is_paid_post,
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
            Self::ProximityAlertTriggered(val) => val.is_paid_post,
            Self::RefundedPayment(val) => val.is_paid_post,
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
            Self::Venue(val) => val.is_paid_post,
            Self::Video(val) => val.is_paid_post,
            Self::VideoChatEnded(val) => val.is_paid_post,
            Self::VideoChatParticipantsInvited(val) => val.is_paid_post,
            Self::VideoChatScheduled(val) => val.is_paid_post,
            Self::VideoChatStarted(val) => val.is_paid_post,
            Self::VideoNote(val) => val.is_paid_post,
            Self::Voice(val) => val.is_paid_post,
            Self::WebAppData(val) => val.is_paid_post,
            Self::WriteAccessAllowed(val) => val.is_paid_post,
        }
    }

    /// Helper method for field `is_topic_message`.
    ///
    /// # Variants
    /// - `MessageAnimation`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageAudio`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageBoostAdded`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageChannelChatCreated`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageChatBackgroundSet`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageChatOwnerChanged`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageChatOwnerLeft`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageChatShared`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageChecklist`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageChecklistTasksAdded`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageChecklistTasksDone`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageConnectedWebsite`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageContact`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageDeleteChatPhoto`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageDice`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageDirectMessagePriceChanged`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageDocument`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageForumTopicClosed`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageForumTopicCreated`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageForumTopicEdited`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageForumTopicReopened`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGame`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGeneralForumTopicHidden`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGeneralForumTopicUnhidden`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGift`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGiftUpgradeSent`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGiveaway`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGiveawayCompleted`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGiveawayCreated`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGiveawayWinners`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageGroupChatCreated`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageInvoice`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageLeftChatMember`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageLocation`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageMessageAutoDeleteTimerChanged`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageMigrateFromChatId`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageMigrateToChatId`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageNewChatMembers`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageNewChatPhoto`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageNewChatTitle`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessagePaidMedia`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessagePaidMessagePriceChanged`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessagePassportData`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessagePhoto`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessagePinnedMessage`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessagePoll`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageProximityAlertTriggered`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageRefundedPayment`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageSticker`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageStory`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageSuccessfulPayment`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageSuggestedPostApprovalFailed`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageSuggestedPostApproved`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageSuggestedPostDeclined`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageSuggestedPostPaid`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageSuggestedPostRefunded`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageSupergroupChatCreated`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageText`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageUniqueGift`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageUsersShared`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageVenue`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageVideo`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageVideoChatEnded`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageVideoChatParticipantsInvited`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageVideoChatScheduled`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageVideoChatStarted`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageVideoNote`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageVoice`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageWebAppData`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    /// - `MessageWriteAccessAllowed`. `true`, if the message is sent to a topic in a forum supergroup or a private chat with the bot
    #[must_use]
    pub fn is_topic_message(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.is_topic_message,
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
            Self::ProximityAlertTriggered(val) => val.is_topic_message,
            Self::RefundedPayment(val) => val.is_topic_message,
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
            Self::Venue(val) => val.is_topic_message,
            Self::Video(val) => val.is_topic_message,
            Self::VideoChatEnded(val) => val.is_topic_message,
            Self::VideoChatParticipantsInvited(val) => val.is_topic_message,
            Self::VideoChatScheduled(val) => val.is_topic_message,
            Self::VideoChatStarted(val) => val.is_topic_message,
            Self::VideoNote(val) => val.is_topic_message,
            Self::Voice(val) => val.is_topic_message,
            Self::WebAppData(val) => val.is_topic_message,
            Self::WriteAccessAllowed(val) => val.is_topic_message,
        }
    }

    /// Helper method for field `left_chat_member`.
    ///
    /// # Variants
    /// - `MessageLeftChatMember`. A member was removed from the group, information about them (this member may be the bot itself)
    #[must_use]
    pub fn left_chat_member(&self) -> Option<&crate::types::User> {
        match self {
            Self::LeftChatMember(val) => Some(val.left_chat_member.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `link_preview_options`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageAudio`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageBoostAdded`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageChannelChatCreated`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageChatBackgroundSet`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageChatOwnerChanged`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageChatOwnerLeft`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageChatShared`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageChecklist`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageChecklistTasksAdded`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageChecklistTasksDone`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageConnectedWebsite`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageContact`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageDeleteChatPhoto`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageDice`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageDirectMessagePriceChanged`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageDocument`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageForumTopicClosed`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageForumTopicCreated`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageForumTopicEdited`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageForumTopicReopened`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGame`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGeneralForumTopicHidden`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGeneralForumTopicUnhidden`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGift`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGiftUpgradeSent`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGiveaway`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGiveawayCompleted`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGiveawayCreated`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGiveawayWinners`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageGroupChatCreated`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageInvoice`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageLeftChatMember`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageLocation`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageMessageAutoDeleteTimerChanged`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageMigrateFromChatId`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageMigrateToChatId`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageNewChatMembers`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageNewChatPhoto`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageNewChatTitle`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessagePaidMedia`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessagePaidMessagePriceChanged`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessagePassportData`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessagePhoto`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessagePinnedMessage`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessagePoll`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageProximityAlertTriggered`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageRefundedPayment`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageSticker`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageStory`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageSuccessfulPayment`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageSuggestedPostApprovalFailed`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageSuggestedPostApproved`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageSuggestedPostDeclined`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageSuggestedPostPaid`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageSuggestedPostRefunded`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageSupergroupChatCreated`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageText`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageUniqueGift`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageUsersShared`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageVenue`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageVideo`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageVideoChatEnded`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageVideoChatParticipantsInvited`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageVideoChatScheduled`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageVideoChatStarted`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageVideoNote`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageVoice`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageWebAppData`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    /// - `MessageWriteAccessAllowed`. Options used for link preview generation for the message, if it is a text message and link preview options were changed
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        match self {
            Self::Animation(val) => val.link_preview_options.as_ref(),
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
            Self::ProximityAlertTriggered(val) => val.link_preview_options.as_ref(),
            Self::RefundedPayment(val) => val.link_preview_options.as_ref(),
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
            Self::Venue(val) => val.link_preview_options.as_ref(),
            Self::Video(val) => val.link_preview_options.as_ref(),
            Self::VideoChatEnded(val) => val.link_preview_options.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.link_preview_options.as_ref(),
            Self::VideoChatScheduled(val) => val.link_preview_options.as_ref(),
            Self::VideoChatStarted(val) => val.link_preview_options.as_ref(),
            Self::VideoNote(val) => val.link_preview_options.as_ref(),
            Self::Voice(val) => val.link_preview_options.as_ref(),
            Self::WebAppData(val) => val.link_preview_options.as_ref(),
            Self::WriteAccessAllowed(val) => val.link_preview_options.as_ref(),
        }
    }

    /// Helper method for field `location`.
    ///
    /// # Variants
    /// - `MessageLocation`. Message is a shared location, information about the location
    #[must_use]
    pub fn location(&self) -> Option<&crate::types::Location> {
        match self {
            Self::Location(val) => Some(&val.location),
            _ => None,
        }
    }

    /// Helper method for field `media_group_id`.
    ///
    /// # Variants
    /// - `MessageAnimation`. The unique identifier of a media message group this message belongs to
    /// - `MessageAudio`. The unique identifier of a media message group this message belongs to
    /// - `MessageBoostAdded`. The unique identifier of a media message group this message belongs to
    /// - `MessageChannelChatCreated`. The unique identifier of a media message group this message belongs to
    /// - `MessageChatBackgroundSet`. The unique identifier of a media message group this message belongs to
    /// - `MessageChatOwnerChanged`. The unique identifier of a media message group this message belongs to
    /// - `MessageChatOwnerLeft`. The unique identifier of a media message group this message belongs to
    /// - `MessageChatShared`. The unique identifier of a media message group this message belongs to
    /// - `MessageChecklist`. The unique identifier of a media message group this message belongs to
    /// - `MessageChecklistTasksAdded`. The unique identifier of a media message group this message belongs to
    /// - `MessageChecklistTasksDone`. The unique identifier of a media message group this message belongs to
    /// - `MessageConnectedWebsite`. The unique identifier of a media message group this message belongs to
    /// - `MessageContact`. The unique identifier of a media message group this message belongs to
    /// - `MessageDeleteChatPhoto`. The unique identifier of a media message group this message belongs to
    /// - `MessageDice`. The unique identifier of a media message group this message belongs to
    /// - `MessageDirectMessagePriceChanged`. The unique identifier of a media message group this message belongs to
    /// - `MessageDocument`. The unique identifier of a media message group this message belongs to
    /// - `MessageForumTopicClosed`. The unique identifier of a media message group this message belongs to
    /// - `MessageForumTopicCreated`. The unique identifier of a media message group this message belongs to
    /// - `MessageForumTopicEdited`. The unique identifier of a media message group this message belongs to
    /// - `MessageForumTopicReopened`. The unique identifier of a media message group this message belongs to
    /// - `MessageGame`. The unique identifier of a media message group this message belongs to
    /// - `MessageGeneralForumTopicHidden`. The unique identifier of a media message group this message belongs to
    /// - `MessageGeneralForumTopicUnhidden`. The unique identifier of a media message group this message belongs to
    /// - `MessageGift`. The unique identifier of a media message group this message belongs to
    /// - `MessageGiftUpgradeSent`. The unique identifier of a media message group this message belongs to
    /// - `MessageGiveaway`. The unique identifier of a media message group this message belongs to
    /// - `MessageGiveawayCompleted`. The unique identifier of a media message group this message belongs to
    /// - `MessageGiveawayCreated`. The unique identifier of a media message group this message belongs to
    /// - `MessageGiveawayWinners`. The unique identifier of a media message group this message belongs to
    /// - `MessageGroupChatCreated`. The unique identifier of a media message group this message belongs to
    /// - `MessageInvoice`. The unique identifier of a media message group this message belongs to
    /// - `MessageLeftChatMember`. The unique identifier of a media message group this message belongs to
    /// - `MessageLocation`. The unique identifier of a media message group this message belongs to
    /// - `MessageMessageAutoDeleteTimerChanged`. The unique identifier of a media message group this message belongs to
    /// - `MessageMigrateFromChatId`. The unique identifier of a media message group this message belongs to
    /// - `MessageMigrateToChatId`. The unique identifier of a media message group this message belongs to
    /// - `MessageNewChatMembers`. The unique identifier of a media message group this message belongs to
    /// - `MessageNewChatPhoto`. The unique identifier of a media message group this message belongs to
    /// - `MessageNewChatTitle`. The unique identifier of a media message group this message belongs to
    /// - `MessagePaidMedia`. The unique identifier of a media message group this message belongs to
    /// - `MessagePaidMessagePriceChanged`. The unique identifier of a media message group this message belongs to
    /// - `MessagePassportData`. The unique identifier of a media message group this message belongs to
    /// - `MessagePhoto`. The unique identifier of a media message group this message belongs to
    /// - `MessagePinnedMessage`. The unique identifier of a media message group this message belongs to
    /// - `MessagePoll`. The unique identifier of a media message group this message belongs to
    /// - `MessageProximityAlertTriggered`. The unique identifier of a media message group this message belongs to
    /// - `MessageRefundedPayment`. The unique identifier of a media message group this message belongs to
    /// - `MessageSticker`. The unique identifier of a media message group this message belongs to
    /// - `MessageStory`. The unique identifier of a media message group this message belongs to
    /// - `MessageSuccessfulPayment`. The unique identifier of a media message group this message belongs to
    /// - `MessageSuggestedPostApprovalFailed`. The unique identifier of a media message group this message belongs to
    /// - `MessageSuggestedPostApproved`. The unique identifier of a media message group this message belongs to
    /// - `MessageSuggestedPostDeclined`. The unique identifier of a media message group this message belongs to
    /// - `MessageSuggestedPostPaid`. The unique identifier of a media message group this message belongs to
    /// - `MessageSuggestedPostRefunded`. The unique identifier of a media message group this message belongs to
    /// - `MessageSupergroupChatCreated`. The unique identifier of a media message group this message belongs to
    /// - `MessageText`. The unique identifier of a media message group this message belongs to
    /// - `MessageUniqueGift`. The unique identifier of a media message group this message belongs to
    /// - `MessageUsersShared`. The unique identifier of a media message group this message belongs to
    /// - `MessageVenue`. The unique identifier of a media message group this message belongs to
    /// - `MessageVideo`. The unique identifier of a media message group this message belongs to
    /// - `MessageVideoChatEnded`. The unique identifier of a media message group this message belongs to
    /// - `MessageVideoChatParticipantsInvited`. The unique identifier of a media message group this message belongs to
    /// - `MessageVideoChatScheduled`. The unique identifier of a media message group this message belongs to
    /// - `MessageVideoChatStarted`. The unique identifier of a media message group this message belongs to
    /// - `MessageVideoNote`. The unique identifier of a media message group this message belongs to
    /// - `MessageVoice`. The unique identifier of a media message group this message belongs to
    /// - `MessageWebAppData`. The unique identifier of a media message group this message belongs to
    /// - `MessageWriteAccessAllowed`. The unique identifier of a media message group this message belongs to
    #[must_use]
    pub fn media_group_id(&self) -> Option<&str> {
        match self {
            Self::Animation(val) => val.media_group_id.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.media_group_id.as_deref(),
            Self::RefundedPayment(val) => val.media_group_id.as_deref(),
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
            Self::Venue(val) => val.media_group_id.as_deref(),
            Self::Video(val) => val.media_group_id.as_deref(),
            Self::VideoChatEnded(val) => val.media_group_id.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.media_group_id.as_deref(),
            Self::VideoChatScheduled(val) => val.media_group_id.as_deref(),
            Self::VideoChatStarted(val) => val.media_group_id.as_deref(),
            Self::VideoNote(val) => val.media_group_id.as_deref(),
            Self::Voice(val) => val.media_group_id.as_deref(),
            Self::WebAppData(val) => val.media_group_id.as_deref(),
            Self::WriteAccessAllowed(val) => val.media_group_id.as_deref(),
        }
    }

    /// Helper method for field `message_auto_delete_timer_changed`.
    ///
    /// # Variants
    /// - `MessageMessageAutoDeleteTimerChanged`. Service message: auto-delete timer settings changed in the chat
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
    /// # Variants
    /// - `MessageAnimation`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageAudio`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageBoostAdded`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageChannelChatCreated`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageChatBackgroundSet`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageChatOwnerChanged`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageChatOwnerLeft`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageChatShared`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageChecklist`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageChecklistTasksAdded`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageChecklistTasksDone`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageConnectedWebsite`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageContact`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageDeleteChatPhoto`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageDice`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageDirectMessagePriceChanged`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageDocument`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageForumTopicClosed`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageForumTopicCreated`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageForumTopicEdited`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageForumTopicReopened`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGame`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGeneralForumTopicHidden`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGeneralForumTopicUnhidden`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGift`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGiftUpgradeSent`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGiveaway`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGiveawayCompleted`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGiveawayCreated`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGiveawayWinners`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageGroupChatCreated`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageInvoice`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageLeftChatMember`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageLocation`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageMessageAutoDeleteTimerChanged`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageMigrateFromChatId`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageMigrateToChatId`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageNewChatMembers`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageNewChatPhoto`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageNewChatTitle`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessagePaidMedia`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessagePaidMessagePriceChanged`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessagePassportData`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessagePhoto`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessagePinnedMessage`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessagePoll`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageProximityAlertTriggered`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageRefundedPayment`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageSticker`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageStory`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageSuccessfulPayment`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageSuggestedPostApprovalFailed`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageSuggestedPostApproved`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageSuggestedPostDeclined`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageSuggestedPostPaid`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageSuggestedPostRefunded`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageSupergroupChatCreated`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageText`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageUniqueGift`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageUsersShared`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageVenue`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageVideo`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageVideoChatEnded`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageVideoChatParticipantsInvited`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageVideoChatScheduled`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageVideoChatStarted`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageVideoNote`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageVoice`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageWebAppData`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    /// - `MessageWriteAccessAllowed`. Unique message identifier inside this chat. In specific instances (e.g., message containing a video sent to a big chat), the server might automatically schedule a message instead of sending it immediately. In such cases, this field will be 0 and the relevant message will be unusable until it is actually sent
    #[must_use]
    pub fn message_id(&self) -> i64 {
        match self {
            Self::Animation(val) => val.message_id,
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
            Self::ProximityAlertTriggered(val) => val.message_id,
            Self::RefundedPayment(val) => val.message_id,
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
            Self::Venue(val) => val.message_id,
            Self::Video(val) => val.message_id,
            Self::VideoChatEnded(val) => val.message_id,
            Self::VideoChatParticipantsInvited(val) => val.message_id,
            Self::VideoChatScheduled(val) => val.message_id,
            Self::VideoChatStarted(val) => val.message_id,
            Self::VideoNote(val) => val.message_id,
            Self::Voice(val) => val.message_id,
            Self::WebAppData(val) => val.message_id,
            Self::WriteAccessAllowed(val) => val.message_id,
        }
    }

    /// Helper method for field `message_thread_id`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageAudio`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageBoostAdded`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageChannelChatCreated`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageChatBackgroundSet`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageChatOwnerChanged`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageChatOwnerLeft`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageChatShared`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageChecklist`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageChecklistTasksAdded`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageChecklistTasksDone`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageConnectedWebsite`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageContact`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageDeleteChatPhoto`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageDice`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageDirectMessagePriceChanged`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageDocument`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageForumTopicClosed`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageForumTopicCreated`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageForumTopicEdited`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageForumTopicReopened`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGame`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGeneralForumTopicHidden`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGeneralForumTopicUnhidden`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGift`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGiftUpgradeSent`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGiveaway`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGiveawayCompleted`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGiveawayCreated`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGiveawayWinners`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageGroupChatCreated`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageInvoice`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageLeftChatMember`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageLocation`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageMessageAutoDeleteTimerChanged`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageMigrateFromChatId`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageMigrateToChatId`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageNewChatMembers`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageNewChatPhoto`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageNewChatTitle`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessagePaidMedia`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessagePaidMessagePriceChanged`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessagePassportData`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessagePhoto`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessagePinnedMessage`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessagePoll`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageProximityAlertTriggered`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageRefundedPayment`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageSticker`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageStory`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageSuccessfulPayment`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageSuggestedPostApprovalFailed`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageSuggestedPostApproved`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageSuggestedPostDeclined`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageSuggestedPostPaid`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageSuggestedPostRefunded`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageSupergroupChatCreated`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageText`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageUniqueGift`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageUsersShared`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageVenue`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageVideo`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageVideoChatEnded`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageVideoChatParticipantsInvited`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageVideoChatScheduled`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageVideoChatStarted`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageVideoNote`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageVoice`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageWebAppData`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    /// - `MessageWriteAccessAllowed`. Unique identifier of a message thread or forum topic to which the message belongs; for supergroups and private chats only
    #[must_use]
    pub fn message_thread_id(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.message_thread_id,
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
            Self::ProximityAlertTriggered(val) => val.message_thread_id,
            Self::RefundedPayment(val) => val.message_thread_id,
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
            Self::Venue(val) => val.message_thread_id,
            Self::Video(val) => val.message_thread_id,
            Self::VideoChatEnded(val) => val.message_thread_id,
            Self::VideoChatParticipantsInvited(val) => val.message_thread_id,
            Self::VideoChatScheduled(val) => val.message_thread_id,
            Self::VideoChatStarted(val) => val.message_thread_id,
            Self::VideoNote(val) => val.message_thread_id,
            Self::Voice(val) => val.message_thread_id,
            Self::WebAppData(val) => val.message_thread_id,
            Self::WriteAccessAllowed(val) => val.message_thread_id,
        }
    }

    /// Helper method for field `migrate_from_chat_id`.
    ///
    /// # Variants
    /// - `MessageMigrateFromChatId`. The supergroup has been migrated from a group with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn migrate_from_chat_id(&self) -> Option<i64> {
        match self {
            Self::MigrateFromChatId(val) => Some(val.migrate_from_chat_id),
            _ => None,
        }
    }

    /// Helper method for field `migrate_to_chat_id`.
    ///
    /// # Variants
    /// - `MessageMigrateToChatId`. The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        match self {
            Self::MigrateToChatId(val) => Some(val.migrate_to_chat_id),
            _ => None,
        }
    }

    /// Helper method for field `new_chat_members`.
    ///
    /// # Variants
    /// - `MessageNewChatMembers`. New members that were added to the group or supergroup and information about them (the bot itself may be one of these members)
    #[must_use]
    pub fn new_chat_members(&self) -> Option<&[crate::types::User]> {
        match self {
            Self::NewChatMembers(val) => Some(val.new_chat_members.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `new_chat_photo`.
    ///
    /// # Variants
    /// - `MessageNewChatPhoto`. A chat photo was change to this value
    #[must_use]
    pub fn new_chat_photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::NewChatPhoto(val) => Some(val.new_chat_photo.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `new_chat_title`.
    ///
    /// # Variants
    /// - `MessageNewChatTitle`. A chat title was changed to this value
    #[must_use]
    pub fn new_chat_title(&self) -> Option<&str> {
        match self {
            Self::NewChatTitle(val) => Some(val.new_chat_title.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `paid_media`.
    ///
    /// # Variants
    /// - `MessagePaidMedia`. Message contains paid media; information about the paid media
    #[must_use]
    pub fn paid_media(&self) -> Option<&crate::types::PaidMediaInfo> {
        match self {
            Self::PaidMedia(val) => Some(&val.paid_media),
            _ => None,
        }
    }

    /// Helper method for field `paid_message_price_changed`.
    ///
    /// # Variants
    /// - `MessagePaidMessagePriceChanged`. Service message: the price for paid messages has changed in the chat
    #[must_use]
    pub fn paid_message_price_changed(&self) -> Option<&crate::types::PaidMessagePriceChanged> {
        match self {
            Self::PaidMessagePriceChanged(val) => Some(&val.paid_message_price_changed),
            _ => None,
        }
    }

    /// Helper method for field `paid_star_count`.
    ///
    /// # Variants
    /// - `MessageAnimation`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageAudio`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageBoostAdded`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageChannelChatCreated`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageChatBackgroundSet`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageChatOwnerChanged`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageChatOwnerLeft`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageChatShared`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageChecklist`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageChecklistTasksAdded`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageChecklistTasksDone`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageConnectedWebsite`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageContact`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageDeleteChatPhoto`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageDice`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageDirectMessagePriceChanged`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageDocument`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageForumTopicClosed`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageForumTopicCreated`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageForumTopicEdited`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageForumTopicReopened`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGame`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGeneralForumTopicHidden`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGeneralForumTopicUnhidden`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGift`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGiftUpgradeSent`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGiveaway`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGiveawayCompleted`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGiveawayCreated`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGiveawayWinners`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageGroupChatCreated`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageInvoice`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageLeftChatMember`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageLocation`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageMessageAutoDeleteTimerChanged`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageMigrateFromChatId`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageMigrateToChatId`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageNewChatMembers`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageNewChatPhoto`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageNewChatTitle`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessagePaidMedia`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessagePaidMessagePriceChanged`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessagePassportData`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessagePhoto`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessagePinnedMessage`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessagePoll`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageProximityAlertTriggered`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageRefundedPayment`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageSticker`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageStory`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageSuccessfulPayment`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageSuggestedPostApprovalFailed`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageSuggestedPostApproved`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageSuggestedPostDeclined`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageSuggestedPostPaid`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageSuggestedPostRefunded`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageSupergroupChatCreated`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageText`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageUniqueGift`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageUsersShared`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageVenue`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageVideo`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageVideoChatEnded`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageVideoChatParticipantsInvited`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageVideoChatScheduled`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageVideoChatStarted`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageVideoNote`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageVoice`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageWebAppData`. The number of Telegram Stars that were paid by the sender of the message to send it
    /// - `MessageWriteAccessAllowed`. The number of Telegram Stars that were paid by the sender of the message to send it
    #[must_use]
    pub fn paid_star_count(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.paid_star_count,
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
            Self::ProximityAlertTriggered(val) => val.paid_star_count,
            Self::RefundedPayment(val) => val.paid_star_count,
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
            Self::Venue(val) => val.paid_star_count,
            Self::Video(val) => val.paid_star_count,
            Self::VideoChatEnded(val) => val.paid_star_count,
            Self::VideoChatParticipantsInvited(val) => val.paid_star_count,
            Self::VideoChatScheduled(val) => val.paid_star_count,
            Self::VideoChatStarted(val) => val.paid_star_count,
            Self::VideoNote(val) => val.paid_star_count,
            Self::Voice(val) => val.paid_star_count,
            Self::WebAppData(val) => val.paid_star_count,
            Self::WriteAccessAllowed(val) => val.paid_star_count,
        }
    }

    /// Helper method for field `passport_data`.
    ///
    /// # Variants
    /// - `MessagePassportData`. Telegram Passport data
    #[must_use]
    pub fn passport_data(&self) -> Option<&crate::types::PassportData> {
        match self {
            Self::PassportData(val) => Some(&val.passport_data),
            _ => None,
        }
    }

    /// Helper method for field `photo`.
    ///
    /// # Variants
    /// - `MessagePhoto`. Message is a photo, available sizes of the photo
    #[must_use]
    pub fn photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::Photo(val) => Some(val.photo.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `pinned_message`.
    ///
    /// # Variants
    /// - `MessagePinnedMessage`. Specified message was pinned. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[must_use]
    pub fn pinned_message(&self) -> Option<&crate::types::MaybeInaccessibleMessage> {
        match self {
            Self::PinnedMessage(val) => Some(val.pinned_message.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `poll`.
    ///
    /// # Variants
    /// - `MessagePoll`. Message is a native poll, information about the poll
    #[must_use]
    pub fn poll(&self) -> Option<&crate::types::Poll> {
        match self {
            Self::Poll(val) => Some(val.poll.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `proximity_alert_triggered`.
    ///
    /// # Variants
    /// - `MessageProximityAlertTriggered`. Service message. A user in the chat triggered another user's proximity alert while sharing Live Location.
    #[must_use]
    pub fn proximity_alert_triggered(&self) -> Option<&crate::types::ProximityAlertTriggered> {
        match self {
            Self::ProximityAlertTriggered(val) => Some(&val.proximity_alert_triggered),
            _ => None,
        }
    }

    /// Helper method for field `quote`.
    ///
    /// # Variants
    /// - `MessageAnimation`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageAudio`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageBoostAdded`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageChannelChatCreated`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageChatBackgroundSet`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageChatOwnerChanged`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageChatOwnerLeft`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageChatShared`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageChecklist`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageChecklistTasksAdded`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageChecklistTasksDone`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageConnectedWebsite`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageContact`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageDeleteChatPhoto`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageDice`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageDirectMessagePriceChanged`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageDocument`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageForumTopicClosed`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageForumTopicCreated`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageForumTopicEdited`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageForumTopicReopened`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGame`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGeneralForumTopicHidden`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGeneralForumTopicUnhidden`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGift`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGiftUpgradeSent`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGiveaway`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGiveawayCompleted`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGiveawayCreated`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGiveawayWinners`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageGroupChatCreated`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageInvoice`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageLeftChatMember`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageLocation`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageMessageAutoDeleteTimerChanged`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageMigrateFromChatId`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageMigrateToChatId`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageNewChatMembers`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageNewChatPhoto`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageNewChatTitle`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessagePaidMedia`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessagePaidMessagePriceChanged`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessagePassportData`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessagePhoto`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessagePinnedMessage`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessagePoll`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageProximityAlertTriggered`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageRefundedPayment`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageSticker`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageStory`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageSuccessfulPayment`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageSuggestedPostApprovalFailed`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageSuggestedPostApproved`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageSuggestedPostDeclined`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageSuggestedPostPaid`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageSuggestedPostRefunded`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageSupergroupChatCreated`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageText`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageUniqueGift`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageUsersShared`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageVenue`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageVideo`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageVideoChatEnded`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageVideoChatParticipantsInvited`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageVideoChatScheduled`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageVideoChatStarted`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageVideoNote`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageVoice`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageWebAppData`. For replies that quote part of the original message, the quoted part of the message
    /// - `MessageWriteAccessAllowed`. For replies that quote part of the original message, the quoted part of the message
    #[must_use]
    pub fn quote(&self) -> Option<&crate::types::TextQuote> {
        match self {
            Self::Animation(val) => val.quote.as_ref(),
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
            Self::ProximityAlertTriggered(val) => val.quote.as_ref(),
            Self::RefundedPayment(val) => val.quote.as_ref(),
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
            Self::Venue(val) => val.quote.as_ref(),
            Self::Video(val) => val.quote.as_ref(),
            Self::VideoChatEnded(val) => val.quote.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.quote.as_ref(),
            Self::VideoChatScheduled(val) => val.quote.as_ref(),
            Self::VideoChatStarted(val) => val.quote.as_ref(),
            Self::VideoNote(val) => val.quote.as_ref(),
            Self::Voice(val) => val.quote.as_ref(),
            Self::WebAppData(val) => val.quote.as_ref(),
            Self::WriteAccessAllowed(val) => val.quote.as_ref(),
        }
    }

    /// Helper method for field `refunded_payment`.
    ///
    /// # Variants
    /// - `MessageRefundedPayment`. Message is a service message about a refunded payment, information about the payment. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn refunded_payment(&self) -> Option<&crate::types::RefundedPayment> {
        match self {
            Self::RefundedPayment(val) => Some(&val.refunded_payment),
            _ => None,
        }
    }

    /// Helper method for field `reply_markup`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageAudio`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageBoostAdded`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageChannelChatCreated`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageChatBackgroundSet`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageChatOwnerChanged`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageChatOwnerLeft`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageChatShared`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageChecklist`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageChecklistTasksAdded`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageChecklistTasksDone`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageConnectedWebsite`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageContact`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageDeleteChatPhoto`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageDice`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageDirectMessagePriceChanged`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageDocument`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageForumTopicClosed`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageForumTopicCreated`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageForumTopicEdited`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageForumTopicReopened`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGame`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGeneralForumTopicHidden`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGeneralForumTopicUnhidden`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGift`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGiftUpgradeSent`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGiveaway`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGiveawayCompleted`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGiveawayCreated`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGiveawayWinners`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageGroupChatCreated`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageInvoice`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageLeftChatMember`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageLocation`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageMessageAutoDeleteTimerChanged`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageMigrateFromChatId`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageMigrateToChatId`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageNewChatMembers`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageNewChatPhoto`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageNewChatTitle`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessagePaidMedia`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessagePaidMessagePriceChanged`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessagePassportData`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessagePhoto`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessagePinnedMessage`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessagePoll`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageProximityAlertTriggered`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageRefundedPayment`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageSticker`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageStory`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageSuccessfulPayment`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageSuggestedPostApprovalFailed`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageSuggestedPostApproved`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageSuggestedPostDeclined`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageSuggestedPostPaid`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageSuggestedPostRefunded`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageSupergroupChatCreated`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageText`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageUniqueGift`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageUsersShared`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageVenue`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageVideo`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageVideoChatEnded`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageVideoChatParticipantsInvited`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageVideoChatScheduled`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageVideoChatStarted`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageVideoNote`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageVoice`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageWebAppData`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    /// - `MessageWriteAccessAllowed`. Inline keyboard attached to the message. `login_url` buttons are represented as ordinary url buttons.
    #[must_use]
    pub fn reply_markup(&self) -> Option<&crate::types::InlineKeyboardMarkup> {
        match self {
            Self::Animation(val) => val.reply_markup.as_ref(),
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
            Self::ProximityAlertTriggered(val) => val.reply_markup.as_ref(),
            Self::RefundedPayment(val) => val.reply_markup.as_ref(),
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
            Self::Venue(val) => val.reply_markup.as_ref(),
            Self::Video(val) => val.reply_markup.as_ref(),
            Self::VideoChatEnded(val) => val.reply_markup.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.reply_markup.as_ref(),
            Self::VideoChatScheduled(val) => val.reply_markup.as_ref(),
            Self::VideoChatStarted(val) => val.reply_markup.as_ref(),
            Self::VideoNote(val) => val.reply_markup.as_ref(),
            Self::Voice(val) => val.reply_markup.as_ref(),
            Self::WebAppData(val) => val.reply_markup.as_ref(),
            Self::WriteAccessAllowed(val) => val.reply_markup.as_ref(),
        }
    }

    /// Helper method for field `reply_to_checklist_task_id`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Identifier of the specific checklist task that is being replied to
    /// - `MessageAudio`. Identifier of the specific checklist task that is being replied to
    /// - `MessageBoostAdded`. Identifier of the specific checklist task that is being replied to
    /// - `MessageChannelChatCreated`. Identifier of the specific checklist task that is being replied to
    /// - `MessageChatBackgroundSet`. Identifier of the specific checklist task that is being replied to
    /// - `MessageChatOwnerChanged`. Identifier of the specific checklist task that is being replied to
    /// - `MessageChatOwnerLeft`. Identifier of the specific checklist task that is being replied to
    /// - `MessageChatShared`. Identifier of the specific checklist task that is being replied to
    /// - `MessageChecklist`. Identifier of the specific checklist task that is being replied to
    /// - `MessageChecklistTasksAdded`. Identifier of the specific checklist task that is being replied to
    /// - `MessageChecklistTasksDone`. Identifier of the specific checklist task that is being replied to
    /// - `MessageConnectedWebsite`. Identifier of the specific checklist task that is being replied to
    /// - `MessageContact`. Identifier of the specific checklist task that is being replied to
    /// - `MessageDeleteChatPhoto`. Identifier of the specific checklist task that is being replied to
    /// - `MessageDice`. Identifier of the specific checklist task that is being replied to
    /// - `MessageDirectMessagePriceChanged`. Identifier of the specific checklist task that is being replied to
    /// - `MessageDocument`. Identifier of the specific checklist task that is being replied to
    /// - `MessageForumTopicClosed`. Identifier of the specific checklist task that is being replied to
    /// - `MessageForumTopicCreated`. Identifier of the specific checklist task that is being replied to
    /// - `MessageForumTopicEdited`. Identifier of the specific checklist task that is being replied to
    /// - `MessageForumTopicReopened`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGame`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGeneralForumTopicHidden`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGeneralForumTopicUnhidden`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGift`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGiftUpgradeSent`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGiveaway`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGiveawayCompleted`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGiveawayCreated`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGiveawayWinners`. Identifier of the specific checklist task that is being replied to
    /// - `MessageGroupChatCreated`. Identifier of the specific checklist task that is being replied to
    /// - `MessageInvoice`. Identifier of the specific checklist task that is being replied to
    /// - `MessageLeftChatMember`. Identifier of the specific checklist task that is being replied to
    /// - `MessageLocation`. Identifier of the specific checklist task that is being replied to
    /// - `MessageMessageAutoDeleteTimerChanged`. Identifier of the specific checklist task that is being replied to
    /// - `MessageMigrateFromChatId`. Identifier of the specific checklist task that is being replied to
    /// - `MessageMigrateToChatId`. Identifier of the specific checklist task that is being replied to
    /// - `MessageNewChatMembers`. Identifier of the specific checklist task that is being replied to
    /// - `MessageNewChatPhoto`. Identifier of the specific checklist task that is being replied to
    /// - `MessageNewChatTitle`. Identifier of the specific checklist task that is being replied to
    /// - `MessagePaidMedia`. Identifier of the specific checklist task that is being replied to
    /// - `MessagePaidMessagePriceChanged`. Identifier of the specific checklist task that is being replied to
    /// - `MessagePassportData`. Identifier of the specific checklist task that is being replied to
    /// - `MessagePhoto`. Identifier of the specific checklist task that is being replied to
    /// - `MessagePinnedMessage`. Identifier of the specific checklist task that is being replied to
    /// - `MessagePoll`. Identifier of the specific checklist task that is being replied to
    /// - `MessageProximityAlertTriggered`. Identifier of the specific checklist task that is being replied to
    /// - `MessageRefundedPayment`. Identifier of the specific checklist task that is being replied to
    /// - `MessageSticker`. Identifier of the specific checklist task that is being replied to
    /// - `MessageStory`. Identifier of the specific checklist task that is being replied to
    /// - `MessageSuccessfulPayment`. Identifier of the specific checklist task that is being replied to
    /// - `MessageSuggestedPostApprovalFailed`. Identifier of the specific checklist task that is being replied to
    /// - `MessageSuggestedPostApproved`. Identifier of the specific checklist task that is being replied to
    /// - `MessageSuggestedPostDeclined`. Identifier of the specific checklist task that is being replied to
    /// - `MessageSuggestedPostPaid`. Identifier of the specific checklist task that is being replied to
    /// - `MessageSuggestedPostRefunded`. Identifier of the specific checklist task that is being replied to
    /// - `MessageSupergroupChatCreated`. Identifier of the specific checklist task that is being replied to
    /// - `MessageText`. Identifier of the specific checklist task that is being replied to
    /// - `MessageUniqueGift`. Identifier of the specific checklist task that is being replied to
    /// - `MessageUsersShared`. Identifier of the specific checklist task that is being replied to
    /// - `MessageVenue`. Identifier of the specific checklist task that is being replied to
    /// - `MessageVideo`. Identifier of the specific checklist task that is being replied to
    /// - `MessageVideoChatEnded`. Identifier of the specific checklist task that is being replied to
    /// - `MessageVideoChatParticipantsInvited`. Identifier of the specific checklist task that is being replied to
    /// - `MessageVideoChatScheduled`. Identifier of the specific checklist task that is being replied to
    /// - `MessageVideoChatStarted`. Identifier of the specific checklist task that is being replied to
    /// - `MessageVideoNote`. Identifier of the specific checklist task that is being replied to
    /// - `MessageVoice`. Identifier of the specific checklist task that is being replied to
    /// - `MessageWebAppData`. Identifier of the specific checklist task that is being replied to
    /// - `MessageWriteAccessAllowed`. Identifier of the specific checklist task that is being replied to
    #[must_use]
    pub fn reply_to_checklist_task_id(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.reply_to_checklist_task_id,
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
            Self::ProximityAlertTriggered(val) => val.reply_to_checklist_task_id,
            Self::RefundedPayment(val) => val.reply_to_checklist_task_id,
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
            Self::Venue(val) => val.reply_to_checklist_task_id,
            Self::Video(val) => val.reply_to_checklist_task_id,
            Self::VideoChatEnded(val) => val.reply_to_checklist_task_id,
            Self::VideoChatParticipantsInvited(val) => val.reply_to_checklist_task_id,
            Self::VideoChatScheduled(val) => val.reply_to_checklist_task_id,
            Self::VideoChatStarted(val) => val.reply_to_checklist_task_id,
            Self::VideoNote(val) => val.reply_to_checklist_task_id,
            Self::Voice(val) => val.reply_to_checklist_task_id,
            Self::WebAppData(val) => val.reply_to_checklist_task_id,
            Self::WriteAccessAllowed(val) => val.reply_to_checklist_task_id,
        }
    }

    /// Helper method for field `reply_to_message`.
    ///
    /// # Variants
    /// - `MessageAnimation`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageAudio`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageBoostAdded`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageChannelChatCreated`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageChatBackgroundSet`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageChatOwnerChanged`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageChatOwnerLeft`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageChatShared`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageChecklist`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageChecklistTasksAdded`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageChecklistTasksDone`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageConnectedWebsite`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageContact`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageDeleteChatPhoto`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageDice`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageDirectMessagePriceChanged`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageDocument`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageForumTopicClosed`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageForumTopicCreated`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageForumTopicEdited`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageForumTopicReopened`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGame`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGeneralForumTopicHidden`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGeneralForumTopicUnhidden`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGift`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGiftUpgradeSent`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGiveaway`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGiveawayCompleted`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGiveawayCreated`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGiveawayWinners`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageGroupChatCreated`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageInvoice`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageLeftChatMember`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageLocation`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageMessageAutoDeleteTimerChanged`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageMigrateFromChatId`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageMigrateToChatId`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageNewChatMembers`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageNewChatPhoto`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageNewChatTitle`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessagePaidMedia`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessagePaidMessagePriceChanged`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessagePassportData`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessagePhoto`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessagePinnedMessage`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessagePoll`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageProximityAlertTriggered`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageRefundedPayment`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageSticker`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageStory`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageSuccessfulPayment`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageSuggestedPostApprovalFailed`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageSuggestedPostApproved`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageSuggestedPostDeclined`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageSuggestedPostPaid`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageSuggestedPostRefunded`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageSupergroupChatCreated`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageText`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageUniqueGift`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageUsersShared`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageVenue`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageVideo`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageVideoChatEnded`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageVideoChatParticipantsInvited`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageVideoChatScheduled`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageVideoChatStarted`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageVideoNote`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageVoice`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageWebAppData`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    /// - `MessageWriteAccessAllowed`. For replies in the same chat and message thread, the original message. Note that the Message object in this field will not contain further `reply_to_message` fields even if it itself is a reply.
    #[must_use]
    pub fn reply_to_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::Animation(val) => val.reply_to_message.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.reply_to_message.as_deref(),
            Self::RefundedPayment(val) => val.reply_to_message.as_deref(),
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
            Self::Venue(val) => val.reply_to_message.as_deref(),
            Self::Video(val) => val.reply_to_message.as_deref(),
            Self::VideoChatEnded(val) => val.reply_to_message.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.reply_to_message.as_deref(),
            Self::VideoChatScheduled(val) => val.reply_to_message.as_deref(),
            Self::VideoChatStarted(val) => val.reply_to_message.as_deref(),
            Self::VideoNote(val) => val.reply_to_message.as_deref(),
            Self::Voice(val) => val.reply_to_message.as_deref(),
            Self::WebAppData(val) => val.reply_to_message.as_deref(),
            Self::WriteAccessAllowed(val) => val.reply_to_message.as_deref(),
        }
    }

    /// Helper method for field `reply_to_story`.
    ///
    /// # Variants
    /// - `MessageAnimation`. For replies to a story, the original story
    /// - `MessageAudio`. For replies to a story, the original story
    /// - `MessageBoostAdded`. For replies to a story, the original story
    /// - `MessageChannelChatCreated`. For replies to a story, the original story
    /// - `MessageChatBackgroundSet`. For replies to a story, the original story
    /// - `MessageChatOwnerChanged`. For replies to a story, the original story
    /// - `MessageChatOwnerLeft`. For replies to a story, the original story
    /// - `MessageChatShared`. For replies to a story, the original story
    /// - `MessageChecklist`. For replies to a story, the original story
    /// - `MessageChecklistTasksAdded`. For replies to a story, the original story
    /// - `MessageChecklistTasksDone`. For replies to a story, the original story
    /// - `MessageConnectedWebsite`. For replies to a story, the original story
    /// - `MessageContact`. For replies to a story, the original story
    /// - `MessageDeleteChatPhoto`. For replies to a story, the original story
    /// - `MessageDice`. For replies to a story, the original story
    /// - `MessageDirectMessagePriceChanged`. For replies to a story, the original story
    /// - `MessageDocument`. For replies to a story, the original story
    /// - `MessageForumTopicClosed`. For replies to a story, the original story
    /// - `MessageForumTopicCreated`. For replies to a story, the original story
    /// - `MessageForumTopicEdited`. For replies to a story, the original story
    /// - `MessageForumTopicReopened`. For replies to a story, the original story
    /// - `MessageGame`. For replies to a story, the original story
    /// - `MessageGeneralForumTopicHidden`. For replies to a story, the original story
    /// - `MessageGeneralForumTopicUnhidden`. For replies to a story, the original story
    /// - `MessageGift`. For replies to a story, the original story
    /// - `MessageGiftUpgradeSent`. For replies to a story, the original story
    /// - `MessageGiveaway`. For replies to a story, the original story
    /// - `MessageGiveawayCompleted`. For replies to a story, the original story
    /// - `MessageGiveawayCreated`. For replies to a story, the original story
    /// - `MessageGiveawayWinners`. For replies to a story, the original story
    /// - `MessageGroupChatCreated`. For replies to a story, the original story
    /// - `MessageInvoice`. For replies to a story, the original story
    /// - `MessageLeftChatMember`. For replies to a story, the original story
    /// - `MessageLocation`. For replies to a story, the original story
    /// - `MessageMessageAutoDeleteTimerChanged`. For replies to a story, the original story
    /// - `MessageMigrateFromChatId`. For replies to a story, the original story
    /// - `MessageMigrateToChatId`. For replies to a story, the original story
    /// - `MessageNewChatMembers`. For replies to a story, the original story
    /// - `MessageNewChatPhoto`. For replies to a story, the original story
    /// - `MessageNewChatTitle`. For replies to a story, the original story
    /// - `MessagePaidMedia`. For replies to a story, the original story
    /// - `MessagePaidMessagePriceChanged`. For replies to a story, the original story
    /// - `MessagePassportData`. For replies to a story, the original story
    /// - `MessagePhoto`. For replies to a story, the original story
    /// - `MessagePinnedMessage`. For replies to a story, the original story
    /// - `MessagePoll`. For replies to a story, the original story
    /// - `MessageProximityAlertTriggered`. For replies to a story, the original story
    /// - `MessageRefundedPayment`. For replies to a story, the original story
    /// - `MessageSticker`. For replies to a story, the original story
    /// - `MessageStory`. For replies to a story, the original story
    /// - `MessageSuccessfulPayment`. For replies to a story, the original story
    /// - `MessageSuggestedPostApprovalFailed`. For replies to a story, the original story
    /// - `MessageSuggestedPostApproved`. For replies to a story, the original story
    /// - `MessageSuggestedPostDeclined`. For replies to a story, the original story
    /// - `MessageSuggestedPostPaid`. For replies to a story, the original story
    /// - `MessageSuggestedPostRefunded`. For replies to a story, the original story
    /// - `MessageSupergroupChatCreated`. For replies to a story, the original story
    /// - `MessageText`. For replies to a story, the original story
    /// - `MessageUniqueGift`. For replies to a story, the original story
    /// - `MessageUsersShared`. For replies to a story, the original story
    /// - `MessageVenue`. For replies to a story, the original story
    /// - `MessageVideo`. For replies to a story, the original story
    /// - `MessageVideoChatEnded`. For replies to a story, the original story
    /// - `MessageVideoChatParticipantsInvited`. For replies to a story, the original story
    /// - `MessageVideoChatScheduled`. For replies to a story, the original story
    /// - `MessageVideoChatStarted`. For replies to a story, the original story
    /// - `MessageVideoNote`. For replies to a story, the original story
    /// - `MessageVoice`. For replies to a story, the original story
    /// - `MessageWebAppData`. For replies to a story, the original story
    /// - `MessageWriteAccessAllowed`. For replies to a story, the original story
    #[must_use]
    pub fn reply_to_story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::Animation(val) => val.reply_to_story.as_ref(),
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
            Self::ProximityAlertTriggered(val) => val.reply_to_story.as_ref(),
            Self::RefundedPayment(val) => val.reply_to_story.as_ref(),
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
            Self::Venue(val) => val.reply_to_story.as_ref(),
            Self::Video(val) => val.reply_to_story.as_ref(),
            Self::VideoChatEnded(val) => val.reply_to_story.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.reply_to_story.as_ref(),
            Self::VideoChatScheduled(val) => val.reply_to_story.as_ref(),
            Self::VideoChatStarted(val) => val.reply_to_story.as_ref(),
            Self::VideoNote(val) => val.reply_to_story.as_ref(),
            Self::Voice(val) => val.reply_to_story.as_ref(),
            Self::WebAppData(val) => val.reply_to_story.as_ref(),
            Self::WriteAccessAllowed(val) => val.reply_to_story.as_ref(),
        }
    }

    /// Helper method for field `sender_boost_count`.
    ///
    /// # Variants
    /// - `MessageAnimation`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageAudio`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageBoostAdded`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageChannelChatCreated`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageChatBackgroundSet`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageChatOwnerChanged`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageChatOwnerLeft`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageChatShared`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageChecklist`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageChecklistTasksAdded`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageChecklistTasksDone`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageConnectedWebsite`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageContact`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageDeleteChatPhoto`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageDice`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageDirectMessagePriceChanged`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageDocument`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageForumTopicClosed`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageForumTopicCreated`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageForumTopicEdited`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageForumTopicReopened`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGame`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGeneralForumTopicHidden`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGeneralForumTopicUnhidden`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGift`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGiftUpgradeSent`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGiveaway`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGiveawayCompleted`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGiveawayCreated`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGiveawayWinners`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageGroupChatCreated`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageInvoice`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageLeftChatMember`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageLocation`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageMessageAutoDeleteTimerChanged`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageMigrateFromChatId`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageMigrateToChatId`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageNewChatMembers`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageNewChatPhoto`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageNewChatTitle`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessagePaidMedia`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessagePaidMessagePriceChanged`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessagePassportData`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessagePhoto`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessagePinnedMessage`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessagePoll`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageProximityAlertTriggered`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageRefundedPayment`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageSticker`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageStory`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageSuccessfulPayment`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageSuggestedPostApprovalFailed`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageSuggestedPostApproved`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageSuggestedPostDeclined`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageSuggestedPostPaid`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageSuggestedPostRefunded`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageSupergroupChatCreated`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageText`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageUniqueGift`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageUsersShared`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageVenue`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageVideo`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageVideoChatEnded`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageVideoChatParticipantsInvited`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageVideoChatScheduled`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageVideoChatStarted`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageVideoNote`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageVoice`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageWebAppData`. If the sender of the message boosted the chat, the number of boosts added by the user
    /// - `MessageWriteAccessAllowed`. If the sender of the message boosted the chat, the number of boosts added by the user
    #[must_use]
    pub fn sender_boost_count(&self) -> Option<i64> {
        match self {
            Self::Animation(val) => val.sender_boost_count,
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
            Self::ProximityAlertTriggered(val) => val.sender_boost_count,
            Self::RefundedPayment(val) => val.sender_boost_count,
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
            Self::Venue(val) => val.sender_boost_count,
            Self::Video(val) => val.sender_boost_count,
            Self::VideoChatEnded(val) => val.sender_boost_count,
            Self::VideoChatParticipantsInvited(val) => val.sender_boost_count,
            Self::VideoChatScheduled(val) => val.sender_boost_count,
            Self::VideoChatStarted(val) => val.sender_boost_count,
            Self::VideoNote(val) => val.sender_boost_count,
            Self::Voice(val) => val.sender_boost_count,
            Self::WebAppData(val) => val.sender_boost_count,
            Self::WriteAccessAllowed(val) => val.sender_boost_count,
        }
    }

    /// Helper method for field `sender_business_bot`.
    ///
    /// # Variants
    /// - `MessageAnimation`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageAudio`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageBoostAdded`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageChannelChatCreated`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageChatBackgroundSet`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageChatOwnerChanged`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageChatOwnerLeft`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageChatShared`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageChecklist`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageChecklistTasksAdded`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageChecklistTasksDone`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageConnectedWebsite`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageContact`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageDeleteChatPhoto`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageDice`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageDirectMessagePriceChanged`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageDocument`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageForumTopicClosed`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageForumTopicCreated`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageForumTopicEdited`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageForumTopicReopened`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGame`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGeneralForumTopicHidden`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGeneralForumTopicUnhidden`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGift`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGiftUpgradeSent`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGiveaway`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGiveawayCompleted`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGiveawayCreated`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGiveawayWinners`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageGroupChatCreated`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageInvoice`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageLeftChatMember`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageLocation`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageMessageAutoDeleteTimerChanged`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageMigrateFromChatId`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageMigrateToChatId`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageNewChatMembers`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageNewChatPhoto`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageNewChatTitle`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessagePaidMedia`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessagePaidMessagePriceChanged`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessagePassportData`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessagePhoto`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessagePinnedMessage`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessagePoll`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageProximityAlertTriggered`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageRefundedPayment`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageSticker`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageStory`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageSuccessfulPayment`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageSuggestedPostApprovalFailed`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageSuggestedPostApproved`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageSuggestedPostDeclined`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageSuggestedPostPaid`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageSuggestedPostRefunded`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageSupergroupChatCreated`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageText`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageUniqueGift`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageUsersShared`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageVenue`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageVideo`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageVideoChatEnded`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageVideoChatParticipantsInvited`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageVideoChatScheduled`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageVideoChatStarted`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageVideoNote`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageVoice`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageWebAppData`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    /// - `MessageWriteAccessAllowed`. The bot that actually sent the message on behalf of the business account. Available only for outgoing messages sent on behalf of the connected business account.
    #[must_use]
    pub fn sender_business_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::Animation(val) => val.sender_business_bot.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.sender_business_bot.as_deref(),
            Self::RefundedPayment(val) => val.sender_business_bot.as_deref(),
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
            Self::Venue(val) => val.sender_business_bot.as_deref(),
            Self::Video(val) => val.sender_business_bot.as_deref(),
            Self::VideoChatEnded(val) => val.sender_business_bot.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.sender_business_bot.as_deref(),
            Self::VideoChatScheduled(val) => val.sender_business_bot.as_deref(),
            Self::VideoChatStarted(val) => val.sender_business_bot.as_deref(),
            Self::VideoNote(val) => val.sender_business_bot.as_deref(),
            Self::Voice(val) => val.sender_business_bot.as_deref(),
            Self::WebAppData(val) => val.sender_business_bot.as_deref(),
            Self::WriteAccessAllowed(val) => val.sender_business_bot.as_deref(),
        }
    }

    /// Helper method for field `sender_chat`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageAudio`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageBoostAdded`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageChannelChatCreated`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageChatBackgroundSet`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageChatOwnerChanged`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageChatOwnerLeft`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageChatShared`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageChecklist`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageChecklistTasksAdded`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageChecklistTasksDone`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageConnectedWebsite`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageContact`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageDeleteChatPhoto`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageDice`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageDirectMessagePriceChanged`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageDocument`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageForumTopicClosed`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageForumTopicCreated`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageForumTopicEdited`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageForumTopicReopened`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGame`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGeneralForumTopicHidden`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGeneralForumTopicUnhidden`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGift`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGiftUpgradeSent`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGiveaway`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGiveawayCompleted`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGiveawayCreated`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGiveawayWinners`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageGroupChatCreated`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageInvoice`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageLeftChatMember`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageLocation`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageMessageAutoDeleteTimerChanged`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageMigrateFromChatId`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageMigrateToChatId`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageNewChatMembers`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageNewChatPhoto`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageNewChatTitle`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessagePaidMedia`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessagePaidMessagePriceChanged`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessagePassportData`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessagePhoto`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessagePinnedMessage`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessagePoll`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageProximityAlertTriggered`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageRefundedPayment`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageSticker`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageStory`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageSuccessfulPayment`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageSuggestedPostApprovalFailed`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageSuggestedPostApproved`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageSuggestedPostDeclined`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageSuggestedPostPaid`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageSuggestedPostRefunded`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageSupergroupChatCreated`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageText`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageUniqueGift`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageUsersShared`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageVenue`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageVideo`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageVideoChatEnded`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageVideoChatParticipantsInvited`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageVideoChatScheduled`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageVideoChatStarted`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageVideoNote`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageVoice`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageWebAppData`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    /// - `MessageWriteAccessAllowed`. Sender of the message when sent on behalf of a chat. For example, the supergroup itself for messages sent by its anonymous administrators or a linked channel for messages automatically forwarded to the channel's discussion group. For backward compatibility, if the message was sent on behalf of a chat, the field from contains a fake sender user in non-channel chats.
    #[must_use]
    pub fn sender_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Animation(val) => val.sender_chat.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.sender_chat.as_deref(),
            Self::RefundedPayment(val) => val.sender_chat.as_deref(),
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
            Self::Venue(val) => val.sender_chat.as_deref(),
            Self::Video(val) => val.sender_chat.as_deref(),
            Self::VideoChatEnded(val) => val.sender_chat.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.sender_chat.as_deref(),
            Self::VideoChatScheduled(val) => val.sender_chat.as_deref(),
            Self::VideoChatStarted(val) => val.sender_chat.as_deref(),
            Self::VideoNote(val) => val.sender_chat.as_deref(),
            Self::Voice(val) => val.sender_chat.as_deref(),
            Self::WebAppData(val) => val.sender_chat.as_deref(),
            Self::WriteAccessAllowed(val) => val.sender_chat.as_deref(),
        }
    }

    /// Helper method for field `show_caption_above_media`.
    ///
    /// # Variants
    /// - `MessageAnimation`. `true`, if the caption must be shown above the message media
    /// - `MessageAudio`. `true`, if the caption must be shown above the message media
    /// - `MessageBoostAdded`. `true`, if the caption must be shown above the message media
    /// - `MessageChannelChatCreated`. `true`, if the caption must be shown above the message media
    /// - `MessageChatBackgroundSet`. `true`, if the caption must be shown above the message media
    /// - `MessageChatOwnerChanged`. `true`, if the caption must be shown above the message media
    /// - `MessageChatOwnerLeft`. `true`, if the caption must be shown above the message media
    /// - `MessageChatShared`. `true`, if the caption must be shown above the message media
    /// - `MessageChecklist`. `true`, if the caption must be shown above the message media
    /// - `MessageChecklistTasksAdded`. `true`, if the caption must be shown above the message media
    /// - `MessageChecklistTasksDone`. `true`, if the caption must be shown above the message media
    /// - `MessageConnectedWebsite`. `true`, if the caption must be shown above the message media
    /// - `MessageContact`. `true`, if the caption must be shown above the message media
    /// - `MessageDeleteChatPhoto`. `true`, if the caption must be shown above the message media
    /// - `MessageDice`. `true`, if the caption must be shown above the message media
    /// - `MessageDirectMessagePriceChanged`. `true`, if the caption must be shown above the message media
    /// - `MessageDocument`. `true`, if the caption must be shown above the message media
    /// - `MessageForumTopicClosed`. `true`, if the caption must be shown above the message media
    /// - `MessageForumTopicCreated`. `true`, if the caption must be shown above the message media
    /// - `MessageForumTopicEdited`. `true`, if the caption must be shown above the message media
    /// - `MessageForumTopicReopened`. `true`, if the caption must be shown above the message media
    /// - `MessageGame`. `true`, if the caption must be shown above the message media
    /// - `MessageGeneralForumTopicHidden`. `true`, if the caption must be shown above the message media
    /// - `MessageGeneralForumTopicUnhidden`. `true`, if the caption must be shown above the message media
    /// - `MessageGift`. `true`, if the caption must be shown above the message media
    /// - `MessageGiftUpgradeSent`. `true`, if the caption must be shown above the message media
    /// - `MessageGiveaway`. `true`, if the caption must be shown above the message media
    /// - `MessageGiveawayCompleted`. `true`, if the caption must be shown above the message media
    /// - `MessageGiveawayCreated`. `true`, if the caption must be shown above the message media
    /// - `MessageGiveawayWinners`. `true`, if the caption must be shown above the message media
    /// - `MessageGroupChatCreated`. `true`, if the caption must be shown above the message media
    /// - `MessageInvoice`. `true`, if the caption must be shown above the message media
    /// - `MessageLeftChatMember`. `true`, if the caption must be shown above the message media
    /// - `MessageLocation`. `true`, if the caption must be shown above the message media
    /// - `MessageMessageAutoDeleteTimerChanged`. `true`, if the caption must be shown above the message media
    /// - `MessageMigrateFromChatId`. `true`, if the caption must be shown above the message media
    /// - `MessageMigrateToChatId`. `true`, if the caption must be shown above the message media
    /// - `MessageNewChatMembers`. `true`, if the caption must be shown above the message media
    /// - `MessageNewChatPhoto`. `true`, if the caption must be shown above the message media
    /// - `MessageNewChatTitle`. `true`, if the caption must be shown above the message media
    /// - `MessagePaidMedia`. `true`, if the caption must be shown above the message media
    /// - `MessagePaidMessagePriceChanged`. `true`, if the caption must be shown above the message media
    /// - `MessagePassportData`. `true`, if the caption must be shown above the message media
    /// - `MessagePhoto`. `true`, if the caption must be shown above the message media
    /// - `MessagePinnedMessage`. `true`, if the caption must be shown above the message media
    /// - `MessagePoll`. `true`, if the caption must be shown above the message media
    /// - `MessageProximityAlertTriggered`. `true`, if the caption must be shown above the message media
    /// - `MessageRefundedPayment`. `true`, if the caption must be shown above the message media
    /// - `MessageSticker`. `true`, if the caption must be shown above the message media
    /// - `MessageStory`. `true`, if the caption must be shown above the message media
    /// - `MessageSuccessfulPayment`. `true`, if the caption must be shown above the message media
    /// - `MessageSuggestedPostApprovalFailed`. `true`, if the caption must be shown above the message media
    /// - `MessageSuggestedPostApproved`. `true`, if the caption must be shown above the message media
    /// - `MessageSuggestedPostDeclined`. `true`, if the caption must be shown above the message media
    /// - `MessageSuggestedPostPaid`. `true`, if the caption must be shown above the message media
    /// - `MessageSuggestedPostRefunded`. `true`, if the caption must be shown above the message media
    /// - `MessageSupergroupChatCreated`. `true`, if the caption must be shown above the message media
    /// - `MessageText`. `true`, if the caption must be shown above the message media
    /// - `MessageUniqueGift`. `true`, if the caption must be shown above the message media
    /// - `MessageUsersShared`. `true`, if the caption must be shown above the message media
    /// - `MessageVenue`. `true`, if the caption must be shown above the message media
    /// - `MessageVideo`. `true`, if the caption must be shown above the message media
    /// - `MessageVideoChatEnded`. `true`, if the caption must be shown above the message media
    /// - `MessageVideoChatParticipantsInvited`. `true`, if the caption must be shown above the message media
    /// - `MessageVideoChatScheduled`. `true`, if the caption must be shown above the message media
    /// - `MessageVideoChatStarted`. `true`, if the caption must be shown above the message media
    /// - `MessageVideoNote`. `true`, if the caption must be shown above the message media
    /// - `MessageVoice`. `true`, if the caption must be shown above the message media
    /// - `MessageWebAppData`. `true`, if the caption must be shown above the message media
    /// - `MessageWriteAccessAllowed`. `true`, if the caption must be shown above the message media
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        match self {
            Self::Animation(val) => val.show_caption_above_media,
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
            Self::ProximityAlertTriggered(val) => val.show_caption_above_media,
            Self::RefundedPayment(val) => val.show_caption_above_media,
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
            Self::Venue(val) => val.show_caption_above_media,
            Self::Video(val) => val.show_caption_above_media,
            Self::VideoChatEnded(val) => val.show_caption_above_media,
            Self::VideoChatParticipantsInvited(val) => val.show_caption_above_media,
            Self::VideoChatScheduled(val) => val.show_caption_above_media,
            Self::VideoChatStarted(val) => val.show_caption_above_media,
            Self::VideoNote(val) => val.show_caption_above_media,
            Self::Voice(val) => val.show_caption_above_media,
            Self::WebAppData(val) => val.show_caption_above_media,
            Self::WriteAccessAllowed(val) => val.show_caption_above_media,
        }
    }

    /// Helper method for field `sticker`.
    ///
    /// # Variants
    /// - `MessageSticker`. Message is a sticker, information about the sticker
    #[must_use]
    pub fn sticker(&self) -> Option<&crate::types::Sticker> {
        match self {
            Self::Sticker(val) => Some(val.sticker.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `story`.
    ///
    /// # Variants
    /// - `MessageStory`. Message is a forwarded story
    #[must_use]
    pub fn story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::Story(val) => Some(&val.story),
            _ => None,
        }
    }

    /// Helper method for field `successful_payment`.
    ///
    /// # Variants
    /// - `MessageSuccessfulPayment`. Message is a service message about a successful payment, information about the payment. More about payments: <https://core.telegram.org/bots/api#payments>
    #[must_use]
    pub fn successful_payment(&self) -> Option<&crate::types::SuccessfulPayment> {
        match self {
            Self::SuccessfulPayment(val) => Some(val.successful_payment.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `suggested_post_approval_failed`.
    ///
    /// # Variants
    /// - `MessageSuggestedPostApprovalFailed`. Service message: approval of a suggested post has failed
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
    /// # Variants
    /// - `MessageSuggestedPostApproved`. Service message: a suggested post was approved
    #[must_use]
    pub fn suggested_post_approved(&self) -> Option<&crate::types::SuggestedPostApproved> {
        match self {
            Self::SuggestedPostApproved(val) => Some(&val.suggested_post_approved),
            _ => None,
        }
    }

    /// Helper method for field `suggested_post_declined`.
    ///
    /// # Variants
    /// - `MessageSuggestedPostDeclined`. Service message: a suggested post was declined
    #[must_use]
    pub fn suggested_post_declined(&self) -> Option<&crate::types::SuggestedPostDeclined> {
        match self {
            Self::SuggestedPostDeclined(val) => Some(&val.suggested_post_declined),
            _ => None,
        }
    }

    /// Helper method for field `suggested_post_info`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageAudio`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageBoostAdded`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageChannelChatCreated`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageChatBackgroundSet`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageChatOwnerChanged`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageChatOwnerLeft`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageChatShared`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageChecklist`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageChecklistTasksAdded`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageChecklistTasksDone`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageConnectedWebsite`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageContact`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageDeleteChatPhoto`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageDice`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageDirectMessagePriceChanged`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageDocument`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageForumTopicClosed`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageForumTopicCreated`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageForumTopicEdited`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageForumTopicReopened`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGame`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGeneralForumTopicHidden`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGeneralForumTopicUnhidden`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGift`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGiftUpgradeSent`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGiveaway`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGiveawayCompleted`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGiveawayCreated`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGiveawayWinners`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageGroupChatCreated`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageInvoice`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageLeftChatMember`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageLocation`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageMessageAutoDeleteTimerChanged`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageMigrateFromChatId`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageMigrateToChatId`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageNewChatMembers`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageNewChatPhoto`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageNewChatTitle`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessagePaidMedia`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessagePaidMessagePriceChanged`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessagePassportData`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessagePhoto`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessagePinnedMessage`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessagePoll`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageProximityAlertTriggered`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageRefundedPayment`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageSticker`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageStory`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageSuccessfulPayment`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageSuggestedPostApprovalFailed`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageSuggestedPostApproved`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageSuggestedPostDeclined`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageSuggestedPostPaid`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageSuggestedPostRefunded`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageSupergroupChatCreated`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageText`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageUniqueGift`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageUsersShared`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageVenue`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageVideo`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageVideoChatEnded`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageVideoChatParticipantsInvited`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageVideoChatScheduled`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageVideoChatStarted`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageVideoNote`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageVoice`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageWebAppData`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    /// - `MessageWriteAccessAllowed`. Information about suggested post parameters if the message is a suggested post in a channel direct messages chat. If the message is an approved or declined suggested post, then it can't be edited.
    #[must_use]
    pub fn suggested_post_info(&self) -> Option<&crate::types::SuggestedPostInfo> {
        match self {
            Self::Animation(val) => val.suggested_post_info.as_ref(),
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
            Self::ProximityAlertTriggered(val) => val.suggested_post_info.as_ref(),
            Self::RefundedPayment(val) => val.suggested_post_info.as_ref(),
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
            Self::Venue(val) => val.suggested_post_info.as_ref(),
            Self::Video(val) => val.suggested_post_info.as_ref(),
            Self::VideoChatEnded(val) => val.suggested_post_info.as_ref(),
            Self::VideoChatParticipantsInvited(val) => val.suggested_post_info.as_ref(),
            Self::VideoChatScheduled(val) => val.suggested_post_info.as_ref(),
            Self::VideoChatStarted(val) => val.suggested_post_info.as_ref(),
            Self::VideoNote(val) => val.suggested_post_info.as_ref(),
            Self::Voice(val) => val.suggested_post_info.as_ref(),
            Self::WebAppData(val) => val.suggested_post_info.as_ref(),
            Self::WriteAccessAllowed(val) => val.suggested_post_info.as_ref(),
        }
    }

    /// Helper method for field `suggested_post_paid`.
    ///
    /// # Variants
    /// - `MessageSuggestedPostPaid`. Service message: payment for a suggested post was received
    #[must_use]
    pub fn suggested_post_paid(&self) -> Option<&crate::types::SuggestedPostPaid> {
        match self {
            Self::SuggestedPostPaid(val) => Some(&val.suggested_post_paid),
            _ => None,
        }
    }

    /// Helper method for field `suggested_post_refunded`.
    ///
    /// # Variants
    /// - `MessageSuggestedPostRefunded`. Service message: payment for a suggested post was refunded
    #[must_use]
    pub fn suggested_post_refunded(&self) -> Option<&crate::types::SuggestedPostRefunded> {
        match self {
            Self::SuggestedPostRefunded(val) => Some(&val.suggested_post_refunded),
            _ => None,
        }
    }

    /// Helper method for field `supergroup_chat_created`.
    ///
    /// # Variants
    /// - `MessageSupergroupChatCreated`. Service message: the supergroup has been created. This field can't be received in a message coming through updates, because bot can't be a member of a supergroup when it is created. It can only be found in `reply_to_message` if someone replies to a very first message in a directly created supergroup.
    #[must_use]
    pub fn supergroup_chat_created(&self) -> Option<bool> {
        match self {
            Self::SupergroupChatCreated(val) => Some(val.supergroup_chat_created),
            _ => None,
        }
    }

    /// Helper method for field `text`.
    ///
    /// # Variants
    /// - `MessageText`. For text messages, the actual UTF-8 text of the message
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Text(val) => Some(val.text.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `unique_gift`.
    ///
    /// # Variants
    /// - `MessageUniqueGift`. Service message: a unique gift was sent or received
    #[must_use]
    pub fn unique_gift(&self) -> Option<&crate::types::UniqueGiftInfo> {
        match self {
            Self::UniqueGift(val) => Some(&val.unique_gift),
            _ => None,
        }
    }

    /// Helper method for field `users_shared`.
    ///
    /// # Variants
    /// - `MessageUsersShared`. Service message: users were shared with the bot
    #[must_use]
    pub fn users_shared(&self) -> Option<&crate::types::UsersShared> {
        match self {
            Self::UsersShared(val) => Some(&val.users_shared),
            _ => None,
        }
    }

    /// Helper method for field `venue`.
    ///
    /// # Variants
    /// - `MessageVenue`. Message is a venue, information about the venue. For backward compatibility, when this field is set, the location field will also be set
    #[must_use]
    pub fn venue(&self) -> Option<&crate::types::Venue> {
        match self {
            Self::Venue(val) => Some(val.venue.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `via_bot`.
    ///
    /// # Variants
    /// - `MessageAnimation`. Bot through which the message was sent
    /// - `MessageAudio`. Bot through which the message was sent
    /// - `MessageBoostAdded`. Bot through which the message was sent
    /// - `MessageChannelChatCreated`. Bot through which the message was sent
    /// - `MessageChatBackgroundSet`. Bot through which the message was sent
    /// - `MessageChatOwnerChanged`. Bot through which the message was sent
    /// - `MessageChatOwnerLeft`. Bot through which the message was sent
    /// - `MessageChatShared`. Bot through which the message was sent
    /// - `MessageChecklist`. Bot through which the message was sent
    /// - `MessageChecklistTasksAdded`. Bot through which the message was sent
    /// - `MessageChecklistTasksDone`. Bot through which the message was sent
    /// - `MessageConnectedWebsite`. Bot through which the message was sent
    /// - `MessageContact`. Bot through which the message was sent
    /// - `MessageDeleteChatPhoto`. Bot through which the message was sent
    /// - `MessageDice`. Bot through which the message was sent
    /// - `MessageDirectMessagePriceChanged`. Bot through which the message was sent
    /// - `MessageDocument`. Bot through which the message was sent
    /// - `MessageForumTopicClosed`. Bot through which the message was sent
    /// - `MessageForumTopicCreated`. Bot through which the message was sent
    /// - `MessageForumTopicEdited`. Bot through which the message was sent
    /// - `MessageForumTopicReopened`. Bot through which the message was sent
    /// - `MessageGame`. Bot through which the message was sent
    /// - `MessageGeneralForumTopicHidden`. Bot through which the message was sent
    /// - `MessageGeneralForumTopicUnhidden`. Bot through which the message was sent
    /// - `MessageGift`. Bot through which the message was sent
    /// - `MessageGiftUpgradeSent`. Bot through which the message was sent
    /// - `MessageGiveaway`. Bot through which the message was sent
    /// - `MessageGiveawayCompleted`. Bot through which the message was sent
    /// - `MessageGiveawayCreated`. Bot through which the message was sent
    /// - `MessageGiveawayWinners`. Bot through which the message was sent
    /// - `MessageGroupChatCreated`. Bot through which the message was sent
    /// - `MessageInvoice`. Bot through which the message was sent
    /// - `MessageLeftChatMember`. Bot through which the message was sent
    /// - `MessageLocation`. Bot through which the message was sent
    /// - `MessageMessageAutoDeleteTimerChanged`. Bot through which the message was sent
    /// - `MessageMigrateFromChatId`. Bot through which the message was sent
    /// - `MessageMigrateToChatId`. Bot through which the message was sent
    /// - `MessageNewChatMembers`. Bot through which the message was sent
    /// - `MessageNewChatPhoto`. Bot through which the message was sent
    /// - `MessageNewChatTitle`. Bot through which the message was sent
    /// - `MessagePaidMedia`. Bot through which the message was sent
    /// - `MessagePaidMessagePriceChanged`. Bot through which the message was sent
    /// - `MessagePassportData`. Bot through which the message was sent
    /// - `MessagePhoto`. Bot through which the message was sent
    /// - `MessagePinnedMessage`. Bot through which the message was sent
    /// - `MessagePoll`. Bot through which the message was sent
    /// - `MessageProximityAlertTriggered`. Bot through which the message was sent
    /// - `MessageRefundedPayment`. Bot through which the message was sent
    /// - `MessageSticker`. Bot through which the message was sent
    /// - `MessageStory`. Bot through which the message was sent
    /// - `MessageSuccessfulPayment`. Bot through which the message was sent
    /// - `MessageSuggestedPostApprovalFailed`. Bot through which the message was sent
    /// - `MessageSuggestedPostApproved`. Bot through which the message was sent
    /// - `MessageSuggestedPostDeclined`. Bot through which the message was sent
    /// - `MessageSuggestedPostPaid`. Bot through which the message was sent
    /// - `MessageSuggestedPostRefunded`. Bot through which the message was sent
    /// - `MessageSupergroupChatCreated`. Bot through which the message was sent
    /// - `MessageText`. Bot through which the message was sent
    /// - `MessageUniqueGift`. Bot through which the message was sent
    /// - `MessageUsersShared`. Bot through which the message was sent
    /// - `MessageVenue`. Bot through which the message was sent
    /// - `MessageVideo`. Bot through which the message was sent
    /// - `MessageVideoChatEnded`. Bot through which the message was sent
    /// - `MessageVideoChatParticipantsInvited`. Bot through which the message was sent
    /// - `MessageVideoChatScheduled`. Bot through which the message was sent
    /// - `MessageVideoChatStarted`. Bot through which the message was sent
    /// - `MessageVideoNote`. Bot through which the message was sent
    /// - `MessageVoice`. Bot through which the message was sent
    /// - `MessageWebAppData`. Bot through which the message was sent
    /// - `MessageWriteAccessAllowed`. Bot through which the message was sent
    #[must_use]
    pub fn via_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::Animation(val) => val.via_bot.as_deref(),
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
            Self::ProximityAlertTriggered(val) => val.via_bot.as_deref(),
            Self::RefundedPayment(val) => val.via_bot.as_deref(),
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
            Self::Venue(val) => val.via_bot.as_deref(),
            Self::Video(val) => val.via_bot.as_deref(),
            Self::VideoChatEnded(val) => val.via_bot.as_deref(),
            Self::VideoChatParticipantsInvited(val) => val.via_bot.as_deref(),
            Self::VideoChatScheduled(val) => val.via_bot.as_deref(),
            Self::VideoChatStarted(val) => val.via_bot.as_deref(),
            Self::VideoNote(val) => val.via_bot.as_deref(),
            Self::Voice(val) => val.via_bot.as_deref(),
            Self::WebAppData(val) => val.via_bot.as_deref(),
            Self::WriteAccessAllowed(val) => val.via_bot.as_deref(),
        }
    }

    /// Helper method for field `video`.
    ///
    /// # Variants
    /// - `MessageVideo`. Message is a video, information about the video
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::Video> {
        match self {
            Self::Video(val) => Some(val.video.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `video_chat_ended`.
    ///
    /// # Variants
    /// - `MessageVideoChatEnded`. Service message: video chat ended
    #[must_use]
    pub fn video_chat_ended(&self) -> Option<&crate::types::VideoChatEnded> {
        match self {
            Self::VideoChatEnded(val) => Some(&val.video_chat_ended),
            _ => None,
        }
    }

    /// Helper method for field `video_chat_participants_invited`.
    ///
    /// # Variants
    /// - `MessageVideoChatParticipantsInvited`. Service message: new participants invited to a video chat
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
    /// # Variants
    /// - `MessageVideoChatScheduled`. Service message: video chat scheduled
    #[must_use]
    pub fn video_chat_scheduled(&self) -> Option<&crate::types::VideoChatScheduled> {
        match self {
            Self::VideoChatScheduled(val) => Some(&val.video_chat_scheduled),
            _ => None,
        }
    }

    /// Helper method for field `video_chat_started`.
    ///
    /// # Variants
    /// - `MessageVideoChatStarted`. Service message: video chat started
    #[must_use]
    pub fn video_chat_started(&self) -> Option<&crate::types::VideoChatStarted> {
        match self {
            Self::VideoChatStarted(val) => Some(&val.video_chat_started),
            _ => None,
        }
    }

    /// Helper method for field `video_note`.
    ///
    /// # Variants
    /// - `MessageVideoNote`. Message is a video note, information about the video message
    #[must_use]
    pub fn video_note(&self) -> Option<&crate::types::VideoNote> {
        match self {
            Self::VideoNote(val) => Some(val.video_note.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `voice`.
    ///
    /// # Variants
    /// - `MessageVoice`. Message is a voice message, information about the file
    #[must_use]
    pub fn voice(&self) -> Option<&crate::types::Voice> {
        match self {
            Self::Voice(val) => Some(&val.voice),
            _ => None,
        }
    }

    /// Helper method for field `web_app_data`.
    ///
    /// # Variants
    /// - `MessageWebAppData`. Service message: data sent by a Web App
    #[must_use]
    pub fn web_app_data(&self) -> Option<&crate::types::WebAppData> {
        match self {
            Self::WebAppData(val) => Some(&val.web_app_data),
            _ => None,
        }
    }

    /// Helper method for field `write_access_allowed`.
    ///
    /// # Variants
    /// - `MessageWriteAccessAllowed`. Service message: the user allowed the bot to write messages after adding it to the attachment or side menu, launching a Web App from a link, or accepting an explicit request from a Web App sent by the method request[`WriteAccess`]
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

    /// Helper method for nested field `correct_option_id`.
    #[must_use]
    pub fn correct_option_id(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::correct_option_id(inner)
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
    pub fn icon_color(&self) -> Option<i64> {
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
