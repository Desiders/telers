use super::ShippingAddress;

use serde::{Deserialize, Serialize};

/// This object represents information about an order.
/// <https://core.telegram.org/bots/api#orderinfo>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct OrderInfo {
    /// *Optional*. User name
    pub name: Option<String>,
    /// *Optional*. User's phone number
    pub phone_number: Option<String>,
    /// *Optional*. User email
    pub email: Option<String>,
    /// *Optional*. User shipping address
    pub shipping_address: Option<ShippingAddress>,
}
