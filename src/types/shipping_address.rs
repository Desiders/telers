use serde::{Deserialize, Serialize};

/// This object represents a shipping address.
/// <https://core.telegram.org/bots/api#shippingaddress>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}

impl Default for ShippingAddress {
    fn default() -> Self {
        Self {
            country_code: String::default(),
            state: String::default(),
            city: String::default(),
            street_line1: String::default(),
            street_line2: String::default(),
            post_code: String::default(),
        }
    }
}
