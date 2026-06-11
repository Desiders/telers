use serde::{Deserialize, Serialize};
/// Cell in a table.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblocktablecell>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockTableCell {
    /// Text in the cell. If omitted, then the cell is invisible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<Box<crate::types::RichText>>,
    /// `true`, if the cell is a header cell
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_header: Option<bool>,
    /// The number of columns the cell spans if it is bigger than 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colspan: Option<i64>,
    /// The number of rows the cell spans if it is bigger than 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowspan: Option<i64>,
    /// Horizontal cell content alignment. Currently, must be one of `left`, `center`, or `right`.
    pub align: Box<str>,
    /// Vertical cell content alignment. Currently, must be one of `top`, `middle`, or `bottom`.
    pub valign: Box<str>,
}
impl RichBlockTableCell {
    /// Creates a new `RichBlockTableCell`.
    ///
    /// # Arguments
    /// * `align` - Horizontal cell content alignment. Currently, must be one of `left`, `center`, or `right`.
    /// * `valign` - Vertical cell content alignment. Currently, must be one of `top`, `middle`, or `bottom`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(align: T0, valign: T1) -> Self {
        Self {
            text: None,
            is_header: None,
            colspan: None,
            rowspan: None,
            align: align.into(),
            valign: valign.into(),
        }
    }

    /// Text in the cell. If omitted, then the cell is invisible.
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Some(Box::new(val.into()));
        self
    }

    /// Text in the cell. If omitted, then the cell is invisible.
    #[must_use]
    pub fn text_option<T: Into<crate::types::RichText>>(mut self, val: Option<T>) -> Self {
        self.text = val.map(|val| Box::new(val.into()));
        self
    }

    /// `true`, if the cell is a header cell
    #[must_use]
    pub fn is_header<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_header = Some(val.into());
        self
    }

    /// `true`, if the cell is a header cell
    #[must_use]
    pub fn is_header_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_header = val.map(Into::into);
        self
    }

    /// The number of columns the cell spans if it is bigger than 1
    #[must_use]
    pub fn colspan<T: Into<i64>>(mut self, val: T) -> Self {
        self.colspan = Some(val.into());
        self
    }

    /// The number of columns the cell spans if it is bigger than 1
    #[must_use]
    pub fn colspan_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.colspan = val.map(Into::into);
        self
    }

    /// The number of rows the cell spans if it is bigger than 1
    #[must_use]
    pub fn rowspan<T: Into<i64>>(mut self, val: T) -> Self {
        self.rowspan = Some(val.into());
        self
    }

    /// The number of rows the cell spans if it is bigger than 1
    #[must_use]
    pub fn rowspan_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.rowspan = val.map(Into::into);
        self
    }

    /// Horizontal cell content alignment. Currently, must be one of `left`, `center`, or `right`.
    #[must_use]
    pub fn align<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.align = val.into();
        self
    }

    /// Vertical cell content alignment. Currently, must be one of `top`, `middle`, or `bottom`.
    #[must_use]
    pub fn valign<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.valign = val.into();
        self
    }
}
