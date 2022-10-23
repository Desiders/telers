use super::{
    CallbackQuery, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message,
    Poll, PollAnswer, PreCheckoutQuery, ShippingQuery,
};

#[derive(Clone, Debug, PartialEq)]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    InlineQuery(InlineQuery),
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
    ShippingQuery(ShippingQuery),
    PreCheckoutQuery(PreCheckoutQuery),
    Poll(Poll),
    PollAnswer(PollAnswer),
    MyChatMember(ChatMemberUpdated),
    ChatMember(ChatMemberUpdated),
    ChatJoinRequest(ChatJoinRequest),
}

impl From<UpdateKind> for Message {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::Message(message)
            | UpdateKind::EditedMessage(message)
            | UpdateKind::ChannelPost(message)
            | UpdateKind::EditedChannelPost(message) => message,
            _ => panic!("Can't convert update to `Message`"),
        }
    }
}

impl From<UpdateKind> for CallbackQuery {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::CallbackQuery(query) => query,
            _ => panic!("Can't convert update to `CallbackQuery`"),
        }
    }
}

impl From<UpdateKind> for InlineQuery {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::InlineQuery(query) => query,
            _ => panic!("Can't convert update to `InlineQuery`"),
        }
    }
}

impl From<UpdateKind> for ChosenInlineResult {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::ChosenInlineResult(result) => result,
            _ => panic!("Can't convert update to `ChosenInlineResult`"),
        }
    }
}

impl From<UpdateKind> for ShippingQuery {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::ShippingQuery(query) => query,
            _ => panic!("Can't convert update to `ShippingQuery`"),
        }
    }
}

impl From<UpdateKind> for PreCheckoutQuery {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::PreCheckoutQuery(query) => query,
            _ => panic!("Can't convert update to `PreCheckoutQuery`"),
        }
    }
}

impl From<UpdateKind> for Poll {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::Poll(poll) => poll,
            _ => panic!("Can't convert update to `Poll`"),
        }
    }
}

impl From<UpdateKind> for PollAnswer {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::PollAnswer(answer) => answer,
            _ => panic!("Can't convert update to `PollAnswer`"),
        }
    }
}

impl From<UpdateKind> for ChatMemberUpdated {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::MyChatMember(member) | UpdateKind::ChatMember(member) => member,
            _ => panic!("Can't convert update to `ChatMemberUpdated`"),
        }
    }
}

impl From<UpdateKind> for ChatJoinRequest {
    fn from(update: UpdateKind) -> Self {
        match update {
            UpdateKind::ChatJoinRequest(request) => request,
            _ => panic!("Can't convert update to `ChatJoinRequest`"),
        }
    }
}
