use crate::FromContext;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// This object represents a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromContext)]
#[context(
    key = "event_chat",
    description = "This object represents a chat. \
    This context is available only if `UserContext` middleware is used (default middleware) and chat in `Update` is not empty."
)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Chat {
    Private(Box<Private>),
    Group(Box<Group>),
    Supergroup(Box<Supergroup>),
    Channel(Box<Channel>),
}

#[skip_serializing_none]
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Private {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Username
    pub username: Option<Box<str>>,
    /// First name of the other party
    pub first_name: Option<Box<str>>,
    /// Last name of the other party
    pub last_name: Option<Box<str>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Group {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title
    pub title: Box<str>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Supergroup {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title
    pub title: Box<str>,
    /// Username
    pub username: Option<Box<str>>,
    /// `true`, if the chat is a forum (has [`topics`](https://telegram.org/blog/topics-in-groups-collectible-usernames#topics-in-groups) enabled)
    pub is_forum: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Channel {
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub id: i64,
    /// Title
    pub title: Box<str>,
    /// Username
    pub username: Option<Box<str>>,
}

impl Chat {
    #[must_use]
    pub const fn id(&self) -> i64 {
        match self {
            Self::Private(chat) => chat.id,
            Self::Group(chat) => chat.id,
            Self::Supergroup(chat) => chat.id,
            Self::Channel(chat) => chat.id,
        }
    }

    #[must_use]
    pub const fn title(&self) -> Option<&str> {
        match self {
            Self::Private(_) => None,
            Self::Group(chat) => Some(&chat.title),
            Self::Supergroup(chat) => Some(&chat.title),
            Self::Channel(chat) => Some(&chat.title),
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn username(&self) -> Option<&str> {
        match self {
            Self::Group(_) => None,
            Self::Private(chat) => match chat.username {
                Some(ref username) => Some(username),
                None => None,
            },
            Self::Supergroup(chat) => match chat.username {
                Some(ref username) => Some(username),
                None => None,
            },
            Self::Channel(chat) => match chat.username {
                Some(ref username) => Some(username),
                None => None,
            },
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn first_name(&self) -> Option<&str> {
        match self {
            Self::Private(chat) => match chat.first_name {
                Some(ref first_name) => Some(first_name),
                None => None,
            },
            _ => None,
        }
    }

    #[allow(clippy::match_as_ref)]
    #[must_use]
    pub const fn last_name(&self) -> Option<&str> {
        match self {
            Self::Private(chat) => match chat.last_name {
                Some(ref last_name) => Some(last_name),
                None => None,
            },
            _ => None,
        }
    }

    #[must_use]
    pub const fn is_forum(&self) -> Option<bool> {
        match self {
            Self::Supergroup(chat) => chat.is_forum,
            _ => None,
        }
    }
}

impl Default for Chat {
    #[must_use]
    fn default() -> Self {
        Self::Private(Box::default())
    }
}
