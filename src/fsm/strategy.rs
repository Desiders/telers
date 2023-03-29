use std::fmt::{self, Debug};

/// Strategy for storing and retrieving data.
///
/// If you use `UserInChat` strategy, you have possible to store different data and state for different chats.
/// If you use `Chat` strategy, then all users in the chat will have the same data and state.
/// If you use `GlobalUser` strategy, then the user will have the same data and state in all chats.
///
/// In case of direct messages, `chat_id` and `user_id` will be equal, so all strategies will work the same way.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Strategy {
    /// `user_id` + `chat_id`
    UserInChat,
    /// `chat_id` + `chat_id`
    Chat,
    /// `user_id` + `user_id`
    GlobalUser,
}

impl Debug for Strategy {
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
        }
    }
}

/// Chat and user id pair.
/// This struct is used to store `chat_id` and `chat_id` pair, that can be equal in some cases.
pub struct IdPair {
    pub chat_id: i64,
    pub user_id: i64,
}

impl Strategy {
    /// Apply strategy to `chat_id` and `user_id`.
    pub fn apply(&self, chat_id: i64, user_id: i64) -> IdPair {
        match self {
            Strategy::UserInChat => IdPair { chat_id, user_id },
            Strategy::Chat => IdPair {
                chat_id,
                user_id: chat_id,
            },
            Strategy::GlobalUser => IdPair {
                chat_id: user_id,
                user_id,
            },
        }
    }
}
