use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ForumTopic},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to create a topic in a forum supergroup chat. The bot must be an administrator in the chat for this to work and must have the `can_manage_topics` administrator rights.
/// # Documentation
/// <https://core.telegram.org/bots/api#createforumtopic>
/// # Returns
/// Returns information about the created topic as a [`ForumTopic`] object
#[skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CreateForumTopic {
    /// Unique identifier for the target chat or username of the target supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatIdKind,
    /// Topic name, 1-128 characters
    pub title: String,
    /// Color of the topic icon in RGB format. Currently, must be one of 7322096 (0x6FB9F0), 16766590 (0xFFD67E), 13338331 (0xCB86DB), 9367192 (0x8EEE98), 16749490 (0xFF93B2), or 16478047 (0xFB6F5F)
    pub icon_color: Option<String>,
    /// Unique identifier of the custom emoji shown as the topic icon. Use [`GetForumTopicIconStickers`](crate::methods::GetForumTopicIconStickers) to get all allowed custom emoji identifiers.
    pub icon_custom_emoji_id: Option<String>,
}

impl CreateForumTopic {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, title: impl Into<String>) -> Self {
        Self {
            chat_id: chat_id.into(),
            title: title.into(),
            icon_color: None,
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
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn icon_color(self, val: impl Into<String>) -> Self {
        Self {
            icon_color: Some(val.into()),
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

impl CreateForumTopic {
    #[must_use]
    pub fn icon_color_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            icon_color: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn icon_custom_emoji_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            icon_custom_emoji_id: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for CreateForumTopic {
    type Method = Self;
    type Return = ForumTopic;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("createForumTopic", self, None)
    }
}
