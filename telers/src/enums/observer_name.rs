use crate::enums::UpdateType;

use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// Enums, which are used to identify default [telegram observers](`crate::event::telegram::Observer`)
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum Telegram {
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "inline_query")]
    InlineQuery,
    #[strum(serialize = "chosen_inline_result")]
    ChosenInlineResult,
    #[strum(serialize = "callback_query")]
    CallbackQuery,
    #[strum(serialize = "channel_post")]
    ChannelPost,
    #[strum(serialize = "edited_message")]
    EditedMessage,
    #[strum(serialize = "edited_channel_post")]
    EditedChannelPost,
    #[strum(serialize = "message_reaction")]
    MessageReaction,
    #[strum(serialize = "message_reaction_count")]
    MessageReactionCount,
    #[strum(serialize = "shipping_query")]
    ShippingQuery,
    #[strum(serialize = "pre_checkout_query")]
    PreCheckoutQuery,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "poll_answer")]
    PollAnswer,
    #[strum(serialize = "my_chat_member")]
    MyChatMember,
    #[strum(serialize = "chat_member")]
    ChatMember,
    #[strum(serialize = "chat_join_request")]
    ChatJoinRequest,
    #[strum(serialize = "chat_boost")]
    ChatBoost,
    #[strum(serialize = "removed_chat_boost")]
    RemovedChatBoost,
    #[strum(serialize = "update")]
    Update,
}

impl Telegram {
    #[must_use]
    pub const fn all() -> [Telegram; 19] {
        [
            Telegram::Message,
            Telegram::InlineQuery,
            Telegram::ChosenInlineResult,
            Telegram::CallbackQuery,
            Telegram::ChannelPost,
            Telegram::EditedMessage,
            Telegram::EditedChannelPost,
            Telegram::MessageReaction,
            Telegram::MessageReactionCount,
            Telegram::ShippingQuery,
            Telegram::PreCheckoutQuery,
            Telegram::Poll,
            Telegram::PollAnswer,
            Telegram::MyChatMember,
            Telegram::ChatMember,
            Telegram::ChatJoinRequest,
            Telegram::ChatBoost,
            Telegram::RemovedChatBoost,
            Telegram::Update,
        ]
    }
}

impl From<Telegram> for Option<UpdateType> {
    fn from(val: Telegram) -> Self {
        match val {
            Telegram::Message => Some(UpdateType::Message),
            Telegram::InlineQuery => Some(UpdateType::InlineQuery),
            Telegram::ChosenInlineResult => Some(UpdateType::ChosenInlineResult),
            Telegram::CallbackQuery => Some(UpdateType::CallbackQuery),
            Telegram::ChannelPost => Some(UpdateType::ChannelPost),
            Telegram::EditedMessage => Some(UpdateType::EditedMessage),
            Telegram::EditedChannelPost => Some(UpdateType::EditedChannelPost),
            Telegram::MessageReaction => Some(UpdateType::MessageReaction),
            Telegram::MessageReactionCount => Some(UpdateType::MessageReactionCount),
            Telegram::ShippingQuery => Some(UpdateType::ShippingQuery),
            Telegram::PreCheckoutQuery => Some(UpdateType::PreCheckoutQuery),
            Telegram::Poll => Some(UpdateType::Poll),
            Telegram::PollAnswer => Some(UpdateType::PollAnswer),
            Telegram::MyChatMember => Some(UpdateType::MyChatMember),
            Telegram::ChatMember => Some(UpdateType::ChatMember),
            Telegram::ChatJoinRequest => Some(UpdateType::ChatJoinRequest),
            Telegram::ChatBoost => Some(UpdateType::ChatBoost),
            Telegram::RemovedChatBoost => Some(UpdateType::RemovedChatBoost),
            Telegram::Update => None,
        }
    }
}

impl PartialEq<UpdateType> for Telegram {
    fn eq(&self, other: &UpdateType) -> bool {
        match self {
            Telegram::Message => *other == UpdateType::Message,
            Telegram::InlineQuery => *other == UpdateType::InlineQuery,
            Telegram::ChosenInlineResult => *other == UpdateType::ChosenInlineResult,
            Telegram::CallbackQuery => *other == UpdateType::CallbackQuery,
            Telegram::ChannelPost => *other == UpdateType::ChannelPost,
            Telegram::EditedMessage => *other == UpdateType::EditedMessage,
            Telegram::EditedChannelPost => *other == UpdateType::EditedChannelPost,
            Telegram::MessageReaction => *other == UpdateType::MessageReaction,
            Telegram::MessageReactionCount => *other == UpdateType::MessageReactionCount,
            Telegram::ShippingQuery => *other == UpdateType::ShippingQuery,
            Telegram::PreCheckoutQuery => *other == UpdateType::PreCheckoutQuery,
            Telegram::Poll => *other == UpdateType::Poll,
            Telegram::PollAnswer => *other == UpdateType::PollAnswer,
            Telegram::MyChatMember => *other == UpdateType::MyChatMember,
            Telegram::ChatMember => *other == UpdateType::ChatMember,
            Telegram::ChatJoinRequest => *other == UpdateType::ChatJoinRequest,
            Telegram::ChatBoost => *other == UpdateType::ChatBoost,
            Telegram::RemovedChatBoost => *other == UpdateType::RemovedChatBoost,
            Telegram::Update => false,
        }
    }
}

/// Enums, which are used to identify default [simple observers](`crate::event::simple::observer::Observer`).
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum Simple {
    #[strum(serialize = "startup")]
    Startup,
    #[strum(serialize = "shutdown")]
    Shutdown,
}

impl Simple {
    #[must_use]
    pub const fn all() -> [Simple; 2] {
        [Simple::Startup, Simple::Shutdown]
    }
}
