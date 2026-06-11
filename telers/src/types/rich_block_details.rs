use serde::{Deserialize, Serialize};
/// An expandable block for details disclosure, corresponding to the HTML tag <`details`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockdetails>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockDetails {
    /// Always shown summary of the block
    pub summary: Box<crate::types::RichText>,
    /// Content of the block
    pub blocks: Box<[crate::types::RichBlock]>,
    /// `true`, if the content of the block is visible by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
}
impl RichBlockDetails {
    /// Creates a new `RichBlockDetails`.
    ///
    /// # Arguments
    /// * `summary` - Always shown summary of the block
    /// * `blocks` - Content of the block
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::RichText>,
        T1Item: Into<crate::types::RichBlock>,
        T1: IntoIterator<Item = T1Item>,
    >(
        summary: T0,
        blocks: T1,
    ) -> Self {
        Self {
            summary: Box::new(summary.into()),
            blocks: blocks.into_iter().map(Into::into).collect(),
            is_open: None,
        }
    }

    /// Always shown summary of the block
    #[must_use]
    pub fn summary<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.summary = Box::new(val.into());
        self
    }

    /// Content of the block
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn blocks<T: Into<Box<[crate::types::RichBlock]>>>(mut self, val: T) -> Self {
        self.blocks = self
            .blocks
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Content of the block
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn block<T: Into<crate::types::RichBlock>>(mut self, val: T) -> Self {
        self.blocks = self
            .blocks
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// `true`, if the content of the block is visible by default
    #[must_use]
    pub fn is_open<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_open = Some(val.into());
        self
    }

    /// `true`, if the content of the block is visible by default
    #[must_use]
    pub fn is_open_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_open = val.map(Into::into);
        self
    }
}
