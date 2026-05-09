use serde::{Deserialize, Serialize};
/// This object represents a shipping address.
/// # Documentation
/// <https://core.telegram.org/bots/api#shippingaddress>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShippingAddress {
    /// Two-letter ISO 3166-1 alpha-2 country code
    pub country_code: Box<str>,
    /// State, if applicable
    pub state: Box<str>,
    /// City
    pub city: Box<str>,
    /// First line for the address
    pub street_line1: Box<str>,
    /// Second line for the address
    pub street_line2: Box<str>,
    /// Address post code
    pub post_code: Box<str>,
}
impl ShippingAddress {
    /// Creates a new `ShippingAddress`.
    ///
    /// # Arguments
    /// * `country_code` - Two-letter ISO 3166-1 alpha-2 country code
    /// * `state` - State, if applicable
    /// * `city` - City
    /// * `street_line1` - First line for the address
    /// * `street_line2` - Second line for the address
    /// * `post_code` - Address post code
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<Box<str>>,
        T2: Into<Box<str>>,
        T3: Into<Box<str>>,
        T4: Into<Box<str>>,
        T5: Into<Box<str>>,
    >(
        country_code: T0,
        state: T1,
        city: T2,
        street_line1: T3,
        street_line2: T4,
        post_code: T5,
    ) -> Self {
        Self {
            country_code: country_code.into(),
            state: state.into(),
            city: city.into(),
            street_line1: street_line1.into(),
            street_line2: street_line2.into(),
            post_code: post_code.into(),
        }
    }

    /// Two-letter ISO 3166-1 alpha-2 country code
    #[must_use]
    pub fn country_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.country_code = val.into();
        self
    }

    /// State, if applicable
    #[must_use]
    pub fn state<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.state = val.into();
        self
    }

    /// City
    #[must_use]
    pub fn city<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.city = val.into();
        self
    }

    /// First line for the address
    #[must_use]
    pub fn street_line1<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.street_line1 = val.into();
        self
    }

    /// Second line for the address
    #[must_use]
    pub fn street_line2<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.street_line2 = val.into();
        self
    }

    /// Address post code
    #[must_use]
    pub fn post_code<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.post_code = val.into();
        self
    }
}
