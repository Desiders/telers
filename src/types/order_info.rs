use super::ShippingAddress;

use serde::Deserialize;

/// This object represents information about an order.
/// # Documentation
/// <https://core.telegram.org/bots/api#orderinfo>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct OrderInfo {
    /// User name
    pub name: Option<String>,
    /// User's phone number
    pub phone_number: Option<String>,
    /// User email
    pub email: Option<String>,
    /// User shipping address
    pub shipping_address: Option<ShippingAddress>,
}
