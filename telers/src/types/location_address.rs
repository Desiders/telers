use serde::{Deserialize, Serialize};
/// Describes the physical address of a location.
/// # Documentation
/// <https://core.telegram.org/bots/api#locationaddress>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocationAddress {
    /// The two-letter ISO 3166-1 alpha-2 country code of the country where the location is located
    pub country_code: Box<str>,
    /// State of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<str>>,
    /// City of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<Box<str>>,
    /// Street address of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<Box<str>>,
}
impl LocationAddress {
    /// Creates a new `LocationAddress`.
    ///
    /// # Arguments
    /// * `country_code` - The two-letter ISO 3166-1 alpha-2 country code of the country where the location is located
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(country_code: T0) -> Self {
        Self {
            country_code: country_code.into(),
            state: None,
            city: None,
            street: None,
        }
    }

    /// The two-letter ISO 3166-1 alpha-2 country code of the country where the location is located
    #[must_use]
    pub fn country_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.country_code = val.into();
        self
    }

    /// State of the location
    #[must_use]
    pub fn state<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.state = Some(val.into());
        self
    }

    /// State of the location
    #[must_use]
    pub fn state_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.state = val.map(Into::into);
        self
    }

    /// City of the location
    #[must_use]
    pub fn city<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.city = Some(val.into());
        self
    }

    /// City of the location
    #[must_use]
    pub fn city_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.city = val.map(Into::into);
        self
    }

    /// Street address of the location
    #[must_use]
    pub fn street<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.street = Some(val.into());
        self
    }

    /// Street address of the location
    #[must_use]
    pub fn street_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.street = val.map(Into::into);
        self
    }
}
