use serde::{Deserialize, Serialize};
/// This object describes the colors of the backdrop of a unique gift.
/// # Documentation
/// <https://core.telegram.org/bots/api#uniquegiftbackdropcolors>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniqueGiftBackdropColors {
    /// The color in the center of the backdrop in RGB format
    pub center_color: i64,
    /// The color on the edges of the backdrop in RGB format
    pub edge_color: i64,
    /// The color to be applied to the symbol in RGB format
    pub symbol_color: i64,
    /// The color for the text on the backdrop in RGB format
    pub text_color: i64,
}
impl UniqueGiftBackdropColors {
    /// Creates a new `UniqueGiftBackdropColors`.
    ///
    /// # Arguments
    /// * `center_color` - The color in the center of the backdrop in RGB format
    /// * `edge_color` - The color on the edges of the backdrop in RGB format
    /// * `symbol_color` - The color to be applied to the symbol in RGB format
    /// * `text_color` - The color for the text on the backdrop in RGB format
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<i64>, T3: Into<i64>>(
        center_color: T0,
        edge_color: T1,
        symbol_color: T2,
        text_color: T3,
    ) -> Self {
        Self {
            center_color: center_color.into(),
            edge_color: edge_color.into(),
            symbol_color: symbol_color.into(),
            text_color: text_color.into(),
        }
    }

    /// The color in the center of the backdrop in RGB format
    #[must_use]
    pub fn center_color<T: Into<i64>>(mut self, val: T) -> Self {
        self.center_color = val.into();
        self
    }

    /// The color on the edges of the backdrop in RGB format
    #[must_use]
    pub fn edge_color<T: Into<i64>>(mut self, val: T) -> Self {
        self.edge_color = val.into();
        self
    }

    /// The color to be applied to the symbol in RGB format
    #[must_use]
    pub fn symbol_color<T: Into<i64>>(mut self, val: T) -> Self {
        self.symbol_color = val.into();
        self
    }

    /// The color for the text on the backdrop in RGB format
    #[must_use]
    pub fn text_color<T: Into<i64>>(mut self, val: T) -> Self {
        self.text_color = val.into();
        self
    }
}
