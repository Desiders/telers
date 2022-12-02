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
}

impl From<Telegram> for &str {
    fn from(observer: Telegram) -> Self {
        match observer {
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
        }
    }
}

pub enum Simple {
    Startup,
    Shutdown,
}

impl From<Simple> for &str {
    fn from(observer: Simple) -> Self {
        match observer {
            Simple::Startup => "startup",
            Simple::Shutdown => "shutdown",
        }
    }
}
