use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Describes why a request was unsuccessful.
/// # Documentation
/// <https://core.telegram.org/bots/api#responseparameters>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ResponseParameters {
    /// The group has been migrated to a supergroup with the specified identifier. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a signed 64-bit integer or double-precision float type are safe for storing this identifier.
    pub migrate_to_chat_id: Option<i64>,
    /// In case of exceeding flood control, the number of seconds left to wait before the request can be repeated
    pub retry_after: Option<f32>,
}
