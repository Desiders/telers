/// This object describes a backdrop, which can be used as the background of a theme of the screen for sharing a link to a user or a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatsharethemebackdrop>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct ChatShareThemeBackdrop {
    /// Unique identifier of the backdrop
    pub id: i64,
    /// Name of the backdrop
    pub name: Box<str>,
    /// Colors of the backdrop
    pub colors: crate::types::UniqueGiftBackdropColors,
}
impl ChatShareThemeBackdrop {
    /// Creates a new `ChatShareThemeBackdrop`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier of the backdrop
    /// * `name` - Name of the backdrop
    /// * `colors` - Colors of the backdrop
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1: Into<Box<str>>,
        T2: Into<crate::types::UniqueGiftBackdropColors>,
    >(
        id: T0,
        name: T1,
        colors: T2,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            colors: colors.into(),
        }
    }

    /// Unique identifier of the backdrop
    #[must_use]
    pub fn id<T: Into<i64>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Name of the backdrop
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }

    /// Colors of the backdrop
    #[must_use]
    pub fn colors<T: Into<crate::types::UniqueGiftBackdropColors>>(mut self, val: T) -> Self {
        self.colors = val.into();
        self
    }
}
