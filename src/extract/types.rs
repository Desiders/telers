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
use std::{cell::RefCell, convert::Infallible, rc::Rc};

/// To be able to use [`Bot`] in [Handler]'s arguments
impl FromEventAndContext for Bot {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(bot: &Bot, _: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(bot.clone())
    }
}

/// To be able to use [`Update`] in [Handler]'s arguments
impl FromEventAndContext for Update {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(update.clone())
    }
}

/// To be able to use [`Context`] in [Handler]'s arguments
impl FromEventAndContext for Rc<RefCell<Context>> {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, _: &Update, context: Rc<RefCell<Context>>) -> Self::Future {
        ok(Rc::clone(&context))
    }
}

/// To be able to use [`Message`] in [Handler]'s arguments
impl FromEventAndContext for Message {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(Message::from(update.clone()))
    }
}

/// To be able to use [`CallbackQuery`] in [`Handler`]'s arguments
impl FromEventAndContext for CallbackQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(CallbackQuery::from(update.clone()))
    }
}

/// To be able to use [`ChosenInlineResult`] in [Handler]'s arguments
impl FromEventAndContext for ChosenInlineResult {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(ChosenInlineResult::from(update.clone()))
    }
}

/// To be able to use [`ShippingQuery`] in [Handler]'s arguments
impl FromEventAndContext for ShippingQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(ShippingQuery::from(update.clone()))
    }
}

/// To be able to use [`PreCheckoutQuery`] in [Handler]'s arguments
impl FromEventAndContext for PreCheckoutQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(PreCheckoutQuery::from(update.clone()))
    }
}

/// To be able to use [`PollAnswer`] in [Handler]'s arguments
impl FromEventAndContext for PollAnswer {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(PollAnswer::from(update.clone()))
    }
}

/// To be able to use [`ChatMemberUpdated`] in [Handler]'s arguments
impl FromEventAndContext for ChatMemberUpdated {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(ChatMemberUpdated::from(update.clone()))
    }
}

/// To be able to use [`ChatJoinRequest`] in [Handler]'s arguments
impl FromEventAndContext for ChatJoinRequest {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(ChatJoinRequest::from(update.clone()))
    }
}

/// To be able to use [`InlineQuery`] in [Handler]'s arguments
impl FromEventAndContext for InlineQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(InlineQuery::from(update.clone()))
    }
}

/// To be able to use [`Poll`] in [Handler]'s arguments
impl FromEventAndContext for Poll {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update, _: Rc<RefCell<Context>>) -> Self::Future {
        ok(Poll::from(update.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::dispatcher::event::TelegramHandler;

    #[test]
    fn test_types_extract() {
        fn assert_impl_handler<T: FromEventAndContext>(_: impl TelegramHandler<T>) {}

        assert_impl_handler(|_: Bot| async { unimplemented!() });
        assert_impl_handler(|_: Update| async { unimplemented!() });
        assert_impl_handler(|_: Rc<RefCell<Context>>| async { unimplemented!() });
        assert_impl_handler(|_: Message| async { unimplemented!() });
        assert_impl_handler(|_: CallbackQuery| async { unimplemented!() });
        assert_impl_handler(|_: ChosenInlineResult| async { unimplemented!() });
        assert_impl_handler(|_: ShippingQuery| async { unimplemented!() });
        assert_impl_handler(|_: PreCheckoutQuery| async { unimplemented!() });
        assert_impl_handler(|_: PollAnswer| async { unimplemented!() });
        assert_impl_handler(|_: ChatMemberUpdated| async { unimplemented!() });
        assert_impl_handler(|_: ChatJoinRequest| async { unimplemented!() });
        assert_impl_handler(|_: InlineQuery| async { unimplemented!() });
        assert_impl_handler(|_: Poll| async { unimplemented!() });
    }
}
