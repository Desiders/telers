use serde::{Deserialize, Serialize};
/// A block containing a list of buttons that are shown in one row, corresponding to the custom HTML tag `<tg-button-row>`.
/// # Documentation
/// <https://core.telegram.org/bots/api#richblockbuttons>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RichBlockButtons {
    /// The buttons
    pub buttons: Box<[crate::types::RichMessageButton]>,
    /// Horizontal alignment of the buttons. Currently, must be one of `left`, `center`, or `right`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<Box<str>>,
}
impl RichBlockButtons {
    /// Creates a new `RichBlockButtons`.
    ///
    /// # Arguments
    /// * `buttons` - The buttons
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0Item: Into<crate::types::RichMessageButton>, T0: IntoIterator<Item = T0Item>>(
        buttons: T0,
    ) -> Self {
        Self {
            buttons: buttons.into_iter().map(Into::into).collect(),
            align: None,
        }
    }

    /// The buttons
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn buttons<T: Into<Box<[crate::types::RichMessageButton]>>>(mut self, val: T) -> Self {
        self.buttons = self
            .buttons
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// The buttons
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn button<T: Into<crate::types::RichMessageButton>>(mut self, val: T) -> Self {
        self.buttons = self
            .buttons
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Horizontal alignment of the buttons. Currently, must be one of `left`, `center`, or `right`.
    #[must_use]
    pub fn align<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.align = Some(val.into());
        self
    }

    /// Horizontal alignment of the buttons. Currently, must be one of `left`, `center`, or `right`.
    #[must_use]
    pub fn align_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.align = val.map(Into::into);
        self
    }
}
