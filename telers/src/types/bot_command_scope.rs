use serde::{Deserialize, Serialize};
/// This object represents the scope to which bot commands are applied. Currently, the following 7 scopes are supported:
/// - [`BotCommandScopeDefault`]
/// - [`BotCommandScopeAllPrivateChats`]
/// - [`BotCommandScopeAllGroupChats`]
/// - [`BotCommandScopeAllChatAdministrators`]
/// - [`BotCommandScopeChat`]
/// - [`BotCommandScopeChatAdministrators`]
/// - [`BotCommandScopeChatMember`]
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscope>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BotCommandScope {
    Default(crate::types::BotCommandScopeDefault),
    AllPrivateChats(crate::types::BotCommandScopeAllPrivateChats),
    AllGroupChats(crate::types::BotCommandScopeAllGroupChats),
    AllChatAdministrators(crate::types::BotCommandScopeAllChatAdministrators),
    Chat(crate::types::BotCommandScopeChat),
    ChatAdministrators(crate::types::BotCommandScopeChatAdministrators),
    ChatMember(crate::types::BotCommandScopeChatMember),
}
impl BotCommandScope {
    /// Helper method for field `chat_id`.
    ///
    /// # Variants
    /// - `BotCommandScopeChat`. Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername). Channel direct messages chats and channel chats aren't supported.
    /// - `BotCommandScopeChatAdministrators`. Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername). Channel direct messages chats and channel chats aren't supported.
    /// - `BotCommandScopeChatMember`. Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername). Channel direct messages chats and channel chats aren't supported.
    #[must_use]
    pub fn chat_id(&self) -> Option<&crate::types::ChatIdKind> {
        match self {
            Self::Chat(val) => Some(&val.chat_id),
            Self::ChatAdministrators(val) => Some(&val.chat_id),
            Self::ChatMember(val) => Some(&val.chat_id),
            _ => None,
        }
    }

    /// Helper method for field `user_id`.
    ///
    /// # Variants
    /// - `BotCommandScopeChatMember`. Unique identifier of the target user
    #[must_use]
    pub fn user_id(&self) -> Option<i64> {
        match self {
            Self::ChatMember(val) => Some(val.user_id),
            _ => None,
        }
    }
}
impl From<crate::types::BotCommandScopeDefault> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeDefault) -> Self {
        Self::Default(val)
    }
}
impl TryFrom<BotCommandScope> for crate::types::BotCommandScopeDefault {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BotCommandScope) -> Result<Self, Self::Error> {
        if let BotCommandScope::Default(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BotCommandScope),
                stringify!(BotCommandScopeDefault),
            ))
        }
    }
}
impl From<crate::types::BotCommandScopeAllPrivateChats> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeAllPrivateChats) -> Self {
        Self::AllPrivateChats(val)
    }
}
impl TryFrom<BotCommandScope> for crate::types::BotCommandScopeAllPrivateChats {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BotCommandScope) -> Result<Self, Self::Error> {
        if let BotCommandScope::AllPrivateChats(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BotCommandScope),
                stringify!(BotCommandScopeAllPrivateChats),
            ))
        }
    }
}
impl From<crate::types::BotCommandScopeAllGroupChats> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeAllGroupChats) -> Self {
        Self::AllGroupChats(val)
    }
}
impl TryFrom<BotCommandScope> for crate::types::BotCommandScopeAllGroupChats {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BotCommandScope) -> Result<Self, Self::Error> {
        if let BotCommandScope::AllGroupChats(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BotCommandScope),
                stringify!(BotCommandScopeAllGroupChats),
            ))
        }
    }
}
impl From<crate::types::BotCommandScopeAllChatAdministrators> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeAllChatAdministrators) -> Self {
        Self::AllChatAdministrators(val)
    }
}
impl TryFrom<BotCommandScope> for crate::types::BotCommandScopeAllChatAdministrators {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BotCommandScope) -> Result<Self, Self::Error> {
        if let BotCommandScope::AllChatAdministrators(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BotCommandScope),
                stringify!(BotCommandScopeAllChatAdministrators),
            ))
        }
    }
}
impl From<crate::types::BotCommandScopeChat> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeChat) -> Self {
        Self::Chat(val)
    }
}
impl TryFrom<BotCommandScope> for crate::types::BotCommandScopeChat {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BotCommandScope) -> Result<Self, Self::Error> {
        if let BotCommandScope::Chat(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BotCommandScope),
                stringify!(BotCommandScopeChat),
            ))
        }
    }
}
impl From<crate::types::BotCommandScopeChatAdministrators> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeChatAdministrators) -> Self {
        Self::ChatAdministrators(val)
    }
}
impl TryFrom<BotCommandScope> for crate::types::BotCommandScopeChatAdministrators {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BotCommandScope) -> Result<Self, Self::Error> {
        if let BotCommandScope::ChatAdministrators(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BotCommandScope),
                stringify!(BotCommandScopeChatAdministrators),
            ))
        }
    }
}
impl From<crate::types::BotCommandScopeChatMember> for BotCommandScope {
    fn from(val: crate::types::BotCommandScopeChatMember) -> Self {
        Self::ChatMember(val)
    }
}
impl TryFrom<BotCommandScope> for crate::types::BotCommandScopeChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: BotCommandScope) -> Result<Self, Self::Error> {
        if let BotCommandScope::ChatMember(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(BotCommandScope),
                stringify!(BotCommandScopeChatMember),
            ))
        }
    }
}
