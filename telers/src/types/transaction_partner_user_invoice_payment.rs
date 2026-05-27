use serde::{Deserialize, Serialize};
/// This object represents a/an invoice payment transaction partner user.
/// # Notes
/// This object represents a transaction partner user from original field `invoice_payment`.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerUserInvoicePayment {
    /// Information about the user
    pub user: Box<crate::types::User>,
    /// Information about the affiliate that received a commission via this transaction. Can be available only for `invoice_payment` and `paid_media_payment` transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<crate::types::AffiliateInfo>,
    /// Bot-specified invoice payload. Can be available only for `invoice_payment` transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_payload: Option<Box<str>>,
    /// The duration of the paid subscription. Can be available only for `invoice_payment` transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i64>,
}
impl TransactionPartnerUserInvoicePayment {
    /// Creates a new `TransactionPartnerUserInvoicePayment`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<crate::types::User>>(user: T0) -> Self {
        Self {
            user: Box::new(user.into()),
            affiliate: None,
            invoice_payload: None,
            subscription_period: None,
        }
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }

    /// Information about the affiliate that received a commission via this transaction. Can be available only for `invoice_payment` and `paid_media_payment` transactions.
    #[must_use]
    pub fn affiliate<T: Into<crate::types::AffiliateInfo>>(mut self, val: T) -> Self {
        self.affiliate = Some(val.into());
        self
    }

    /// Information about the affiliate that received a commission via this transaction. Can be available only for `invoice_payment` and `paid_media_payment` transactions.
    #[must_use]
    pub fn affiliate_option<T: Into<crate::types::AffiliateInfo>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.affiliate = val.map(Into::into);
        self
    }

    /// Bot-specified invoice payload. Can be available only for `invoice_payment` transactions.
    #[must_use]
    pub fn invoice_payload<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.invoice_payload = Some(val.into());
        self
    }

    /// Bot-specified invoice payload. Can be available only for `invoice_payment` transactions.
    #[must_use]
    pub fn invoice_payload_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.invoice_payload = val.map(Into::into);
        self
    }

    /// The duration of the paid subscription. Can be available only for `invoice_payment` transactions.
    #[must_use]
    pub fn subscription_period<T: Into<i64>>(mut self, val: T) -> Self {
        self.subscription_period = Some(val.into());
        self
    }

    /// The duration of the paid subscription. Can be available only for `invoice_payment` transactions.
    #[must_use]
    pub fn subscription_period_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.subscription_period = val.map(Into::into);
        self
    }
}
