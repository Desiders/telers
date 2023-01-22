use serde::Deserialize;

/// This object represents a shipping address.
/// # Documentation
/// <https://core.telegram.org/bots/api#shippingaddress>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
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
