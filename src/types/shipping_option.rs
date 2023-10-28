use super::LabeledPrice;

use serde::{Deserialize, Serialize};

/// This object represents one shipping option.
/// # Documentation
/// <https://core.telegram.org/bots/api#shippingoption>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize)]
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
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        prices: impl IntoIterator<Item = LabeledPrice>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            prices: prices.into_iter().collect(),
        }
    }

    #[must_use]
    pub fn id(self, val: impl Into<String>) -> Self {
        Self {
            id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn price(self, val: LabeledPrice) -> Self {
        Self {
            prices: self.prices.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn prices(self, val: impl IntoIterator<Item = LabeledPrice>) -> Self {
        Self {
            prices: self.prices.into_iter().chain(val).collect(),
            ..self
        }
    }
}
