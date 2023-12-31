use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;

/// Use this method to close an open `General` topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#editgeneralforumtopic>
/// # Returns
/// Returns `true` on success
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct EditGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// New topic name, 1-128 characters
    pub name: String,
}

impl EditGeneralForumTopic {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, name: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            name: name.into(),
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
    pub fn name(self, val: impl Into<String>) -> Self {
        Self {
            name: val.into(),
            ..self
        }
    }
}

impl TelegramMethod for EditGeneralForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("editGeneralForumTopic", self, None)
    }
}

impl AsRef<EditGeneralForumTopic> for EditGeneralForumTopic {
    fn as_ref(&self) -> &Self {
        self
    }
}
