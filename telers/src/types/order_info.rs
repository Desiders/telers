use super::ShippingAddress;

use serde::Deserialize;

/// This object represents information about an order.
/// # Documentation
/// <https://core.telegram.org/bots/api#orderinfo>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct OrderInfo {
    /// User name
    pub name: Option<Box<str>>,
    /// User's phone number
    pub phone_number: Option<Box<str>>,
    /// User email
    pub email: Option<Box<str>>,
    /// User shipping address
    pub shipping_address: Option<ShippingAddress>,
}
