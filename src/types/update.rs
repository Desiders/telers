use super::{
    CallbackQuery, Chat, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery,
    Message, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, User,
};

use serde::Deserialize;

/// This [`object`](https://core.telegram.org/bots/api#available-types) represents an incoming update.
/// At most **one** of the optional parameters can be present in any given update.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you're using [`webhooks`](https://core.telegram.org/bots/api#setwebhook), since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming message of any kind — text, photo, sticker, etc.
    pub message: Option<Message>,
    /// New version of a message that is known to the bot and was edited
    pub edited_message: Option<Message>,
    /// New incoming channel post of any kind — text, photo, sticker, etc.
    pub channel_post: Option<Message>,
    /// New version of a channel post that is known to the bot and was edited
    pub edited_channel_post: Option<Message>,
    /// New incoming inline query
    pub inline_query: Option<InlineQuery>,
    /// The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the [`feedback collecting`](https://core.telegram.org/bots/inline#collecting-feedback) for details on how to enable these updates for your bot.
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// New incoming callback query
    pub callback_query: Option<CallbackQuery>,
    /// New incoming shipping query. Only for invoices with flexible price
    pub shipping_query: Option<ShippingQuery>,
    /// New incoming pre-checkout query. Contains full information about checkout
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    pub poll: Option<Poll>,
    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    pub poll_answer: Option<PollAnswer>,
    /// New incoming my_chat_member update.
    pub my_chat_member: Option<ChatMemberUpdated>,
    /// New incoming chat_member update.
    pub chat_member: Option<ChatMemberUpdated>,
    /// A request to join the chat has been sent. The bot must have the *can_invite_users* administrator right in the chat to receive these updates.
    pub chat_join_request: Option<ChatJoinRequest>,
}

impl Update {
    /// Returns the [`User`] who sent the update
    #[must_use]
    pub fn user(&self) -> Option<&User> {
        if let Some(message) = &self.message {
            message.from.as_ref()
        } else if let Some(inline_query) = &self.inline_query {
            Some(&inline_query.from)
        } else if let Some(chosen_inline_result) = &self.chosen_inline_result {
            Some(&chosen_inline_result.from)
        } else if let Some(callback_query) = &self.callback_query {
            Some(&callback_query.from)
        } else if let Some(message) = &self.edited_message {
            message.from.as_ref()
        } else if let Some(shipping_query) = &self.shipping_query {
            Some(&shipping_query.from)
        } else if let Some(pre_checkout_query) = &self.pre_checkout_query {
            Some(&pre_checkout_query.from)
        } else if let Some(poll_answer) = &self.poll_answer {
            Some(&poll_answer.user)
        } else if let Some(chat_member_updated) = &self.my_chat_member {
            Some(&chat_member_updated.from)
        } else if let Some(chat_member_updated) = &self.chat_member {
            Some(&chat_member_updated.from)
        } else if let Some(chat_join_request) = &self.chat_join_request {
            Some(&chat_join_request.from)
        // } else if self.channel_post.is_some()
        //     | self.edited_channel_post.is_some()
        //     | self.poll.is_some()
        // {
        //     // These updates don't have a user, so we return `None`
        //     None
        } else {
            None
        }
    }

    /// Alias to [`Update::user`] method
    #[must_use]
    pub fn from(&self) -> Option<&User> {
        self.user()
    }

    /// Returns the [`Chat`] where the update was sent
    #[must_use]
    pub fn chat(&self) -> Option<&Chat> {
        if let Some(message) = &self.message {
            Some(&message.chat)
        } else if let Some(callback_query) = &self.callback_query {
            Some(&callback_query.message.as_ref()?.chat)
        } else if let Some(message) = &self.channel_post {
            Some(&message.chat)
        } else if let Some(message) = &self.edited_message {
            Some(&message.chat)
        } else if let Some(message) = &self.edited_channel_post {
            Some(&message.chat)
        } else if let Some(chat_member_updated) = &self.my_chat_member {
            Some(&chat_member_updated.chat)
        } else if let Some(chat_member_updated) = &self.chat_member {
            Some(&chat_member_updated.chat)
        } else if let Some(chat_join_request) = &self.chat_join_request {
            Some(&chat_join_request.chat)
        // } else if self.inline_query.is_some()
        //     | self.chosen_inline_result.is_some()
        //     | self.shipping_query.is_some()
        //     | self.pre_checkout_query.is_some()
        //     | self.poll.is_some()
        //     | self.poll_answer.is_some()
        // {
        //     // These updates don't have a chat, so we return `None`
        //     None
        } else {
            None
        }
    }

    /// Shortcut to get both [`User`] and [`Chat`] from the update
    #[must_use]
    pub fn user_and_chat(&self) -> (Option<&User>, Option<&Chat>) {
        (self.user(), self.chat())
    }
}
