use serde::{Deserialize, Serialize};
/// Rich formatted message.
/// # Documentation
/// <https://core.telegram.org/bots/api#richmessage>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichMessage {
    /// Content of the message
    pub blocks: Box<[crate::types::RichBlock]>,
    /// `true`, if the rich message must be shown right-to-left
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_rtl: Option<bool>,
}
impl RichMessage {
    /// Creates a new `RichMessage`.
    ///
    /// # Arguments
    /// * `blocks` - Content of the message
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0Item: Into<crate::types::RichBlock>, T0: IntoIterator<Item = T0Item>>(
        blocks: T0,
    ) -> Self {
        Self {
            blocks: blocks.into_iter().map(Into::into).collect(),
            is_rtl: None,
        }
    }

    /// Content of the message
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

    /// Content of the message
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

    /// `true`, if the rich message must be shown right-to-left
    #[must_use]
    pub fn is_rtl<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_rtl = Some(val.into());
        self
    }

    /// `true`, if the rich message must be shown right-to-left
    #[must_use]
    pub fn is_rtl_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_rtl = val.map(Into::into);
        self
    }
}
