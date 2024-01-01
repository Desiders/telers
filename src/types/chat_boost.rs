use super::ChatBoostSource;

use serde::Deserialize;

/// This object contains information about a chat boost.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboost>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatBoost {
    /// Unique identifier of the boost
    #[serde(rename = "boost_id")]
    pub id: i64,
    /// Point in time (Unix timestamp) when the chat was boosted
    pub add_date: i64,
    /// Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged
    pub expiration_date: i64,
    /// Source of the added boost
    pub source: ChatBoostSource,
}
