use serde::{Deserialize, Serialize};
/// Represents the scope of bot commands, covering a specific member of a group or supergroup chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#botcommandscopechatmember>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BotCommandScopeChatMember {
    /// Unique identifier for the target chat or username of the target supergroup in the format @username. Channel direct messages chats and channel chats aren't supported.
    pub chat_id: crate::types::ChatIdKind,
    /// Unique identifier of the target user
    pub user_id: i64,
}
impl BotCommandScopeChatMember {
    /// Creates a new `BotCommandScopeChatMember`.
    ///
    /// # Arguments
    /// * `chat_id` - Unique identifier for the target chat or username of the target supergroup in the format @username. Channel direct messages chats and channel chats aren't supported.
    /// * `user_id` - Unique identifier of the target user
    #[must_use]
    pub fn new<T0: Into<crate::types::ChatIdKind>, T1: Into<i64>>(
        chat_id: T0,
        user_id: T1,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            user_id: user_id.into(),
        }
    }

    /// Unique identifier for the target chat or username of the target supergroup in the format @username. Channel direct messages chats and channel chats aren't supported.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier of the target user
    #[must_use]
    pub fn user_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.user_id = val.into();
        self
    }
}
