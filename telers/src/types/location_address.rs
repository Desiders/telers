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
    pub fn country_code<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.country_code = val.into();
        this
    }

    /// State of the location
    #[must_use]
    pub fn state<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.state = Some(val.into());
        this
    }

    /// State of the location
    #[must_use]
    pub fn state_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.state = val.map(Into::into);
        this
    }

    /// City of the location
    #[must_use]
    pub fn city<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.city = Some(val.into());
        this
    }

    /// City of the location
    #[must_use]
    pub fn city_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.city = val.map(Into::into);
        this
    }

    /// Street address of the location
    #[must_use]
    pub fn street<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.street = Some(val.into());
        this
    }

    /// Street address of the location
    #[must_use]
    pub fn street_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.street = val.map(Into::into);
        this
    }
}
