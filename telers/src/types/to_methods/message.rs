use crate::{
    methods::{CopyMessage, DeleteMessage, ForwardMessage},
    types::{
        ChatIdKind, Message, MessageAnimation, MessageAudio, MessageBoostAdded,
        MessageChannelChatCreated, MessageChatBackgroundSet, MessageChatOwnerChanged,
        MessageChatOwnerLeft, MessageChatShared, MessageChecklist, MessageChecklistTasksAdded,
        MessageChecklistTasksDone, MessageConnectedWebsite, MessageContact, MessageDeleteChatPhoto,
        MessageDice, MessageDirectMessagePriceChanged, MessageDocument, MessageForumTopicClosed,
        MessageForumTopicCreated, MessageForumTopicEdited, MessageForumTopicReopened, MessageGame,
        MessageGeneralForumTopicHidden, MessageGeneralForumTopicUnhidden, MessageGift,
        MessageGiftUpgradeSent, MessageGiveaway, MessageGiveawayCompleted, MessageGiveawayCreated,
        MessageGiveawayWinners, MessageGroupChatCreated, MessageInvoice, MessageLeftChatMember,
        MessageLocation, MessageMessageAutoDeleteTimerChanged, MessageMigrateFromChatId,
        MessageMigrateToChatId, MessageNewChatMembers, MessageNewChatPhoto, MessageNewChatTitle,
        MessagePaidMedia, MessagePaidMessagePriceChanged, MessagePassportData, MessagePhoto,
        MessagePinnedMessage, MessagePoll, MessageProximityAlertTriggered, MessageRefundedPayment,
        MessageSticker, MessageStory, MessageSuccessfulPayment, MessageSuggestedPostApprovalFailed,
        MessageSuggestedPostApproved, MessageSuggestedPostDeclined, MessageSuggestedPostPaid,
        MessageSuggestedPostRefunded, MessageSupergroupChatCreated, MessageText, MessageUniqueGift,
        MessageUsersShared, MessageVenue, MessageVideo, MessageVideoChatEnded,
        MessageVideoChatParticipantsInvited, MessageVideoChatScheduled, MessageVideoChatStarted,
        MessageVideoNote, MessageVoice, MessageWebAppData, MessageWriteAccessAllowed,
    },
};
impl Message {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat().id(), self.message_id())
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat().id(), self.message_id())
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat().id(), self.message_id())
    }
}
impl MessageAnimation {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageAnimation>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageAnimation>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageAnimation>>::from(self).delete_message()
    }
}
impl MessageAudio {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageAudio>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageAudio>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageAudio>>::from(self).delete_message()
    }
}
impl MessageBoostAdded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageBoostAdded>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageBoostAdded>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageBoostAdded>>::from(self).delete_message()
    }
}
impl MessageChannelChatCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageChannelChatCreated>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageChannelChatCreated>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageChannelChatCreated>>::from(self).delete_message()
    }
}
impl MessageChatBackgroundSet {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageChatBackgroundSet>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageChatBackgroundSet>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageChatBackgroundSet>>::from(self).delete_message()
    }
}
impl MessageChatOwnerChanged {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageChatOwnerChanged>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageChatOwnerChanged>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageChatOwnerChanged>>::from(self).delete_message()
    }
}
impl MessageChatOwnerLeft {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageChatOwnerLeft>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageChatOwnerLeft>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageChatOwnerLeft>>::from(self).delete_message()
    }
}
impl MessageChatShared {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageChatShared>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageChatShared>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageChatShared>>::from(self).delete_message()
    }
}
impl MessageChecklist {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageChecklist>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageChecklist>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageChecklist>>::from(self).delete_message()
    }
}
impl MessageChecklistTasksAdded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageChecklistTasksAdded>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageChecklistTasksAdded>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageChecklistTasksAdded>>::from(self).delete_message()
    }
}
impl MessageChecklistTasksDone {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageChecklistTasksDone>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageChecklistTasksDone>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageChecklistTasksDone>>::from(self).delete_message()
    }
}
impl MessageConnectedWebsite {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageConnectedWebsite>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageConnectedWebsite>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageConnectedWebsite>>::from(self).delete_message()
    }
}
impl MessageContact {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageContact>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageContact>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageContact>>::from(self).delete_message()
    }
}
impl MessageDeleteChatPhoto {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageDeleteChatPhoto>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageDeleteChatPhoto>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageDeleteChatPhoto>>::from(self).delete_message()
    }
}
impl MessageDice {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageDice>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageDice>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageDice>>::from(self).delete_message()
    }
}
impl MessageDirectMessagePriceChanged {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageDirectMessagePriceChanged>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageDirectMessagePriceChanged>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageDirectMessagePriceChanged>>::from(self).delete_message()
    }
}
impl MessageDocument {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageDocument>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageDocument>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageDocument>>::from(self).delete_message()
    }
}
impl MessageForumTopicClosed {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageForumTopicClosed>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageForumTopicClosed>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageForumTopicClosed>>::from(self).delete_message()
    }
}
impl MessageForumTopicCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageForumTopicCreated>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageForumTopicCreated>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageForumTopicCreated>>::from(self).delete_message()
    }
}
impl MessageForumTopicEdited {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageForumTopicEdited>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageForumTopicEdited>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageForumTopicEdited>>::from(self).delete_message()
    }
}
impl MessageForumTopicReopened {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageForumTopicReopened>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageForumTopicReopened>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageForumTopicReopened>>::from(self).delete_message()
    }
}
impl MessageGame {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGame>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGame>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGame>>::from(self).delete_message()
    }
}
impl MessageGeneralForumTopicHidden {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGeneralForumTopicHidden>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGeneralForumTopicHidden>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGeneralForumTopicHidden>>::from(self).delete_message()
    }
}
impl MessageGeneralForumTopicUnhidden {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGeneralForumTopicUnhidden>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGeneralForumTopicUnhidden>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGeneralForumTopicUnhidden>>::from(self).delete_message()
    }
}
impl MessageGift {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGift>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGift>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGift>>::from(self).delete_message()
    }
}
impl MessageGiftUpgradeSent {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGiftUpgradeSent>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGiftUpgradeSent>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGiftUpgradeSent>>::from(self).delete_message()
    }
}
impl MessageGiveaway {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGiveaway>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGiveaway>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGiveaway>>::from(self).delete_message()
    }
}
impl MessageGiveawayCompleted {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGiveawayCompleted>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGiveawayCompleted>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGiveawayCompleted>>::from(self).delete_message()
    }
}
impl MessageGiveawayCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGiveawayCreated>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGiveawayCreated>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGiveawayCreated>>::from(self).delete_message()
    }
}
impl MessageGiveawayWinners {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGiveawayWinners>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGiveawayWinners>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGiveawayWinners>>::from(self).delete_message()
    }
}
impl MessageGroupChatCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageGroupChatCreated>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageGroupChatCreated>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageGroupChatCreated>>::from(self).delete_message()
    }
}
impl MessageInvoice {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageInvoice>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageInvoice>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageInvoice>>::from(self).delete_message()
    }
}
impl MessageLeftChatMember {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageLeftChatMember>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageLeftChatMember>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageLeftChatMember>>::from(self).delete_message()
    }
}
impl MessageLocation {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageLocation>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageLocation>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageLocation>>::from(self).delete_message()
    }
}
impl MessageMessageAutoDeleteTimerChanged {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageMessageAutoDeleteTimerChanged>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageMessageAutoDeleteTimerChanged>>::from(self)
            .to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageMessageAutoDeleteTimerChanged>>::from(self).delete_message()
    }
}
impl MessageMigrateFromChatId {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageMigrateFromChatId>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageMigrateFromChatId>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageMigrateFromChatId>>::from(self).delete_message()
    }
}
impl MessageMigrateToChatId {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageMigrateToChatId>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageMigrateToChatId>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageMigrateToChatId>>::from(self).delete_message()
    }
}
impl MessageNewChatMembers {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageNewChatMembers>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageNewChatMembers>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageNewChatMembers>>::from(self).delete_message()
    }
}
impl MessageNewChatPhoto {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageNewChatPhoto>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageNewChatPhoto>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageNewChatPhoto>>::from(self).delete_message()
    }
}
impl MessageNewChatTitle {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageNewChatTitle>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageNewChatTitle>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageNewChatTitle>>::from(self).delete_message()
    }
}
impl MessagePaidMedia {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessagePaidMedia>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessagePaidMedia>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessagePaidMedia>>::from(self).delete_message()
    }
}
impl MessagePaidMessagePriceChanged {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessagePaidMessagePriceChanged>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessagePaidMessagePriceChanged>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessagePaidMessagePriceChanged>>::from(self).delete_message()
    }
}
impl MessagePassportData {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessagePassportData>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessagePassportData>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessagePassportData>>::from(self).delete_message()
    }
}
impl MessagePhoto {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessagePhoto>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessagePhoto>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessagePhoto>>::from(self).delete_message()
    }
}
impl MessagePinnedMessage {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessagePinnedMessage>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessagePinnedMessage>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessagePinnedMessage>>::from(self).delete_message()
    }
}
impl MessagePoll {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessagePoll>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessagePoll>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessagePoll>>::from(self).delete_message()
    }
}
impl MessageProximityAlertTriggered {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageProximityAlertTriggered>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageProximityAlertTriggered>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageProximityAlertTriggered>>::from(self).delete_message()
    }
}
impl MessageRefundedPayment {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageRefundedPayment>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageRefundedPayment>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageRefundedPayment>>::from(self).delete_message()
    }
}
impl MessageSticker {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageSticker>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageSticker>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageSticker>>::from(self).delete_message()
    }
}
impl MessageStory {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageStory>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageStory>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageStory>>::from(self).delete_message()
    }
}
impl MessageSuccessfulPayment {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageSuccessfulPayment>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageSuccessfulPayment>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageSuccessfulPayment>>::from(self).delete_message()
    }
}
impl MessageSuggestedPostApprovalFailed {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageSuggestedPostApprovalFailed>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageSuggestedPostApprovalFailed>>::from(self)
            .to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageSuggestedPostApprovalFailed>>::from(self).delete_message()
    }
}
impl MessageSuggestedPostApproved {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageSuggestedPostApproved>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageSuggestedPostApproved>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageSuggestedPostApproved>>::from(self).delete_message()
    }
}
impl MessageSuggestedPostDeclined {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageSuggestedPostDeclined>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageSuggestedPostDeclined>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageSuggestedPostDeclined>>::from(self).delete_message()
    }
}
impl MessageSuggestedPostPaid {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageSuggestedPostPaid>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageSuggestedPostPaid>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageSuggestedPostPaid>>::from(self).delete_message()
    }
}
impl MessageSuggestedPostRefunded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageSuggestedPostRefunded>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageSuggestedPostRefunded>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageSuggestedPostRefunded>>::from(self).delete_message()
    }
}
impl MessageSupergroupChatCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageSupergroupChatCreated>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageSupergroupChatCreated>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageSupergroupChatCreated>>::from(self).delete_message()
    }
}
impl MessageText {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageText>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageText>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageText>>::from(self).delete_message()
    }
}
impl MessageUniqueGift {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageUniqueGift>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageUniqueGift>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageUniqueGift>>::from(self).delete_message()
    }
}
impl MessageUsersShared {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageUsersShared>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageUsersShared>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageUsersShared>>::from(self).delete_message()
    }
}
impl MessageVenue {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageVenue>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageVenue>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageVenue>>::from(self).delete_message()
    }
}
impl MessageVideo {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageVideo>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageVideo>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageVideo>>::from(self).delete_message()
    }
}
impl MessageVideoChatEnded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageVideoChatEnded>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageVideoChatEnded>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageVideoChatEnded>>::from(self).delete_message()
    }
}
impl MessageVideoChatParticipantsInvited {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageVideoChatParticipantsInvited>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageVideoChatParticipantsInvited>>::from(self)
            .to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageVideoChatParticipantsInvited>>::from(self).delete_message()
    }
}
impl MessageVideoChatScheduled {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageVideoChatScheduled>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageVideoChatScheduled>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageVideoChatScheduled>>::from(self).delete_message()
    }
}
impl MessageVideoChatStarted {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageVideoChatStarted>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageVideoChatStarted>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageVideoChatStarted>>::from(self).delete_message()
    }
}
impl MessageVideoNote {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageVideoNote>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageVideoNote>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageVideoNote>>::from(self).delete_message()
    }
}
impl MessageVoice {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageVoice>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageVoice>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageVoice>>::from(self).delete_message()
    }
}
impl MessageWebAppData {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageWebAppData>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageWebAppData>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageWebAppData>>::from(self).delete_message()
    }
}
impl MessageWriteAccessAllowed {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(self, chat_id: T) -> CopyMessage {
        <Message as From<MessageWriteAccessAllowed>>::from(self).to_copy_message(chat_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(self, chat_id: T) -> ForwardMessage {
        <Message as From<MessageWriteAccessAllowed>>::from(self).to_forward_message(chat_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(self) -> DeleteMessage {
        <Message as From<MessageWriteAccessAllowed>>::from(self).delete_message()
    }
}
