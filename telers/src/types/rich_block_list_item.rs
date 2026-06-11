use serde::{Deserialize, Serialize};
/// An item of a list.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblocklistitem>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockListItem {
    /// Label of the item
    pub label: Box<str>,
    /// The content of the item
    pub blocks: Box<[crate::types::RichBlock]>,
    /// `true`, if the item has a checkbox
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_checkbox: Option<bool>,
    /// `true`, if the item has a checked checkbox
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_checked: Option<bool>,
    /// For ordered lists, the numeric value of the item label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// For ordered lists, the type of the item label; must be one of `a` for lowercase letters, `A` for uppercase letters, `i` for lowercase Roman numerals, `I` for uppercase Roman numerals, or `1` for decimal numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<str>>,
}
impl RichBlockListItem {
    /// Creates a new `RichBlockListItem`.
    ///
    /// # Arguments
    /// * `label` - Label of the item
    /// * `blocks` - The content of the item
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1Item: Into<crate::types::RichBlock>,
        T1: IntoIterator<Item = T1Item>,
    >(
        label: T0,
        blocks: T1,
    ) -> Self {
        Self {
            label: label.into(),
            blocks: blocks.into_iter().map(Into::into).collect(),
            has_checkbox: None,
            is_checked: None,
            value: None,
            r#type: None,
        }
    }

    /// Label of the item
    #[must_use]
    pub fn label<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.label = val.into();
        self
    }

    /// The content of the item
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

    /// The content of the item
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

    /// `true`, if the item has a checkbox
    #[must_use]
    pub fn has_checkbox<T: Into<bool>>(mut self, val: T) -> Self {
        self.has_checkbox = Some(val.into());
        self
    }

    /// `true`, if the item has a checkbox
    #[must_use]
    pub fn has_checkbox_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.has_checkbox = val.map(Into::into);
        self
    }

    /// `true`, if the item has a checked checkbox
    #[must_use]
    pub fn is_checked<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_checked = Some(val.into());
        self
    }

    /// `true`, if the item has a checked checkbox
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
