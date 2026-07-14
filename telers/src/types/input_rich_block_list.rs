use serde::{Deserialize, Serialize};
/// A list of blocks, corresponding to the HTML tag <`ul`> or <`ol`> with multiple nested tags <`li`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblocklist>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockList {
    /// Items of the list
    pub items: Box<[crate::types::InputRichBlockListItem]>,
}
impl InputRichBlockList {
    /// Creates a new `InputRichBlockList`.
    ///
    /// # Arguments
    /// * `items` - Items of the list
    #[must_use]
    pub fn new<
        T0Item: Into<crate::types::InputRichBlockListItem>,
        T0: IntoIterator<Item = T0Item>,
    >(
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
    pub fn items<T: Into<Box<[crate::types::InputRichBlockListItem]>>>(mut self, val: T) -> Self {
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
    pub fn item<T: Into<crate::types::InputRichBlockListItem>>(mut self, val: T) -> Self {
        self.items = self
            .items
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
