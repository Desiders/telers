use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, ReactionType},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to change the chosen reactions on a message. Service messages can't be reacted to. Automatically forwarded messages from a channel to its discussion group have the same available reactions as messages in the channel. In albums, bots must react to the first message.
/// # Documentation
/// <https://core.telegram.org/bots/api#setmessagereaction>
/// # Returns
/// Returns `true` on success
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct SetMessageReaction {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Identifier of the target message
    pub message_id: i64,
    /// New list of reaction types to set on the message. Currently, as non-premium users, bots can set up to one reaction per message. A custom emoji reaction can be used if it is either already present on the message or explicitly allowed by chat administrators.
    pub reaction: Option<Vec<ReactionType>>,
    /// Pass `true` to set the reaction with a big animation
    pub is_big: Option<bool>,
}

impl SetMessageReaction {
    #[must_use]
    pub fn new(chat_id: impl Into<ChatIdKind>, message_id: i64) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_id,
            reaction: None,
            is_big: None,
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
    pub fn message_id(self, val: i64) -> Self {
        Self {
            message_id: val,
            ..self
        }
    }

    #[must_use]
    pub fn reaction(self, val: impl Into<ReactionType>) -> Self {
        Self {
            reaction: Some(
                self.reaction
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val.into()))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn reactions<T, I>(self, val: I) -> Self
    where
        T: Into<ReactionType>,
        I: IntoIterator<Item = T>,
    {
        Self {
            reaction: Some(
                self.reaction
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val.into_iter().map(Into::into))
                    .collect(),
            ),
            ..self
        }
    }

    #[must_use]
    pub fn is_big(self, val: bool) -> Self {
        Self {
            is_big: Some(val),
            ..self
        }
    }
}

impl SetMessageReaction {
    #[must_use]
    pub fn reaction_option(self, val: Option<impl Into<ReactionType>>) -> Self {
        Self {
            reaction: val.map(|val| {
                self.reaction
                    .unwrap_or_default()
                    .into_iter()
                    .chain(Some(val.into()))
                    .collect()
            }),
            ..self
        }
    }

    #[must_use]
    pub fn reactions_option<T, I>(self, val: Option<I>) -> Self
    where
        T: Into<ReactionType>,
        I: IntoIterator<Item = T>,
    {
        Self {
            reaction: val.map(|val| {
                self.reaction
                    .unwrap_or_default()
                    .into_iter()
                    .chain(val.into_iter().map(Into::into))
                    .collect()
            }),
            ..self
        }
    }

    #[must_use]
    pub fn is_big_option(self, val: Option<bool>) -> Self {
        Self {
            is_big: val,
            ..self
        }
    }
}

impl TelegramMethod for SetMessageReaction {
    type Method = Self;
    type Return = bool;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("setMessageReaction", self, None)
    }
}

impl AsRef<SetMessageReaction> for SetMessageReaction {
    fn as_ref(&self) -> &Self {
        self
    }
}
