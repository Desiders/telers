use super::{InaccessibleMessage, Message};

use serde::Deserialize;

/// This object describes a message that can be inaccessible to the bot. It can be one of
/// - [`Message`]
/// - [`InaccessibleMessage`]
/// # Documentation
/// <https://core.telegram.org/bots/api#maybeinaccessiblemessage>
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Message(Message),
    InaccessibleMessage(InaccessibleMessage),
}

impl From<Message> for MaybeInaccessibleMessage {
    fn from(message: Message) -> Self {
        Self::Message(message)
    }
}

impl From<InaccessibleMessage> for MaybeInaccessibleMessage {
    fn from(inaccessible_message: InaccessibleMessage) -> Self {
        Self::InaccessibleMessage(inaccessible_message)
    }
}
