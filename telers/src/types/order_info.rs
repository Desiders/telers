use serde::{Deserialize, Serialize};
/// This object represents information about an order.
/// # Documentation
/// <https://core.telegram.org/bots/api#orderinfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderInfo {
    /// User name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<str>>,
    /// User's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<Box<str>>,
    /// User email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Box<str>>,
    /// User shipping address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<crate::types::ShippingAddress>,
}
impl OrderInfo {
    /// Creates a new `OrderInfo`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            name: None,
            phone_number: None,
            email: None,
            shipping_address: None,
        }
    }

    /// User name
    #[must_use]
    pub fn name<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.name = Some(val.into());
        self
    }

    /// User name
    #[must_use]
    pub fn name_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.name = val.map(Into::into);
        self
    }

    /// User's phone number
    #[must_use]
    pub fn phone_number<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.phone_number = Some(val.into());
        self
    }

    /// User's phone number
    #[must_use]
    pub fn phone_number_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.phone_number = val.map(Into::into);
        self
    }

    /// User email
    #[must_use]
    pub fn email<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.email = Some(val.into());
        self
    }

    /// User email
    #[must_use]
    pub fn email_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.email = val.map(Into::into);
        self
    }

    /// User shipping address
    #[must_use]
    pub fn shipping_address<T: Into<crate::types::ShippingAddress>>(mut self, val: T) -> Self {
        self.shipping_address = Some(val.into());
        self
    }

    /// User shipping address
    #[must_use]
    pub fn shipping_address_option<T: Into<crate::types::ShippingAddress>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.shipping_address = val.map(Into::into);
        self
    }
}
impl Default for OrderInfo {
    fn default() -> Self {
        Self::new()
    }
}
