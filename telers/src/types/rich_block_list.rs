use serde::{Deserialize, Serialize};
/// A list of blocks, corresponding to the HTML tag <`ul`> or <`ol`> with multiple nested tags <`li`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblocklist>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockList {
    /// Items of the list
    pub items: Box<[crate::types::RichBlockListItem]>,
}
impl RichBlockList {
    /// Creates a new `RichBlockList`.
    ///
    /// # Arguments
    /// * `items` - Items of the list
    #[must_use]
    pub fn new<T0Item: Into<crate::types::RichBlockListItem>, T0: IntoIterator<Item = T0Item>>(
        items: T0,
    ) -> Self {
        Self {
            items: items.into_iter().map(Into::into).collect(),
        }
    }

    /// Items of the list
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn items<T: Into<Box<[crate::types::RichBlockListItem]>>>(mut self, val: T) -> Self {
        self.items = self
            .items
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Items of the list
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn item<T: Into<crate::types::RichBlockListItem>>(mut self, val: T) -> Self {
        self.items = self
            .items
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
