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
    pub fn address<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.address = val.into();
        this
    }

    /// Location of the business
    #[must_use]
    pub fn location<T: Into<crate::types::Location>>(self, val: T) -> Self {
        let mut this = self;
        this.location = Some(val.into());
        this
    }

    /// Location of the business
    #[must_use]
    pub fn location_option<T: Into<crate::types::Location>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.location = val.map(Into::into);
        this
    }
}
