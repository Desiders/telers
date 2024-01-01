use super::{Chat, ChatBoostSource};

use serde::Deserialize;

/// This object represents a boost added to a chat or changed.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostupdated>
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ChatBoostUpdated {
    /// Chat which was boosted
    pub chat: Chat,
    /// Infomation about the chat boost
    pub boost: ChatBoostSource,
}
