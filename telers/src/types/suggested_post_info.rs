use crate::types::SuggestedPostPrice;

use serde::{Deserialize, Serialize};

/// Contains information about a suggested post.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostinfo>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct SuggestedPostInfo {
    /// State of the suggested post. Currently, it can be one of “pending”, “approved”, “declined”.
    pub state: Box<str>,
    /// Proposed price of the post. If the field is omitted, then the post is unpaid.
    pub price: Option<SuggestedPostPrice>,
    /// Proposed send date of the post. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user or administrator who approves it.
    pub send_date: Option<i64>,
}
