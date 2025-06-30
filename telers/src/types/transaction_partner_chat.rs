use crate::types::{Chat, Gift};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes a transaction with a chat
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartnerchat>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TransactionPartnerChat {
    /// Information about the chat
    pub chat: Chat,
    /// The gift sent to the chat by the bot
    pub gift: Option<Gift>,
}
