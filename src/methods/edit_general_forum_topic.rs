use super::base::{Request, TelegramMethod};

use crate::{client::Bot, types::ChatIdKind};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to close an open `General` topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#editgeneralforumtopic>
/// # Returns
/// Returns `True` on success.
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct EditGeneralForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// New topic name, 1-128 characters
    pub name: String,
}

impl EditGeneralForumTopic {
    #[must_use]
    pub fn new<C: Into<ChatIdKind>, T: Into<String>>(chat_id: C, name: T) -> Self {
        Self {
            chat_id: chat_id.into(),
            name: name.into(),
        }
    }

    #[must_use]
    pub fn chat_id<T: Into<ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    #[must_use]
    pub fn name<T: Into<String>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
}

impl TelegramMethod for EditGeneralForumTopic {
    type Method = Self;
    type Return = bool;

    fn build_request(&self, _: &Bot) -> Request<Self::Method> {
        Request::new("editGeneralForumTopic", self, None)
    }
}
