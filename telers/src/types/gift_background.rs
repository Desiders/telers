use serde::{Deserialize, Serialize};
/// This object describes the background of a gift.
/// # Documentation
/// <https://core.telegram.org/bots/api#giftbackground>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GiftBackground {
    /// Center color of the background in RGB format
    pub center_color: i64,
    /// Edge color of the background in RGB format
    pub edge_color: i64,
    /// Text color of the background in RGB format
    pub text_color: i64,
}
impl GiftBackground {
    /// Creates a new `GiftBackground`.
    ///
    /// # Arguments
    /// * `center_color` - Center color of the background in RGB format
    /// * `edge_color` - Edge color of the background in RGB format
    /// * `text_color` - Text color of the background in RGB format
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<i64>>(
        center_color: T0,
        edge_color: T1,
        text_color: T2,
    ) -> Self {
        Self {
            center_color: center_color.into(),
            edge_color: edge_color.into(),
            text_color: text_color.into(),
        }
    }

    /// Center color of the background in RGB format
    #[must_use]
    pub fn center_color<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.center_color = val.into();
        this
    }

    /// Edge color of the background in RGB format
    #[must_use]
    pub fn edge_color<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.edge_color = val.into();
        this
    }

    /// Text color of the background in RGB format
    #[must_use]
    pub fn text_color<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.text_color = val.into();
        this
    }
}
