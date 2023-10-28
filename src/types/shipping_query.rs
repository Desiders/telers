use super::{ShippingAddress, Update, User};

use crate::errors::ConvertUpdateToTypeError;

use serde::Deserialize;

/// This object contains information about an incoming shipping query.
/// # Documentation
/// <https://core.telegram.org/bots/api#shippingquery>
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: Box<str>,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: Box<str>,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}

impl ShippingQuery {
    /// Gets the sender user ID from the shipping query
    #[must_use]
    pub const fn sender_user_id(&self) -> i64 {
        self.from.id
    }

    /// Gets the sender user ID from the shipping query
    /// # Notes
    /// Alias to `sender_user_id` method
    #[must_use]
    pub const fn user_id(&self) -> i64 {
        self.sender_user_id()
    }
}

impl TryFrom<Update> for ShippingQuery {
    type Error = ConvertUpdateToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        if let Some(shipping_query) = update.shipping_query {
            Ok(shipping_query)
        } else {
            Err(ConvertUpdateToTypeError::new("ShippingQuery"))
        }
    }
}
