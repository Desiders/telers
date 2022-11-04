use super::{
    CallbackQuery, Chat, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult, InlineQuery,
    Message, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, User,
};

use serde::{Deserialize, Serialize};

/// This `object <https://core.telegram.org/bots/api#available-types>` represents an incoming update.
/// At most **one** of the optional parameters can be present in any given update.
/// <https://core.telegram.org/bots/api#update>
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you're using `webhooks <https://core.telegram.org/bots/api#setwebhook>`, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// *Optional*. New incoming message of any kind — text, photo, sticker, etc.
    pub message: Option<Message>,
    /// *Optional*. New version of a message that is known to the bot and was edited
    pub edited_message: Option<Message>,
    /// *Optional*. New incoming channel post of any kind — text, photo, sticker, etc.
    pub channel_post: Option<Message>,
    /// *Optional*. New version of a channel post that is known to the bot and was edited
    pub edited_channel_post: Option<Message>,
    /// *Optional*. New incoming inline query
    pub inline_query: Option<InlineQuery>,
    /// *Optional*. The result of an inline query that was chosen by a user and sent to their chat partner. Please see our documentation on the `feedback collecting <https://core.telegram.org/bots/inline#collecting-feedback>` for details on how to enable these updates for your bot.
    pub chosen_inline_result: Option<ChosenInlineResult>,
    /// *Optional*. New incoming callback query
    pub callback_query: Option<CallbackQuery>,
    /// *Optional*. New incoming shipping query. Only for invoices with flexible price
    pub shipping_query: Option<ShippingQuery>,
    /// *Optional*. New incoming pre-checkout query. Contains full information about checkout
    pub pre_checkout_query: Option<PreCheckoutQuery>,
    /// *Optional*. New poll state. Bots receive only updates about stopped polls and polls, which are sent by the bot
    pub poll: Option<Poll>,
    /// *Optional*. A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    pub poll_answer: Option<PollAnswer>,
    /// *Optional*. New incoming my_chat_member update.
    pub my_chat_member: Option<ChatMemberUpdated>,
    /// *Optional*. New incoming chat_member update.
    pub chat_member: Option<ChatMemberUpdated>,
    /// *Optional*. A request to join the chat has been sent. The bot must have the *can_invite_users* administrator right in the chat to receive these updates.
    pub chat_join_request: Option<ChatJoinRequest>,
}

impl Update {
    /// Alias for `update_id`
    #[must_use]
    pub fn id(&self) -> i64 {
        self.update_id
    }

    #[must_use]
    pub fn user(&self) -> Option<&User> {
        if let Some(message) = &self.message {
            message.from.as_ref()
        } else if let Some(message) = &self.edited_message {
            message.from.as_ref()
        } else if let Some(message) = &self.channel_post {
            message.from.as_ref()
        } else if let Some(message) = &self.edited_channel_post {
            message.from.as_ref()
        } else if let Some(inline_query) = &self.inline_query {
            Some(&inline_query.from)
        } else if let Some(chosen_inline_result) = &self.chosen_inline_result {
            Some(&chosen_inline_result.from)
        } else if let Some(callback_query) = &self.callback_query {
            Some(&callback_query.from)
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
        } else {
            None
        }
    }

    #[must_use]
    pub fn chat(&self) -> Option<&Chat> {
        if let Some(message) = &self.message {
            Some(&message.chat)
        } else if let Some(message) = &self.edited_message {
            Some(&message.chat)
        } else if let Some(message) = &self.channel_post {
            Some(&message.chat)
        } else if let Some(message) = &self.edited_channel_post {
            Some(&message.chat)
        } else if let Some(callback_query) = &self.callback_query {
            Some(&callback_query.message.as_ref()?.chat)
        } else if let Some(chat_member_updated) = &self.my_chat_member {
            Some(&chat_member_updated.chat)
        } else if let Some(chat_member_updated) = &self.chat_member {
            Some(&chat_member_updated.chat)
        } else if let Some(chat_join_request) = &self.chat_join_request {
            Some(&chat_join_request.chat)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Chat, Message, Update, User};

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "update_id": 123,
            "message": {
                "message_id": 1,
                "from": {
                    "id": 1,
                    "is_bot": false,
                    "first_name": "first_name",
                    "last_name": "last_name",
                    "username": "username",
                    "language_code": "language_code"
                },
                "chat": {
                    "id": 1,
                    "first_name": "first_name",
                    "last_name": "last_name",
                    "username": "username",
                    "type": "private"
                },
                "date": 1,
                "text": "text"
            }
        }"#;

        let deserializer = &mut serde_json::Deserializer::from_str(json);
        let result: Result<Update, _> = serde_path_to_error::deserialize(deserializer);

        match result {
            Ok(update) => {
                assert_eq!(
                    update.message,
                    Some(Message {
                        message_id: 1,
                        from: Some(User {
                            id: 1,
                            is_bot: false,
                            first_name: "first_name".to_string(),
                            last_name: Some("last_name".to_string()),
                            username: Some("username".to_string()),
                            language_code: Some("language_code".to_string()),
                            ..Default::default()
                        }),
                        date: 1,
                        chat: Box::new(Chat {
                            id: 1,
                            first_name: Some("first_name".to_string()),
                            last_name: Some("last_name".to_string()),
                            username: Some("username".to_string()),
                            chat_type: "private".to_string(),
                            ..Default::default()
                        }),
                        text: Some("text".to_string()),
                        ..Default::default()
                    }),
                );
            }
            Err(err) => {
                println!("Path: {}", err.path());

                let _: Update = serde_json::from_str(json).unwrap(); // for traceback
            }
        };
    }
}
