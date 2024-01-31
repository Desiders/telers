use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to edit name and icon of a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have can_manage_topics administrator rights, unless it is the creator of the topic.
/// # Documentation
/// <https://core.telegram.org/bots/api#editforumtopic>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct EditForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread of the forum topic
    pub message_thread_id: i64,
    /// New topic name, 0-128 characters. If not specified or empty, the current name of the topic will be kept
    pub name: Option<String>,
    /// New unique identifier of the custom emoji shown as the topic icon. Use [`GetForumTopicIconStickers`](crate::methods::GetForumTopicIconStickers) to get all allowed custom emoji identifiers. Pass an empty string to remove the icon. If not specified, the current icon will be kept.
    pub icon_custom_emoji_id: Option<String>,
}

impl EditForumTopic {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, message_thread_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id,
            name: None,
            icon_custom_emoji_id: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id(self, val: i64) -> Self {
        Self {
            message_thread_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn name(self, val: impl Into<String>) -> Self {
        Self {
            name: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn icon_custom_emoji_id(self, val: impl Into<String>) -> Self {
        Self {
            icon_custom_emoji_id: Some(val.into()),
            ..self
        }
    }
}

impl EditForumTopic {
    #[must_use]
    pub fn icon_custom_emoji_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            icon_custom_emoji_id: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for EditForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("editForumTopic", self, None)
    }
}

impl AsRef<EditForumTopic> for EditForumTopic {
    fn as_ref(&self) -> &Self {
        self
    }
}
