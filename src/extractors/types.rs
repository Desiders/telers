use super::FromEventAndContext;

use crate::{
    client::Bot,
    context::Context,
    errors::ConvertUpdateToTypeError,
    types::{
        CallbackQuery, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery,
        Message, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, Update,
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

/// To be able to use [`Message`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`Message`] and clone it
impl<Client> FromEventAndContext<Client> for Message {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Message::try_from((*update).clone())
    }
}

/// To be able to use [`CallbackQuery`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`CallbackQuery`] and clone it
impl<Client> FromEventAndContext<Client> for CallbackQuery {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        CallbackQuery::try_from((*update).clone())
    }
}

/// To be able to use [`ChosenInlineResult`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`ChosenInlineResult`] and clone it
impl<Client> FromEventAndContext<Client> for ChosenInlineResult {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        ChosenInlineResult::try_from((*update).clone())
    }
}

/// To be able to use [`ShippingQuery`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`ShippingQuery`] and clone it
impl<Client> FromEventAndContext<Client> for ShippingQuery {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        ShippingQuery::try_from((*update).clone())
    }
}

/// To be able to use [`PreCheckoutQuery`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`PreCheckoutQuery`] and clone it
impl<Client> FromEventAndContext<Client> for PreCheckoutQuery {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        PreCheckoutQuery::try_from((*update).clone())
    }
}

/// To be able to use [`PollAnswer`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`PollAnswer`] and clone it
impl<Client> FromEventAndContext<Client> for PollAnswer {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        PollAnswer::try_from((*update).clone())
    }
}

/// To be able to use [`ChatMemberUpdated`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`ChatMemberUpdated`] and clone it
impl<Client> FromEventAndContext<Client> for ChatMemberUpdated {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        ChatMemberUpdated::try_from((*update).clone())
    }
}

/// To be able to use [`ChatJoinRequest`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`ChatJoinRequest`] and clone it
impl<Client> FromEventAndContext<Client> for ChatJoinRequest {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        ChatJoinRequest::try_from((*update).clone())
    }
}

/// To be able to use [`InlineQuery`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`InlineQuery`] and clone it
impl<Client> FromEventAndContext<Client> for InlineQuery {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        InlineQuery::try_from((*update).clone())
    }
}

/// To be able to use [`Poll`] in handler arguments,
/// this implementation will try to convert [`Update`] to [`Poll`] and clone it
impl<Client> FromEventAndContext<Client> for Poll {
    type Error = ConvertUpdateToTypeError;

    fn extract(
        _bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        _context: Arc<Context>,
    ) -> Result<Self, Self::Error> {
        Poll::try_from((*update).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{client::Reqwest, event::telegram::handler::Handler};

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
        assert_impl_handler(|_: Arc<Update>| async { unreachable!() });
        assert_impl_handler(|_: Arc<Context>| async { unreachable!() });
        assert_impl_handler(|_: Message| async { unreachable!() });
        assert_impl_handler(|_: CallbackQuery| async { unreachable!() });
        assert_impl_handler(|_: ChosenInlineResult| async { unreachable!() });
        assert_impl_handler(|_: ShippingQuery| async { unreachable!() });
        assert_impl_handler(|_: PreCheckoutQuery| async { unreachable!() });
        assert_impl_handler(|_: PollAnswer| async { unreachable!() });
        assert_impl_handler(|_: ChatMemberUpdated| async { unreachable!() });
        assert_impl_handler(|_: ChatJoinRequest| async { unreachable!() });
        assert_impl_handler(|_: InlineQuery| async { unreachable!() });
        assert_impl_handler(|_: Poll| async { unreachable!() });
    }

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
            .unwrap_err();
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
        .is_none());
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

        inner_extract::<Result<Message, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<CallbackQuery, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<ChosenInlineResult, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<ShippingQuery, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<PreCheckoutQuery, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<PollAnswer, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<ChatMemberUpdated, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<ChatJoinRequest, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<InlineQuery, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
        inner_extract::<Result<Poll, ConvertUpdateToTypeError>>(
            Arc::clone(&bot),
            Arc::clone(&update),
            Arc::clone(&context),
        )
        .unwrap()
        .unwrap_err();
    }
}
