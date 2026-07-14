use crate::types::Update;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object represents an incoming update.
/// At most one of the optional fields can be present in any given update.
/// Currently, it can be one of
/// - [`crate::types::UpdateBusinessConnection`]
/// - [`crate::types::UpdateBusinessMessage`]
/// - [`crate::types::UpdateCallbackQuery`]
/// - [`crate::types::UpdateChannelPost`]
/// - [`crate::types::UpdateChatBoost`]
/// - [`crate::types::UpdateChatJoinRequest`]
/// - [`crate::types::UpdateChatMember`]
/// - [`crate::types::UpdateChosenInlineResult`]
/// - [`crate::types::UpdateDeletedBusinessMessages`]
/// - [`crate::types::UpdateEditedBusinessMessage`]
/// - [`crate::types::UpdateEditedChannelPost`]
/// - [`crate::types::UpdateEditedMessage`]
/// - [`crate::types::UpdateGuestMessage`]
/// - [`crate::types::UpdateInlineQuery`]
/// - [`crate::types::UpdateManagedBot`]
/// - [`crate::types::UpdateMessage`]
/// - [`crate::types::UpdateMessageReaction`]
/// - [`crate::types::UpdateMessageReactionCount`]
/// - [`crate::types::UpdateMyChatMember`]
/// - [`crate::types::UpdatePoll`]
/// - [`crate::types::UpdatePollAnswer`]
/// - [`crate::types::UpdatePreCheckoutQuery`]
/// - [`crate::types::UpdatePurchasedPaidMedia`]
/// - [`crate::types::UpdateRemovedChatBoost`]
/// - [`crate::types::UpdateShippingQuery`]
/// - [`crate::types::UpdateSubscription`]
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum UpdateType {
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
    #[strum(serialize = "guest_message")]
    GuestMessage,
    #[strum(serialize = "inline_query")]
    InlineQuery,
    #[strum(serialize = "managed_bot")]
    ManagedBot,
    #[strum(serialize = "message")]
    Message,
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
    #[strum(serialize = "subscription")]
    Subscription,
}
impl UpdateType {
    #[must_use]
    pub const fn all() -> [UpdateType; 26usize] {
        [
            UpdateType::BusinessConnection,
            UpdateType::BusinessMessage,
            UpdateType::CallbackQuery,
            UpdateType::ChannelPost,
            UpdateType::ChatBoost,
            UpdateType::ChatJoinRequest,
            UpdateType::ChatMember,
            UpdateType::ChosenInlineResult,
            UpdateType::DeletedBusinessMessages,
            UpdateType::EditedBusinessMessage,
            UpdateType::EditedChannelPost,
            UpdateType::EditedMessage,
            UpdateType::GuestMessage,
            UpdateType::InlineQuery,
            UpdateType::ManagedBot,
            UpdateType::Message,
            UpdateType::MessageReaction,
            UpdateType::MessageReactionCount,
            UpdateType::MyChatMember,
            UpdateType::Poll,
            UpdateType::PollAnswer,
            UpdateType::PreCheckoutQuery,
            UpdateType::PurchasedPaidMedia,
            UpdateType::RemovedChatBoost,
            UpdateType::ShippingQuery,
            UpdateType::Subscription,
        ]
    }
}
impl From<UpdateType> for Box<str> {
    fn from(val: UpdateType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<UpdateType> for String {
    fn from(val: UpdateType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for UpdateType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a Update> for UpdateType {
    fn from(val: &'a Update) -> Self {
        match val {
            Update::BusinessConnection(_) => UpdateType::BusinessConnection,
            Update::BusinessMessage(_) => UpdateType::BusinessMessage,
            Update::CallbackQuery(_) => UpdateType::CallbackQuery,
            Update::ChannelPost(_) => UpdateType::ChannelPost,
            Update::ChatBoost(_) => UpdateType::ChatBoost,
            Update::ChatJoinRequest(_) => UpdateType::ChatJoinRequest,
            Update::ChatMember(_) => UpdateType::ChatMember,
            Update::ChosenInlineResult(_) => UpdateType::ChosenInlineResult,
            Update::DeletedBusinessMessages(_) => UpdateType::DeletedBusinessMessages,
            Update::EditedBusinessMessage(_) => UpdateType::EditedBusinessMessage,
            Update::EditedChannelPost(_) => UpdateType::EditedChannelPost,
            Update::EditedMessage(_) => UpdateType::EditedMessage,
            Update::GuestMessage(_) => UpdateType::GuestMessage,
            Update::InlineQuery(_) => UpdateType::InlineQuery,
            Update::ManagedBot(_) => UpdateType::ManagedBot,
            Update::Message(_) => UpdateType::Message,
            Update::MessageReaction(_) => UpdateType::MessageReaction,
            Update::MessageReactionCount(_) => UpdateType::MessageReactionCount,
            Update::MyChatMember(_) => UpdateType::MyChatMember,
            Update::Poll(_) => UpdateType::Poll,
            Update::PollAnswer(_) => UpdateType::PollAnswer,
            Update::PreCheckoutQuery(_) => UpdateType::PreCheckoutQuery,
            Update::PurchasedPaidMedia(_) => UpdateType::PurchasedPaidMedia,
            Update::RemovedChatBoost(_) => UpdateType::RemovedChatBoost,
            Update::ShippingQuery(_) => UpdateType::ShippingQuery,
            Update::Subscription(_) => UpdateType::Subscription,
        }
    }
}
impl From<Update> for UpdateType {
    fn from(val: Update) -> Self {
        UpdateType::from(&val)
    }
}
