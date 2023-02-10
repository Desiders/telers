use super::Message;

use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(bool),
}

impl From<Message> for MessageOrTrue {
    fn from(val: Message) -> Self {
        Self::Message(val)
    }
}

impl From<bool> for MessageOrTrue {
    fn from(_val: bool) -> Self {
        Self::True(true)
    }
}
