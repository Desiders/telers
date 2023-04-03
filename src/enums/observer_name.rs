use std::fmt::{self, Debug};

/// Enums, which are used to identify default [telegram observers](`crate::dispatcher::event::telegram::observer::Observer`).
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Telegram {
    Message,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ChannelPost,
    EditedMessage,
    EditedChannelPost,
    ShippingQuery,
    PreCheckoutQuery,
    Poll,
    PollAnswer,
    MyChatMember,
    ChatMember,
    ChatJoinRequest,
    Update,
}

impl Debug for Telegram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Telegram {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Telegram::Message => "message",
            Telegram::InlineQuery => "inline_query",
            Telegram::ChosenInlineResult => "chosen_inline_result",
            Telegram::CallbackQuery => "callback_query",
            Telegram::ChannelPost => "channel_post",
            Telegram::EditedMessage => "edited_message",
            Telegram::EditedChannelPost => "edited_channel_post",
            Telegram::ShippingQuery => "shipping_query",
            Telegram::PreCheckoutQuery => "pre_checkout_query",
            Telegram::Poll => "poll",
            Telegram::PollAnswer => "poll_answer",
            Telegram::MyChatMember => "my_chat_member",
            Telegram::ChatMember => "chat_member",
            Telegram::ChatJoinRequest => "chat_join_request",
            Telegram::Update => "update",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Telegram; 15] {
        &[
            Telegram::Message,
            Telegram::InlineQuery,
            Telegram::ChosenInlineResult,
            Telegram::CallbackQuery,
            Telegram::ChannelPost,
            Telegram::EditedMessage,
            Telegram::EditedChannelPost,
            Telegram::ShippingQuery,
            Telegram::PreCheckoutQuery,
            Telegram::Poll,
            Telegram::PollAnswer,
            Telegram::MyChatMember,
            Telegram::ChatMember,
            Telegram::ChatJoinRequest,
            Telegram::Update,
        ]
    }
}

impl From<Telegram> for String {
    fn from(scope: Telegram) -> Self {
        scope.as_str().to_string()
    }
}

/// Enums, which are used to identify default [simple observers](`crate::dispatcher::event::simple::observer::Observer`).
pub enum Simple {
    Startup,
    Shutdown,
}

impl Simple {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Simple::Startup => "startup",
            Simple::Shutdown => "shutdown",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [Simple; 2] {
        &[Simple::Startup, Simple::Shutdown]
    }
}

impl From<Simple> for String {
    fn from(scope: Simple) -> Self {
        scope.as_str().to_string()
    }
}
