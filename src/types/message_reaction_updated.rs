use super::{Chat, ReactionType, Update, UpdateKind, User};

use crate::errors::ConvertToTypeError;

use serde::Deserialize;

/// This object represents a change of a reaction on a message performed by a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#messagereactionupdated>
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
pub struct MessageReactionUpdated {
    /// The chat containing the message the user reacted to
    pub chat: Chat,
    /// Unique identifier of the message inside the chat
    #[serde(rename = "message_id")]
    pub id: i64,
    /// The user that changed the reaction, if the user isn't anonymous
    pub user: Option<User>,
    /// The chat on behalf of which the reaction was changed, if the user is anonymous
    pub actor_chat: Option<Chat>,
    /// Date of the change in Unix time
    pub date: i64,
    /// Previous list of reaction types that were set by the user
    pub old_reaction: Box<[ReactionType]>,
    /// New list of reaction types that have been set by the user
    pub new_reaction: Box<[ReactionType]>,
}

impl TryFrom<Update> for MessageReactionUpdated {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::MessageReaction(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "MessageReaction")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn serialize() {
        let jsons = [
            serde_json::json!(
                {
                    "chat": {
                        "id": 1,
                        "title": "test",
                        "type": "supergroup"
                    },
                    "message_id": 1,
                    "user": {
                        "id": 1,
                        "is_bot": false,
                        "first_name": "first_name"
                    },
                    "date": 1,
                    "old_reaction": [
                        {"type":"emoji","emoji":"👍"},
                        {"type":"custom_emoji","custom_emoji":"123"}
                    ],
                    "new_reaction": [
                        {"type":"custom_emoji","custom_emoji":"123"},
                        {"type":"emoji","emoji":"👍"}
                    ]
                }
            ),
            serde_json::json!(
            {
                "chat": {
                    "id": 1,
                    "title": "test",
                    "type": "supergroup"
                },
                "message_id": 1,
                "actor_chat": {
                    "id": 1,
                    "title": "test",
                    "type": "supergroup"
                },
                "date": 1,
                "old_reaction": [
                    {"type":"emoji","emoji":"👍"},
                    {"type":"custom_emoji","custom_emoji":"123"}
                ],
                "new_reaction": [
                    {"type":"custom_emoji","custom_emoji":"123"},
                    {"type":"emoji","emoji":"👍"}
                ]
            }),
        ];

        for json in jsons.iter() {
            let _: MessageReactionUpdated = serde_json::from_value(json.clone()).unwrap();
        }
    }
}
