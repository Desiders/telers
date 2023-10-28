use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};

/// This enum represents all possible types of the bot command scope
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscope>
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BotCommandScopeType {
    Default,
    AllPrivateChats,
    AllGroupChats,
    AllChatAdministrators,
    Chat,
    ChatAdministrators,
    ChatMember,
}

impl BotCommandScopeType {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            BotCommandScopeType::Default => "default",
            BotCommandScopeType::AllPrivateChats => "all_private_chats",
            BotCommandScopeType::AllGroupChats => "all_group_chats",
            BotCommandScopeType::AllChatAdministrators => "all_chat_administrators",
            BotCommandScopeType::Chat => "chat",
            BotCommandScopeType::ChatAdministrators => "chat_administrators",
            BotCommandScopeType::ChatMember => "chat_member",
        }
    }

    #[must_use]
    pub const fn all() -> &'static [BotCommandScopeType; 7] {
        &[
            BotCommandScopeType::Default,
            BotCommandScopeType::AllPrivateChats,
            BotCommandScopeType::AllGroupChats,
            BotCommandScopeType::AllChatAdministrators,
            BotCommandScopeType::Chat,
            BotCommandScopeType::ChatAdministrators,
            BotCommandScopeType::ChatMember,
        ]
    }
}

impl Deref for BotCommandScopeType {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl Display for BotCommandScopeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<BotCommandScopeType> for Box<str> {
    fn from(scope: BotCommandScopeType) -> Self {
        scope.into()
    }
}

impl From<BotCommandScopeType> for String {
    fn from(scope: BotCommandScopeType) -> Self {
        scope.as_str().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for BotCommandScopeType {
    fn eq(&self, other: &&'a str) -> bool {
        self == other
    }
}
