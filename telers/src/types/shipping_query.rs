use serde::{Deserialize, Serialize};
/// This object contains information about an incoming shipping query.
/// # Documentation
/// <https://core.telegram.org/bots/api#shippingquery>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: Box<str>,
    /// User who sent the query
    pub from: Box<crate::types::User>,
    /// Bot-specified invoice payload
    pub invoice_payload: Box<str>,
    /// User specified shipping address
    pub shipping_address: crate::types::ShippingAddress,
}
impl ShippingQuery {
    /// Creates a new `ShippingQuery`.
    ///
    /// # Arguments
    /// * `id` - Unique query identifier
    /// * `from` - User who sent the query
    /// * `invoice_payload` - Bot-specified invoice payload
    /// * `shipping_address` - User specified shipping address
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::User>,
        T2: Into<Box<str>>,
        T3: Into<crate::types::ShippingAddress>,
    >(
        id: T0,
        from: T1,
        invoice_payload: T2,
        shipping_address: T3,
    ) -> Self {
        Self {
            id: id.into(),
            from: Box::new(from.into()),
            invoice_payload: invoice_payload.into(),
            shipping_address: shipping_address.into(),
        }
    }

    /// Unique query identifier
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// User who sent the query
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.from = Box::new(val.into());
        self
    }

    /// Bot-specified invoice payload
    #[must_use]
    pub fn invoice_payload<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.invoice_payload = val.into();
        self
    }

    /// User specified shipping address
    #[must_use]
    pub fn shipping_address<T: Into<crate::types::ShippingAddress>>(mut self, val: T) -> Self {
        self.shipping_address = val.into();
        self
    }
}
