use std::fmt::{self, Debug};

pub enum BotCommandScopeType {
    Default,
    AllPrivateChats,
    AllGroupChats,
    AllChatAdministrators,
    Chat,
    ChatAdministrators,
    ChatMember,
}

impl Debug for BotCommandScopeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
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

impl From<BotCommandScopeType> for String {
    fn from(scope: BotCommandScopeType) -> Self {
        scope.as_str().to_string()
    }
}

impl<'a> From<&'a BotCommandScopeType> for String {
    fn from(scope: &'a BotCommandScopeType) -> Self {
        scope.as_str().to_string()
    }
}
