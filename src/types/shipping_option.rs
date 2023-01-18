use super::LabeledPrice;

use serde::{Deserialize, Serialize};

/// This object represents one shipping option.
/// <https://core.telegram.org/bots/api#shippingoption>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: String,
    /// Option title
    pub title: String,
    /// List of price portions
    pub prices: Vec<LabeledPrice>,
}

impl ShippingOption {
    #[must_use]
    pub fn new<T: Into<String>>(id: T, title: T, prices: Vec<LabeledPrice>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            prices,
        }
    }

    #[must_use]
    pub fn id<T: Into<String>>(mut self, id: T) -> Self {
        self.id = id.into();
        self
    }

    #[must_use]
    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    #[must_use]
    pub fn prices(mut self, prices: Vec<LabeledPrice>) -> Self {
        self.prices = prices;
        self
    }
}
