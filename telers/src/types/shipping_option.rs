use serde::{Deserialize, Serialize};
/// This object represents one shipping option.
/// # Documentation
/// <https://core.telegram.org/bots/api#shippingoption>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShippingOption {
    /// Shipping option identifier
    pub id: Box<str>,
    /// Option title
    pub title: Box<str>,
    /// List of price portions
    pub prices: Box<[crate::types::LabeledPrice]>,
}
impl ShippingOption {
    /// Creates a new `ShippingOption`.
    ///
    /// # Arguments
    /// * `id` - Shipping option identifier
    /// * `title` - Option title
    /// * `prices` - List of price portions
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2Item: Into<crate::types::LabeledPrice>,
        T2: IntoIterator<Item = T2Item>,
    >(
        id: T0,
        title: T1,
        prices: T2,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            prices: prices.into_iter().map(Into::into).collect(),
        }
    }

    /// Shipping option identifier
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Option title
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    /// List of price portions
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn prices<T: Into<Box<[crate::types::LabeledPrice]>>>(mut self, val: T) -> Self {
        self.prices = self
            .prices
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// List of price portions
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn price<T: Into<crate::types::LabeledPrice>>(mut self, val: T) -> Self {
        self.prices = self
            .prices
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
