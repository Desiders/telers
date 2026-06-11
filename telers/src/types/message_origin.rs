use serde::{Deserialize, Serialize};
/// This object describes the origin of a message. It can be one of
/// - [`crate::types::MessageOriginUser`]
/// - [`crate::types::MessageOriginHiddenUser`]
/// - [`crate::types::MessageOriginChat`]
/// - [`crate::types::MessageOriginChannel`]
/// # Documentation
/// <https://core.telegram.org/bots/api#messageorigin>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOrigin {
    User(crate::types::MessageOriginUser),
    HiddenUser(crate::types::MessageOriginHiddenUser),
    Chat(crate::types::MessageOriginChat),
    Channel(crate::types::MessageOriginChannel),
}
impl MessageOrigin {
    /// Helper method for field `author_signature`.
    ///
    /// # Variants
    /// - `MessageOriginChat`. For messages originally sent by an anonymous chat administrator, original message author signature
    /// - `MessageOriginChannel`. Signature of the original post author
    #[must_use]
    pub fn author_signature(&self) -> Option<&str> {
        match self {
            Self::Chat(val) => val.author_signature.as_deref(),
            Self::Channel(val) => val.author_signature.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `chat`.
    ///
    /// Channel chat to which the message was originally sent
    #[must_use]
    pub fn chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Channel(val) => Some(val.chat.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `date`.
    ///
    /// Date the message was sent originally in Unix time
    #[must_use]
    pub fn date(&self) -> i64 {
        match self {
            Self::User(val) => val.date,
            Self::HiddenUser(val) => val.date,
            Self::Chat(val) => val.date,
            Self::Channel(val) => val.date,
        }
    }

    /// Helper method for field `message_id`.
    ///
    /// Unique message identifier inside the chat
    #[must_use]
    pub fn message_id(&self) -> Option<i64> {
        match self {
            Self::Channel(val) => Some(val.message_id),
            _ => None,
        }
    }

    /// Helper method for field `sender_chat`.
    ///
    /// Chat that sent the message originally
    #[must_use]
    pub fn sender_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Chat(val) => Some(val.sender_chat.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `sender_user`.
    ///
    /// User that sent the message originally
    #[must_use]
    pub fn sender_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::User(val) => Some(val.sender_user.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `sender_user_name`.
    ///
    /// Name of the user that sent the message originally
    #[must_use]
    pub fn sender_user_name(&self) -> Option<&str> {
        match self {
            Self::HiddenUser(val) => Some(val.sender_user_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `added_to_attachment_menu`.
    #[must_use]
    pub fn added_to_attachment_menu(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.added_to_attachment_menu
            }
            _ => None,
        }
    }

    /// Helper method for nested field `allows_users_to_create_topics`.
    #[must_use]
    pub fn allows_users_to_create_topics(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.allows_users_to_create_topics
            }
            _ => None,
        }
    }

    /// Helper method for nested field `can_connect_to_business`.
    #[must_use]
    pub fn can_connect_to_business(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.can_connect_to_business
            }
            _ => None,
        }
    }

    /// Helper method for nested field `can_join_groups`.
    #[must_use]
    pub fn can_join_groups(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.can_join_groups
            }
            _ => None,
        }
    }

    /// Helper method for nested field `can_manage_bots`.
    #[must_use]
    pub fn can_manage_bots(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.can_manage_bots
            }
            _ => None,
        }
    }

    /// Helper method for nested field `can_read_all_group_messages`.
    #[must_use]
    pub fn can_read_all_group_messages(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.can_read_all_group_messages
            }
            _ => None,
        }
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::Channel(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::first_name(inner)
            }
            Self::Chat(val) => {
                let inner = val.sender_chat.as_ref();
                crate::types::Chat::first_name(inner)
            }
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                Some(inner.first_name.as_ref())
            }
            Self::HiddenUser(_) => None,
        }
    }

    /// Helper method for nested field `has_main_web_app`.
    #[must_use]
    pub fn has_main_web_app(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.has_main_web_app
            }
            _ => None,
        }
    }

    /// Helper method for nested field `has_topics_enabled`.
    #[must_use]
    pub fn has_topics_enabled(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.has_topics_enabled
            }
            _ => None,
        }
    }

    /// Helper method for nested field `id`.
    #[must_use]
    pub fn id(&self) -> Option<i64> {
        match self {
            Self::Channel(val) => {
                let inner = val.chat.as_ref();
                Some(crate::types::Chat::id(inner))
            }
            Self::Chat(val) => {
                let inner = val.sender_chat.as_ref();
                Some(crate::types::Chat::id(inner))
            }
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                Some(inner.id)
            }
            Self::HiddenUser(_) => None,
        }
    }

    /// Helper method for nested field `is_bot`.
    #[must_use]
    pub fn is_bot(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                Some(inner.is_bot)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_direct_messages`.
    #[must_use]
    pub fn is_direct_messages(&self) -> Option<bool> {
        match self {
            Self::Channel(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::is_direct_messages(inner)
            }
            Self::Chat(val) => {
                let inner = val.sender_chat.as_ref();
                crate::types::Chat::is_direct_messages(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_forum`.
    #[must_use]
    pub fn is_forum(&self) -> Option<bool> {
        match self {
            Self::Channel(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::is_forum(inner)
            }
            Self::Chat(val) => {
                let inner = val.sender_chat.as_ref();
                crate::types::Chat::is_forum(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_premium`.
    #[must_use]
    pub fn is_premium(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.is_premium
            }
            _ => None,
        }
    }

    /// Helper method for nested field `language_code`.
    #[must_use]
    pub fn language_code(&self) -> Option<&str> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.language_code.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::Channel(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::last_name(inner)
            }
            Self::Chat(val) => {
                let inner = val.sender_chat.as_ref();
                crate::types::Chat::last_name(inner)
            }
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.last_name.as_deref()
            }
            Self::HiddenUser(_) => None,
        }
    }

    /// Helper method for nested field `supports_guest_queries`.
    #[must_use]
    pub fn supports_guest_queries(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.supports_guest_queries
            }
            _ => None,
        }
    }

    /// Helper method for nested field `supports_inline_queries`.
    #[must_use]
    pub fn supports_inline_queries(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.supports_inline_queries
            }
            _ => None,
        }
    }

    /// Helper method for nested field `supports_join_request_queries`.
    #[must_use]
    pub fn supports_join_request_queries(&self) -> Option<bool> {
        match self {
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.supports_join_request_queries
            }
            _ => None,
        }
    }

    /// Helper method for nested field `title`.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Channel(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::title(inner)
            }
            Self::Chat(val) => {
                let inner = val.sender_chat.as_ref();
                crate::types::Chat::title(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::Channel(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::username(inner)
            }
            Self::Chat(val) => {
                let inner = val.sender_chat.as_ref();
                crate::types::Chat::username(inner)
            }
            Self::User(val) => {
                let inner = val.sender_user.as_ref();
                inner.username.as_deref()
            }
            Self::HiddenUser(_) => None,
        }
    }
}
impl From<crate::types::MessageOriginUser> for MessageOrigin {
    fn from(val: crate::types::MessageOriginUser) -> Self {
        Self::User(val)
    }
}
impl TryFrom<MessageOrigin> for crate::types::MessageOriginUser {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageOrigin) -> Result<Self, Self::Error> {
        if let MessageOrigin::User(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageOrigin),
                stringify!(MessageOriginUser),
            ))
        }
    }
}
impl From<crate::types::MessageOriginHiddenUser> for MessageOrigin {
    fn from(val: crate::types::MessageOriginHiddenUser) -> Self {
        Self::HiddenUser(val)
    }
}
impl TryFrom<MessageOrigin> for crate::types::MessageOriginHiddenUser {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageOrigin) -> Result<Self, Self::Error> {
        if let MessageOrigin::HiddenUser(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageOrigin),
                stringify!(MessageOriginHiddenUser),
            ))
        }
    }
}
impl From<crate::types::MessageOriginChat> for MessageOrigin {
    fn from(val: crate::types::MessageOriginChat) -> Self {
        Self::Chat(val)
    }
}
impl TryFrom<MessageOrigin> for crate::types::MessageOriginChat {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageOrigin) -> Result<Self, Self::Error> {
        if let MessageOrigin::Chat(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageOrigin),
                stringify!(MessageOriginChat),
            ))
        }
    }
}
impl From<crate::types::MessageOriginChannel> for MessageOrigin {
    fn from(val: crate::types::MessageOriginChannel) -> Self {
        Self::Channel(val)
    }
}
impl TryFrom<MessageOrigin> for crate::types::MessageOriginChannel {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: MessageOrigin) -> Result<Self, Self::Error> {
        if let MessageOrigin::Channel(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(MessageOrigin),
                stringify!(MessageOriginChannel),
            ))
        }
    }
}
