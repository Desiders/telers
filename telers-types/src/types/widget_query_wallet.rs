/// This object describes a TON wallet of the user, which is used in a query from a widget.
/// # Documentation
/// <https://core.telegram.org/bots/api#widgetquerywallet>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct WidgetQueryWallet {
    /// Address of the wallet
    pub address: Box<str>,
    /// Balance of the wallet, in the smallest units of the cryptocurrency, as a decimal number
    pub amount: Box<str>,
    /// Hex-encoded public key of the wallet; omitted if unknown
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub public_key: Option<Box<str>>,
    /// Base64-encoded serialized `StateInit` of the wallet contract; omitted if and only if `public_key` is omitted
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub state_init: Option<Box<str>>,
}
impl WidgetQueryWallet {
    /// Creates a new `WidgetQueryWallet`.
    ///
    /// # Arguments
    /// * `address` - Address of the wallet
    /// * `amount` - Balance of the wallet, in the smallest units of the cryptocurrency, as a decimal number
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(address: T0, amount: T1) -> Self {
        Self {
            address: address.into(),
            amount: amount.into(),
            public_key: None,
            state_init: None,
        }
    }

    /// Address of the wallet
    #[must_use]
    pub fn address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.address = val.into();
        self
    }

    /// Balance of the wallet, in the smallest units of the cryptocurrency, as a decimal number
    #[must_use]
    pub fn amount<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.amount = val.into();
        self
    }

    /// Hex-encoded public key of the wallet; omitted if unknown
    #[must_use]
    pub fn public_key<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.public_key = Some(val.into());
        self
    }

    /// Hex-encoded public key of the wallet; omitted if unknown
    #[must_use]
    pub fn public_key_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.public_key = val.map(Into::into);
        self
    }

    /// Base64-encoded serialized `StateInit` of the wallet contract; omitted if and only if `public_key` is omitted
    #[must_use]
    pub fn state_init<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.state_init = Some(val.into());
        self
    }

    /// Base64-encoded serialized `StateInit` of the wallet contract; omitted if and only if `public_key` is omitted
    #[must_use]
    pub fn state_init_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.state_init = val.map(Into::into);
        self
    }
}
