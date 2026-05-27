use serde::{Deserialize, Serialize};
/// This object represent a list of gifts.
/// # Documentation
/// <https://core.telegram.org/bots/api#gifts>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Gifts {
    /// The list of gifts
    pub gifts: Box<[crate::types::Gift]>,
}
impl Gifts {
    /// Creates a new `Gifts`.
    ///
    /// # Arguments
    /// * `gifts` - The list of gifts
    #[must_use]
    pub fn new<T0Item: Into<crate::types::Gift>, T0: IntoIterator<Item = T0Item>>(
        gifts: T0,
    ) -> Self {
        Self {
            gifts: gifts.into_iter().map(Into::into).collect(),
        }
    }

    /// The list of gifts
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn gifts<T: Into<Box<[crate::types::Gift]>>>(mut self, val: T) -> Self {
        self.gifts = self
            .gifts
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// The list of gifts
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn gift<T: Into<crate::types::Gift>>(mut self, val: T) -> Self {
        self.gifts = self
            .gifts
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
