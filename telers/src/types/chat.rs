use serde::{Deserialize, Serialize};
/// This object represents a chat.
/// Currently, it can be one of
/// - [`ChatChannel`]
/// - [`ChatGroup`]
/// - [`ChatPrivate`]
/// - [`ChatSupergroup`]
/// # Documentation
/// <https://core.telegram.org/bots/api#chat>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Chat {
    Private(crate::types::ChatPrivate),
    Group(crate::types::ChatGroup),
    Supergroup(crate::types::ChatSupergroup),
    Channel(crate::types::ChatChannel),
}
impl Chat {
    /// Helper method for field `first_name`.
    ///
    /// First name of the other party in a private chat
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.first_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `id`.
    ///
    /// Unique identifier for this chat. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    #[must_use]
    pub fn id(&self) -> i64 {
        match self {
            Self::Private(val) => val.id,
            Self::Group(val) => val.id,
            Self::Supergroup(val) => val.id,
            Self::Channel(val) => val.id,
        }
    }

    /// Helper method for field `is_direct_messages`.
    ///
    /// `true`, if the chat is the direct messages chat of a channel
    #[must_use]
    pub fn is_direct_messages(&self) -> Option<bool> {
        match self {
            Self::Private(val) => val.is_direct_messages,
            Self::Group(val) => val.is_direct_messages,
            Self::Supergroup(val) => val.is_direct_messages,
            Self::Channel(val) => val.is_direct_messages,
        }
    }

    /// Helper method for field `is_forum`.
    ///
    /// `true`, if the supergroup chat is a forum (has topics enabled)
    #[must_use]
    pub fn is_forum(&self) -> Option<bool> {
        match self {
            Self::Supergroup(val) => val.is_forum,
            _ => None,
        }
    }

    /// Helper method for field `last_name`.
    ///
    /// Last name of the other party in a private chat
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.last_name.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `title`.
    ///
    /// Title, for supergroups, channels and group chats
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Supergroup(val) => val.title.as_deref(),
            Self::Channel(val) => val.title.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `username`.
    ///
    /// Username, for private chats, supergroups and channels if available
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::Private(val) => val.username.as_deref(),
            Self::Supergroup(val) => val.username.as_deref(),
            Self::Channel(val) => val.username.as_deref(),
            Self::Group(_) => None,
        }
    }
}
impl From<crate::types::ChatPrivate> for Chat {
    fn from(val: crate::types::ChatPrivate) -> Self {
        Self::Private(val)
    }
}
impl TryFrom<Chat> for crate::types::ChatPrivate {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Chat) -> Result<Self, Self::Error> {
        if let Chat::Private(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(stringify!(Chat), stringify!(ChatPrivate)))
        }
    }
}
impl From<crate::types::ChatGroup> for Chat {
    fn from(val: crate::types::ChatGroup) -> Self {
        Self::Group(val)
    }
}
impl TryFrom<Chat> for crate::types::ChatGroup {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Chat) -> Result<Self, Self::Error> {
        if let Chat::Group(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(stringify!(Chat), stringify!(ChatGroup)))
        }
    }
}
impl From<crate::types::ChatSupergroup> for Chat {
    fn from(val: crate::types::ChatSupergroup) -> Self {
        Self::Supergroup(val)
    }
}
impl TryFrom<Chat> for crate::types::ChatSupergroup {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Chat) -> Result<Self, Self::Error> {
        if let Chat::Supergroup(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(Chat),
                stringify!(ChatSupergroup),
            ))
        }
    }
}
impl From<crate::types::ChatChannel> for Chat {
    fn from(val: crate::types::ChatChannel) -> Self {
        Self::Channel(val)
    }
}
impl TryFrom<Chat> for crate::types::ChatChannel {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: Chat) -> Result<Self, Self::Error> {
        if let Chat::Channel(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(stringify!(Chat), stringify!(ChatChannel)))
        }
    }
}
