use serde::{Deserialize, Serialize};
/// A block quotation, corresponding to the HTML tag <`blockquote`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockblockquotation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockBlockQuotation {
    /// Content of the block
    pub blocks: Box<[crate::types::RichBlock]>,
    /// Credit of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<Box<crate::types::RichText>>,
}
impl RichBlockBlockQuotation {
    /// Creates a new `RichBlockBlockQuotation`.
    ///
    /// # Arguments
    /// * `blocks` - Content of the block
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0Item: Into<crate::types::RichBlock>, T0: IntoIterator<Item = T0Item>>(
        blocks: T0,
    ) -> Self {
        Self {
            blocks: blocks.into_iter().map(Into::into).collect(),
            credit: None,
        }
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

    /// Credit of the block
    #[must_use]
    pub fn credit<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.credit = Some(Box::new(val.into()));
        self
    }

    /// Credit of the block
    #[must_use]
    pub fn credit_option<T: Into<crate::types::RichText>>(mut self, val: Option<T>) -> Self {
        self.credit = val.map(|val| Box::new(val.into()));
        self
    }
}
