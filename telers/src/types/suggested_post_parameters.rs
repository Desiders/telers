use crate::types::SuggestedPostPrice;

use serde::{Deserialize, Serialize};

/// Contains parameters of a post that is being suggested by the bot.
/// # Documentation
/// <https://core.telegram.org/bots/api#suggestedpostparameters>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
pub struct SuggestedPostParameters {
    /// Proposed price for the post. If the field is omitted, then the post is unpaid.
    pub price: Option<SuggestedPostPrice>,
    /// Proposed send date of the post. If specified, then the date must be between 300 second and 2678400 seconds (30 days) in the future. If the field is omitted, then the post can be published at any time within 30 days at the sole discretion of the user who approves it.
    pub send_date: Option<i64>,
}
