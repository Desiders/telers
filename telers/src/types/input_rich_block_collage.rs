use serde::{Deserialize, Serialize};
/// A collage, corresponding to the custom HTML tag `<tg-collage>`.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblockcollage>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockCollage {
    /// Elements of the collage
    pub blocks: Box<[crate::types::InputRichBlock]>,
    /// Caption of the block
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<crate::types::RichBlockCaption>,
}
impl InputRichBlockCollage {
    /// Creates a new `InputRichBlockCollage`.
    ///
    /// # Arguments
    /// * `blocks` - Elements of the collage
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0Item: Into<crate::types::InputRichBlock>, T0: IntoIterator<Item = T0Item>>(
        blocks: T0,
    ) -> Self {
        Self {
            blocks: blocks.into_iter().map(Into::into).collect(),
            caption: None,
        }
    }

    /// Elements of the collage
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn blocks<T: Into<Box<[crate::types::InputRichBlock]>>>(mut self, val: T) -> Self {
        self.blocks = self
            .blocks
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Elements of the collage
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn block<T: Into<crate::types::InputRichBlock>>(mut self, val: T) -> Self {
        self.blocks = self
            .blocks
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Caption of the block
    #[must_use]
    pub fn caption<T: Into<crate::types::RichBlockCaption>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Caption of the block
    #[must_use]
    pub fn caption_option<T: Into<crate::types::RichBlockCaption>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.caption = val.map(Into::into);
        self
    }
}
