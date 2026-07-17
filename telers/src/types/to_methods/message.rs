use crate::{
    methods::{CopyMessage, DeleteMessage, ForwardMessage},
    types::{
        ChatIdKind, Message, MessageAnimation, MessageAudio, MessageBoostAdded,
        MessageChannelChatCreated, MessageChatBackgroundSet, MessageChatOwnerChanged,
        MessageChatOwnerLeft, MessageChatShared, MessageChecklist, MessageChecklistTasksAdded,
        MessageChecklistTasksDone, MessageCommunityChatAdded, MessageCommunityChatRemoved,
        MessageConnectedWebsite, MessageContact, MessageDeleteChatPhoto, MessageDice,
        MessageDirectMessagePriceChanged, MessageDocument, MessageForumTopicClosed,
        MessageForumTopicCreated, MessageForumTopicEdited, MessageForumTopicReopened, MessageGame,
        MessageGeneralForumTopicHidden, MessageGeneralForumTopicUnhidden, MessageGift,
        MessageGiftUpgradeSent, MessageGiveaway, MessageGiveawayCompleted, MessageGiveawayCreated,
        MessageGiveawayWinners, MessageGroupChatCreated, MessageInvoice, MessageLeftChatMember,
        MessageLivePhoto, MessageLocation, MessageManagedBotCreated,
        MessageMessageAutoDeleteTimerChanged, MessageMigrateFromChatId, MessageMigrateToChatId,
        MessageNewChatMembers, MessageNewChatPhoto, MessageNewChatTitle, MessagePaidMedia,
        MessagePaidMessagePriceChanged, MessagePassportData, MessagePhoto, MessagePinnedMessage,
        MessagePoll, MessagePollOptionAdded, MessagePollOptionDeleted,
        MessageProximityAlertTriggered, MessageRefundedPayment, MessageRichMessage, MessageSticker,
        MessageStory, MessageSuccessfulPayment, MessageSuggestedPostApprovalFailed,
        MessageSuggestedPostApproved, MessageSuggestedPostDeclined, MessageSuggestedPostPaid,
        MessageSuggestedPostRefunded, MessageSupergroupChatCreated, MessageText, MessageUniqueGift,
        MessageUsersShared, MessageVenue, MessageVideo, MessageVideoChatEnded,
        MessageVideoChatParticipantsInvited, MessageVideoChatScheduled, MessageVideoChatStarted,
        MessageVideoNote, MessageVoice, MessageWebAppData, MessageWriteAccessAllowed,
    },
    utils::text::Renderer,
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

    /// Renders the message text and its entities as an HTML string, if the message has text.
    #[must_use]
    pub fn html_text(&self) -> Option<String> {
        self.text()
            .map(|text| Renderer::new(text, self.entities().unwrap_or(&[])).as_html())
    }

    /// Renders the message text and its entities as a MarkdownV2 string, if the message has text.
    #[must_use]
    pub fn markdown_text(&self) -> Option<String> {
        self.text()
            .map(|text| Renderer::new(text, self.entities().unwrap_or(&[])).as_markdown())
    }

    /// Renders the message caption and its entities as an HTML string, if the message has a caption.
    #[must_use]
    pub fn html_caption(&self) -> Option<String> {
        self.caption()
            .map(|caption| Renderer::new(caption, self.caption_entities().unwrap_or(&[])).as_html())
    }

    /// Renders the message caption and its entities as a MarkdownV2 string, if the message has a caption.
    #[must_use]
    pub fn markdown_caption(&self) -> Option<String> {
        self.caption().map(|caption| {
            Renderer::new(caption, self.caption_entities().unwrap_or(&[])).as_markdown()
        })
    }
}
impl MessageAnimation {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageLivePhoto {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageVenue {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageAudio {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageBoostAdded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageChannelChatCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageChatBackgroundSet {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageChatOwnerChanged {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageChatOwnerLeft {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageChatShared {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageChecklist {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageChecklistTasksAdded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageChecklistTasksDone {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageCommunityChatAdded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageCommunityChatRemoved {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageConnectedWebsite {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageContact {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageDeleteChatPhoto {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageDice {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageDirectMessagePriceChanged {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageDocument {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageForumTopicClosed {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageForumTopicCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageForumTopicEdited {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageForumTopicReopened {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGame {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGeneralForumTopicHidden {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGeneralForumTopicUnhidden {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGift {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGiftUpgradeSent {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGiveaway {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGiveawayCompleted {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGiveawayCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGiveawayWinners {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageGroupChatCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageInvoice {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageLeftChatMember {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageLocation {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageManagedBotCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageMessageAutoDeleteTimerChanged {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageMigrateFromChatId {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageMigrateToChatId {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageNewChatMembers {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageNewChatPhoto {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageNewChatTitle {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessagePaidMedia {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessagePaidMessagePriceChanged {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessagePassportData {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessagePhoto {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessagePinnedMessage {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessagePoll {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessagePollOptionAdded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessagePollOptionDeleted {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageProximityAlertTriggered {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageRefundedPayment {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageRichMessage {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageSticker {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageStory {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageSuccessfulPayment {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageSuggestedPostApprovalFailed {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageSuggestedPostApproved {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageSuggestedPostDeclined {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageSuggestedPostPaid {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageSuggestedPostRefunded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageSupergroupChatCreated {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageText {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageUniqueGift {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageUsersShared {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageVideo {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageVideoChatEnded {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageVideoChatParticipantsInvited {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageVideoChatScheduled {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageVideoChatStarted {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageVideoNote {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageVoice {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageWebAppData {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
impl MessageWriteAccessAllowed {
    /// Creates [`CopyMessage`] for this message.
    #[must_use]
    pub fn to_copy_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`ForwardMessage`] for this message.
    #[must_use]
    pub fn to_forward_message<T: Into<ChatIdKind>>(&self, chat_id: T) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat.id(), self.message_id)
    }

    /// Creates [`DeleteMessage`] for this message.
    #[must_use]
    pub fn delete_message(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat.id(), self.message_id)
    }
}
