use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to unhide the `General` topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#unhidegeneralforumtopic>
/// # Returns
/// Returns `True` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct UnhideGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
}

impl UnhideGeneralForumTopic {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
        }
    }
}

impl TelegramMethod for UnhideGeneralForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("unhideGeneralForumTopic", self, None)
    }
}

impl AsRef<UnhideGeneralForumTopic> for UnhideGeneralForumTopic {
    fn as_ref(&self) -> &Self {
        self
    }
}
