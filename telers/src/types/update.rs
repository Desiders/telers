use serde::{Deserialize, Serialize};
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
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Update {
    BusinessConnection(crate::types::UpdateBusinessConnection),
    BusinessMessage(crate::types::UpdateBusinessMessage),
    CallbackQuery(crate::types::UpdateCallbackQuery),
    ChannelPost(crate::types::UpdateChannelPost),
    ChatBoost(crate::types::UpdateChatBoost),
    ChatJoinRequest(crate::types::UpdateChatJoinRequest),
    ChatMember(crate::types::UpdateChatMember),
    ChosenInlineResult(crate::types::UpdateChosenInlineResult),
    DeletedBusinessMessages(crate::types::UpdateDeletedBusinessMessages),
    EditedBusinessMessage(crate::types::UpdateEditedBusinessMessage),
    EditedChannelPost(crate::types::UpdateEditedChannelPost),
    EditedMessage(crate::types::UpdateEditedMessage),
    GuestMessage(crate::types::UpdateGuestMessage),
    InlineQuery(crate::types::UpdateInlineQuery),
    ManagedBot(crate::types::UpdateManagedBot),
    Message(crate::types::UpdateMessage),
    MessageReaction(crate::types::UpdateMessageReaction),
    MessageReactionCount(crate::types::UpdateMessageReactionCount),
    MyChatMember(crate::types::UpdateMyChatMember),
    Poll(crate::types::UpdatePoll),
    PollAnswer(crate::types::UpdatePollAnswer),
    PreCheckoutQuery(crate::types::UpdatePreCheckoutQuery),
    PurchasedPaidMedia(crate::types::UpdatePurchasedPaidMedia),
    RemovedChatBoost(crate::types::UpdateRemovedChatBoost),
    ShippingQuery(crate::types::UpdateShippingQuery),
    Subscription(crate::types::UpdateSubscription),
}
impl Update {
    /// Helper method for field `business_connection`.
    ///
    /// The bot was connected to or disconnected from a business account, or a user edited an existing connection with the bot
    #[must_use]
    pub fn business_connection(&self) -> Option<&crate::types::BusinessConnection> {
        match self {
            Self::BusinessConnection(val) => Some(&val.business_connection),
            _ => None,
        }
    }

    /// Helper method for field `business_message`.
    ///
    /// New message from a connected business account
    #[must_use]
    pub fn business_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::BusinessMessage(val) => Some(val.business_message.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `callback_query`.
    ///
    /// New incoming callback query
    #[must_use]
    pub fn callback_query(&self) -> Option<&crate::types::CallbackQuery> {
        match self {
            Self::CallbackQuery(val) => Some(&val.callback_query),
            _ => None,
        }
    }

    /// Helper method for field `channel_post`.
    ///
    /// New incoming channel post of any kind - text, photo, sticker, etc.
    #[must_use]
    pub fn channel_post(&self) -> Option<&crate::types::Message> {
        match self {
            Self::ChannelPost(val) => Some(val.channel_post.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `chat_boost`.
    ///
    /// A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    #[must_use]
    pub fn chat_boost(&self) -> Option<&crate::types::ChatBoostUpdated> {
        match self {
            Self::ChatBoost(val) => Some(&val.chat_boost),
            _ => None,
        }
    }

    /// Helper method for field `chat_join_request`.
    ///
    /// A request to join the chat has been sent. The bot must have the `can_invite_users` administrator right in the chat to receive these updates.
    #[must_use]
    pub fn chat_join_request(&self) -> Option<&crate::types::ChatJoinRequest> {
        match self {
            Self::ChatJoinRequest(val) => Some(&val.chat_join_request),
            _ => None,
        }
    }

    /// Helper method for field `chat_member`.
    ///
    /// A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `chat_member` in the list of `allowed_updates` to receive these updates.
    #[must_use]
    pub fn chat_member(&self) -> Option<&crate::types::ChatMemberUpdated> {
        match self {
            Self::ChatMember(val) => Some(&val.chat_member),
            _ => None,
        }
    }

    /// Helper method for field `chosen_inline_result`.
    ///
    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the feedback collecting for details on how to enable these updates for your bot.
    #[must_use]
    pub fn chosen_inline_result(&self) -> Option<&crate::types::ChosenInlineResult> {
        match self {
            Self::ChosenInlineResult(val) => Some(&val.chosen_inline_result),
            _ => None,
        }
    }

    /// Helper method for field `deleted_business_messages`.
    ///
    /// Messages were deleted from a connected business account
    #[must_use]
    pub fn deleted_business_messages(&self) -> Option<&crate::types::BusinessMessagesDeleted> {
        match self {
            Self::DeletedBusinessMessages(val) => Some(&val.deleted_business_messages),
            _ => None,
        }
    }

    /// Helper method for field `edited_business_message`.
    ///
    /// New version of a message from a connected business account
    #[must_use]
    pub fn edited_business_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::EditedBusinessMessage(val) => Some(val.edited_business_message.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `edited_channel_post`.
    ///
    /// New version of a channel post that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[must_use]
    pub fn edited_channel_post(&self) -> Option<&crate::types::Message> {
        match self {
            Self::EditedChannelPost(val) => Some(val.edited_channel_post.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `edited_message`.
    ///
    /// New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[must_use]
    pub fn edited_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::EditedMessage(val) => Some(val.edited_message.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `guest_message`.
    ///
    /// New guest message. The bot can use the field Message.guest_query_id and the method answerGuestQuery to send a message in response.
    #[must_use]
    pub fn guest_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::GuestMessage(val) => Some(val.guest_message.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `inline_query`.
    ///
    /// New incoming inline query
    #[must_use]
    pub fn inline_query(&self) -> Option<&crate::types::InlineQuery> {
        match self {
            Self::InlineQuery(val) => Some(&val.inline_query),
            _ => None,
        }
    }

    /// Helper method for field `managed_bot`.
    ///
    /// A new bot was created to be managed by the bot, or token or owner of a managed bot was changed
    #[must_use]
    pub fn managed_bot(&self) -> Option<&crate::types::ManagedBotUpdated> {
        match self {
            Self::ManagedBot(val) => Some(&val.managed_bot),
            _ => None,
        }
    }

    /// Helper method for field `message`.
    ///
    /// New incoming message of any kind - text, photo, sticker, etc.
    #[must_use]
    pub fn message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::Message(val) => Some(val.message.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `message_reaction`.
    ///
    /// A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `message_reaction` in the list of `allowed_updates` to receive these updates. The update isn't received for reactions set by bots.
    #[must_use]
    pub fn message_reaction(&self) -> Option<&crate::types::MessageReactionUpdated> {
        match self {
            Self::MessageReaction(val) => Some(&val.message_reaction),
            _ => None,
        }
    }

    /// Helper method for field `message_reaction_count`.
    ///
    /// Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `message_reaction_count` in the list of `allowed_updates` to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
    #[must_use]
    pub fn message_reaction_count(&self) -> Option<&crate::types::MessageReactionCountUpdated> {
        match self {
            Self::MessageReactionCount(val) => Some(&val.message_reaction_count),
            _ => None,
        }
    }

    /// Helper method for field `my_chat_member`.
    ///
    /// The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[must_use]
    pub fn my_chat_member(&self) -> Option<&crate::types::ChatMemberUpdated> {
        match self {
            Self::MyChatMember(val) => Some(&val.my_chat_member),
            _ => None,
        }
    }

    /// Helper method for field `poll`.
    ///
    /// New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot.
    #[must_use]
    pub fn poll(&self) -> Option<&crate::types::Poll> {
        match self {
            Self::Poll(val) => Some(val.poll.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `poll_answer`.
    ///
    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[must_use]
    pub fn poll_answer(&self) -> Option<&crate::types::PollAnswer> {
        match self {
            Self::PollAnswer(val) => Some(&val.poll_answer),
            _ => None,
        }
    }

    /// Helper method for field `pre_checkout_query`.
    ///
    /// New incoming pre-checkout query. Contains full information about checkout.
    #[must_use]
    pub fn pre_checkout_query(&self) -> Option<&crate::types::PreCheckoutQuery> {
        match self {
            Self::PreCheckoutQuery(val) => Some(&val.pre_checkout_query),
            _ => None,
        }
    }

    /// Helper method for field `purchased_paid_media`.
    ///
    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    #[must_use]
    pub fn purchased_paid_media(&self) -> Option<&crate::types::PaidMediaPurchased> {
        match self {
            Self::PurchasedPaidMedia(val) => Some(&val.purchased_paid_media),
            _ => None,
        }
    }

    /// Helper method for field `removed_chat_boost`.
    ///
    /// A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    #[must_use]
    pub fn removed_chat_boost(&self) -> Option<&crate::types::ChatBoostRemoved> {
        match self {
            Self::RemovedChatBoost(val) => Some(&val.removed_chat_boost),
            _ => None,
        }
    }

    /// Helper method for field `shipping_query`.
    ///
    /// New incoming shipping query. Only for invoices with flexible price.
    #[must_use]
    pub fn shipping_query(&self) -> Option<&crate::types::ShippingQuery> {
        match self {
            Self::ShippingQuery(val) => Some(&val.shipping_query),
            _ => None,
        }
    }

    /// Helper method for field `subscription`.
    ///
    /// User payment subscription has changed
    #[must_use]
    pub fn subscription(&self) -> Option<&crate::types::BotSubscriptionUpdated> {
        match self {
            Self::Subscription(val) => Some(&val.subscription),
            _ => None,
        }
    }

    /// Helper method for field `update_id`.
    ///
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id(&self) -> i64 {
        match self {
            Self::BusinessConnection(val) => val.update_id,
            Self::BusinessMessage(val) => val.update_id,
            Self::CallbackQuery(val) => val.update_id,
            Self::ChannelPost(val) => val.update_id,
            Self::ChatBoost(val) => val.update_id,
            Self::ChatJoinRequest(val) => val.update_id,
            Self::ChatMember(val) => val.update_id,
            Self::ChosenInlineResult(val) => val.update_id,
            Self::DeletedBusinessMessages(val) => val.update_id,
            Self::EditedBusinessMessage(val) => val.update_id,
            Self::EditedChannelPost(val) => val.update_id,
            Self::EditedMessage(val) => val.update_id,
            Self::GuestMessage(val) => val.update_id,
            Self::InlineQuery(val) => val.update_id,
            Self::ManagedBot(val) => val.update_id,
            Self::Message(val) => val.update_id,
            Self::MessageReaction(val) => val.update_id,
            Self::MessageReactionCount(val) => val.update_id,
            Self::MyChatMember(val) => val.update_id,
            Self::Poll(val) => val.update_id,
            Self::PollAnswer(val) => val.update_id,
            Self::PreCheckoutQuery(val) => val.update_id,
            Self::PurchasedPaidMedia(val) => val.update_id,
            Self::RemovedChatBoost(val) => val.update_id,
            Self::ShippingQuery(val) => val.update_id,
            Self::Subscription(val) => val.update_id,
        }
    }

    /// Helper method for nested field `actor_chat`.
    #[must_use]
    pub fn actor_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::MessageReaction(val) => {
                let inner = &val.message_reaction;
                inner.actor_chat.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `allows_multiple_answers`.
    #[must_use]
    pub fn allows_multiple_answers(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::allows_multiple_answers(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `allows_revoting`.
    #[must_use]
    pub fn allows_revoting(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::allows_revoting(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `animation`.
    #[must_use]
    pub fn animation(&self) -> Option<&crate::types::Animation> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::animation(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::animation(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::animation(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::animation(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::animation(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::animation(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::animation(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `audio`.
    #[must_use]
    pub fn audio(&self) -> Option<&crate::types::Audio> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::audio(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::audio(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::audio(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::audio(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::audio(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::audio(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::audio(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `author_signature`.
    #[must_use]
    pub fn author_signature(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::author_signature(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::author_signature(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::author_signature(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::author_signature(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::author_signature(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::author_signature(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::author_signature(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `bio`.
    #[must_use]
    pub fn bio(&self) -> Option<&str> {
        match self {
            Self::ChatJoinRequest(val) => {
                let inner = &val.chat_join_request;
                inner.bio.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `boost`.
    #[must_use]
    pub fn boost(&self) -> Option<&crate::types::ChatBoost> {
        match self {
            Self::ChatBoost(val) => {
                let inner = &val.chat_boost;
                Some(&inner.boost)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `boost_added`.
    #[must_use]
    pub fn boost_added(&self) -> Option<&crate::types::ChatBoostAdded> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::boost_added(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::boost_added(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::boost_added(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::boost_added(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::boost_added(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::boost_added(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::boost_added(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `boost_id`.
    #[must_use]
    pub fn boost_id(&self) -> Option<&str> {
        match self {
            Self::RemovedChatBoost(val) => {
                let inner = &val.removed_chat_boost;
                Some(inner.boost_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `bot`.
    #[must_use]
    pub fn bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::ManagedBot(val) => {
                let inner = &val.managed_bot;
                Some(inner.bot.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `business_connection_id`.
    #[must_use]
    pub fn business_connection_id(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::business_connection_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::business_connection_id(inner)
            }
            Self::DeletedBusinessMessages(val) => {
                let inner = &val.deleted_business_messages;
                Some(inner.business_connection_id.as_ref())
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::business_connection_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::business_connection_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::business_connection_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::business_connection_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::business_connection_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `caption`.
    #[must_use]
    pub fn caption(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::caption(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::caption(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::caption(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::caption(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::caption(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::caption(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::caption(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `caption_entities`.
    #[must_use]
    pub fn caption_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::caption_entities(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::caption_entities(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::caption_entities(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::caption_entities(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::caption_entities(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::caption_entities(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::caption_entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `channel_chat_created`.
    #[must_use]
    pub fn channel_chat_created(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::channel_chat_created(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::channel_chat_created(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::channel_chat_created(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::channel_chat_created(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::channel_chat_created(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::channel_chat_created(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::channel_chat_created(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `chat`.
    #[must_use]
    pub fn chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                Some(crate::types::Message::chat(inner))
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                Some(crate::types::Message::chat(inner))
            }
            Self::ChatBoost(val) => {
                let inner = &val.chat_boost;
                Some(inner.chat.as_ref())
            }
            Self::ChatJoinRequest(val) => {
                let inner = &val.chat_join_request;
                Some(inner.chat.as_ref())
            }
            Self::ChatMember(val) => {
                let inner = &val.chat_member;
                Some(inner.chat.as_ref())
            }
            Self::DeletedBusinessMessages(val) => {
                let inner = &val.deleted_business_messages;
                Some(inner.chat.as_ref())
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                Some(crate::types::Message::chat(inner))
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                Some(crate::types::Message::chat(inner))
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                Some(crate::types::Message::chat(inner))
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                Some(crate::types::Message::chat(inner))
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                Some(crate::types::Message::chat(inner))
            }
            Self::MessageReaction(val) => {
                let inner = &val.message_reaction;
                Some(inner.chat.as_ref())
            }
            Self::MessageReactionCount(val) => {
                let inner = &val.message_reaction_count;
                Some(inner.chat.as_ref())
            }
            Self::MyChatMember(val) => {
                let inner = &val.my_chat_member;
                Some(inner.chat.as_ref())
            }
            Self::RemovedChatBoost(val) => {
                let inner = &val.removed_chat_boost;
                Some(inner.chat.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `chat_background_set`.
    #[must_use]
    pub fn chat_background_set(&self) -> Option<&crate::types::ChatBackground> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::chat_background_set(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::chat_background_set(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::chat_background_set(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::chat_background_set(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::chat_background_set(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::chat_background_set(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::chat_background_set(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `chat_instance`.
    #[must_use]
    pub fn chat_instance(&self) -> Option<&str> {
        match self {
            Self::CallbackQuery(val) => {
                let inner = &val.callback_query;
                Some(inner.chat_instance.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `chat_owner_changed`.
    #[must_use]
    pub fn chat_owner_changed(&self) -> Option<&crate::types::ChatOwnerChanged> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::chat_owner_changed(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::chat_owner_changed(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::chat_owner_changed(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::chat_owner_changed(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::chat_owner_changed(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::chat_owner_changed(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::chat_owner_changed(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `chat_owner_left`.
    #[must_use]
    pub fn chat_owner_left(&self) -> Option<&crate::types::ChatOwnerLeft> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::chat_owner_left(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::chat_owner_left(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::chat_owner_left(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::chat_owner_left(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::chat_owner_left(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::chat_owner_left(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::chat_owner_left(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `chat_shared`.
    #[must_use]
    pub fn chat_shared(&self) -> Option<&crate::types::ChatShared> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::chat_shared(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::chat_shared(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::chat_shared(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::chat_shared(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::chat_shared(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::chat_shared(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::chat_shared(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `chat_type`.
    #[must_use]
    pub fn chat_type(&self) -> Option<&str> {
        match self {
            Self::InlineQuery(val) => {
                let inner = &val.inline_query;
                inner.chat_type.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `checklist`.
    #[must_use]
    pub fn checklist(&self) -> Option<&crate::types::Checklist> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::checklist(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::checklist(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::checklist(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::checklist(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::checklist(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::checklist(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::checklist(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `checklist_tasks_added`.
    #[must_use]
    pub fn checklist_tasks_added(&self) -> Option<&crate::types::ChecklistTasksAdded> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::checklist_tasks_added(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::checklist_tasks_added(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::checklist_tasks_added(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::checklist_tasks_added(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::checklist_tasks_added(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::checklist_tasks_added(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::checklist_tasks_added(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `checklist_tasks_done`.
    #[must_use]
    pub fn checklist_tasks_done(&self) -> Option<&crate::types::ChecklistTasksDone> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::checklist_tasks_done(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::checklist_tasks_done(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::checklist_tasks_done(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::checklist_tasks_done(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::checklist_tasks_done(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::checklist_tasks_done(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::checklist_tasks_done(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `close_date`.
    #[must_use]
    pub fn close_date(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::close_date(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `community_chat_added`.
    #[must_use]
    pub fn community_chat_added(&self) -> Option<&crate::types::CommunityChatAdded> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::community_chat_added(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::community_chat_added(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::community_chat_added(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::community_chat_added(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::community_chat_added(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::community_chat_added(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::community_chat_added(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `community_chat_removed`.
    #[must_use]
    pub fn community_chat_removed(&self) -> Option<&crate::types::CommunityChatRemoved> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::community_chat_removed(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::community_chat_removed(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::community_chat_removed(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::community_chat_removed(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::community_chat_removed(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::community_chat_removed(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::community_chat_removed(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `connected_website`.
    #[must_use]
    pub fn connected_website(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::connected_website(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::connected_website(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::connected_website(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::connected_website(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::connected_website(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::connected_website(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::connected_website(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `contact`.
    #[must_use]
    pub fn contact(&self) -> Option<&crate::types::Contact> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::contact(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::contact(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::contact(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::contact(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::contact(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::contact(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::contact(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `correct_option_ids`.
    #[must_use]
    pub fn correct_option_ids(&self) -> Option<&[i64]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::correct_option_ids(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `country_codes`.
    #[must_use]
    pub fn country_codes(&self) -> Option<&[Box<str>]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::country_codes(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `currency`.
    #[must_use]
    pub fn currency(&self) -> Option<&str> {
        match self {
            Self::PreCheckoutQuery(val) => {
                let inner = &val.pre_checkout_query;
                Some(inner.currency.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `data`.
    #[must_use]
    pub fn data(&self) -> Option<&str> {
        match self {
            Self::CallbackQuery(val) => {
                let inner = &val.callback_query;
                inner.data.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `date`.
    #[must_use]
    pub fn date(&self) -> Option<i64> {
        match self {
            Self::BusinessConnection(val) => {
                let inner = &val.business_connection;
                Some(inner.date)
            }
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                Some(crate::types::Message::date(inner))
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                Some(crate::types::Message::date(inner))
            }
            Self::ChatJoinRequest(val) => {
                let inner = &val.chat_join_request;
                Some(inner.date)
            }
            Self::ChatMember(val) => {
                let inner = &val.chat_member;
                Some(inner.date)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                Some(crate::types::Message::date(inner))
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                Some(crate::types::Message::date(inner))
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                Some(crate::types::Message::date(inner))
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                Some(crate::types::Message::date(inner))
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                Some(crate::types::Message::date(inner))
            }
            Self::MessageReaction(val) => {
                let inner = &val.message_reaction;
                Some(inner.date)
            }
            Self::MessageReactionCount(val) => {
                let inner = &val.message_reaction_count;
                Some(inner.date)
            }
            Self::MyChatMember(val) => {
                let inner = &val.my_chat_member;
                Some(inner.date)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `delete_chat_photo`.
    #[must_use]
    pub fn delete_chat_photo(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::delete_chat_photo(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::delete_chat_photo(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::delete_chat_photo(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::delete_chat_photo(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::delete_chat_photo(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::delete_chat_photo(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::delete_chat_photo(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `description`.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::description(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `description_entities`.
    #[must_use]
    pub fn description_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::description_entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `dice`.
    #[must_use]
    pub fn dice(&self) -> Option<&crate::types::Dice> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::dice(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::dice(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::dice(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::dice(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::dice(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::dice(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::dice(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `direct_message_price_changed`.
    #[must_use]
    pub fn direct_message_price_changed(&self) -> Option<&crate::types::DirectMessagePriceChanged> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::direct_message_price_changed(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::direct_message_price_changed(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::direct_message_price_changed(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::direct_message_price_changed(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::direct_message_price_changed(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::direct_message_price_changed(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::direct_message_price_changed(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `direct_messages_topic`.
    #[must_use]
    pub fn direct_messages_topic(&self) -> Option<&crate::types::DirectMessagesTopic> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::direct_messages_topic(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::direct_messages_topic(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::direct_messages_topic(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::direct_messages_topic(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::direct_messages_topic(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::direct_messages_topic(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::direct_messages_topic(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `document`.
    #[must_use]
    pub fn document(&self) -> Option<&crate::types::Document> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::document(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::document(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::document(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::document(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::document(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::document(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::document(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `edit_date`.
    #[must_use]
    pub fn edit_date(&self) -> Option<i64> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::edit_date(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::edit_date(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::edit_date(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::edit_date(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::edit_date(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::edit_date(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::edit_date(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `effect_id`.
    #[must_use]
    pub fn effect_id(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::effect_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::effect_id(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::effect_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::effect_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::effect_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::effect_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::effect_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `entities`.
    #[must_use]
    pub fn entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::entities(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::entities(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::entities(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::entities(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::entities(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::entities(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `ephemeral_message_id`.
    #[must_use]
    pub fn ephemeral_message_id(&self) -> Option<i64> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::ephemeral_message_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::ephemeral_message_id(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::ephemeral_message_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::ephemeral_message_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::ephemeral_message_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::ephemeral_message_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::ephemeral_message_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `explanation`.
    #[must_use]
    pub fn explanation(&self) -> Option<&str> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::explanation(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `explanation_entities`.
    #[must_use]
    pub fn explanation_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::explanation_entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `explanation_media`.
    #[must_use]
    pub fn explanation_media(&self) -> Option<&crate::types::PollMedia> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::explanation_media(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `external_reply`.
    #[must_use]
    pub fn external_reply(&self) -> Option<&crate::types::ExternalReplyInfo> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::external_reply(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::external_reply(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::external_reply(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::external_reply(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::external_reply(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::external_reply(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::external_reply(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `forum_topic_closed`.
    #[must_use]
    pub fn forum_topic_closed(&self) -> Option<&crate::types::ForumTopicClosed> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::forum_topic_closed(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::forum_topic_closed(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::forum_topic_closed(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::forum_topic_closed(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::forum_topic_closed(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::forum_topic_closed(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::forum_topic_closed(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `forum_topic_created`.
    #[must_use]
    pub fn forum_topic_created(&self) -> Option<&crate::types::ForumTopicCreated> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::forum_topic_created(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::forum_topic_created(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::forum_topic_created(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::forum_topic_created(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::forum_topic_created(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::forum_topic_created(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::forum_topic_created(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `forum_topic_edited`.
    #[must_use]
    pub fn forum_topic_edited(&self) -> Option<&crate::types::ForumTopicEdited> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::forum_topic_edited(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::forum_topic_edited(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::forum_topic_edited(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::forum_topic_edited(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::forum_topic_edited(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::forum_topic_edited(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::forum_topic_edited(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `forum_topic_reopened`.
    #[must_use]
    pub fn forum_topic_reopened(&self) -> Option<&crate::types::ForumTopicReopened> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::forum_topic_reopened(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::forum_topic_reopened(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::forum_topic_reopened(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::forum_topic_reopened(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::forum_topic_reopened(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::forum_topic_reopened(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::forum_topic_reopened(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `forward_origin`.
    #[must_use]
    pub fn forward_origin(&self) -> Option<&crate::types::MessageOrigin> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::forward_origin(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::forward_origin(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::forward_origin(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::forward_origin(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::forward_origin(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::forward_origin(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::forward_origin(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `from`.
    #[must_use]
    pub fn from(&self) -> Option<&crate::types::User> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::from(inner)
            }
            Self::CallbackQuery(val) => {
                let inner = &val.callback_query;
                Some(inner.from.as_ref())
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::from(inner)
            }
            Self::ChatJoinRequest(val) => {
                let inner = &val.chat_join_request;
                Some(inner.from.as_ref())
            }
            Self::ChatMember(val) => {
                let inner = &val.chat_member;
                Some(inner.from.as_ref())
            }
            Self::ChosenInlineResult(val) => {
                let inner = &val.chosen_inline_result;
                Some(inner.from.as_ref())
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::from(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::from(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::from(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::from(inner)
            }
            Self::InlineQuery(val) => {
                let inner = &val.inline_query;
                Some(inner.from.as_ref())
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::from(inner)
            }
            Self::MyChatMember(val) => {
                let inner = &val.my_chat_member;
                Some(inner.from.as_ref())
            }
            Self::PreCheckoutQuery(val) => {
                let inner = &val.pre_checkout_query;
                Some(inner.from.as_ref())
            }
            Self::PurchasedPaidMedia(val) => {
                let inner = &val.purchased_paid_media;
                Some(inner.from.as_ref())
            }
            Self::ShippingQuery(val) => {
                let inner = &val.shipping_query;
                Some(inner.from.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `game`.
    #[must_use]
    pub fn game(&self) -> Option<&crate::types::Game> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::game(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::game(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::game(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::game(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::game(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::game(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::game(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `game_short_name`.
    #[must_use]
    pub fn game_short_name(&self) -> Option<&str> {
        match self {
            Self::CallbackQuery(val) => {
                let inner = &val.callback_query;
                inner.game_short_name.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `general_forum_topic_hidden`.
    #[must_use]
    pub fn general_forum_topic_hidden(&self) -> Option<&crate::types::GeneralForumTopicHidden> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::general_forum_topic_hidden(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::general_forum_topic_hidden(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::general_forum_topic_hidden(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::general_forum_topic_hidden(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::general_forum_topic_hidden(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::general_forum_topic_hidden(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::general_forum_topic_hidden(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `general_forum_topic_unhidden`.
    #[must_use]
    pub fn general_forum_topic_unhidden(&self) -> Option<&crate::types::GeneralForumTopicUnhidden> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::general_forum_topic_unhidden(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::general_forum_topic_unhidden(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::general_forum_topic_unhidden(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::general_forum_topic_unhidden(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::general_forum_topic_unhidden(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::general_forum_topic_unhidden(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::general_forum_topic_unhidden(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `gift`.
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::gift(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::gift(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::gift(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::gift(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::gift(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::gift(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::gift(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `gift_upgrade_sent`.
    #[must_use]
    pub fn gift_upgrade_sent(&self) -> Option<&crate::types::GiftInfo> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::gift_upgrade_sent(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::gift_upgrade_sent(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::gift_upgrade_sent(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::gift_upgrade_sent(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::gift_upgrade_sent(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::gift_upgrade_sent(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::gift_upgrade_sent(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `giveaway`.
    #[must_use]
    pub fn giveaway(&self) -> Option<&crate::types::Giveaway> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::giveaway(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::giveaway(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::giveaway(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::giveaway(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::giveaway(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::giveaway(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::giveaway(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `giveaway_completed`.
    #[must_use]
    pub fn giveaway_completed(&self) -> Option<&crate::types::GiveawayCompleted> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::giveaway_completed(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::giveaway_completed(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::giveaway_completed(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::giveaway_completed(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::giveaway_completed(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::giveaway_completed(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::giveaway_completed(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `giveaway_created`.
    #[must_use]
    pub fn giveaway_created(&self) -> Option<&crate::types::GiveawayCreated> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::giveaway_created(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::giveaway_created(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::giveaway_created(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::giveaway_created(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::giveaway_created(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::giveaway_created(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::giveaway_created(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `giveaway_winners`.
    #[must_use]
    pub fn giveaway_winners(&self) -> Option<&crate::types::GiveawayWinners> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::giveaway_winners(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::giveaway_winners(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::giveaway_winners(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::giveaway_winners(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::giveaway_winners(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::giveaway_winners(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::giveaway_winners(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `group_chat_created`.
    #[must_use]
    pub fn group_chat_created(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::group_chat_created(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::group_chat_created(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::group_chat_created(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::group_chat_created(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::group_chat_created(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::group_chat_created(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::group_chat_created(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `guest_bot_caller_chat`.
    #[must_use]
    pub fn guest_bot_caller_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::guest_bot_caller_chat(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::guest_bot_caller_chat(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::guest_bot_caller_chat(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::guest_bot_caller_chat(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::guest_bot_caller_chat(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::guest_bot_caller_chat(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::guest_bot_caller_chat(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `guest_bot_caller_user`.
    #[must_use]
    pub fn guest_bot_caller_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::guest_bot_caller_user(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::guest_bot_caller_user(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::guest_bot_caller_user(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::guest_bot_caller_user(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::guest_bot_caller_user(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::guest_bot_caller_user(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::guest_bot_caller_user(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `guest_query_id`.
    #[must_use]
    pub fn guest_query_id(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::guest_query_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::guest_query_id(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::guest_query_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::guest_query_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::guest_query_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::guest_query_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::guest_query_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `has_media_spoiler`.
    #[must_use]
    pub fn has_media_spoiler(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::has_media_spoiler(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::has_media_spoiler(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::has_media_spoiler(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::has_media_spoiler(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::has_media_spoiler(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::has_media_spoiler(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::has_media_spoiler(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `has_protected_content`.
    #[must_use]
    pub fn has_protected_content(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::has_protected_content(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::has_protected_content(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::has_protected_content(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::has_protected_content(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::has_protected_content(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::has_protected_content(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::has_protected_content(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `id`.
    #[must_use]
    pub fn id(&self) -> Option<&str> {
        match self {
            Self::BusinessConnection(val) => {
                let inner = &val.business_connection;
                Some(inner.id.as_ref())
            }
            Self::CallbackQuery(val) => {
                let inner = &val.callback_query;
                Some(inner.id.as_ref())
            }
            Self::InlineQuery(val) => {
                let inner = &val.inline_query;
                Some(inner.id.as_ref())
            }
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::id(inner))
            }
            Self::PreCheckoutQuery(val) => {
                let inner = &val.pre_checkout_query;
                Some(inner.id.as_ref())
            }
            Self::ShippingQuery(val) => {
                let inner = &val.shipping_query;
                Some(inner.id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `inline_message_id`.
    #[must_use]
    pub fn inline_message_id(&self) -> Option<&str> {
        match self {
            Self::CallbackQuery(val) => {
                let inner = &val.callback_query;
                inner.inline_message_id.as_deref()
            }
            Self::ChosenInlineResult(val) => {
                let inner = &val.chosen_inline_result;
                inner.inline_message_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `invite_link`.
    #[must_use]
    pub fn invite_link(&self) -> Option<&crate::types::ChatInviteLink> {
        match self {
            Self::ChatJoinRequest(val) => {
                let inner = &val.chat_join_request;
                inner.invite_link.as_ref()
            }
            Self::ChatMember(val) => {
                let inner = &val.chat_member;
                inner.invite_link.as_ref()
            }
            Self::MyChatMember(val) => {
                let inner = &val.my_chat_member;
                inner.invite_link.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `invoice`.
    #[must_use]
    pub fn invoice(&self) -> Option<&crate::types::Invoice> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::invoice(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::invoice(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::invoice(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::invoice(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::invoice(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::invoice(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::invoice(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `invoice_payload`.
    #[must_use]
    pub fn invoice_payload(&self) -> Option<&str> {
        match self {
            Self::PreCheckoutQuery(val) => {
                let inner = &val.pre_checkout_query;
                Some(inner.invoice_payload.as_ref())
            }
            Self::ShippingQuery(val) => {
                let inner = &val.shipping_query;
                Some(inner.invoice_payload.as_ref())
            }
            Self::Subscription(val) => {
                let inner = &val.subscription;
                Some(inner.invoice_payload.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_anonymous`.
    #[must_use]
    pub fn is_anonymous(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::is_anonymous(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_automatic_forward`.
    #[must_use]
    pub fn is_automatic_forward(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::is_automatic_forward(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::is_automatic_forward(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::is_automatic_forward(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::is_automatic_forward(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::is_automatic_forward(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::is_automatic_forward(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::is_automatic_forward(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_closed`.
    #[must_use]
    pub fn is_closed(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::is_closed(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_enabled`.
    #[must_use]
    pub fn is_enabled(&self) -> Option<bool> {
        match self {
            Self::BusinessConnection(val) => {
                let inner = &val.business_connection;
                Some(inner.is_enabled)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_from_offline`.
    #[must_use]
    pub fn is_from_offline(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::is_from_offline(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::is_from_offline(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::is_from_offline(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::is_from_offline(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::is_from_offline(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::is_from_offline(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::is_from_offline(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_paid_post`.
    #[must_use]
    pub fn is_paid_post(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::is_paid_post(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::is_paid_post(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::is_paid_post(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::is_paid_post(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::is_paid_post(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::is_paid_post(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::is_paid_post(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_topic_message`.
    #[must_use]
    pub fn is_topic_message(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::is_topic_message(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::is_topic_message(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::is_topic_message(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::is_topic_message(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::is_topic_message(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::is_topic_message(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::is_topic_message(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `left_chat_member`.
    #[must_use]
    pub fn left_chat_member(&self) -> Option<&crate::types::User> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::left_chat_member(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::left_chat_member(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::left_chat_member(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::left_chat_member(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::left_chat_member(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::left_chat_member(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::left_chat_member(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `link_preview_options`.
    #[must_use]
    pub fn link_preview_options(&self) -> Option<&crate::types::LinkPreviewOptions> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::link_preview_options(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::link_preview_options(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::link_preview_options(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::link_preview_options(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::link_preview_options(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::link_preview_options(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::link_preview_options(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `live_photo`.
    #[must_use]
    pub fn live_photo(&self) -> Option<&crate::types::LivePhoto> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::live_photo(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::live_photo(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::live_photo(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::live_photo(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::live_photo(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::live_photo(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::live_photo(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `location`.
    #[must_use]
    pub fn location(&self) -> Option<&crate::types::Location> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::location(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::location(inner)
            }
            Self::ChosenInlineResult(val) => {
                let inner = &val.chosen_inline_result;
                inner.location.as_ref()
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::location(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::location(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::location(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::location(inner)
            }
            Self::InlineQuery(val) => {
                let inner = &val.inline_query;
                inner.location.as_ref()
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::location(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `managed_bot_created`.
    #[must_use]
    pub fn managed_bot_created(&self) -> Option<&crate::types::ManagedBotCreated> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::managed_bot_created(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::managed_bot_created(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::managed_bot_created(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::managed_bot_created(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::managed_bot_created(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::managed_bot_created(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::managed_bot_created(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `media`.
    #[must_use]
    pub fn media(&self) -> Option<&crate::types::PollMedia> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::media(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `media_group_id`.
    #[must_use]
    pub fn media_group_id(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::media_group_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::media_group_id(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::media_group_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::media_group_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::media_group_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::media_group_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::media_group_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `members_only`.
    #[must_use]
    pub fn members_only(&self) -> Option<bool> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::members_only(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `message_auto_delete_timer_changed`.
    #[must_use]
    pub fn message_auto_delete_timer_changed(
        &self,
    ) -> Option<&crate::types::MessageAutoDeleteTimerChanged> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::message_auto_delete_timer_changed(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::message_auto_delete_timer_changed(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::message_auto_delete_timer_changed(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::message_auto_delete_timer_changed(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::message_auto_delete_timer_changed(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::message_auto_delete_timer_changed(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::message_auto_delete_timer_changed(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `message_id`.
    #[must_use]
    pub fn message_id(&self) -> Option<i64> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                Some(crate::types::Message::message_id(inner))
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                Some(crate::types::Message::message_id(inner))
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                Some(crate::types::Message::message_id(inner))
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                Some(crate::types::Message::message_id(inner))
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                Some(crate::types::Message::message_id(inner))
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                Some(crate::types::Message::message_id(inner))
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                Some(crate::types::Message::message_id(inner))
            }
            Self::MessageReaction(val) => {
                let inner = &val.message_reaction;
                Some(inner.message_id)
            }
            Self::MessageReactionCount(val) => {
                let inner = &val.message_reaction_count;
                Some(inner.message_id)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `message_ids`.
    #[must_use]
    pub fn message_ids(&self) -> Option<&[i64]> {
        match self {
            Self::DeletedBusinessMessages(val) => {
                let inner = &val.deleted_business_messages;
                Some(inner.message_ids.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `message_thread_id`.
    #[must_use]
    pub fn message_thread_id(&self) -> Option<i64> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::message_thread_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::message_thread_id(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::message_thread_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::message_thread_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::message_thread_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::message_thread_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::message_thread_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `migrate_from_chat_id`.
    #[must_use]
    pub fn migrate_from_chat_id(&self) -> Option<i64> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::migrate_from_chat_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::migrate_from_chat_id(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::migrate_from_chat_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::migrate_from_chat_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::migrate_from_chat_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::migrate_from_chat_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::migrate_from_chat_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `migrate_to_chat_id`.
    #[must_use]
    pub fn migrate_to_chat_id(&self) -> Option<i64> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::migrate_to_chat_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::migrate_to_chat_id(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::migrate_to_chat_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::migrate_to_chat_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::migrate_to_chat_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::migrate_to_chat_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::migrate_to_chat_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `new_chat_member`.
    #[must_use]
    pub fn new_chat_member(&self) -> Option<&crate::types::ChatMember> {
        match self {
            Self::ChatMember(val) => {
                let inner = &val.chat_member;
                Some(&inner.new_chat_member)
            }
            Self::MyChatMember(val) => {
                let inner = &val.my_chat_member;
                Some(&inner.new_chat_member)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `new_chat_members`.
    #[must_use]
    pub fn new_chat_members(&self) -> Option<&[crate::types::User]> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::new_chat_members(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::new_chat_members(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::new_chat_members(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::new_chat_members(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::new_chat_members(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::new_chat_members(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::new_chat_members(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `new_chat_photo`.
    #[must_use]
    pub fn new_chat_photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::new_chat_photo(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::new_chat_photo(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::new_chat_photo(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::new_chat_photo(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::new_chat_photo(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::new_chat_photo(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::new_chat_photo(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `new_chat_title`.
    #[must_use]
    pub fn new_chat_title(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::new_chat_title(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::new_chat_title(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::new_chat_title(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::new_chat_title(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::new_chat_title(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::new_chat_title(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::new_chat_title(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `new_reaction`.
    #[must_use]
    pub fn new_reaction(&self) -> Option<&[crate::types::ReactionType]> {
        match self {
            Self::MessageReaction(val) => {
                let inner = &val.message_reaction;
                Some(inner.new_reaction.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `offset`.
    #[must_use]
    pub fn offset(&self) -> Option<&str> {
        match self {
            Self::InlineQuery(val) => {
                let inner = &val.inline_query;
                Some(inner.offset.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `old_chat_member`.
    #[must_use]
    pub fn old_chat_member(&self) -> Option<&crate::types::ChatMember> {
        match self {
            Self::ChatMember(val) => {
                let inner = &val.chat_member;
                Some(&inner.old_chat_member)
            }
            Self::MyChatMember(val) => {
                let inner = &val.my_chat_member;
                Some(&inner.old_chat_member)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `old_reaction`.
    #[must_use]
    pub fn old_reaction(&self) -> Option<&[crate::types::ReactionType]> {
        match self {
            Self::MessageReaction(val) => {
                let inner = &val.message_reaction;
                Some(inner.old_reaction.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `open_period`.
    #[must_use]
    pub fn open_period(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::open_period(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `option_ids`.
    #[must_use]
    pub fn option_ids(&self) -> Option<&[i64]> {
        match self {
            Self::PollAnswer(val) => {
                let inner = &val.poll_answer;
                Some(inner.option_ids.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `option_persistent_ids`.
    #[must_use]
    pub fn option_persistent_ids(&self) -> Option<&[Box<str>]> {
        match self {
            Self::PollAnswer(val) => {
                let inner = &val.poll_answer;
                Some(inner.option_persistent_ids.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `options`.
    #[must_use]
    pub fn options(&self) -> Option<&[crate::types::PollOption]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::options(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `order_info`.
    #[must_use]
    pub fn order_info(&self) -> Option<&crate::types::OrderInfo> {
        match self {
            Self::PreCheckoutQuery(val) => {
                let inner = &val.pre_checkout_query;
                inner.order_info.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `paid_media`.
    #[must_use]
    pub fn paid_media(&self) -> Option<&crate::types::PaidMediaInfo> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::paid_media(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::paid_media(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::paid_media(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::paid_media(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::paid_media(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::paid_media(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::paid_media(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `paid_media_payload`.
    #[must_use]
    pub fn paid_media_payload(&self) -> Option<&str> {
        match self {
            Self::PurchasedPaidMedia(val) => {
                let inner = &val.purchased_paid_media;
                Some(inner.paid_media_payload.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `paid_message_price_changed`.
    #[must_use]
    pub fn paid_message_price_changed(&self) -> Option<&crate::types::PaidMessagePriceChanged> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::paid_message_price_changed(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::paid_message_price_changed(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::paid_message_price_changed(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::paid_message_price_changed(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::paid_message_price_changed(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::paid_message_price_changed(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::paid_message_price_changed(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `paid_star_count`.
    #[must_use]
    pub fn paid_star_count(&self) -> Option<i64> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::paid_star_count(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::paid_star_count(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::paid_star_count(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::paid_star_count(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::paid_star_count(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::paid_star_count(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::paid_star_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `passport_data`.
    #[must_use]
    pub fn passport_data(&self) -> Option<&crate::types::PassportData> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::passport_data(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::passport_data(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::passport_data(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::passport_data(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::passport_data(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::passport_data(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::passport_data(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `photo`.
    #[must_use]
    pub fn photo(&self) -> Option<&[crate::types::PhotoSize]> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::photo(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::photo(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::photo(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::photo(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::photo(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::photo(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::photo(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `pinned_message`.
    #[must_use]
    pub fn pinned_message(&self) -> Option<&crate::types::MaybeInaccessibleMessage> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::pinned_message(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::pinned_message(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::pinned_message(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::pinned_message(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::pinned_message(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::pinned_message(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::pinned_message(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `poll_id`.
    #[must_use]
    pub fn poll_id(&self) -> Option<&str> {
        match self {
            Self::PollAnswer(val) => {
                let inner = &val.poll_answer;
                Some(inner.poll_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `poll_option_added`.
    #[must_use]
    pub fn poll_option_added(&self) -> Option<&crate::types::PollOptionAdded> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::poll_option_added(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::poll_option_added(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::poll_option_added(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::poll_option_added(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::poll_option_added(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::poll_option_added(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::poll_option_added(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `poll_option_deleted`.
    #[must_use]
    pub fn poll_option_deleted(&self) -> Option<&crate::types::PollOptionDeleted> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::poll_option_deleted(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::poll_option_deleted(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::poll_option_deleted(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::poll_option_deleted(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::poll_option_deleted(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::poll_option_deleted(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::poll_option_deleted(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `proximity_alert_triggered`.
    #[must_use]
    pub fn proximity_alert_triggered(&self) -> Option<&crate::types::ProximityAlertTriggered> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::proximity_alert_triggered(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::proximity_alert_triggered(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::proximity_alert_triggered(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::proximity_alert_triggered(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::proximity_alert_triggered(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::proximity_alert_triggered(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::proximity_alert_triggered(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `query`.
    #[must_use]
    pub fn query(&self) -> Option<&str> {
        match self {
            Self::ChosenInlineResult(val) => {
                let inner = &val.chosen_inline_result;
                Some(inner.query.as_ref())
            }
            Self::InlineQuery(val) => {
                let inner = &val.inline_query;
                Some(inner.query.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `query_id`.
    #[must_use]
    pub fn query_id(&self) -> Option<&str> {
        match self {
            Self::ChatJoinRequest(val) => {
                let inner = &val.chat_join_request;
                inner.query_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `question`.
    #[must_use]
    pub fn question(&self) -> Option<&str> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::question(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `question_entities`.
    #[must_use]
    pub fn question_entities(&self) -> Option<&[crate::types::MessageEntity]> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                crate::types::Poll::question_entities(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `quote`.
    #[must_use]
    pub fn quote(&self) -> Option<&crate::types::TextQuote> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::quote(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::quote(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::quote(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::quote(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::quote(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::quote(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::quote(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `reactions`.
    #[must_use]
    pub fn reactions(&self) -> Option<&[crate::types::ReactionCount]> {
        match self {
            Self::MessageReactionCount(val) => {
                let inner = &val.message_reaction_count;
                Some(inner.reactions.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `receiver_user`.
    #[must_use]
    pub fn receiver_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::receiver_user(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::receiver_user(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::receiver_user(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::receiver_user(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::receiver_user(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::receiver_user(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::receiver_user(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `refunded_payment`.
    #[must_use]
    pub fn refunded_payment(&self) -> Option<&crate::types::RefundedPayment> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::refunded_payment(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::refunded_payment(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::refunded_payment(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::refunded_payment(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::refunded_payment(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::refunded_payment(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::refunded_payment(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `remove_date`.
    #[must_use]
    pub fn remove_date(&self) -> Option<i64> {
        match self {
            Self::RemovedChatBoost(val) => {
                let inner = &val.removed_chat_boost;
                Some(inner.remove_date)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `reply_markup`.
    #[must_use]
    pub fn reply_markup(&self) -> Option<&crate::types::InlineKeyboardMarkup> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::reply_markup(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::reply_markup(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::reply_markup(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::reply_markup(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::reply_markup(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::reply_markup(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::reply_markup(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `reply_to_checklist_task_id`.
    #[must_use]
    pub fn reply_to_checklist_task_id(&self) -> Option<i64> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::reply_to_checklist_task_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::reply_to_checklist_task_id(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::reply_to_checklist_task_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::reply_to_checklist_task_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::reply_to_checklist_task_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::reply_to_checklist_task_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::reply_to_checklist_task_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `reply_to_message`.
    #[must_use]
    pub fn reply_to_message(&self) -> Option<&crate::types::Message> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::reply_to_message(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::reply_to_message(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::reply_to_message(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::reply_to_message(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::reply_to_message(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::reply_to_message(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::reply_to_message(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `reply_to_poll_option_id`.
    #[must_use]
    pub fn reply_to_poll_option_id(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::reply_to_poll_option_id(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::reply_to_poll_option_id(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::reply_to_poll_option_id(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::reply_to_poll_option_id(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::reply_to_poll_option_id(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::reply_to_poll_option_id(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::reply_to_poll_option_id(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `reply_to_story`.
    #[must_use]
    pub fn reply_to_story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::reply_to_story(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::reply_to_story(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::reply_to_story(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::reply_to_story(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::reply_to_story(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::reply_to_story(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::reply_to_story(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `result_id`.
    #[must_use]
    pub fn result_id(&self) -> Option<&str> {
        match self {
            Self::ChosenInlineResult(val) => {
                let inner = &val.chosen_inline_result;
                Some(inner.result_id.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `rich_message`.
    #[must_use]
    pub fn rich_message(&self) -> Option<&crate::types::RichMessage> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::rich_message(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::rich_message(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::rich_message(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::rich_message(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::rich_message(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::rich_message(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::rich_message(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `rights`.
    #[must_use]
    pub fn rights(&self) -> Option<&crate::types::BusinessBotRights> {
        match self {
            Self::BusinessConnection(val) => {
                let inner = &val.business_connection;
                inner.rights.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `sender_boost_count`.
    #[must_use]
    pub fn sender_boost_count(&self) -> Option<i64> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::sender_boost_count(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::sender_boost_count(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::sender_boost_count(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::sender_boost_count(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::sender_boost_count(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::sender_boost_count(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::sender_boost_count(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `sender_business_bot`.
    #[must_use]
    pub fn sender_business_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::sender_business_bot(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::sender_business_bot(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::sender_business_bot(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::sender_business_bot(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::sender_business_bot(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::sender_business_bot(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::sender_business_bot(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `sender_chat`.
    #[must_use]
    pub fn sender_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::sender_chat(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::sender_chat(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::sender_chat(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::sender_chat(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::sender_chat(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::sender_chat(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::sender_chat(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `sender_tag`.
    #[must_use]
    pub fn sender_tag(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::sender_tag(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::sender_tag(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::sender_tag(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::sender_tag(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::sender_tag(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::sender_tag(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::sender_tag(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `shipping_address`.
    #[must_use]
    pub fn shipping_address(&self) -> Option<&crate::types::ShippingAddress> {
        match self {
            Self::ShippingQuery(val) => {
                let inner = &val.shipping_query;
                Some(&inner.shipping_address)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `shipping_option_id`.
    #[must_use]
    pub fn shipping_option_id(&self) -> Option<&str> {
        match self {
            Self::PreCheckoutQuery(val) => {
                let inner = &val.pre_checkout_query;
                inner.shipping_option_id.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `show_caption_above_media`.
    #[must_use]
    pub fn show_caption_above_media(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::show_caption_above_media(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::show_caption_above_media(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::show_caption_above_media(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::show_caption_above_media(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::show_caption_above_media(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::show_caption_above_media(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::show_caption_above_media(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `source`.
    #[must_use]
    pub fn source(&self) -> Option<&crate::types::ChatBoostSource> {
        match self {
            Self::RemovedChatBoost(val) => {
                let inner = &val.removed_chat_boost;
                Some(&inner.source)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `state`.
    #[must_use]
    pub fn state(&self) -> Option<&str> {
        match self {
            Self::Subscription(val) => {
                let inner = &val.subscription;
                Some(inner.state.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `sticker`.
    #[must_use]
    pub fn sticker(&self) -> Option<&crate::types::Sticker> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::sticker(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::sticker(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::sticker(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::sticker(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::sticker(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::sticker(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::sticker(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `story`.
    #[must_use]
    pub fn story(&self) -> Option<&crate::types::Story> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::story(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::story(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::story(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::story(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::story(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::story(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::story(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `successful_payment`.
    #[must_use]
    pub fn successful_payment(&self) -> Option<&crate::types::SuccessfulPayment> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::successful_payment(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::successful_payment(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::successful_payment(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::successful_payment(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::successful_payment(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::successful_payment(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::successful_payment(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `suggested_post_approval_failed`.
    #[must_use]
    pub fn suggested_post_approval_failed(
        &self,
    ) -> Option<&crate::types::SuggestedPostApprovalFailed> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::suggested_post_approval_failed(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::suggested_post_approval_failed(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::suggested_post_approval_failed(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::suggested_post_approval_failed(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::suggested_post_approval_failed(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::suggested_post_approval_failed(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::suggested_post_approval_failed(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `suggested_post_approved`.
    #[must_use]
    pub fn suggested_post_approved(&self) -> Option<&crate::types::SuggestedPostApproved> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::suggested_post_approved(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::suggested_post_approved(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::suggested_post_approved(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::suggested_post_approved(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::suggested_post_approved(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::suggested_post_approved(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::suggested_post_approved(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `suggested_post_declined`.
    #[must_use]
    pub fn suggested_post_declined(&self) -> Option<&crate::types::SuggestedPostDeclined> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::suggested_post_declined(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::suggested_post_declined(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::suggested_post_declined(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::suggested_post_declined(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::suggested_post_declined(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::suggested_post_declined(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::suggested_post_declined(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `suggested_post_info`.
    #[must_use]
    pub fn suggested_post_info(&self) -> Option<&crate::types::SuggestedPostInfo> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::suggested_post_info(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::suggested_post_info(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::suggested_post_info(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::suggested_post_info(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::suggested_post_info(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::suggested_post_info(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::suggested_post_info(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `suggested_post_paid`.
    #[must_use]
    pub fn suggested_post_paid(&self) -> Option<&crate::types::SuggestedPostPaid> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::suggested_post_paid(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::suggested_post_paid(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::suggested_post_paid(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::suggested_post_paid(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::suggested_post_paid(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::suggested_post_paid(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::suggested_post_paid(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `suggested_post_refunded`.
    #[must_use]
    pub fn suggested_post_refunded(&self) -> Option<&crate::types::SuggestedPostRefunded> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::suggested_post_refunded(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::suggested_post_refunded(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::suggested_post_refunded(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::suggested_post_refunded(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::suggested_post_refunded(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::suggested_post_refunded(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::suggested_post_refunded(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `supergroup_chat_created`.
    #[must_use]
    pub fn supergroup_chat_created(&self) -> Option<bool> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::supergroup_chat_created(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::supergroup_chat_created(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::supergroup_chat_created(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::supergroup_chat_created(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::supergroup_chat_created(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::supergroup_chat_created(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::supergroup_chat_created(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `text`.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::text(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::text(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::text(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::text(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::text(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::text(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::text(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `total_amount`.
    #[must_use]
    pub fn total_amount(&self) -> Option<i64> {
        match self {
            Self::PreCheckoutQuery(val) => {
                let inner = &val.pre_checkout_query;
                Some(inner.total_amount)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `total_voter_count`.
    #[must_use]
    pub fn total_voter_count(&self) -> Option<i64> {
        match self {
            Self::Poll(val) => {
                let inner = val.poll.as_ref();
                Some(crate::types::Poll::total_voter_count(inner))
            }
            _ => None,
        }
    }

    /// Helper method for nested field `unique_gift`.
    #[must_use]
    pub fn unique_gift(&self) -> Option<&crate::types::UniqueGiftInfo> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::unique_gift(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::unique_gift(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::unique_gift(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::unique_gift(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::unique_gift(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::unique_gift(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::unique_gift(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `user`.
    #[must_use]
    pub fn user(&self) -> Option<&crate::types::User> {
        match self {
            Self::BusinessConnection(val) => {
                let inner = &val.business_connection;
                Some(inner.user.as_ref())
            }
            Self::ManagedBot(val) => {
                let inner = &val.managed_bot;
                Some(inner.user.as_ref())
            }
            Self::MessageReaction(val) => {
                let inner = &val.message_reaction;
                inner.user.as_deref()
            }
            Self::PollAnswer(val) => {
                let inner = &val.poll_answer;
                inner.user.as_deref()
            }
            Self::Subscription(val) => {
                let inner = &val.subscription;
                Some(inner.user.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `user_chat_id`.
    #[must_use]
    pub fn user_chat_id(&self) -> Option<i64> {
        match self {
            Self::BusinessConnection(val) => {
                let inner = &val.business_connection;
                Some(inner.user_chat_id)
            }
            Self::ChatJoinRequest(val) => {
                let inner = &val.chat_join_request;
                Some(inner.user_chat_id)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `users_shared`.
    #[must_use]
    pub fn users_shared(&self) -> Option<&crate::types::UsersShared> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::users_shared(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::users_shared(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::users_shared(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::users_shared(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::users_shared(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::users_shared(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::users_shared(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `venue`.
    #[must_use]
    pub fn venue(&self) -> Option<&crate::types::Venue> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::venue(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::venue(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::venue(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::venue(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::venue(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::venue(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::venue(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `via_bot`.
    #[must_use]
    pub fn via_bot(&self) -> Option<&crate::types::User> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::via_bot(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::via_bot(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::via_bot(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::via_bot(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::via_bot(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::via_bot(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::via_bot(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `via_chat_folder_invite_link`.
    #[must_use]
    pub fn via_chat_folder_invite_link(&self) -> Option<bool> {
        match self {
            Self::ChatMember(val) => {
                let inner = &val.chat_member;
                inner.via_chat_folder_invite_link
            }
            Self::MyChatMember(val) => {
                let inner = &val.my_chat_member;
                inner.via_chat_folder_invite_link
            }
            _ => None,
        }
    }

    /// Helper method for nested field `via_join_request`.
    #[must_use]
    pub fn via_join_request(&self) -> Option<bool> {
        match self {
            Self::ChatMember(val) => {
                let inner = &val.chat_member;
                inner.via_join_request
            }
            Self::MyChatMember(val) => {
                let inner = &val.my_chat_member;
                inner.via_join_request
            }
            _ => None,
        }
    }

    /// Helper method for nested field `video`.
    #[must_use]
    pub fn video(&self) -> Option<&crate::types::Video> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::video(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::video(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::video(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::video(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::video(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::video(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::video(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `video_chat_ended`.
    #[must_use]
    pub fn video_chat_ended(&self) -> Option<&crate::types::VideoChatEnded> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::video_chat_ended(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::video_chat_ended(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::video_chat_ended(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::video_chat_ended(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::video_chat_ended(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::video_chat_ended(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::video_chat_ended(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `video_chat_participants_invited`.
    #[must_use]
    pub fn video_chat_participants_invited(
        &self,
    ) -> Option<&crate::types::VideoChatParticipantsInvited> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::video_chat_participants_invited(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::video_chat_participants_invited(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::video_chat_participants_invited(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::video_chat_participants_invited(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::video_chat_participants_invited(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::video_chat_participants_invited(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::video_chat_participants_invited(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `video_chat_scheduled`.
    #[must_use]
    pub fn video_chat_scheduled(&self) -> Option<&crate::types::VideoChatScheduled> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::video_chat_scheduled(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::video_chat_scheduled(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::video_chat_scheduled(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::video_chat_scheduled(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::video_chat_scheduled(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::video_chat_scheduled(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::video_chat_scheduled(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `video_chat_started`.
    #[must_use]
    pub fn video_chat_started(&self) -> Option<&crate::types::VideoChatStarted> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::video_chat_started(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::video_chat_started(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::video_chat_started(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::video_chat_started(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::video_chat_started(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::video_chat_started(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::video_chat_started(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `video_note`.
    #[must_use]
    pub fn video_note(&self) -> Option<&crate::types::VideoNote> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::video_note(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::video_note(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::video_note(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::video_note(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::video_note(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::video_note(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::video_note(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `voice`.
    #[must_use]
    pub fn voice(&self) -> Option<&crate::types::Voice> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::voice(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::voice(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::voice(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::voice(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::voice(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::voice(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::voice(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `voter_chat`.
    #[must_use]
    pub fn voter_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::PollAnswer(val) => {
                let inner = &val.poll_answer;
                inner.voter_chat.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `web_app_data`.
    #[must_use]
    pub fn web_app_data(&self) -> Option<&crate::types::WebAppData> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::web_app_data(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::web_app_data(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::web_app_data(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::web_app_data(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::web_app_data(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::web_app_data(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::web_app_data(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `write_access_allowed`.
    #[must_use]
    pub fn write_access_allowed(&self) -> Option<&crate::types::WriteAccessAllowed> {
        match self {
            Self::BusinessMessage(val) => {
                let inner = val.business_message.as_ref();
                crate::types::Message::write_access_allowed(inner)
            }
            Self::ChannelPost(val) => {
                let inner = val.channel_post.as_ref();
                crate::types::Message::write_access_allowed(inner)
            }
            Self::EditedBusinessMessage(val) => {
                let inner = val.edited_business_message.as_ref();
                crate::types::Message::write_access_allowed(inner)
            }
            Self::EditedChannelPost(val) => {
                let inner = val.edited_channel_post.as_ref();
                crate::types::Message::write_access_allowed(inner)
            }
            Self::EditedMessage(val) => {
                let inner = val.edited_message.as_ref();
                crate::types::Message::write_access_allowed(inner)
            }
            Self::GuestMessage(val) => {
                let inner = val.guest_message.as_ref();
                crate::types::Message::write_access_allowed(inner)
            }
            Self::Message(val) => {
                let inner = val.message.as_ref();
                crate::types::Message::write_access_allowed(inner)
            }
            _ => None,
        }
    }
}
impl<Client> crate::Extractor<Client> for Update {
    type Error = std::convert::Infallible;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone();
        async move { Ok(val) }
    }
}
impl<Client> crate::Extractor<Client> for std::sync::Arc<Update> {
    type Error = std::convert::Infallible;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = request.update.clone();
        async move { Ok(val) }
    }
}
impl TryFrom<Update> for crate::types::BotSubscriptionUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::Subscription(val) => Ok(val.subscription),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(BotSubscriptionUpdated),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::BotSubscriptionUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::BusinessConnection {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::BusinessConnection(val) => Ok(val.business_connection),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(BusinessConnection),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::BusinessConnection {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::BusinessMessagesDeleted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::DeletedBusinessMessages(val) => Ok(val.deleted_business_messages),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(BusinessMessagesDeleted),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::BusinessMessagesDeleted {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::CallbackQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::CallbackQuery(val) => Ok(val.callback_query),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(CallbackQuery),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::CallbackQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::ChatBoostRemoved {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::RemovedChatBoost(val) => Ok(val.removed_chat_boost),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(ChatBoostRemoved),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::ChatBoostRemoved {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::ChatBoostUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::ChatBoost(val) => Ok(val.chat_boost),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(ChatBoostUpdated),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::ChatBoostUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::ChatJoinRequest {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::ChatJoinRequest(val) => Ok(val.chat_join_request),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(ChatJoinRequest),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::ChatJoinRequest {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::ChatMemberUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::ChatMember(val) => Ok(val.chat_member),
            Update::MyChatMember(val) => Ok(val.my_chat_member),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(ChatMemberUpdated),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::ChatMemberUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::ChosenInlineResult {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::ChosenInlineResult(val) => Ok(val.chosen_inline_result),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(ChosenInlineResult),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::ChosenInlineResult {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::InlineQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::InlineQuery(val) => Ok(val.inline_query),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(InlineQuery),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::InlineQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::ManagedBotUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::ManagedBot(val) => Ok(val.managed_bot),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(ManagedBotUpdated),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::ManagedBotUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::Message {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::BusinessMessage(val) => Ok(*val.business_message),
            Update::ChannelPost(val) => Ok(*val.channel_post),
            Update::EditedBusinessMessage(val) => Ok(*val.edited_business_message),
            Update::EditedChannelPost(val) => Ok(*val.edited_channel_post),
            Update::EditedMessage(val) => Ok(*val.edited_message),
            Update::GuestMessage(val) => Ok(*val.guest_message),
            Update::Message(val) => Ok(*val.message),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(Message),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::Message {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageAnimation {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageLivePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageLivePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageVenue {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageVenue {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageAudio {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageAudio {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageBoostAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageBoostAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageChannelChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageChannelChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageChatBackgroundSet {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageChatBackgroundSet {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageChatOwnerChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageChatOwnerChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageChatOwnerLeft {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageChatOwnerLeft {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageChatShared {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageChatShared {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageChecklist {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageChecklist {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageChecklistTasksAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageChecklistTasksAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageChecklistTasksDone {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageChecklistTasksDone {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageCommunityChatAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageCommunityChatAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageCommunityChatRemoved {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageCommunityChatRemoved {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageConnectedWebsite {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageConnectedWebsite {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageContact {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageContact {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageDeleteChatPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageDeleteChatPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageDice {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageDice {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageDirectMessagePriceChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageDirectMessagePriceChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageDocument {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageForumTopicClosed {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageForumTopicClosed {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageForumTopicCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageForumTopicCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageForumTopicEdited {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageForumTopicEdited {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageForumTopicReopened {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageForumTopicReopened {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGame {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGame {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGeneralForumTopicHidden {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGeneralForumTopicHidden {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGeneralForumTopicUnhidden {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGeneralForumTopicUnhidden {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGift {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGift {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGiftUpgradeSent {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGiftUpgradeSent {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGiveaway {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGiveaway {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGiveawayCompleted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGiveawayCompleted {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGiveawayCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGiveawayCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGiveawayWinners {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGiveawayWinners {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageGroupChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageGroupChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageInvoice {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageInvoice {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageLeftChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageLeftChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageLocation {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageLocation {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageManagedBotCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageManagedBotCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageMessageAutoDeleteTimerChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageMessageAutoDeleteTimerChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageMigrateFromChatId {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageMigrateFromChatId {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageMigrateToChatId {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageMigrateToChatId {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageNewChatMembers {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageNewChatMembers {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageNewChatPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageNewChatPhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageNewChatTitle {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageNewChatTitle {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessagePaidMedia {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessagePaidMedia {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessagePaidMessagePriceChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessagePaidMessagePriceChanged {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessagePassportData {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessagePassportData {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessagePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessagePhoto {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessagePinnedMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessagePinnedMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessagePoll {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessagePoll {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessagePollOptionAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessagePollOptionAdded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessagePollOptionDeleted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessagePollOptionDeleted {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageProximityAlertTriggered {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageProximityAlertTriggered {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageRefundedPayment {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageRefundedPayment {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageRichMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageRichMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageSticker {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageSticker {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageStory {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageStory {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageSuccessfulPayment {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageSuccessfulPayment {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageSuggestedPostApprovalFailed {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageSuggestedPostApprovalFailed {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageSuggestedPostApproved {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageSuggestedPostApproved {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageSuggestedPostDeclined {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageSuggestedPostDeclined {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageSuggestedPostPaid {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageSuggestedPostPaid {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageSuggestedPostRefunded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageSuggestedPostRefunded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageSupergroupChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageSupergroupChatCreated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageText {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageText {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageUniqueGift {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageUniqueGift {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageUsersShared {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageUsersShared {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageVideo {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageVideoChatEnded {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageVideoChatEnded {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageVideoChatParticipantsInvited {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageVideoChatParticipantsInvited {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageVideoChatScheduled {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageVideoChatScheduled {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageVideoChatStarted {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageVideoChatStarted {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageVideoNote {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageVideoNote {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageVoice {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageVoice {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageWebAppData {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageWebAppData {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageWriteAccessAllowed {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageWriteAccessAllowed {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Message = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageReactionCountUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::MessageReactionCount(val) => Ok(val.message_reaction_count),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(MessageReactionCountUpdated),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageReactionCountUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::MessageReactionUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::MessageReaction(val) => Ok(val.message_reaction),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(MessageReactionUpdated),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::MessageReactionUpdated {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::PaidMediaPurchased {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::PurchasedPaidMedia(val) => Ok(val.purchased_paid_media),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(PaidMediaPurchased),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::PaidMediaPurchased {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::Poll {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::Poll(val) => Ok(*val.poll),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(Poll),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::Poll {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::PollRegular {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Poll = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::PollRegular {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::PollQuiz {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Poll = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::PollQuiz {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::PollUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        let parent: crate::types::Poll = val.try_into()?;
        parent.try_into()
    }
}
impl<Client> crate::Extractor<Client> for crate::types::PollUnknown {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::PollAnswer {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::PollAnswer(val) => Ok(val.poll_answer),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(PollAnswer),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::PollAnswer {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::PreCheckoutQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::PreCheckoutQuery(val) => Ok(val.pre_checkout_query),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(PreCheckoutQuery),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::PreCheckoutQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl TryFrom<Update> for crate::types::ShippingQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, crate::errors::ConvertToTypeError> {
        match val {
            Update::ShippingQuery(val) => Ok(val.shipping_query),
            _ => Err(crate::errors::ConvertToTypeError::new(
                stringify!(Update),
                stringify!(ShippingQuery),
            )),
        }
    }
}
impl<Client> crate::Extractor<Client> for crate::types::ShippingQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = (*request.update).clone().try_into();
        async move { val }
    }
}
impl From<crate::types::UpdateBusinessConnection> for Update {
    fn from(val: crate::types::UpdateBusinessConnection) -> Self {
        Self::BusinessConnection(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateBusinessConnection {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::BusinessConnection(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateBusinessConnection),
            ))
        }
    }
}
impl From<crate::types::UpdateBusinessMessage> for Update {
    fn from(val: crate::types::UpdateBusinessMessage) -> Self {
        Self::BusinessMessage(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateBusinessMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::BusinessMessage(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateBusinessMessage),
            ))
        }
    }
}
impl From<crate::types::UpdateCallbackQuery> for Update {
    fn from(val: crate::types::UpdateCallbackQuery) -> Self {
        Self::CallbackQuery(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateCallbackQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::CallbackQuery(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateCallbackQuery),
            ))
        }
    }
}
impl From<crate::types::UpdateChannelPost> for Update {
    fn from(val: crate::types::UpdateChannelPost) -> Self {
        Self::ChannelPost(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateChannelPost {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::ChannelPost(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateChannelPost),
            ))
        }
    }
}
impl From<crate::types::UpdateChatBoost> for Update {
    fn from(val: crate::types::UpdateChatBoost) -> Self {
        Self::ChatBoost(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateChatBoost {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::ChatBoost(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateChatBoost),
            ))
        }
    }
}
impl From<crate::types::UpdateChatJoinRequest> for Update {
    fn from(val: crate::types::UpdateChatJoinRequest) -> Self {
        Self::ChatJoinRequest(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateChatJoinRequest {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::ChatJoinRequest(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateChatJoinRequest),
            ))
        }
    }
}
impl From<crate::types::UpdateChatMember> for Update {
    fn from(val: crate::types::UpdateChatMember) -> Self {
        Self::ChatMember(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::ChatMember(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateChatMember),
            ))
        }
    }
}
impl From<crate::types::UpdateChosenInlineResult> for Update {
    fn from(val: crate::types::UpdateChosenInlineResult) -> Self {
        Self::ChosenInlineResult(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateChosenInlineResult {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::ChosenInlineResult(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateChosenInlineResult),
            ))
        }
    }
}
impl From<crate::types::UpdateDeletedBusinessMessages> for Update {
    fn from(val: crate::types::UpdateDeletedBusinessMessages) -> Self {
        Self::DeletedBusinessMessages(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateDeletedBusinessMessages {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::DeletedBusinessMessages(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateDeletedBusinessMessages),
            ))
        }
    }
}
impl From<crate::types::UpdateEditedBusinessMessage> for Update {
    fn from(val: crate::types::UpdateEditedBusinessMessage) -> Self {
        Self::EditedBusinessMessage(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateEditedBusinessMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::EditedBusinessMessage(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateEditedBusinessMessage),
            ))
        }
    }
}
impl From<crate::types::UpdateEditedChannelPost> for Update {
    fn from(val: crate::types::UpdateEditedChannelPost) -> Self {
        Self::EditedChannelPost(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateEditedChannelPost {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::EditedChannelPost(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateEditedChannelPost),
            ))
        }
    }
}
impl From<crate::types::UpdateEditedMessage> for Update {
    fn from(val: crate::types::UpdateEditedMessage) -> Self {
        Self::EditedMessage(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateEditedMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::EditedMessage(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateEditedMessage),
            ))
        }
    }
}
impl From<crate::types::UpdateGuestMessage> for Update {
    fn from(val: crate::types::UpdateGuestMessage) -> Self {
        Self::GuestMessage(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateGuestMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::GuestMessage(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateGuestMessage),
            ))
        }
    }
}
impl From<crate::types::UpdateInlineQuery> for Update {
    fn from(val: crate::types::UpdateInlineQuery) -> Self {
        Self::InlineQuery(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateInlineQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::InlineQuery(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateInlineQuery),
            ))
        }
    }
}
impl From<crate::types::UpdateManagedBot> for Update {
    fn from(val: crate::types::UpdateManagedBot) -> Self {
        Self::ManagedBot(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateManagedBot {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::ManagedBot(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateManagedBot),
            ))
        }
    }
}
impl From<crate::types::UpdateMessage> for Update {
    fn from(val: crate::types::UpdateMessage) -> Self {
        Self::Message(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::Message(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateMessage),
            ))
        }
    }
}
impl From<crate::types::UpdateMessageReaction> for Update {
    fn from(val: crate::types::UpdateMessageReaction) -> Self {
        Self::MessageReaction(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateMessageReaction {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::MessageReaction(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateMessageReaction),
            ))
        }
    }
}
impl From<crate::types::UpdateMessageReactionCount> for Update {
    fn from(val: crate::types::UpdateMessageReactionCount) -> Self {
        Self::MessageReactionCount(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateMessageReactionCount {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::MessageReactionCount(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateMessageReactionCount),
            ))
        }
    }
}
impl From<crate::types::UpdateMyChatMember> for Update {
    fn from(val: crate::types::UpdateMyChatMember) -> Self {
        Self::MyChatMember(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateMyChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::MyChatMember(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateMyChatMember),
            ))
        }
    }
}
impl From<crate::types::UpdatePoll> for Update {
    fn from(val: crate::types::UpdatePoll) -> Self {
        Self::Poll(val)
    }
}
impl TryFrom<Update> for crate::types::UpdatePoll {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::Poll(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(stringify!(Update), stringify!(UpdatePoll)))
        }
    }
}
impl From<crate::types::UpdatePollAnswer> for Update {
    fn from(val: crate::types::UpdatePollAnswer) -> Self {
        Self::PollAnswer(val)
    }
}
impl TryFrom<Update> for crate::types::UpdatePollAnswer {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::PollAnswer(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdatePollAnswer),
            ))
        }
    }
}
impl From<crate::types::UpdatePreCheckoutQuery> for Update {
    fn from(val: crate::types::UpdatePreCheckoutQuery) -> Self {
        Self::PreCheckoutQuery(val)
    }
}
impl TryFrom<Update> for crate::types::UpdatePreCheckoutQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::PreCheckoutQuery(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdatePreCheckoutQuery),
            ))
        }
    }
}
impl From<crate::types::UpdatePurchasedPaidMedia> for Update {
    fn from(val: crate::types::UpdatePurchasedPaidMedia) -> Self {
        Self::PurchasedPaidMedia(val)
    }
}
impl TryFrom<Update> for crate::types::UpdatePurchasedPaidMedia {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::PurchasedPaidMedia(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdatePurchasedPaidMedia),
            ))
        }
    }
}
impl From<crate::types::UpdateRemovedChatBoost> for Update {
    fn from(val: crate::types::UpdateRemovedChatBoost) -> Self {
        Self::RemovedChatBoost(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateRemovedChatBoost {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::RemovedChatBoost(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateRemovedChatBoost),
            ))
        }
    }
}
impl From<crate::types::UpdateShippingQuery> for Update {
    fn from(val: crate::types::UpdateShippingQuery) -> Self {
        Self::ShippingQuery(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateShippingQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::ShippingQuery(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateShippingQuery),
            ))
        }
    }
}
impl From<crate::types::UpdateSubscription> for Update {
    fn from(val: crate::types::UpdateSubscription) -> Self {
        Self::Subscription(val)
    }
}
impl TryFrom<Update> for crate::types::UpdateSubscription {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Update) -> Result<Self, Self::Error> {
        if let Update::Subscription(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Update),
                stringify!(UpdateSubscription),
            ))
        }
    }
}
