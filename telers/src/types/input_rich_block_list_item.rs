use serde::{Deserialize, Serialize};
/// An item of a list to be sent.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputrichblocklistitem>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputRichBlockListItem {
    /// The content of the item
    pub blocks: Box<[crate::types::InputRichBlock]>,
    /// Pass `true` if the item has a checkbox
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_checkbox: Option<bool>,
    /// Pass `true` if the item has a checked checkbox
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_checked: Option<bool>,
    /// For ordered lists, the numeric value of the item label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// For ordered lists, the type of the item label; must be one of `a` for lowercase letters, `A` for uppercase letters, `i` for lowercase Roman numerals, `I` for uppercase Roman numerals, or `1` for decimal numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<str>>,
}
impl InputRichBlockListItem {
    /// Creates a new `InputRichBlockListItem`.
    ///
    /// # Arguments
    /// * `blocks` - The content of the item
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0Item: Into<crate::types::InputRichBlock>, T0: IntoIterator<Item = T0Item>>(
        blocks: T0,
    ) -> Self {
        Self {
            blocks: blocks.into_iter().map(Into::into).collect(),
            has_checkbox: None,
            is_checked: None,
            value: None,
            r#type: None,
        }
    }

    /// The content of the item
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

    /// The content of the item
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

    /// Pass `true` if the item has a checkbox
    #[must_use]
    pub fn has_checkbox<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_checkbox = Some(val.into());
        self
    }

    /// Pass `true` if the item has a checkbox
    #[must_use]
    pub fn has_checkbox_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_checkbox = val.map(Into::into);
        self
    }

    /// Pass `true` if the item has a checked checkbox
    #[must_use]
    pub fn is_checked<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_checked = Some(val.into());
        self
    }

    /// Pass `true` if the item has a checked checkbox
    #[must_use]
    pub fn is_checked_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_checked = val.map(Into::into);
        self
    }

    /// For ordered lists, the numeric value of the item label
    #[must_use]
    pub fn value<T: Into<i64>>(mut self, val: T) -> Self {
        self.value = Some(val.into());
        self
    }

    /// For ordered lists, the numeric value of the item label
    #[must_use]
    pub fn value_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.value = val.map(Into::into);
        self
    }

    /// For ordered lists, the type of the item label; must be one of `a` for lowercase letters, `A` for uppercase letters, `i` for lowercase Roman numerals, `I` for uppercase Roman numerals, or `1` for decimal numbers
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = Some(val.into());
        self
    }

    /// For ordered lists, the type of the item label; must be one of `a` for lowercase letters, `A` for uppercase letters, `i` for lowercase Roman numerals, `I` for uppercase Roman numerals, or `1` for decimal numbers
    #[must_use]
    pub fn type_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.r#type = val.map(Into::into);
        self
    }
}
