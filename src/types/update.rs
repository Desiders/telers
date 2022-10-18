use super::{Chat, UpdateKind, User};

use serde::{Deserialize, Serialize};

/// This `object <https://core.telegram.org/bots/api#available-types>`_ represents an incoming update.
/// At most **one** of the optional parameters can be present in any given update.
/// <https://core.telegram.org/bots/api#update>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Update {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This ID becomes especially handy if you're using `webhooks <https://core.telegram.org/bots/api#setwebhook>`_, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming update of any kind — `Message`, `EditedMessage`, etc.
    #[serde(flatten)]
    pub kind: UpdateKind,
}

impl Update {
    /// Alias for `update_id`
    #[must_use]
    pub fn id(&self) -> i64 {
        self.update_id
    }

    #[must_use]
    pub fn user(&self) -> Option<&User> {
        match &self.kind {
            UpdateKind::Message(message)
            | UpdateKind::EditedMessage(message)
            | UpdateKind::ChannelPost(message)
            | UpdateKind::EditedChannelPost(message) => message.from.as_ref(),
            UpdateKind::InlineQuery(inline_query) => Some(&inline_query.from),
            UpdateKind::ChosenInlineResult(chosen_inline_result) => {
                Some(&chosen_inline_result.from)
            }
            UpdateKind::CallbackQuery(callback_query) => Some(&callback_query.from),
            UpdateKind::ShippingQuery(shipping_query) => Some(&shipping_query.from),
            UpdateKind::PreCheckoutQuery(pre_checkout_query) => Some(&pre_checkout_query.from),
            UpdateKind::PollAnswer(poll_answer) => Some(&poll_answer.user),
            UpdateKind::MyChatMember(chat_member_updated)
            | UpdateKind::ChatMember(chat_member_updated) => Some(&chat_member_updated.from),
            _ => None,
        }
    }

    #[must_use]
    pub fn chat(&self) -> Option<&Chat> {
        match &self.kind {
            UpdateKind::Message(message)
            | UpdateKind::EditedMessage(message)
            | UpdateKind::ChannelPost(message)
            | UpdateKind::EditedChannelPost(message) => Some(&message.chat),
            UpdateKind::CallbackQuery(callback_query) => {
                Some(&callback_query.message.as_ref()?.chat)
            }
            UpdateKind::MyChatMember(chat_member_updated)
            | UpdateKind::ChatMember(chat_member_updated) => Some(&chat_member_updated.chat),
            UpdateKind::ChatJoinRequest(chat_join_request) => Some(&chat_join_request.chat),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Update, UpdateKind};

    #[test]
    fn test_deserialize() {
        let json = r#"{
            "update_id": 123456789,
            "message": {
                "message_id": 1,
                "from": {
                    "id": 123456789,
                    "is_bot": false,
                    "first_name": "John",
                    "last_name": "Doe",
                    "username": "johndoe",
                    "language_code": "en"
                },
                "chat": {
                    "id": 123456789,
                    "first_name": "John",
                    "last_name": "Doe",
                    "username": "johndoe",
                    "type": "private"
                },
                "date": 1600000000,
                "text": "Hello, world!"
            }
        }"#;

        let update: Update = serde_json::from_str(json).unwrap();

        assert_eq!(update.update_id, 123456789);
        assert!(matches!(update.kind, UpdateKind::Message(_)));
    }
}
