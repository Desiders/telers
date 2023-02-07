use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to reopen a closed `General` topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights. The topic will be automatically unhidden if it was hidden.
/// # Documentation
/// <https://core.telegram.org/bots/api#reopengeneralforumtopic>
/// # Returns
/// Returns `True` on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct ReopenGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
}

impl ReopenGeneralForumTopic {
    #[must_use]
    pub fn new<T: Into<ChatIdKind>>(chat_id: T) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }
}

impl TelegramMethod for ReopenGeneralForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request(&self, _bot: &Bot) -> Request<Self::Method> {
        Request::new("reopenGeneralForumTopic", self, None)
    }
}
