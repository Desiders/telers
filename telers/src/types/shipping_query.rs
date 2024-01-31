use super::{ShippingAddress, Update, UpdateKind, User};

use crate::{errors::ConvertToTypeError, FromEvent};

use serde::Deserialize;

/// This object contains information about an incoming shipping query.
/// # Documentation
/// <https://core.telegram.org/bots/api#shippingquery>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, FromEvent)]
#[event(try_from = Update)]
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

impl TryFrom<Update> for ShippingQuery {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::ShippingQuery(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "ShippingQuery")),
        }
    }
}
