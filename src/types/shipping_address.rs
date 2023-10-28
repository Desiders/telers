use serde::Deserialize;

/// This object represents a shipping address.
/// # Documentation
/// <https://core.telegram.org/bots/api#shippingaddress>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
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
