use super::{from_update, try_from_update, FromEventAndContext};

use crate::{
    client::Bot,
    context::Context,
    errors::ConvertToTypeError,
    types::{
        CallbackQuery, ChatBoostRemoved, ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated,
        ChosenInlineResult, InlineQuery, Message, MessageAnimation, MessageAudio,
        MessageChannelChatCreated, MessageChatShared, MessageConnectedWebsite, MessageContact,
        MessageDeleteChatPhoto, MessageDice, MessageDocument, MessageEmpty,
        MessageForumTopicClosed, MessageForumTopicCreated, MessageForumTopicEdited,
        MessageForumTopicReopened, MessageGame, MessageGeneralForumTopicHidden,
        MessageGeneralForumTopicUnhidden, MessageGiveaway, MessageGiveawayCompleted,
        MessageGiveawayCreated, MessageGiveawayWinners, MessageGroupChatCreated, MessageInvoice,
        MessageLeftChatMember, MessageLocation, MessageMessageAutoDeleteTimerChanged,
        MessageMigrateFromChat, MessageMigrateToChat, MessageNewChatMembers, MessageNewChatPhoto,
        MessageNewChatTitle, MessagePassportData, MessagePhoto, MessagePinned, MessagePoll,
        MessageProximityAlertTriggered, MessageReactionCountUpdated, MessageReactionUpdated,
        MessageSticker, MessageStory, MessageSuccessfulPayment, MessageSupergroupChatCreated,
        MessageText, MessageUsersShared, MessageVenue, MessageVideo, MessageVideoChatEnded,
        MessageVideoChatParticipantsInvited, MessageVideoChatScheduled, MessageVideoChatStarted,
        MessageVideoNote, MessageVoice, MessageWebAppData, MessageWriteAccessAllowed, Poll,
        PollAnswer, PollQuiz, PollRegular, PreCheckoutQuery, ShippingQuery, Update, UpdateKind,
    },
};

use std::{convert::Infallible, sync::Arc};

/// To be able to use [`Bot`] in handler arguments,
/// this implementation will clone [`Bot`] and return it
impl<Client: Clone> FromEventAndContext<Client> for Bot<Client> {
    type Error = Infallible;

    fn extract(
        bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok((*bot).clone())
    }
}

/// To be able to use [`Arc<Bot>`] in handler arguments,
/// this implementation will return [`Arc<Bot>`] without cloning [`Bot`] itself
impl<Client> FromEventAndContext<Client> for Arc<Bot<Client>> {
    type Error = Infallible;

    fn extract(
        bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(bot)
    }
}

/// To be able to use [`Update`] in handler arguments,
/// this implementation will clone [`Update`] and return it
impl<Client> FromEventAndContext<Client> for Update {
    type Error = Infallible;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok((*update).clone())
    }
}

/// To be able to use [`Arc<Update>`] in handler arguments,
/// this implementation will return [`Arc<Update>`] without cloning [`Update`] itself
impl<Client> FromEventAndContext<Client> for Arc<Update> {
    type Error = Infallible;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(update)
    }
}

/// To be able to use [`Arc<Context>`] in handler arguments,
/// this implementation will return [`Arc<Context>`] without cloning [`Context`] itself
/// # Note
/// Currently implementation of [`FromEventAndContext`] for [`Arc<Context>`] is required,
/// because [`Context`] can't be cloned directly, so [`Arc<Context>`] is used instead
impl<Client> FromEventAndContext<Client> for Arc<Context> {
    type Error = Infallible;

    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(context)
    }
}

// To be able to use [`UpdateKind`] in handler arguments
from_update!([Client], UpdateKind);

// To be able to use [`Message`] and all variants in handler arguments
try_from_update!([Client], Message);
try_from_update!([Client], MessageText);
try_from_update!([Client], MessageAnimation);
try_from_update!([Client], MessageAudio);
try_from_update!([Client], MessageChannelChatCreated);
try_from_update!([Client], MessageUsersShared);
try_from_update!([Client], MessageChatShared);
try_from_update!([Client], MessageConnectedWebsite);
try_from_update!([Client], MessageContact);
try_from_update!([Client], MessageDeleteChatPhoto);
try_from_update!([Client], MessageDice);
try_from_update!([Client], MessageDocument);
try_from_update!([Client], MessageForumTopicClosed);
try_from_update!([Client], MessageForumTopicCreated);
try_from_update!([Client], MessageForumTopicEdited);
try_from_update!([Client], MessageForumTopicReopened);
try_from_update!([Client], MessageGame);
try_from_update!([Client], MessageGeneralForumTopicHidden);
try_from_update!([Client], MessageGeneralForumTopicUnhidden);
try_from_update!([Client], MessageGroupChatCreated);
try_from_update!([Client], MessageInvoice);
try_from_update!([Client], MessageLeftChatMember);
try_from_update!([Client], MessageLocation);
try_from_update!([Client], MessageMessageAutoDeleteTimerChanged);
try_from_update!([Client], MessageMigrateFromChat);
try_from_update!([Client], MessageMigrateToChat);
try_from_update!([Client], MessageNewChatMembers);
try_from_update!([Client], MessageNewChatPhoto);
try_from_update!([Client], MessageNewChatTitle);
try_from_update!([Client], MessagePassportData);
try_from_update!([Client], MessagePhoto);
try_from_update!([Client], MessagePinned);
try_from_update!([Client], MessagePoll);
try_from_update!([Client], MessageProximityAlertTriggered);
try_from_update!([Client], MessageSticker);
try_from_update!([Client], MessageStory);
try_from_update!([Client], MessageSuccessfulPayment);
try_from_update!([Client], MessageSupergroupChatCreated);
try_from_update!([Client], MessageVenue);
try_from_update!([Client], MessageVideo);
try_from_update!([Client], MessageVideoChatEnded);
try_from_update!([Client], MessageVideoChatParticipantsInvited);
try_from_update!([Client], MessageVideoChatScheduled);
try_from_update!([Client], MessageVideoChatStarted);
try_from_update!([Client], MessageVideoNote);
try_from_update!([Client], MessageVoice);
try_from_update!([Client], MessageWebAppData);
try_from_update!([Client], MessageWriteAccessAllowed);
try_from_update!([Client], MessageGiveawayCreated);
try_from_update!([Client], MessageGiveaway);
try_from_update!([Client], MessageGiveawayCompleted);
try_from_update!([Client], MessageGiveawayWinners);
try_from_update!([Client], MessageReactionUpdated);
try_from_update!([Client], MessageReactionCountUpdated);
try_from_update!([Client], MessageEmpty);

// To be able to use [`Poll`] and all [`PollKind`] variants in handler arguments
try_from_update!([Client], Poll);
try_from_update!([Client], PollRegular);
try_from_update!([Client], PollQuiz);

// To be able to use [`CallbackQuery`] in handler arguments
try_from_update!([Client], CallbackQuery);

// To be able to use [`ChosenInlineResult`] in handler arguments
try_from_update!([Client], ChosenInlineResult);

// To be able to use [`ShippingQuery`] in handler arguments
try_from_update!([Client], ShippingQuery);

// To be able to use [`PreCheckoutQuery`] in handler arguments
try_from_update!([Client], PreCheckoutQuery);

// To be able to use [`PollAnswer`] in handler arguments
try_from_update!([Client], PollAnswer);

// To be able to use [`ChatMemberUpdated`] in handler arguments
try_from_update!([Client], ChatMemberUpdated);

// To be able to use [`ChatJoinRequest`] in handler arguments
try_from_update!([Client], ChatJoinRequest);

// To be able to use [`InlineQuery`] in handler arguments
try_from_update!([Client], InlineQuery);

// To be able to use [`ChatBoostUpdated`] in handler arguments
try_from_update!([Client], ChatBoostUpdated);

// To be able to use [`ChatBoostRemoved`] in handler arguments
try_from_update!([Client], ChatBoostRemoved);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Reqwest, errors::ConvertToTypeError, event::telegram::handler::Handler};

    #[allow(clippy::needless_pass_by_value)]
    fn inner_extract<T: FromEventAndContext<Reqwest>>(
        bot: Arc<Bot<Reqwest>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<T, T::Error> {
        T::extract(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
    }

    #[test]
    fn test_impl_extract_types() {
        fn assert_impl_handler<T: FromEventAndContext<Reqwest>>(_: impl Handler<T>) {}

        assert_impl_handler(|_: Bot<Reqwest>| async { unreachable!() });
        assert_impl_handler(|_: Arc<Bot<Reqwest>>| async { unreachable!() });
        assert_impl_handler(|_: Update| async { unreachable!() });
        assert_impl_handler(|_: UpdateKind| async { unreachable!() });
        assert_impl_handler(|_: Arc<Update>| async { unreachable!() });
        assert_impl_handler(|_: Arc<Context>| async { unreachable!() });
        assert_impl_handler(|_: Message| async { unreachable!() });
        assert_impl_handler(|_: MessageText| async { unreachable!() });
        assert_impl_handler(|_: MessageAnimation| async { unreachable!() });
        assert_impl_handler(|_: MessageAudio| async { unreachable!() });
        assert_impl_handler(|_: MessageChannelChatCreated| async { unreachable!() });
        assert_impl_handler(|_: MessageUsersShared| async { unreachable!() });
        assert_impl_handler(|_: MessageChatShared| async { unreachable!() });
        assert_impl_handler(|_: MessageConnectedWebsite| async { unreachable!() });
        assert_impl_handler(|_: MessageContact| async { unreachable!() });
        assert_impl_handler(|_: MessageDeleteChatPhoto| async { unreachable!() });
        assert_impl_handler(|_: MessageDice| async { unreachable!() });
        assert_impl_handler(|_: MessageDocument| async { unreachable!() });
        assert_impl_handler(|_: MessageForumTopicClosed| async { unreachable!() });
        assert_impl_handler(|_: MessageForumTopicCreated| async { unreachable!() });
        assert_impl_handler(|_: MessageForumTopicEdited| async { unreachable!() });
        assert_impl_handler(|_: MessageForumTopicReopened| async { unreachable!() });
        assert_impl_handler(|_: MessageGame| async { unreachable!() });
        assert_impl_handler(|_: MessageGeneralForumTopicHidden| async { unreachable!() });
        assert_impl_handler(|_: MessageGeneralForumTopicUnhidden| async { unreachable!() });
        assert_impl_handler(|_: MessageGroupChatCreated| async { unreachable!() });
        assert_impl_handler(|_: MessageInvoice| async { unreachable!() });
        assert_impl_handler(|_: MessageLeftChatMember| async { unreachable!() });
        assert_impl_handler(|_: MessageLocation| async { unreachable!() });
        assert_impl_handler(|_: MessageMessageAutoDeleteTimerChanged| async { unreachable!() });
        assert_impl_handler(|_: MessageMigrateFromChat| async { unreachable!() });
        assert_impl_handler(|_: MessageMigrateToChat| async { unreachable!() });
        assert_impl_handler(|_: MessageNewChatMembers| async { unreachable!() });
        assert_impl_handler(|_: MessageNewChatPhoto| async { unreachable!() });
        assert_impl_handler(|_: MessageNewChatTitle| async { unreachable!() });
        assert_impl_handler(|_: MessagePassportData| async { unreachable!() });
        assert_impl_handler(|_: MessagePhoto| async { unreachable!() });
        assert_impl_handler(|_: MessagePinned| async { unreachable!() });
        assert_impl_handler(|_: MessagePoll| async { unreachable!() });
        assert_impl_handler(|_: MessageProximityAlertTriggered| async { unreachable!() });
        assert_impl_handler(|_: MessageSticker| async { unreachable!() });
        assert_impl_handler(|_: MessageStory| async { unreachable!() });
        assert_impl_handler(|_: MessageSuccessfulPayment| async { unreachable!() });
        assert_impl_handler(|_: MessageSupergroupChatCreated| async { unreachable!() });
        assert_impl_handler(|_: MessageVenue| async { unreachable!() });
        assert_impl_handler(|_: MessageVideo| async { unreachable!() });
        assert_impl_handler(|_: MessageVideoChatEnded| async { unreachable!() });
        assert_impl_handler(|_: MessageVideoChatParticipantsInvited| async { unreachable!() });
        assert_impl_handler(|_: MessageVideoChatScheduled| async { unreachable!() });
        assert_impl_handler(|_: MessageVideoChatStarted| async { unreachable!() });
        assert_impl_handler(|_: MessageVideoNote| async { unreachable!() });
        assert_impl_handler(|_: MessageVoice| async { unreachable!() });
        assert_impl_handler(|_: MessageWebAppData| async { unreachable!() });
        assert_impl_handler(|_: MessageWriteAccessAllowed| async { unreachable!() });
        assert_impl_handler(|_: MessageGiveawayCreated| async { unreachable!() });
        assert_impl_handler(|_: MessageGiveaway| async { unreachable!() });
        assert_impl_handler(|_: MessageGiveawayCompleted| async { unreachable!() });
        assert_impl_handler(|_: MessageGiveawayWinners| async { unreachable!() });
        assert_impl_handler(|_: MessageUsersShared| async { unreachable!() });
        assert_impl_handler(|_: MessageReactionUpdated| async { unreachable!() });
        assert_impl_handler(|_: MessageReactionCountUpdated| async { unreachable!() });
        assert_impl_handler(|_: MessageEmpty| async { unreachable!() });
        assert_impl_handler(|_: Poll| async { unreachable!() });
        assert_impl_handler(|_: PollRegular| async { unreachable!() });
        assert_impl_handler(|_: PollQuiz| async { unreachable!() });
        assert_impl_handler(|_: CallbackQuery| async { unreachable!() });
        assert_impl_handler(|_: ChosenInlineResult| async { unreachable!() });
        assert_impl_handler(|_: ShippingQuery| async { unreachable!() });
        assert_impl_handler(|_: PreCheckoutQuery| async { unreachable!() });
        assert_impl_handler(|_: PollAnswer| async { unreachable!() });
        assert_impl_handler(|_: ChatMemberUpdated| async { unreachable!() });
        assert_impl_handler(|_: ChatJoinRequest| async { unreachable!() });
        assert_impl_handler(|_: InlineQuery| async { unreachable!() });
        assert_impl_handler(|_: ChatBoostUpdated| async { unreachable!() });
        assert_impl_handler(|_: ChatBoostRemoved| async { unreachable!() });
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_extract() {
        let bot = Arc::new(Bot::default());
        let update = Arc::new(Update::default());
        let context = Arc::new(Context::default());

        inner_extract::<Bot<Reqwest>>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap();
        inner_extract::<Arc<Bot<Reqwest>>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap();
        inner_extract::<Update>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap();
        inner_extract::<Arc<Update>>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap();
        inner_extract::<Arc<Context>>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap();

        inner_extract::<Message>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap();
        inner_extract::<CallbackQuery>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap_err();
        inner_extract::<ChosenInlineResult>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap_err();
        inner_extract::<ShippingQuery>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap_err();
        inner_extract::<PreCheckoutQuery>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap_err();
        inner_extract::<PollAnswer>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap_err();
        inner_extract::<ChatMemberUpdated>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap_err();
        inner_extract::<ChatJoinRequest>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap_err();
        inner_extract::<InlineQuery>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap_err();
        inner_extract::<Poll>(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context))
            .unwrap_err();

        assert!(inner_extract::<Option<Message>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .is_some());
        assert!(inner_extract::<Option<CallbackQuery>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .is_none());
        assert!(inner_extract::<Option<ChosenInlineResult>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .is_none());
        assert!(inner_extract::<Option<ShippingQuery>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .is_none());
        assert!(inner_extract::<Option<PreCheckoutQuery>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .is_none());
        assert!(inner_extract::<Option<PollAnswer>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .is_none());
        assert!(inner_extract::<Option<ChatMemberUpdated>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .is_none());
        assert!(inner_extract::<Option<ChatJoinRequest>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .is_none());
        assert!(inner_extract::<Option<InlineQuery>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .is_none());
        assert!(inner_extract::<Option<Poll>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context)
        )
        .unwrap()
        .is_none());

        inner_extract::<Result<Message, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap();
        inner_extract::<Result<CallbackQuery, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<ChosenInlineResult, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<ShippingQuery, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<PreCheckoutQuery, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<PollAnswer, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<ChatMemberUpdated, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<ChatJoinRequest, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<InlineQuery, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<Poll, ConvertToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
    }
}
