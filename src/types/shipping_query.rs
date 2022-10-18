use super::{ShippingAddress, User};

use serde::{Deserialize, Serialize};

/// This object contains information about an incoming shipping query.
/// <https://core.telegram.org/bots/api#shippingquery>_
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: String,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}

impl Default for ShippingQuery {
    fn default() -> Self {
        Self {
            id: String::default(),
            from: User::default(),
            invoice_payload: String::default(),
            shipping_address: ShippingAddress::default(),
        }
    }
}
