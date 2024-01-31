use super::Message;

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum MessageOrTrue {
    Message(Message),
    True(bool),
}
