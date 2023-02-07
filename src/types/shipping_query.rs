use super::{ShippingAddress, Update, User};

use crate::error::ConvertUpdateToTypeError;

use serde::Deserialize;

/// This object contains information about an incoming shipping query.
/// # Documentation
/// <https://core.telegram.org/bots/api#shippingquery>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
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

impl TryFrom<Update> for ShippingQuery {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(shipping_query) = update.shipping_query {
            Ok(shipping_query)
        } else {
            Err(ConvertUpdateToTypeError::new(format!(
                "Update `{update:?}` doesn't contain `ShippingQuery`"
            )))
        }
    }
}
