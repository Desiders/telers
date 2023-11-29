use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};

/// This enum represents all possible types of the bot command scope
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscope>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum BotCommandScopeType {
    #[strum(serialize = "default")]
    Default,
    #[strum(serialize = "all_private_chats")]
    AllPrivateChats,
    #[strum(serialize = "all_group_chats")]
    AllGroupChats,
    #[strum(serialize = "all_chat_administrators")]
    AllChatAdministrators,
    #[strum(serialize = "chat")]
    Chat,
    #[strum(serialize = "chat_administrators")]
    ChatAdministrators,
    #[strum(serialize = "chat_member")]
    ChatMember,
}

impl BotCommandScopeType {
    #[must_use]
    pub const fn all() -> [BotCommandScopeType; 7] {
        [
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

impl From<BotCommandScopeType> for Box<str> {
    fn from(scope: BotCommandScopeType) -> Self {
        Into::<&'static str>::into(scope).into()
    }
}

impl From<BotCommandScopeType> for String {
    fn from(scope: BotCommandScopeType) -> Self {
        scope.as_ref().to_owned()
    }
}

impl<'a> PartialEq<&'a str> for BotCommandScopeType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
