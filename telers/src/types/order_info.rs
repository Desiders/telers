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
    pub fn name<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.name = Some(val.into());
        this
    }

    /// User name
    #[must_use]
    pub fn name_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.name = val.map(Into::into);
        this
    }

    /// User's phone number
    #[must_use]
    pub fn phone_number<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.phone_number = Some(val.into());
        this
    }

    /// User's phone number
    #[must_use]
    pub fn phone_number_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.phone_number = val.map(Into::into);
        this
    }

    /// User email
    #[must_use]
    pub fn email<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.email = Some(val.into());
        this
    }

    /// User email
    #[must_use]
    pub fn email_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.email = val.map(Into::into);
        this
    }

    /// User shipping address
    #[must_use]
    pub fn shipping_address<T: Into<crate::types::ShippingAddress>>(self, val: T) -> Self {
        let mut this = self;
        this.shipping_address = Some(val.into());
        this
    }

    /// User shipping address
    #[must_use]
    pub fn shipping_address_option<T: Into<crate::types::ShippingAddress>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.shipping_address = val.map(Into::into);
        this
    }
}
impl Default for OrderInfo {
    fn default() -> Self {
        Self::new()
    }
}
