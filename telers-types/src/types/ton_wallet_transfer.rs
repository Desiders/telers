/// Describes a service message about a transfer with a TON wallet.
/// # Documentation
/// <https://core.telegram.org/bots/api#tonwallettransfer>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct TonWalletTransfer {
    /// Unique identifier of the transaction; may be empty if the transfer hasn't been sent yet
    pub transaction_id: Box<str>,
    /// Address of the transfer peer in the TON blockchain
    pub peer_address: Box<str>,
    /// The amount of transferred cryptocurrency, in the smallest units of the cryptocurrency, as a decimal number; negative for outgoing transfers
    pub amount: Box<str>,
    /// Comment added to the transfer; omitted if the comment is encrypted
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub comment: Option<Box<str>>,
}
impl TonWalletTransfer {
    /// Creates a new `TonWalletTransfer`.
    ///
    /// # Arguments
    /// * `transaction_id` - Unique identifier of the transaction; may be empty if the transfer hasn't been sent yet
    /// * `peer_address` - Address of the transfer peer in the TON blockchain
    /// * `amount` - The amount of transferred cryptocurrency, in the smallest units of the cryptocurrency, as a decimal number; negative for outgoing transfers
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<Box<str>>>(
        transaction_id: T0,
        peer_address: T1,
        amount: T2,
    ) -> Self {
        Self {
            transaction_id: transaction_id.into(),
            peer_address: peer_address.into(),
            amount: amount.into(),
            comment: None,
        }
    }

    /// Unique identifier of the transaction; may be empty if the transfer hasn't been sent yet
    #[must_use]
    pub fn transaction_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.transaction_id = val.into();
        self
    }

    /// Address of the transfer peer in the TON blockchain
    #[must_use]
    pub fn peer_address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.peer_address = val.into();
        self
    }

    /// The amount of transferred cryptocurrency, in the smallest units of the cryptocurrency, as a decimal number; negative for outgoing transfers
    #[must_use]
    pub fn amount<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.amount = val.into();
        self
    }

    /// Comment added to the transfer; omitted if the comment is encrypted
    #[must_use]
    pub fn comment<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.comment = Some(val.into());
        self
    }

    /// Comment added to the transfer; omitted if the comment is encrypted
    #[must_use]
    pub fn comment_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.comment = val.map(Into::into);
        self
    }
}
