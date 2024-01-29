use super::FromEventAndContext;

use crate::{client::Bot, context::Context, types::Update};

use std::{convert::Infallible, sync::Arc};

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        errors::{ConvertToTypeError, ExtractionError},
        types::{
            CallbackQuery, ChatBoostRemoved, ChatBoostUpdated, ChatJoinRequest, ChatMemberUpdated,
            ChosenInlineResult, InlineQuery, Message, MessageAnimation, MessageAudio,
            MessageChannelChatCreated, MessageChatShared, MessageConnectedWebsite, MessageContact,
            MessageDeleteChatPhoto, MessageDice, MessageDocument, MessageForumTopicClosed,
            MessageForumTopicCreated, MessageForumTopicEdited, MessageForumTopicReopened,
            MessageGame, MessageGeneralForumTopicHidden, MessageGeneralForumTopicUnhidden,
            MessageGiveaway, MessageGiveawayCompleted, MessageGiveawayCreated,
            MessageGiveawayWinners, MessageGroupChatCreated, MessageInvoice, MessageLeftChatMember,
            MessageLocation, MessageMessageAutoDeleteTimerChanged, MessageMigrateFromChat,
            MessageMigrateToChat, MessageNewChatMembers, MessageNewChatPhoto, MessageNewChatTitle,
            MessagePassportData, MessagePhoto, MessagePinned, MessagePoll,
            MessageProximityAlertTriggered, MessageReactionCountUpdated, MessageReactionUpdated,
            MessageSticker, MessageStory, MessageSuccessfulPayment, MessageSupergroupChatCreated,
            MessageText, MessageUsersShared, MessageVenue, MessageVideo, MessageVideoChatEnded,
            MessageVideoChatParticipantsInvited, MessageVideoChatScheduled,
            MessageVideoChatStarted, MessageVideoNote, MessageVoice, MessageWebAppData,
            MessageWriteAccessAllowed, Poll, PollAnswer, PollQuiz, PollRegular, PreCheckoutQuery,
            ShippingQuery, UpdateKind,
        },
    };

    #[allow(unreachable_code)]
    fn _check_bounds<Client, T: FromEventAndContext<Client>>() {
        unimplemented!("This function is only used for checking bounds");

        _check_bounds::<_, Bot>();
        _check_bounds::<_, Arc<Bot>>();
        _check_bounds::<Client, Update>();
        _check_bounds::<Client, Arc<Update>>();
        _check_bounds::<Client, Arc<Context>>();
        _check_bounds::<Client, UpdateKind>();

        // Message-related bounds
        _check_bounds::<Client, Message>();
        _check_bounds::<Client, MessageText>();
        _check_bounds::<Client, MessageAnimation>();
        _check_bounds::<Client, MessageAudio>();
        _check_bounds::<Client, MessageChannelChatCreated>();
        _check_bounds::<Client, MessageUsersShared>();
        _check_bounds::<Client, MessageChatShared>();
        _check_bounds::<Client, MessageConnectedWebsite>();
        _check_bounds::<Client, MessageContact>();
        _check_bounds::<Client, MessageDeleteChatPhoto>();
        _check_bounds::<Client, MessageDice>();
        _check_bounds::<Client, MessageDocument>();
        _check_bounds::<Client, MessageForumTopicClosed>();
        _check_bounds::<Client, MessageForumTopicCreated>();
        _check_bounds::<Client, MessageForumTopicEdited>();
        _check_bounds::<Client, MessageForumTopicReopened>();
        _check_bounds::<Client, MessageGame>();
        _check_bounds::<Client, MessageGeneralForumTopicHidden>();
        _check_bounds::<Client, MessageGeneralForumTopicUnhidden>();
        _check_bounds::<Client, MessageGroupChatCreated>();
        _check_bounds::<Client, MessageInvoice>();
        _check_bounds::<Client, MessageLeftChatMember>();
        _check_bounds::<Client, MessageLocation>();
        _check_bounds::<Client, MessageMessageAutoDeleteTimerChanged>();
        _check_bounds::<Client, MessageMigrateFromChat>();
        _check_bounds::<Client, MessageMigrateToChat>();
        _check_bounds::<Client, MessageNewChatMembers>();
        _check_bounds::<Client, MessageNewChatPhoto>();
        _check_bounds::<Client, MessageNewChatTitle>();
        _check_bounds::<Client, MessagePassportData>();
        _check_bounds::<Client, MessagePhoto>();
        _check_bounds::<Client, MessagePinned>();
        _check_bounds::<Client, MessagePoll>();
        _check_bounds::<Client, MessageProximityAlertTriggered>();
        _check_bounds::<Client, MessageSticker>();
        _check_bounds::<Client, MessageStory>();
        _check_bounds::<Client, MessageSuccessfulPayment>();
        _check_bounds::<Client, MessageSupergroupChatCreated>();
        _check_bounds::<Client, MessageVenue>();
        _check_bounds::<Client, MessageVideo>();
        _check_bounds::<Client, MessageVideoChatEnded>();
        _check_bounds::<Client, MessageVideoChatParticipantsInvited>();
        _check_bounds::<Client, MessageVideoChatScheduled>();
        _check_bounds::<Client, MessageVideoChatStarted>();
        _check_bounds::<Client, MessageVideoNote>();
        _check_bounds::<Client, MessageVoice>();
        _check_bounds::<Client, MessageWebAppData>();
        _check_bounds::<Client, MessageWriteAccessAllowed>();
        _check_bounds::<Client, MessageGiveawayCreated>();
        _check_bounds::<Client, MessageGiveaway>();
        _check_bounds::<Client, MessageGiveawayCompleted>();
        _check_bounds::<Client, MessageGiveawayWinners>();
        _check_bounds::<Client, MessageUsersShared>();

        _check_bounds::<Client, MessageReactionUpdated>();
        _check_bounds::<Client, MessageReactionCountUpdated>();
        _check_bounds::<Client, CallbackQuery>();
        _check_bounds::<Client, ChosenInlineResult>();
        _check_bounds::<Client, ShippingQuery>();
        _check_bounds::<Client, PreCheckoutQuery>();
        _check_bounds::<Client, PollAnswer>();
        _check_bounds::<Client, ChatMemberUpdated>();
        _check_bounds::<Client, ChatJoinRequest>();
        _check_bounds::<Client, InlineQuery>();

        // Poll-related bounds
        _check_bounds::<Client, Poll>();
        _check_bounds::<Client, PollRegular>();
        _check_bounds::<Client, PollQuiz>();

        _check_bounds::<Client, ChatBoostUpdated>();
        _check_bounds::<Client, ChatBoostRemoved>();
    }

    #[allow(unreachable_code)]
    fn _check_bounds_option<Client, T: FromEventAndContext<Client>>() {
        unimplemented!("This function is only used for checking bounds");

        _check_bounds::<_, Option<Bot>>();
        _check_bounds::<_, Option<Arc<Bot>>>();
        _check_bounds::<Client, Option<Update>>();
        _check_bounds::<Client, Option<Arc<Update>>>();
        _check_bounds::<Client, Option<Arc<Context>>>();
        _check_bounds::<Client, Option<UpdateKind>>();

        // Message-related bounds
        _check_bounds::<Client, Option<Message>>();
        _check_bounds::<Client, Option<MessageText>>();
        _check_bounds::<Client, Option<MessageAnimation>>();
        _check_bounds::<Client, Option<MessageAudio>>();
        _check_bounds::<Client, Option<MessageChannelChatCreated>>();
        _check_bounds::<Client, Option<MessageUsersShared>>();
        _check_bounds::<Client, Option<MessageChatShared>>();
        _check_bounds::<Client, Option<MessageConnectedWebsite>>();
        _check_bounds::<Client, Option<MessageContact>>();
        _check_bounds::<Client, Option<MessageDeleteChatPhoto>>();
        _check_bounds::<Client, Option<MessageDice>>();
        _check_bounds::<Client, Option<MessageDocument>>();
        _check_bounds::<Client, Option<MessageForumTopicClosed>>();
        _check_bounds::<Client, Option<MessageForumTopicCreated>>();
        _check_bounds::<Client, Option<MessageForumTopicEdited>>();
        _check_bounds::<Client, Option<MessageForumTopicReopened>>();
        _check_bounds::<Client, Option<MessageGame>>();
        _check_bounds::<Client, Option<MessageGeneralForumTopicHidden>>();
        _check_bounds::<Client, Option<MessageGeneralForumTopicUnhidden>>();
        _check_bounds::<Client, Option<MessageGroupChatCreated>>();
        _check_bounds::<Client, Option<MessageInvoice>>();
        _check_bounds::<Client, Option<MessageLeftChatMember>>();
        _check_bounds::<Client, Option<MessageLocation>>();
        _check_bounds::<Client, Option<MessageMessageAutoDeleteTimerChanged>>();
        _check_bounds::<Client, Option<MessageMigrateFromChat>>();
        _check_bounds::<Client, Option<MessageMigrateToChat>>();
        _check_bounds::<Client, Option<MessageNewChatMembers>>();
        _check_bounds::<Client, Option<MessageNewChatPhoto>>();
        _check_bounds::<Client, Option<MessageNewChatTitle>>();
        _check_bounds::<Client, Option<MessagePassportData>>();
        _check_bounds::<Client, Option<MessagePhoto>>();
        _check_bounds::<Client, Option<MessagePinned>>();
        _check_bounds::<Client, Option<MessagePoll>>();
        _check_bounds::<Client, Option<MessageProximityAlertTriggered>>();
        _check_bounds::<Client, Option<MessageSticker>>();
        _check_bounds::<Client, Option<MessageStory>>();
        _check_bounds::<Client, Option<MessageSuccessfulPayment>>();
        _check_bounds::<Client, Option<MessageSupergroupChatCreated>>();
        _check_bounds::<Client, Option<MessageVenue>>();
        _check_bounds::<Client, Option<MessageVideo>>();
        _check_bounds::<Client, Option<MessageVideoChatEnded>>();
        _check_bounds::<Client, Option<MessageVideoChatParticipantsInvited>>();
        _check_bounds::<Client, Option<MessageVideoChatScheduled>>();
        _check_bounds::<Client, Option<MessageVideoChatStarted>>();
        _check_bounds::<Client, Option<MessageVideoNote>>();
        _check_bounds::<Client, Option<MessageVoice>>();
        _check_bounds::<Client, Option<MessageWebAppData>>();
        _check_bounds::<Client, Option<MessageWriteAccessAllowed>>();
        _check_bounds::<Client, Option<MessageGiveawayCreated>>();
        _check_bounds::<Client, Option<MessageGiveaway>>();
        _check_bounds::<Client, Option<MessageGiveawayCompleted>>();
        _check_bounds::<Client, Option<MessageGiveawayWinners>>();
        _check_bounds::<Client, Option<MessageUsersShared>>();

        _check_bounds::<Client, Option<MessageReactionUpdated>>();
        _check_bounds::<Client, Option<MessageReactionCountUpdated>>();

        _check_bounds::<Client, Option<CallbackQuery>>();
        _check_bounds::<Client, Option<ChosenInlineResult>>();
        _check_bounds::<Client, Option<ShippingQuery>>();
        _check_bounds::<Client, Option<PreCheckoutQuery>>();
        _check_bounds::<Client, Option<PollAnswer>>();
        _check_bounds::<Client, Option<ChatMemberUpdated>>();
        _check_bounds::<Client, Option<ChatJoinRequest>>();
        _check_bounds::<Client, Option<InlineQuery>>();

        // Poll-related bounds
        _check_bounds::<Client, Option<Poll>>();
        _check_bounds::<Client, Option<PollRegular>>();
        _check_bounds::<Client, Option<PollQuiz>>();

        _check_bounds::<Client, Option<ChatBoostUpdated>>();
        _check_bounds::<Client, Option<ChatBoostRemoved>>();
    }

    #[allow(unreachable_code)]
    fn _check_bounds_result<Client, T: FromEventAndContext<Client>, Err: Into<ExtractionError>>() {
        unimplemented!("This function is only used for checking bounds");

        _check_bounds::<_, Result<Bot, Infallible>>();
        _check_bounds::<_, Result<Arc<Bot>, Infallible>>();
        _check_bounds::<Client, Result<Update, Infallible>>();
        _check_bounds::<Client, Result<Arc<Update>, Infallible>>();
        _check_bounds::<Client, Result<Arc<Context>, Infallible>>();

        // Message-related bounds
        _check_bounds::<Client, Result<Message, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageText, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageAnimation, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageAudio, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageChannelChatCreated, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageUsersShared, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageChatShared, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageConnectedWebsite, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageContact, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageDeleteChatPhoto, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageDice, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageDocument, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageForumTopicClosed, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageForumTopicCreated, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageForumTopicEdited, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageForumTopicReopened, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageGame, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageGeneralForumTopicHidden, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageGeneralForumTopicUnhidden, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageGroupChatCreated, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageInvoice, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageLeftChatMember, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageLocation, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageMessageAutoDeleteTimerChanged, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageMigrateFromChat, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageMigrateToChat, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageNewChatMembers, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageNewChatPhoto, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageNewChatTitle, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessagePassportData, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessagePhoto, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessagePinned, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessagePoll, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageProximityAlertTriggered, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageSticker, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageStory, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageSuccessfulPayment, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageSupergroupChatCreated, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageVenue, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageVideo, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageVideoChatEnded, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageVideoChatParticipantsInvited, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageVideoChatScheduled, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageVideoChatStarted, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageVideoNote, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageVoice, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageWebAppData, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageWriteAccessAllowed, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageGiveawayCreated, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageGiveaway, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageGiveawayCompleted, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageGiveawayWinners, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageUsersShared, ConvertToTypeError>>();

        _check_bounds::<Client, Result<MessageReactionUpdated, ConvertToTypeError>>();
        _check_bounds::<Client, Result<MessageReactionCountUpdated, ConvertToTypeError>>();
        _check_bounds::<Client, Result<CallbackQuery, ConvertToTypeError>>();
        _check_bounds::<Client, Result<ChosenInlineResult, ConvertToTypeError>>();
        _check_bounds::<Client, Result<ShippingQuery, ConvertToTypeError>>();
        _check_bounds::<Client, Result<PreCheckoutQuery, ConvertToTypeError>>();
        _check_bounds::<Client, Result<PollAnswer, ConvertToTypeError>>();
        _check_bounds::<Client, Result<ChatMemberUpdated, ConvertToTypeError>>();
        _check_bounds::<Client, Result<ChatJoinRequest, ConvertToTypeError>>();
        _check_bounds::<Client, Result<InlineQuery, ConvertToTypeError>>();

        // Poll-related bounds
        _check_bounds::<Client, Result<Poll, ConvertToTypeError>>();
        _check_bounds::<Client, Result<PollRegular, ConvertToTypeError>>();
        _check_bounds::<Client, Result<PollQuiz, ConvertToTypeError>>();

        _check_bounds::<Client, Result<ChatBoostUpdated, ConvertToTypeError>>();
        _check_bounds::<Client, Result<ChatBoostRemoved, ConvertToTypeError>>();
    }
}
