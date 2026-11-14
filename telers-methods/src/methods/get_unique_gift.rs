/// Use this method to get information about a unique gift by its name. Returns a [`crate::types::UniqueGift`] object on success.
/// # Documentation
/// <https://core.telegram.org/bots/api#getuniquegift>
/// # Returns
/// - `crate::types::UniqueGift`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct GetUniqueGift {
    /// Unique name of the gift
    pub name: Box<str>,
}
impl GetUniqueGift {
    /// Creates a new `GetUniqueGift`.
    ///
    /// # Arguments
    /// * `name` - Unique name of the gift
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(name: T0) -> Self {
        Self {
            name: name.into(),
        }
    }

    /// Unique name of the gift
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = val.into();
        self
    }
}
impl super::TelegramMethod for GetUniqueGift {
    type Method = Self;
    type Return = crate::types::UniqueGift;

    fn build_request(self) -> super::Request<Self::Method> {
        super::Request::new("getUniqueGift", self, None)
    }
}
