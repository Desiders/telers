use serde::{Deserialize, Serialize};
/// Represents the scope of bot commands, covering all administrators of a specific group or supergroup chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopechatadministrators>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministrators {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername). Channel direct messages chats and channel chats aren't supported.
    pub chat_id: crate::types::ChatIdKind,
}
impl BotCommandScopeChatAdministrators {
    /// Creates a new `BotCommandScopeChatAdministrators`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername). Channel direct messages chats and channel chats aren't supported.
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>>(chat_id: T0) -> Self {
        Self {
            chat_id: chat_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername). Channel direct messages chats and channel chats aren't supported.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = val.into();
        this
    }
}
