use std::fmt::{self, Display};

/// Strategy for storing and retrieving data.
///
/// # Variants
/// * [`Strategy::UserInChat`] - `user_id` + `chat_id`.\
///   If you use `UserInChat` strategy, you have possible to store different state for user+chat pairs,
///   so each user in each chat will have its own state.
/// * [`Strategy::Chat`] - `chat_id` + `chat_id`.\
///   If you use `Chat` strategy, then all users in the chat will have the same state.
/// * [`Strategy::GlobalUser`] - `user_id` + `user_id`.\
///   If you use `GlobalUser` strategy, then the user will have the same state in all chats,
///   so each user will have its own state in all chats.
/// * [`Strategy::UserInThread`] - `user_id` + `chat_id` + `message_thread_id`.\
///   If you use `UserInThread` strategy, you have possible to store different state for user+chat+thread pairs,
///   so each user in each thread in each chat will have its own state.
/// * [`Strategy::ChatThread`] - `chat_id` + `chat_id` + `message_thread_id`.\
///   If you use `ChatThread` strategy, you have possible to store different state for chat+thread pairs,
///   so each thread in each chat will have its own state.
///
/// If you need to have different states for business connections, you can use strategies with `business_connection_id` field.\
/// Strategies with `business_connection_id` field:
/// * [`Strategy::UserInChatAndConnection`] - `user_id` + `chat_id` + `business_connection_id`.\
///   Identical to `UserInChat`, but with an additional `business_connection_id` field.
/// * [`Strategy::ChatAndConnection`] - `chat_id` + `chat_id` + `business_connection_id`.\
///   Identical to `Chat`, but with an additional `business_connection_id` field.
/// * [`Strategy::GlobalUserAndConnection`] - `user_id` + `user_id` + `business_connection_id`.\
///   Identical to `GlobalUser`, but with an additional `business_connection_id` field.
/// * [`Strategy::UserInThreadAndConnection`] - `user_id` + `chat_id` + `message_thread_id` + `business_connection_id`.\
///   Identical to `UserInThread`, but with an additional `business_connection_id` field.
/// * [`Strategy::ChatThreadAndConnection`] - `chat_id` + `chat_id` + `message_thread_id` + `business_connection_id`.\
///   Identical to `ChatThread`, but with an additional `business_connection_id` field.
///
/// # Notes
/// In case of direct messages, `chat_id` and `user_id` will be equal, so all strategies will work the same way.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Strategy {
    /// `user_id` + `chat_id`
    UserInChat,
    /// `chat_id` + `chat_id`
    Chat,
    /// `user_id` + `user_id`
    GlobalUser,
    /// `user_id` + `chat_id` + `message_thread_id`
    UserInThread,
    /// `chat_id` + `chat_id` + `message_thread_id`
    ChatThread,

    /// `user_id` + `chat_id` + `business_connection_id`
    UserInChatAndConnection,
    /// `chat_id` + `chat_id` + `business_connection_id`
    ChatAndConnection,
    /// `user_id` + `user_id` + `business_connection_id`
    GlobalUserAndConnection,
    /// `user_id` + `chat_id` + `message_thread_id` + `business_connection_id`
    UserInThreadAndConnection,
    /// `chat_id` + `chat_id` + `message_thread_id` + `business_connection_id`
    ChatThreadAndConnection,
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
            Strategy::ChatThread => "chat_thread",
            Strategy::UserInChatAndConnection => "user_in_chat_and_connection",
            Strategy::ChatAndConnection => "chat_and_connection",
            Strategy::GlobalUserAndConnection => "global_user_and_connection",
            Strategy::UserInThreadAndConnection => "user_in_thread_and_connection",
            Strategy::ChatThreadAndConnection => "chat_thread_and_connection",
        }
    }
}

pub struct IdPair {
    pub chat_id: i64,
    pub user_id: i64,
    pub message_thread_id: Option<i64>,
    pub business_connection_id: Option<String>,
}

impl Strategy {
    /// Apply strategy to `chat_id`, `user_id` and `message_thread_id`
    #[must_use]
    pub fn apply(
        &self,
        chat_id: i64,
        user_id: i64,
        message_thread_id: Option<i64>,
        business_connection_id: Option<String>,
    ) -> IdPair {
        match self {
            Strategy::UserInChat => IdPair {
                chat_id,
                user_id,
                message_thread_id: None,
                business_connection_id: None,
            },
            Strategy::UserInChatAndConnection => IdPair {
                chat_id,
                user_id,
                message_thread_id: None,
                business_connection_id,
            },
            Strategy::Chat => IdPair {
                chat_id,
                user_id: chat_id,
                message_thread_id: None,
                business_connection_id: None,
            },
            Strategy::ChatAndConnection => IdPair {
                chat_id,
                user_id: chat_id,
                message_thread_id: None,
                business_connection_id,
            },
            Strategy::GlobalUser => IdPair {
                chat_id: user_id,
                user_id,
                message_thread_id: None,
                business_connection_id: None,
            },
            Strategy::GlobalUserAndConnection => IdPair {
                chat_id: user_id,
                user_id,
                message_thread_id: None,
                business_connection_id,
            },
            Strategy::UserInThread => IdPair {
                chat_id,
                user_id,
                message_thread_id,
                business_connection_id: None,
            },
            Strategy::UserInThreadAndConnection => IdPair {
                chat_id,
                user_id,
                message_thread_id,
                business_connection_id,
            },
            Strategy::ChatThread => IdPair {
                chat_id,
                user_id: chat_id,
                message_thread_id,
                business_connection_id: None,
            },
            Strategy::ChatThreadAndConnection => IdPair {
                chat_id,
                user_id: chat_id,
                message_thread_id,
                business_connection_id,
            },
        }
    }
}
