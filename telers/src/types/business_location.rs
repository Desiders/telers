use serde::{Deserialize, Serialize};
/// Contains information about the location of a Telegram Business account.
/// # Documentation
/// <https://core.telegram.org/bots/api#businesslocation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessLocation {
    /// Address of the business
    pub address: Box<str>,
    /// Location of the business
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::types::Location>,
}
impl BusinessLocation {
    /// Creates a new `BusinessLocation`.
    ///
    /// # Arguments
    /// * `address` - Address of the business
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(address: T0) -> Self {
        Self {
            address: address.into(),
            location: None,
        }
    }

    /// Address of the business
    #[must_use]
    pub fn address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.address = val.into();
        self
    }

    /// Location of the business
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(mut self, val: T) -> Self {
        self.location = Some(val.into());
        self
    }

    /// Location of the business
    #[must_use]
    pub fn location_option<T: Into<crate::types::Location>>(mut self, val: Option<T>) -> Self {
        self.location = val.map(Into::into);
        self
    }
}
