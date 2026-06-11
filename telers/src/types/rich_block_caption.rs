use serde::{Deserialize, Serialize};
/// Caption of a rich formatted block.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockcaption>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockCaption {
    /// Block caption
    pub text: Box<crate::types::RichText>,
    /// Block credit which corresponds to the HTML tag <`cite`>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<Box<crate::types::RichText>>,
}
impl RichBlockCaption {
    /// Creates a new `RichBlockCaption`.
    ///
    /// # Arguments
    /// * `text` - Block caption
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::RichText>>(text: T0) -> Self {
        Self {
            text: Box::new(text.into()),
            credit: None,
        }
    }

    /// Block caption
    #[must_use]
    pub fn text<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.text = Box::new(val.into());
        self
    }

    /// Block credit which corresponds to the HTML tag <`cite`>
    #[must_use]
    pub fn credit<T: Into<crate::types::RichText>>(mut self, val: T) -> Self {
        self.credit = Some(Box::new(val.into()));
        self
    }

    /// Block credit which corresponds to the HTML tag <`cite`>
    #[must_use]
    pub fn credit_option<T: Into<crate::types::RichText>>(mut self, val: Option<T>) -> Self {
        self.credit = val.map(|val| Box::new(val.into()));
        self
    }
}
