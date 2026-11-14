/// This object contains a list of backdrops, which can be used as the background of a theme of the screen for sharing a link to a user or a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatsharethemebackdrops>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct ChatShareThemeBackdrops {
    /// The list of backdrops
    pub backdrops: Box<[crate::types::ChatShareThemeBackdrop]>,
}
impl ChatShareThemeBackdrops {
    /// Creates a new `ChatShareThemeBackdrops`.
    ///
    /// # Arguments
    /// * `backdrops` - The list of backdrops
    #[must_use]
    pub fn new<
        T0Item: Into<crate::types::ChatShareThemeBackdrop>,
        T0: IntoIterator<Item = T0Item>,
    >(
        backdrops: T0,
    ) -> Self {
        Self {
            backdrops: backdrops.into_iter().map(Into::into).collect(),
        }
    }

    /// The list of backdrops
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn backdrops<T: Into<Box<[crate::types::ChatShareThemeBackdrop]>>>(
        mut self,
        val: T,
    ) -> Self {
        self.backdrops = self
            .backdrops
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// The list of backdrops
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn backdrop<T: Into<crate::types::ChatShareThemeBackdrop>>(mut self, val: T) -> Self {
        self.backdrops = self
            .backdrops
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
