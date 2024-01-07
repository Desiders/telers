use super::{MessageOriginChannel, MessageOriginChat, MessageOriginHiddenUser, MessageOriginUser};

use serde::Deserialize;

/// This object describes the origin of a message. It can be one of
/// - [`MessageOriginUser`]
/// - [`MessageOriginChat`]
/// - [`MessageOriginHiddenUser`]
/// - [`MessageOriginChannel`]
/// # Documentation
/// <https://core.telegram.org/bots/api#messageorigin>
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum MessageOrigin {
    /// The message was originally sent by a known user.
    User(MessageOriginUser),
    /// The message was originally sent by an unknown user.
    HiddenUser(MessageOriginHiddenUser),
    /// The message was originally sent on behalf of a chat to a group chat.
    Chat(MessageOriginChat),
    /// The message was originally sent to a channel chat.
    Channel(MessageOriginChannel),
}

impl Default for MessageOrigin {
    #[must_use]
    fn default() -> Self {
        Self::User(MessageOriginUser::default())
    }
}

impl From<MessageOriginUser> for MessageOrigin {
    fn from(origin: MessageOriginUser) -> Self {
        Self::User(origin)
    }
}

impl From<MessageOriginHiddenUser> for MessageOrigin {
    fn from(origin: MessageOriginHiddenUser) -> Self {
        Self::HiddenUser(origin)
    }
}

impl From<MessageOriginChat> for MessageOrigin {
    fn from(origin: MessageOriginChat) -> Self {
        Self::Chat(origin)
    }
}

impl From<MessageOriginChannel> for MessageOrigin {
    fn from(origin: MessageOriginChannel) -> Self {
        Self::Channel(origin)
    }
}
