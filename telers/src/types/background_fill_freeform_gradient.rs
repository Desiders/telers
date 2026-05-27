use serde::{Deserialize, Serialize};
/// The background is a freeform gradient that rotates after every message in the chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#backgroundfillfreeformgradient>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BackgroundFillFreeformGradient {
    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    pub colors: Box<[i64]>,
}
impl BackgroundFillFreeformGradient {
    /// Creates a new `BackgroundFillFreeformGradient`.
    ///
    /// # Arguments
    /// * `colors` - A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    #[must_use]
    pub fn new<T0Item: Into<i64>, T0: IntoIterator<Item = T0Item>>(colors: T0) -> Self {
        Self {
            colors: colors.into_iter().map(Into::into).collect(),
        }
    }

    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn colors<T: Into<Box<[i64]>>>(mut self, val: T) -> Self {
        self.colors = self
            .colors
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn color<T: Into<i64>>(mut self, val: T) -> Self {
        self.colors = self
            .colors
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
