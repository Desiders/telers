//! This module contains functionality for extracting data from the event and context to the handler arguments.
//!
//! [`FromEventAndContext`] is the main trait which need to be implemented for extracting data.
//! If you want to use your own types as handler arguments, you need to implement this trait for them.
//! By default, this trait is implemented for the most common middlewares, types and filters, so you can use them without any additional actions.
//! The trait also is implemented for `Option<T>`, `Result<T, E>` where `T: FromEventAndContext`,
//! so you can don't implement it for your types if you want to use them as optional or result arguments.
//!
//! # Implementing trait
//!
//! Ways to implement [`FromEventAndContext`] for your own types:
//! * Implement it directly (much boilerplate code, but it's needed for complex types)
//! * Use the [`FromContext`] macro (simple way to implement this for types in a [`Context`] by its key)
//! * Use the [`FromEvent`] macro (simple way to implement this for types in an event, for example, [`Update`])
//!
//! ## Implementing directly
//!
//! Simple example with extracting id from [`Update`]:
//!
//! ```rust
//! use telers::{types::Update, extractors::FromEventAndContext, context::Context, client::Bot};
//! use std::{sync::Arc, convert::Infallible};
//!
//! struct UpdateId(i64);
//!
//! impl FromEventAndContext for UpdateId {
//!  type Error = Infallible;
//!
//!  fn extract(bot: Arc<Bot>, update: Arc<Update>, context: Arc<Context>) -> Result<Self, Self::Error> {
//!   Ok(UpdateId(update.id))
//!  }
//! }
//! ```
//!
//! This example will extract the [`Update`] id to the handler argument.
//! After that, you can use this argument in the handler:
//!
//! ```ignore
//! async fn handler(update_id: UpdateId) {
//!  println!("Update id: {}", id.0);
//! }
//! ```
//!
//! Another example with extracting id of the user who sent the message from [`Update`]:
//!
//! ```rust
//! use telers::{types::Update, extractors::FromEventAndContext, context::Context, client::Bot, errors::ConvertToTypeError};
//! use std::sync::Arc;
//!
//! struct UpdateFromId(i64);
//!
//! impl FromEventAndContext for UpdateFromId {
//!  type Error = ConvertToTypeError; // you can use your own error type, this is just an example
//!
//!  fn extract(bot: Arc<Bot>, update: Arc<Update>, context: Arc<Context>) -> Result<Self, Self::Error> {
//!   match update.from_id() {
//!    Some(from_id) => Ok(UpdateFromId(from_id)),
//!    None => Err(ConvertToTypeError::new("Update", "UpdateFromId")),
//!   }
//!  }
//! }
//! ```
//!
//! In some cases we sure that some data is not none, so in one handler we can use `Option` and in another handler we can use the type directly.
//! After we implemented [`FromEventAndContext`] for our type, we can use it in both cases,
//! because the trait is implemented for `Option<T>` and `Result<T, E>` where `T: FromEventAndContext`:
//!
//! ```rust
//! use telers::{types::Update, extractors::FromEventAndContext, context::Context, client::Bot, errors::ConvertToTypeError};
//! use std::sync::Arc;
//!
//! struct UpdateFromId(i64);
//!
//! impl FromEventAndContext for UpdateFromId {
//!  type Error = ConvertToTypeError; // you can use your own error type, this is just an example
//!
//!  fn extract(bot: Arc<Bot>, update: Arc<Update>, context: Arc<Context>) -> Result<Self, Self::Error> {
//!   match update.from_id() {
//!    Some(from_id) => Ok(UpdateFromId(from_id)),
//!    None => Err(ConvertToTypeError::new("Update", "UpdateFromId")),
//!   }
//!  }
//! }
//! ```
//!
//! After that, you can use this argument in the handlers:
//!
//! ```ignore
//! // Here `from_id` can't be `None` (for example we use filter which checks that `from_id` is not `None`)
//! async fn handler_first(from_id: UpdateFromId) {
//!  println!("Update from id: {}", from_id.0);
//! }
//!
//! // Here `from_id` can be `None`
//! async fn handler_second(from_id: Option<UpdateFromId>) {
//!  if let Some(from_id) = from_id {
//!   println!("Update from id: {}", from_id.0);
//!  }
//! }
//! ```
//!
//! ## Implementing with [`FromEvent`] macro
//!
//! Simple example with extracting id from [`Update`]:
//!
//! ```rust
//! use telers::{types::Update, extractors::FromEvent};
//!
//! #[derive(FromEvent)]
//! #[event(from = Update)]
//! struct UpdateId(i64);
//!
//! // We need to implement `From<Update>` for `UpdateId` by ourselves (this is required by `FromEvent` macro)
//! impl From<Update> for UpdateId {
//!  fn from(update: Update) -> Self {
//!   Self(update.id)
//!  }
//! }
//! ```
//!
//! Here we used `#[event(from = Update)]` attribute to specify the type from which the type will be converted.
//!
//! We also can use `#[event(try_from = "...")]`, but in this case we need to implement [`TryFrom`] for our type instead of [`From`]:
//!
//! ```rust
//! use telers::{types::Update, extractors::FromEvent, errors::ConvertToTypeError};
//!
//! #[derive(FromEvent)]
//! #[event(try_from = Update)] // you can specify [`ConvertToTypeError`] as error type, but it's not necessary, because it's default
//! struct UpdateFromId(i64);
//!
//! impl TryFrom<Update> for UpdateFromId {
//!  type Error = ConvertToTypeError;
//!
//!  fn try_from(update: Update) -> Result<Self, Self::Error> {
//!   match update.from_id() {
//!    Some(id) => Ok(Self(id)),
//!    None => Err(ConvertToTypeError::new("Update", "UpdateFromId")),
//!   }
//!  }
//! }
//! ```
//!
//! By default, the error type is [`ConvertToTypeError`],
//! but you can specify your own error type with `#[event(error = "...")]` attribute:
//!
//! ```rust
//! use telers::{types::Update, extractors::FromEvent};
//! use std::convert::Infallible;
//!
//! #[derive(FromEvent)]
//! #[event(try_from = Update, error = Infallible)]
//! struct UpdateId(i64);
//!
//! impl TryFrom<Update> for UpdateId { // we use `TryFrom` here just for example, you need to use `From` if error is impossible
//!  type Error = Infallible;
//!
//!  fn try_from(update: Update) -> Result<Self, Self::Error> {
//!   Ok(Self(update.id))
//!  }
//! }
//! ```
//!
//! ## Implementing with [`FromContext`] macro
//! todo: add example

pub use crate::{FromContext, FromEvent};

use crate::{
    client::{Bot, Reqwest},
    context::Context,
    errors::ExtractionError,
    types::Update,
};

use std::{convert::Infallible, sync::Arc};

/// Trait for extracting data from [`Update`] and [`Context`] to handlers arguments
pub trait FromEventAndContext<Client = Reqwest>: Sized {
    type Error: Into<ExtractionError>;

    /// Extracts data from [`Update`], [`Context`] and [`Bot`] to handler argument
    /// # Errors
    /// If extraction was unsuccessful
    ///
    /// Possible variants:
    /// * No found data in context by key
    /// * Data in context by key has wrong type. For example, you try to extract `i32` from `String`.
    /// * Custom user error
    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error>;
}

/// To be able to use [`Option`] as handler argument
/// This implementation will return `None` if extraction was unsuccessful, and [`Some(value)`] otherwise
impl<Client, T: FromEventAndContext<Client>> FromEventAndContext<Client> for Option<T> {
    type Error = Infallible;

    #[inline]
    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        match T::extract(bot, update, context) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Ok(None),
        }
    }
}

/// To be able to use [`Result`] as handler argument
/// This implementation will return [`Ok(value)`] if extraction was successful, and [`Err(error)`] otherwise,
/// where `error` is `T::Error` converted to `E`
impl<Client, T, E> FromEventAndContext<Client> for Result<T, E>
where
    T: FromEventAndContext<Client>,
    T::Error: Into<E>,
{
    type Error = Infallible;

    #[inline]
    fn extract(
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(T::extract(bot, update, context).map_err(Into::into))
    }
}

/// To be able to use handler without arguments
/// Handler without arguments will be called with [`()`] argument (unit type)
impl<Client> FromEventAndContext<Client> for () {
    type Error = Infallible;

    #[inline]
    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(())
    }
}

impl<Client: Clone> FromEventAndContext<Client> for Bot<Client> {
    type Error = Infallible;

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn extract(
        _bot: Arc<Bot<Client>>,
        _update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Ok(context)
    }
}

#[allow(non_snake_case)]
mod factory_from_event_and_context {
    //! This module is used to implement [`FromEventAndContext`] for tuple arguments, each of which implements it
    //! If one of the arguments fails to extract, the whole extraction fails, and the error is returned

    use super::{Arc, Bot, Context, ExtractionError, FromEventAndContext, Update};

    macro_rules! factory ({ $($param:ident)* } => {
        impl<Client, $($param: FromEventAndContext<Client>,)*> FromEventAndContext<Client> for ($($param,)*) {
            type Error = ExtractionError;

            #[inline]
            fn extract(bot: Arc<Bot<Client>>, update: Arc<Update>, context: Arc<Context>) -> Result<Self, Self::Error> {
                Ok(($($param::extract(Arc::clone(&bot), Arc::clone(&update), Arc::clone(&context)).map_err(Into::into)?,)*))
            }
        }
    });

    // To be able to extract tuple with 1 arguments
    factory! { A }
    // To be able to extract tuple with 2 arguments
    factory! { A B }
    // To be able to extract tuple with 3 arguments
    factory! { A B C }
    // To be able to extract tuple with 4 arguments
    factory! { A B C D }
    // To be able to extract tuple with 5 arguments
    factory! { A B C D E}
    // To be able to extract tuple with 6 arguments
    factory! { A B C D E F }
    // To be able to extract tuple with 7 arguments
    factory! { A B C D E F G}
    // To be able to extract tuple with 8 arguments
    factory! { A B C D E F G H }
    // To be able to extract tuple with 9 arguments
    factory! { A B C D E F G H I}
    // To be able to extract tuple with 10 arguments
    factory! { A B C D E F G H I J }
    // To be able to extract tuple with 11 arguments
    factory! { A B C D E F G H I J K}
    // To be able to extract tuple with 12 arguments
    factory! { A B C D E F G H I J K L }
    // To be able to extract tuple with 13 arguments
    factory! { A B C D E F G H I J K L M}
    // To be able to extract tuple with 14 arguments
    factory! { A B C D E F G H I J K L M N }
    // To be able to extract tuple with 15 arguments
    factory! { A B C D E F G H I J K L M N O}
    // To be able to extract tuple with 16 arguments
    factory! { A B C D E F G H I J K L M N O P }
    // To be able to extract tuple with 17 arguments
    factory! { A B C D E F G H I J K L M N O P Q}
    // To be able to extract tuple with 18 arguments
    factory! { A B C D E F G H I J K L M N O P Q R }
    // To be able to extract tuple with 19 arguments
    factory! { A B C D E F G H I J K L M N O P Q R S }
    // To be able to extract tuple with 20 arguments
    factory! { A B C D E F G H I J K L M N O P Q R S T }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        errors::ConvertToTypeError,
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

    #[test]
    fn test_arg_number() {
        fn assert_impl_handler<Client, T: FromEventAndContext<Client>>(_: T) {}

        assert_impl_handler::<Reqwest, _>(());
        assert_impl_handler::<Reqwest, _>((
            (), // 1
            (), // 2
            (), // 3
            (), // 4
            (), // 5
            (), // 6
            (), // 7
            (), // 8
            (), // 9
            (), // 10
            (), // 11
            (), // 12
            (), // 13
            (), // 14
            (), // 15
            (), // 16
            (), // 17
            (), // 18
            (), // 19
            (), // 20
        ));
    }

    #[test]
    fn test_unit_extract() {
        let bot = Arc::new(Bot::<Reqwest>::default());
        let update = Arc::new(Update::default());
        let context = Arc::new(Context::default());

        let (): () =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
        let _: Option<()> =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
        let _: Result<(), Infallible> =
            FromEventAndContext::extract(bot.clone(), update.clone(), context.clone()).unwrap();
    }

    #[allow(unreachable_code)]
    fn _check_bounds<Client, T: FromEventAndContext<Client>>() {
        unimplemented!("This function is only used for checking bounds");

        _check_bounds::<Client, ()>();

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

        _check_bounds::<Client, Option<()>>();

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

        _check_bounds::<Client, Result<(), Infallible>>();

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
