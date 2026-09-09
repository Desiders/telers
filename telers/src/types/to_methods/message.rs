use crate::{
    methods::{
        CopyMessage, DeleteMessage, EditMessageCaption, EditMessageLiveLocation, EditMessageMedia,
        EditMessageReplyMarkup, EditMessageText, ForwardMessage, PinChatMessage, SendAnimation,
        SendAudio, SendContact, SendDice, SendDocument, SendGame, SendInvoice, SendLocation,
        SendMediaGroup, SendMessage, SendPaidMedia, SendPhoto, SendPoll, SendRichMessage,
        SendSticker, SendVenue, SendVideo, SendVideoNote, SendVoice, SetMessageReaction,
        StopMessageLiveLocation, UnpinChatMessage,
    },
    types::{
        EphemeralMessageParameters, Message, MessageAnimation, MessageAudio, MessageBoostAdded,
        MessageChannelChatCreated, MessageChatBackgroundSet, MessageChatOwnerChanged,
        MessageChatOwnerLeft, MessageChatShared, MessageChecklist, MessageChecklistTasksAdded,
        MessageChecklistTasksDone, MessageCommunityChatAdded, MessageCommunityChatJoined,
        MessageCommunityChatRemoved, MessageConnectedWebsite, MessageContact,
        MessageDeleteChatPhoto, MessageDice, MessageDirectMessagePriceChanged, MessageDocument,
        MessageForumTopicClosed, MessageForumTopicCreated, MessageForumTopicEdited,
        MessageForumTopicReopened, MessageGame, MessageGeneralForumTopicHidden,
        MessageGeneralForumTopicUnhidden, MessageGift, MessageGiftUpgradeSent, MessageGiveaway,
        MessageGiveawayCompleted, MessageGiveawayCreated, MessageGiveawayWinners,
        MessageGroupChatCreated, MessageInvoice, MessageLeftChatMember, MessageLivePhoto,
        MessageLocation, MessageManagedBotCreated, MessageMessageAutoDeleteTimerChanged,
        MessageMigrateFromChatId, MessageMigrateToChatId, MessageNewChatMembers,
        MessageNewChatPhoto, MessageNewChatTitle, MessagePaidMedia, MessagePaidMessagePriceChanged,
        MessagePassportData, MessagePhoto, MessagePinnedMessage, MessagePoll,
        MessagePollOptionAdded, MessagePollOptionDeleted, MessageProximityAlertTriggered,
        MessageRefundedPayment, MessageRichMessage, MessageSticker, MessageStory,
        MessageSuccessfulPayment, MessageSuggestedPostApprovalFailed, MessageSuggestedPostApproved,
        MessageSuggestedPostDeclined, MessageSuggestedPostPaid, MessageSuggestedPostRefunded,
        MessageSupergroupChatCreated, MessageText, MessageUniqueGift, MessageUnknown,
        MessageUsersShared, MessageVenue, MessageVideo, MessageVideoChatEnded,
        MessageVideoChatParticipantsInvited, MessageVideoChatScheduled, MessageVideoChatStarted,
        MessageVideoNote, MessageVoice, MessageWebAppData, MessageWriteAccessAllowed,
        ReplyParameters,
    },
    utils::text::Renderer,
};
/// Shortcuts that create methods for the message with its fields filled,
/// for example [`MessageShortcuts::answer`] creates [`SendMessage`] to the chat of the message.
/// Optional fields of the methods are set with their builders, as usual.
///
/// It's implemented for [`Message`] and its subtypes.
/// The required methods are the fields of the message the shortcuts are built from.
/// # Notes
/// The trait must be in scope to call the shortcuts: `use telers::types::MessageShortcuts as _;`
pub trait MessageShortcuts {
    /// Helper method for field `chat`.
    #[must_use]
    fn chat(&self) -> &crate::types::Chat;
    /// Helper method for field `message_id`.
    #[must_use]
    fn message_id(&self) -> i64;
    /// Helper method for field `message_thread_id`.
    #[must_use]
    fn message_thread_id(&self) -> Option<i64>;
    /// Helper method for field `is_topic_message`.
    #[must_use]
    fn is_topic_message(&self) -> Option<bool>;
    /// Helper method for field `business_connection_id`.
    #[must_use]
    fn business_connection_id(&self) -> Option<&str>;
    /// Helper method for field `ephemeral_message_id`.
    #[must_use]
    fn ephemeral_message_id(&self) -> Option<i64>;
    /// Helper method for field `from`.
    #[must_use]
    fn from(&self) -> Option<&crate::types::User>;
    /// Creates `SendMessage` to the chat of this message.
    #[must_use]
    fn answer<T0: Into<Box<str>>>(&self, text: T0) -> SendMessage {
        SendMessage::new(self.chat().id(), text)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendMessage` to reply to this message.
    #[must_use]
    fn reply<T0: Into<Box<str>>>(&self, text: T0) -> SendMessage {
        SendMessage::new(self.chat().id(), text)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendRichMessage` to the chat of this message.
    #[must_use]
    fn answer_rich<T0: Into<crate::types::InputRichMessage>>(
        &self,
        rich_message: T0,
    ) -> SendRichMessage {
        SendRichMessage::new(self.chat().id(), rich_message)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendRichMessage` to reply to this message.
    #[must_use]
    fn reply_rich<T0: Into<crate::types::InputRichMessage>>(
        &self,
        rich_message: T0,
    ) -> SendRichMessage {
        SendRichMessage::new(self.chat().id(), rich_message)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendAnimation` to the chat of this message.
    #[must_use]
    fn answer_animation<T0: Into<crate::types::InputFile>>(&self, animation: T0) -> SendAnimation {
        SendAnimation::new(self.chat().id(), animation)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendAnimation` to reply to this message.
    #[must_use]
    fn reply_animation<T0: Into<crate::types::InputFile>>(&self, animation: T0) -> SendAnimation {
        SendAnimation::new(self.chat().id(), animation)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendAudio` to the chat of this message.
    #[must_use]
    fn answer_audio<T0: Into<crate::types::InputFile>>(&self, audio: T0) -> SendAudio {
        SendAudio::new(self.chat().id(), audio)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendAudio` to reply to this message.
    #[must_use]
    fn reply_audio<T0: Into<crate::types::InputFile>>(&self, audio: T0) -> SendAudio {
        SendAudio::new(self.chat().id(), audio)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendContact` to the chat of this message.
    #[must_use]
    fn answer_contact<T0: Into<Box<str>>, T1: Into<Box<str>>>(
        &self,
        phone_number: T0,
        first_name: T1,
    ) -> SendContact {
        SendContact::new(self.chat().id(), phone_number, first_name)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendContact` to reply to this message.
    #[must_use]
    fn reply_contact<T0: Into<Box<str>>, T1: Into<Box<str>>>(
        &self,
        phone_number: T0,
        first_name: T1,
    ) -> SendContact {
        SendContact::new(self.chat().id(), phone_number, first_name)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendDocument` to the chat of this message.
    #[must_use]
    fn answer_document<T0: Into<crate::types::InputFile>>(&self, document: T0) -> SendDocument {
        SendDocument::new(self.chat().id(), document)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendDocument` to reply to this message.
    #[must_use]
    fn reply_document<T0: Into<crate::types::InputFile>>(&self, document: T0) -> SendDocument {
        SendDocument::new(self.chat().id(), document)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendGame` to the chat of this message.
    #[must_use]
    fn answer_game<T0: Into<Box<str>>>(&self, game_short_name: T0) -> SendGame {
        SendGame::new(self.chat().id(), game_short_name)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendGame` to reply to this message.
    #[must_use]
    fn reply_game<T0: Into<Box<str>>>(&self, game_short_name: T0) -> SendGame {
        SendGame::new(self.chat().id(), game_short_name)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendInvoice` to the chat of this message.
    #[must_use]
    fn answer_invoice<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<Box<str>>,
        T4Item: Into<crate::types::LabeledPrice>,
        T4: IntoIterator<Item = T4Item>,
    >(
        &self,
        title: T0,
        description: T1,
        payload: T2,
        currency: T3,
        prices: T4,
    ) -> SendInvoice {
        SendInvoice::new(
            self.chat().id(),
            title,
            description,
            payload,
            currency,
            prices,
        )
        .message_thread_id_option(
            self.message_thread_id()
                .filter(|_| self.is_topic_message() == Some(true)),
        )
    }
    /// Creates `SendInvoice` to reply to this message.
    #[must_use]
    fn reply_invoice<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<Box<str>>,
        T4Item: Into<crate::types::LabeledPrice>,
        T4: IntoIterator<Item = T4Item>,
    >(
        &self,
        title: T0,
        description: T1,
        payload: T2,
        currency: T3,
        prices: T4,
    ) -> SendInvoice {
        SendInvoice::new(
            self.chat().id(),
            title,
            description,
            payload,
            currency,
            prices,
        )
        .message_thread_id_option(
            self.message_thread_id()
                .filter(|_| self.is_topic_message() == Some(true)),
        )
        .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendLocation` to the chat of this message.
    #[must_use]
    fn answer_location<T0: Into<f64>, T1: Into<f64>>(
        &self,
        latitude: T0,
        longitude: T1,
    ) -> SendLocation {
        SendLocation::new(self.chat().id(), latitude, longitude)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendLocation` to reply to this message.
    #[must_use]
    fn reply_location<T0: Into<f64>, T1: Into<f64>>(
        &self,
        latitude: T0,
        longitude: T1,
    ) -> SendLocation {
        SendLocation::new(self.chat().id(), latitude, longitude)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendMediaGroup` to the chat of this message.
    #[must_use]
    fn answer_media_group<
        T0Item: Into<crate::types::InputMedia>,
        T0: IntoIterator<Item = T0Item>,
    >(
        &self,
        media: T0,
    ) -> SendMediaGroup {
        SendMediaGroup::new(self.chat().id(), media)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendMediaGroup` to reply to this message.
    #[must_use]
    fn reply_media_group<
        T0Item: Into<crate::types::InputMedia>,
        T0: IntoIterator<Item = T0Item>,
    >(
        &self,
        media: T0,
    ) -> SendMediaGroup {
        SendMediaGroup::new(self.chat().id(), media)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendPhoto` to the chat of this message.
    #[must_use]
    fn answer_photo<T0: Into<crate::types::InputFile>>(&self, photo: T0) -> SendPhoto {
        SendPhoto::new(self.chat().id(), photo)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendPhoto` to reply to this message.
    #[must_use]
    fn reply_photo<T0: Into<crate::types::InputFile>>(&self, photo: T0) -> SendPhoto {
        SendPhoto::new(self.chat().id(), photo)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendPoll` to the chat of this message.
    #[must_use]
    fn answer_poll<
        T0: Into<Box<str>>,
        T1Item: Into<crate::types::InputPollOption>,
        T1: IntoIterator<Item = T1Item>,
    >(
        &self,
        question: T0,
        options: T1,
    ) -> SendPoll {
        SendPoll::new(self.chat().id(), question, options)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendPoll` to reply to this message.
    #[must_use]
    fn reply_poll<
        T0: Into<Box<str>>,
        T1Item: Into<crate::types::InputPollOption>,
        T1: IntoIterator<Item = T1Item>,
    >(
        &self,
        question: T0,
        options: T1,
    ) -> SendPoll {
        SendPoll::new(self.chat().id(), question, options)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendDice` to the chat of this message.
    #[must_use]
    fn answer_dice(&self) -> SendDice {
        SendDice::new(self.chat().id())
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendDice` to reply to this message.
    #[must_use]
    fn reply_dice(&self) -> SendDice {
        SendDice::new(self.chat().id())
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendSticker` to the chat of this message.
    #[must_use]
    fn answer_sticker<T0: Into<crate::types::InputFile>>(&self, sticker: T0) -> SendSticker {
        SendSticker::new(self.chat().id(), sticker)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendSticker` to reply to this message.
    #[must_use]
    fn reply_sticker<T0: Into<crate::types::InputFile>>(&self, sticker: T0) -> SendSticker {
        SendSticker::new(self.chat().id(), sticker)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendVenue` to the chat of this message.
    #[must_use]
    fn answer_venue<T0: Into<f64>, T1: Into<f64>, T2: Into<Box<str>>, T3: Into<Box<str>>>(
        &self,
        latitude: T0,
        longitude: T1,
        title: T2,
        address: T3,
    ) -> SendVenue {
        SendVenue::new(self.chat().id(), latitude, longitude, title, address)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendVenue` to reply to this message.
    #[must_use]
    fn reply_venue<T0: Into<f64>, T1: Into<f64>, T2: Into<Box<str>>, T3: Into<Box<str>>>(
        &self,
        latitude: T0,
        longitude: T1,
        title: T2,
        address: T3,
    ) -> SendVenue {
        SendVenue::new(self.chat().id(), latitude, longitude, title, address)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendVideo` to the chat of this message.
    #[must_use]
    fn answer_video<T0: Into<crate::types::InputFile>>(&self, video: T0) -> SendVideo {
        SendVideo::new(self.chat().id(), video)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendVideo` to reply to this message.
    #[must_use]
    fn reply_video<T0: Into<crate::types::InputFile>>(&self, video: T0) -> SendVideo {
        SendVideo::new(self.chat().id(), video)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendVideoNote` to the chat of this message.
    #[must_use]
    fn answer_video_note<T0: Into<crate::types::InputFile>>(
        &self,
        video_note: T0,
    ) -> SendVideoNote {
        SendVideoNote::new(self.chat().id(), video_note)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendVideoNote` to reply to this message.
    #[must_use]
    fn reply_video_note<T0: Into<crate::types::InputFile>>(&self, video_note: T0) -> SendVideoNote {
        SendVideoNote::new(self.chat().id(), video_note)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendVoice` to the chat of this message.
    #[must_use]
    fn answer_voice<T0: Into<crate::types::InputFile>>(&self, voice: T0) -> SendVoice {
        SendVoice::new(self.chat().id(), voice)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendVoice` to reply to this message.
    #[must_use]
    fn reply_voice<T0: Into<crate::types::InputFile>>(&self, voice: T0) -> SendVoice {
        SendVoice::new(self.chat().id(), voice)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .ephemeral_message_parameters_option(self.as_ephemeral_message_parameters())
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `SendPaidMedia` to the chat of this message.
    #[must_use]
    fn answer_paid_media<
        T0: Into<u16>,
        T1Item: Into<crate::types::InputPaidMedia>,
        T1: IntoIterator<Item = T1Item>,
    >(
        &self,
        star_count: T0,
        media: T1,
    ) -> SendPaidMedia {
        SendPaidMedia::new(self.chat().id(), star_count, media)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
    }
    /// Creates `SendPaidMedia` to reply to this message.
    #[must_use]
    fn reply_paid_media<
        T0: Into<u16>,
        T1Item: Into<crate::types::InputPaidMedia>,
        T1: IntoIterator<Item = T1Item>,
    >(
        &self,
        star_count: T0,
        media: T1,
    ) -> SendPaidMedia {
        SendPaidMedia::new(self.chat().id(), star_count, media)
            .business_connection_id_option(self.business_connection_id())
            .message_thread_id_option(
                self.message_thread_id()
                    .filter(|_| self.is_topic_message() == Some(true)),
            )
            .reply_parameters(self.as_reply_parameters())
    }
    /// Creates `CopyMessage` for this message.
    #[must_use]
    fn copy_to<T0: Into<crate::types::ChatIdKind>>(&self, chat_id: T0) -> CopyMessage {
        CopyMessage::new(chat_id, self.chat().id(), self.message_id())
    }
    /// Creates `ForwardMessage` for this message.
    #[must_use]
    fn forward<T0: Into<crate::types::ChatIdKind>>(&self, chat_id: T0) -> ForwardMessage {
        ForwardMessage::new(chat_id, self.chat().id(), self.message_id())
    }
    /// Creates `EditMessageText` for this message.
    #[must_use]
    fn edit_text(&self) -> EditMessageText {
        EditMessageText::new()
            .business_connection_id_option(self.business_connection_id())
            .chat_id(self.chat().id())
            .message_id(self.message_id())
    }
    /// Creates `EditMessageCaption` for this message.
    #[must_use]
    fn edit_caption(&self) -> EditMessageCaption {
        EditMessageCaption::new()
            .business_connection_id_option(self.business_connection_id())
            .chat_id(self.chat().id())
            .message_id(self.message_id())
    }
    /// Creates `EditMessageMedia` for this message.
    #[must_use]
    fn edit_media<T0: Into<crate::types::InputMedia>>(&self, media: T0) -> EditMessageMedia {
        EditMessageMedia::new(media)
            .business_connection_id_option(self.business_connection_id())
            .chat_id(self.chat().id())
            .message_id(self.message_id())
    }
    /// Creates `EditMessageReplyMarkup` for this message.
    #[must_use]
    fn edit_reply_markup(&self) -> EditMessageReplyMarkup {
        EditMessageReplyMarkup::new()
            .business_connection_id_option(self.business_connection_id())
            .chat_id(self.chat().id())
            .message_id(self.message_id())
    }
    /// Creates `EditMessageLiveLocation` for this message.
    #[must_use]
    fn edit_live_location<T0: Into<f64>, T1: Into<f64>>(
        &self,
        latitude: T0,
        longitude: T1,
    ) -> EditMessageLiveLocation {
        EditMessageLiveLocation::new(latitude, longitude)
            .business_connection_id_option(self.business_connection_id())
            .chat_id(self.chat().id())
            .message_id(self.message_id())
    }
    /// Creates `StopMessageLiveLocation` for this message.
    #[must_use]
    fn stop_live_location(&self) -> StopMessageLiveLocation {
        StopMessageLiveLocation::new()
            .business_connection_id_option(self.business_connection_id())
            .chat_id(self.chat().id())
            .message_id(self.message_id())
    }
    /// Creates `DeleteMessage` for this message.
    #[must_use]
    fn delete(&self) -> DeleteMessage {
        DeleteMessage::new(self.chat().id(), self.message_id())
    }
    /// Creates `PinChatMessage` for this message.
    #[must_use]
    fn pin(&self) -> PinChatMessage {
        PinChatMessage::new(self.chat().id(), self.message_id())
            .business_connection_id_option(self.business_connection_id())
    }
    /// Creates `UnpinChatMessage` for this message.
    #[must_use]
    fn unpin(&self) -> UnpinChatMessage {
        UnpinChatMessage::new(self.chat().id())
            .business_connection_id_option(self.business_connection_id())
            .message_id(self.message_id())
    }
    /// Creates `SetMessageReaction` for this message.
    #[must_use]
    fn react(&self) -> SetMessageReaction {
        SetMessageReaction::new(self.chat().id(), self.message_id())
    }
    /// Creates [`ReplyParameters`] to reply to this message.
    /// # Notes
    /// An ephemeral message is addressed by `ephemeral_message_id`,
    /// because its `message_id` is always 0 and `chat_id` isn't supported for it.
    #[must_use]
    fn as_reply_parameters(&self) -> ReplyParameters {
        match self.ephemeral_message_id() {
            Some(ephemeral_message_id) => {
                ReplyParameters::new().ephemeral_message_id(ephemeral_message_id)
            }
            None => ReplyParameters::new()
                .message_id(self.message_id())
                .chat_id(self.chat().id()),
        }
    }
    /// Creates [`EphemeralMessageParameters`] to reply to this message if it's ephemeral,
    /// because a reply to an ephemeral message must be an ephemeral message too.
    /// # Returns
    /// `None` if the message isn't ephemeral or has no sender to address the reply to
    #[must_use]
    fn as_ephemeral_message_parameters(&self) -> Option<EphemeralMessageParameters> {
        self.ephemeral_message_id()?;
        self.from()
            .map(|from| EphemeralMessageParameters::new(from.id))
    }
}
impl MessageShortcuts for Message {
    fn chat(&self) -> &crate::types::Chat {
        self.chat()
    }

    fn message_id(&self) -> i64 {
        self.message_id()
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id()
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message()
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id()
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from()
    }
}
impl MessageShortcuts for MessageAnimation {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageLivePhoto {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageVenue {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageAudio {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageBoostAdded {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageChannelChatCreated {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageChatBackgroundSet {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageChatOwnerChanged {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageChatOwnerLeft {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageChatShared {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageChecklist {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageChecklistTasksAdded {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageChecklistTasksDone {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageCommunityChatAdded {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageCommunityChatJoined {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageCommunityChatRemoved {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageConnectedWebsite {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageContact {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageDeleteChatPhoto {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageDice {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageDirectMessagePriceChanged {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageDocument {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageForumTopicClosed {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageForumTopicCreated {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageForumTopicEdited {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageForumTopicReopened {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGame {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGeneralForumTopicHidden {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGeneralForumTopicUnhidden {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGift {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGiftUpgradeSent {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGiveaway {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGiveawayCompleted {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGiveawayCreated {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGiveawayWinners {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageGroupChatCreated {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageInvoice {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageLeftChatMember {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageLocation {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageManagedBotCreated {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageMessageAutoDeleteTimerChanged {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageMigrateFromChatId {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageMigrateToChatId {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageNewChatMembers {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageNewChatPhoto {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageNewChatTitle {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessagePaidMedia {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessagePaidMessagePriceChanged {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessagePassportData {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessagePhoto {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessagePinnedMessage {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessagePoll {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessagePollOptionAdded {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessagePollOptionDeleted {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageProximityAlertTriggered {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageRefundedPayment {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageRichMessage {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageSticker {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageStory {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageSuccessfulPayment {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageSuggestedPostApprovalFailed {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageSuggestedPostApproved {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageSuggestedPostDeclined {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageSuggestedPostPaid {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageSuggestedPostRefunded {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageSupergroupChatCreated {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageText {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageUniqueGift {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageUsersShared {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageVideo {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageVideoChatEnded {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageVideoChatParticipantsInvited {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageVideoChatScheduled {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageVideoChatStarted {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageVideoNote {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageVoice {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageWebAppData {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageWriteAccessAllowed {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl MessageShortcuts for MessageUnknown {
    fn chat(&self) -> &crate::types::Chat {
        self.chat.as_ref()
    }

    fn message_id(&self) -> i64 {
        self.message_id
    }

    fn message_thread_id(&self) -> Option<i64> {
        self.message_thread_id
    }

    fn is_topic_message(&self) -> Option<bool> {
        self.is_topic_message
    }

    fn business_connection_id(&self) -> Option<&str> {
        self.business_connection_id.as_deref()
    }

    fn ephemeral_message_id(&self) -> Option<i64> {
        self.ephemeral_message_id
    }

    fn from(&self) -> Option<&crate::types::User> {
        self.from.as_deref()
    }
}
impl Message {
    /// Renders the message text and its entities as an HTML string, if the message has text.
    #[must_use]
    pub fn html_text(&self) -> Option<String> {
        self.text()
            .map(|text| Renderer::new(text, self.entities().unwrap_or(&[])).as_html())
    }

    /// Renders the message text and its entities as a `MarkdownV2` string, if the message has text.
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

    /// Renders the message caption and its entities as a `MarkdownV2` string, if the message has a caption.
    #[must_use]
    pub fn markdown_caption(&self) -> Option<String> {
        self.caption().map(|caption| {
            Renderer::new(caption, self.caption_entities().unwrap_or(&[])).as_markdown()
        })
    }
}
