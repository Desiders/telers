use super::extractor::FromEventAndContext;
use crate::{
    client::Bot,
    types::{
        CallbackQuery, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery,
        Message, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, Update,
    },
};

use futures::future::{ok, Ready};
use std::convert::Infallible;

impl FromEventAndContext for Message {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(Message::from(update.clone()))
    }
}

impl FromEventAndContext for CallbackQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(CallbackQuery::from(update.clone()))
    }
}

impl FromEventAndContext for ChosenInlineResult {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(ChosenInlineResult::from(update.clone()))
    }
}

impl FromEventAndContext for ShippingQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(ShippingQuery::from(update.clone()))
    }
}

impl FromEventAndContext for PreCheckoutQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(PreCheckoutQuery::from(update.clone()))
    }
}

impl FromEventAndContext for PollAnswer {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(PollAnswer::from(update.clone()))
    }
}

impl FromEventAndContext for ChatMemberUpdated {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(ChatMemberUpdated::from(update.clone()))
    }
}

impl FromEventAndContext for ChatJoinRequest {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(ChatJoinRequest::from(update.clone()))
    }
}

impl FromEventAndContext for InlineQuery {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(InlineQuery::from(update.clone()))
    }
}

impl FromEventAndContext for Poll {
    type Error = Infallible;
    type Future = Ready<Result<Self, Self::Error>>;

    fn extract(_: &Bot, update: &Update) -> Self::Future {
        ok(Poll::from(update.clone()))
    }
}
