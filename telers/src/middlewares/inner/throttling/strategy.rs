/// Which peer IDs form the throttling key.
///
/// # Variants
/// * [`Strategy::UserInChat`] - `user_id` + `chat_id`
/// * [`Strategy::Chat`] - `chat_id` + `chat_id`
/// * [`Strategy::GlobalUser`] - `user_id` + `user_id`
/// * [`Strategy::UserInThread`] - `user_id` + `chat_id` + `message_thread_id`
/// * [`Strategy::ChatThread`] - `chat_id` + `chat_id` + `message_thread_id`
///
/// Strategies with `business_connection_id` field:
/// * [`Strategy::UserInChatAndConnection`] - `user_id` + `chat_id` + `business_connection_id`
/// * [`Strategy::ChatAndConnection`] - `chat_id` + `chat_id` + `business_connection_id`
/// * [`Strategy::GlobalUserAndConnection`] - `user_id` + `user_id` + `business_connection_id`
/// * [`Strategy::UserInThreadAndConnection`] - `user_id` + `chat_id` + `message_thread_id` + `business_connection_id`
/// * [`Strategy::ChatThreadAndConnection`] - `chat_id` + `chat_id` + `message_thread_id` + `business_connection_id`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

/// The throttling key built from peer IDs.
#[allow(clippy::struct_field_names)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdPair {
    chat_id: i64,
    user_id: i64,
    message_thread_id: Option<i64>,
    business_connection_id: Option<String>,
}

impl Strategy {
    /// Build the throttling key from peer IDs.
    #[must_use]
    pub fn apply(
        self,
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
