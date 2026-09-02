use serde::{Deserialize, Serialize};
/// A table, corresponding to the HTML tag <`table`>.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblocktable>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockTable {
    /// Cells of the table
    pub cells: Box<[Box<[crate::types::RichBlockTableCell]>]>,
    /// `true`, if the table has borders
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bordered: Option<bool>,
    /// `true`, if the table is striped
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_striped: Option<bool>,
    /// `true`, if table cells have smaller indents
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compact: Option<bool>,
    /// Caption of the table
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<Box<crate::types::RichText>>,
}
impl RichBlockTable {
    /// Creates a new `RichBlockTable`.
    ///
    /// # Arguments
    /// * `cells` - Cells of the table
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0Item: Into<Box<[crate::types::RichBlockTableCell]>>,
        T0: IntoIterator<Item = T0Item>,
    >(
        cells: T0,
    ) -> Self {
        Self {
            cells: cells.into_iter().map(Into::into).collect(),
            is_bordered: None,
            is_striped: None,
            is_compact: None,
            caption: None,
        }
    }

    /// Cells of the table
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn cells<T: Into<Box<[Box<[crate::types::RichBlockTableCell]>]>>>(
        mut self,
        val: T,
    ) -> Self {
        self.cells = self
            .cells
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Cells of the table
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn cell<T: Into<Box<[crate::types::RichBlockTableCell]>>>(mut self, val: T) -> Self {
        self.cells = self
            .cells
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// `true`, if the table has borders
    #[must_use]
    pub fn is_bordered<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_bordered = Some(val.into());
        self
    }

    /// `true`, if the table has borders
    #[must_use]
    pub fn is_bordered_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_bordered = val.map(Into::into);
        self
    }

    /// `true`, if the table is striped
    #[must_use]
    pub fn is_striped<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_striped = Some(val.into());
        self
    }

    /// `true`, if the table is striped
    #[must_use]
    pub fn is_striped_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_striped = val.map(Into::into);
        self
    }

    /// `true`, if table cells have smaller indents
    #[must_use]
    pub fn is_compact<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_compact = Some(val.into());
        self
    }

    /// `true`, if table cells have smaller indents
    #[must_use]
    pub fn is_compact_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_compact = val.map(Into::into);
        self
    }

    /// Caption of the table
    #[must_use]
    pub fn caption<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.caption = Some(Box::new(val.into()));
        self
    }

    /// Caption of the table
    #[must_use]
    pub fn caption_option<T: Into<crate::types::RichText>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(|val| Box::new(val.into()));
        self
    }
}
