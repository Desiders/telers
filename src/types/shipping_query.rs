use super::{ShippingAddress, Update, User};

use serde::{Deserialize, Serialize};

/// This object contains information about an incoming shipping query.
/// <https://core.telegram.org/bots/api#shippingquery>_
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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

impl From<Update> for ShippingQuery {
    fn from(update: Update) -> Self {
        update
            .shipping_query
            .expect("Update isn't a `ShippingQuery`")
    }
}
