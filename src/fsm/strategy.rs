use std::fmt::{self, Display};

/// Strategy for storing and retrieving data.
///
/// If you use `UserInChat` strategy, you have possible to store different data and state for different chats.
/// If you use `Chat` strategy, then all users in the chat will have the same data and state.
/// If you use `GlobalUser` strategy, then the user will have the same data and state in all chats.
///
/// In case of direct messages, `chat_id` and `user_id` will be equal, so all strategies will work the same way.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Strategy {
    /// `user_id` + `chat_id`
    UserInChat,
    /// `chat_id` + `chat_id`
    Chat,
    /// `user_id` + `user_id`
    GlobalUser,
    /// `user_id` + `chat_id` + `message_thread_id`
    UserInThread,
}

impl Display for Strategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Default for Strategy {
    fn default() -> Self {
        Self::UserInChat
    }
}

impl Strategy {
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Strategy::UserInChat => "user_in_chat",
            Strategy::Chat => "chat",
            Strategy::GlobalUser => "global_user",
            Strategy::UserInThread => "user_in_thread",
        }
    }
}

pub struct IdPair {
    pub chat_id: i64,
    pub user_id: i64,
    pub message_thread_id: Option<i64>,
}

impl Strategy {
    /// Apply strategy to `chat_id`, `user_id` and `message_thread_id`
    #[must_use]
    pub fn apply(&self, chat_id: i64, user_id: i64, message_thread_id: Option<i64>) -> IdPair {
        match self {
            Strategy::UserInChat => IdPair {
                chat_id,
                user_id,
                message_thread_id: None,
            },
            Strategy::Chat => IdPair {
                chat_id,
                user_id: chat_id,
                message_thread_id: None,
            },
            Strategy::GlobalUser => IdPair {
                chat_id: user_id,
                user_id,
                message_thread_id: None,
            },
            Strategy::UserInThread => IdPair {
                chat_id,
                user_id,
                message_thread_id,
            },
        }
    }
}
