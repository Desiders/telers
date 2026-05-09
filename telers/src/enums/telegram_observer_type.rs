use crate::{enums::UpdateType, types::Update};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This enum represents all possible telegram observer types.
/// It contains all [`UpdateType`] variants plus `Update`.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum TelegramObserverType {
    #[strum(serialize = "business_connection")]
    BusinessConnection,
    #[strum(serialize = "business_message")]
    BusinessMessage,
    #[strum(serialize = "callback_query")]
    CallbackQuery,
    #[strum(serialize = "channel_post")]
    ChannelPost,
    #[strum(serialize = "chat_boost")]
    ChatBoost,
    #[strum(serialize = "chat_join_request")]
    ChatJoinRequest,
    #[strum(serialize = "chat_member")]
    ChatMember,
    #[strum(serialize = "chosen_inline_result")]
    ChosenInlineResult,
    #[strum(serialize = "deleted_business_messages")]
    DeletedBusinessMessages,
    #[strum(serialize = "edited_business_message")]
    EditedBusinessMessage,
    #[strum(serialize = "edited_channel_post")]
    EditedChannelPost,
    #[strum(serialize = "edited_message")]
    EditedMessage,
    #[strum(serialize = "inline_query")]
    InlineQuery,
    #[strum(serialize = "managed_bot")]
    ManagedBot,
    #[strum(serialize = "message")]
    Message,
    #[strum(serialize = "guest_message")]
    GuestMessage,
    #[strum(serialize = "message_reaction")]
    MessageReaction,
    #[strum(serialize = "message_reaction_count")]
    MessageReactionCount,
    #[strum(serialize = "my_chat_member")]
    MyChatMember,
    #[strum(serialize = "poll")]
    Poll,
    #[strum(serialize = "poll_answer")]
    PollAnswer,
    #[strum(serialize = "pre_checkout_query")]
    PreCheckoutQuery,
    #[strum(serialize = "purchased_paid_media")]
    PurchasedPaidMedia,
    #[strum(serialize = "removed_chat_boost")]
    RemovedChatBoost,
    #[strum(serialize = "shipping_query")]
    ShippingQuery,
    #[strum(serialize = "update")]
    Update,
}
macro_rules! with_telegram_observer_variants {
    ($callback:ident $(, $args:tt)*) => {
        $callback ! { $($args,)* (BusinessConnection, business_connection),
        (BusinessMessage, business_message), (CallbackQuery, callback_query),
        (ChannelPost, channel_post), (ChatBoost, chat_boost), (ChatJoinRequest,
        chat_join_request), (ChatMember, chat_member), (ChosenInlineResult,
        chosen_inline_result), (DeletedBusinessMessages, deleted_business_messages),
        (EditedBusinessMessage, edited_business_message), (EditedChannelPost,
        edited_channel_post), (EditedMessage, edited_message), (InlineQuery,
        inline_query), (ManagedBot, managed_bot), (Message, message), (GuestMessage,
        guest_message), (MessageReaction, message_reaction), (MessageReactionCount,
        message_reaction_count), (MyChatMember, my_chat_member), (Poll, poll),
        (PollAnswer, poll_answer), (PreCheckoutQuery, pre_checkout_query),
        (PurchasedPaidMedia, purchased_paid_media), (RemovedChatBoost,
        removed_chat_boost), (ShippingQuery, shipping_query), (Update, update), }
    };
}
pub(crate) use with_telegram_observer_variants;
impl TelegramObserverType {
    #[must_use]
    pub const fn all() -> [TelegramObserverType; 26usize] {
        [
            TelegramObserverType::BusinessConnection,
            TelegramObserverType::BusinessMessage,
            TelegramObserverType::CallbackQuery,
            TelegramObserverType::ChannelPost,
            TelegramObserverType::ChatBoost,
            TelegramObserverType::ChatJoinRequest,
            TelegramObserverType::ChatMember,
            TelegramObserverType::ChosenInlineResult,
            TelegramObserverType::DeletedBusinessMessages,
            TelegramObserverType::EditedBusinessMessage,
            TelegramObserverType::EditedChannelPost,
            TelegramObserverType::EditedMessage,
            TelegramObserverType::InlineQuery,
            TelegramObserverType::ManagedBot,
            TelegramObserverType::Message,
            TelegramObserverType::GuestMessage,
            TelegramObserverType::MessageReaction,
            TelegramObserverType::MessageReactionCount,
            TelegramObserverType::MyChatMember,
            TelegramObserverType::Poll,
            TelegramObserverType::PollAnswer,
            TelegramObserverType::PreCheckoutQuery,
            TelegramObserverType::PurchasedPaidMedia,
            TelegramObserverType::RemovedChatBoost,
            TelegramObserverType::ShippingQuery,
            TelegramObserverType::Update,
        ]
    }
}
impl From<TelegramObserverType> for Box<str> {
    fn from(val: TelegramObserverType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<TelegramObserverType> for String {
    fn from(val: TelegramObserverType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for TelegramObserverType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl From<UpdateType> for TelegramObserverType {
    fn from(val: UpdateType) -> Self {
        match val {
            UpdateType::BusinessConnection => TelegramObserverType::BusinessConnection,
            UpdateType::BusinessMessage => TelegramObserverType::BusinessMessage,
            UpdateType::CallbackQuery => TelegramObserverType::CallbackQuery,
            UpdateType::ChannelPost => TelegramObserverType::ChannelPost,
            UpdateType::ChatBoost => TelegramObserverType::ChatBoost,
            UpdateType::ChatJoinRequest => TelegramObserverType::ChatJoinRequest,
            UpdateType::ChatMember => TelegramObserverType::ChatMember,
            UpdateType::ChosenInlineResult => TelegramObserverType::ChosenInlineResult,
            UpdateType::DeletedBusinessMessages => TelegramObserverType::DeletedBusinessMessages,
            UpdateType::EditedBusinessMessage => TelegramObserverType::EditedBusinessMessage,
            UpdateType::EditedChannelPost => TelegramObserverType::EditedChannelPost,
            UpdateType::EditedMessage => TelegramObserverType::EditedMessage,
            UpdateType::InlineQuery => TelegramObserverType::InlineQuery,
            UpdateType::ManagedBot => TelegramObserverType::ManagedBot,
            UpdateType::Message => TelegramObserverType::Message,
            UpdateType::GuestMessage => TelegramObserverType::GuestMessage,
            UpdateType::MessageReaction => TelegramObserverType::MessageReaction,
            UpdateType::MessageReactionCount => TelegramObserverType::MessageReactionCount,
            UpdateType::MyChatMember => TelegramObserverType::MyChatMember,
            UpdateType::Poll => TelegramObserverType::Poll,
            UpdateType::PollAnswer => TelegramObserverType::PollAnswer,
            UpdateType::PreCheckoutQuery => TelegramObserverType::PreCheckoutQuery,
            UpdateType::PurchasedPaidMedia => TelegramObserverType::PurchasedPaidMedia,
            UpdateType::RemovedChatBoost => TelegramObserverType::RemovedChatBoost,
            UpdateType::ShippingQuery => TelegramObserverType::ShippingQuery,
        }
    }
}
impl<'a> From<&'a UpdateType> for TelegramObserverType {
    fn from(val: &'a UpdateType) -> Self {
        TelegramObserverType::from(*val)
    }
}
impl<'a> From<&'a Update> for TelegramObserverType {
    fn from(val: &'a Update) -> Self {
        TelegramObserverType::from(UpdateType::from(val))
    }
}
impl From<Update> for TelegramObserverType {
    fn from(val: Update) -> Self {
        TelegramObserverType::from(&val)
    }
}
