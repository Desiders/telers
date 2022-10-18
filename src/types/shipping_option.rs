use super::LabeledPrice;

use serde::{Deserialize, Serialize};

/// This object represents one shipping option.
/// <https://core.telegram.org/bots/api#shippingoption>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}
