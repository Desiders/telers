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
    #[strum(serialize = "update")]
    Update,
}

impl Telegram {
    #[must_use]
    pub const fn all() -> [Telegram; 16] {
        [
            Telegram::Message,
            Telegram::InlineQuery,
            Telegram::ChosenInlineResult,
            Telegram::CallbackQuery,
            Telegram::ChannelPost,
            Telegram::EditedMessage,
            Telegram::EditedChannelPost,
            Telegram::MessageReaction,
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

impl From<Telegram> for Box<str> {
    fn from(scope: Telegram) -> Self {
        scope.into()
    }
}

impl From<Telegram> for String {
    fn from(scope: Telegram) -> Self {
        scope.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for Telegram {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
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

impl From<Simple> for Box<str> {
    fn from(scope: Simple) -> Self {
        Into::<&'static str>::into(scope).into()
    }
}

impl<'a> PartialEq<&'a str> for Simple {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
