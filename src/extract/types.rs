use crate::{
    client::Bot,
    context::Context,
    extract::FromEventAndContext,
    types::{
        CallbackQuery, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery,
        Message, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, Update,
    },
};

use futures::future::{ok, Ready};
use std::{convert::Infallible, sync::Arc};

/// To be able to use [`Bot`] in handler arguments
impl FromEventAndContext for Bot {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(bot: Arc<Bot>, _: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok((*bot).clone())
    }
}

/// To be able to use [`Arc<Bot>`] in handler arguments
impl FromEventAndContext for Arc<Bot> {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(bot: Arc<Bot>, _: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(bot)
    }
}

/// To be able to use [`Update`] in handler arguments
impl FromEventAndContext for Update {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok((*update).clone())
    }
}

/// To be able to use [`Arc<Update>`] in handler arguments
impl FromEventAndContext for Arc<Update> {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(update)
    }
}

/// To be able to use [`Context`] in handler arguments
impl FromEventAndContext for Arc<Context> {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, _: Arc<Update>, context: Arc<Context>) -> Self::Future {
        ok(context)
    }
}

/// To be able to use [`Message`] in handler arguments
impl FromEventAndContext for Message {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(Message::from((*update).clone()))
    }
}

/// To be able to use [`CallbackQuery`] in handler arguments
impl FromEventAndContext for CallbackQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(CallbackQuery::from((*update).clone()))
    }
}

/// To be able to use [`ChosenInlineResult`] in handler arguments
impl FromEventAndContext for ChosenInlineResult {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(ChosenInlineResult::from((*update).clone()))
    }
}

/// To be able to use [`ShippingQuery`] in handler arguments
impl FromEventAndContext for ShippingQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(ShippingQuery::from((*update).clone()))
    }
}

/// To be able to use [`PreCheckoutQuery`] in handler arguments
impl FromEventAndContext for PreCheckoutQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(PreCheckoutQuery::from((*update).clone()))
    }
}

/// To be able to use [`PollAnswer`] in handler arguments
impl FromEventAndContext for PollAnswer {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(PollAnswer::from((*update).clone()))
    }
}

/// To be able to use [`ChatMemberUpdated`] in handler arguments
impl FromEventAndContext for ChatMemberUpdated {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(ChatMemberUpdated::from((*update).clone()))
    }
}

/// To be able to use [`ChatJoinRequest`] in handler arguments
impl FromEventAndContext for ChatJoinRequest {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(ChatJoinRequest::from((*update).clone()))
    }
}

/// To be able to use [`InlineQuery`] in handler arguments
impl FromEventAndContext for InlineQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(InlineQuery::from((*update).clone()))
    }
}

/// To be able to use [`Poll`] in handler arguments
impl FromEventAndContext for Poll {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: Arc<Bot>, update: Arc<Update>, _: Arc<Context>) -> Self::Future {
        ok(Poll::from((*update).clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dispatcher::event::telegram::handler::Handler;

    #[test]
    fn test_types_extract() {
        fn assert_impl_handler<T: FromEventAndContext>(_: impl Handler<T>) {}

        assert_impl_handler(|_: Bot| async { unreachable!() });
        assert_impl_handler(|_: Arc<Bot>| async { unreachable!() });
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
}
