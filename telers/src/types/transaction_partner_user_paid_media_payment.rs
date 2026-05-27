use serde::{Deserialize, Serialize};
/// This object represents a/an paid media payment transaction partner user.
/// # Notes
/// This object represents a transaction partner user from original field `paid_media_payment`.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerUserPaidMediaPayment {
    /// Information about the user
    pub user: Box<crate::types::User>,
    /// Information about the affiliate that received a commission via this transaction. Can be available only for `invoice_payment` and `paid_media_payment` transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<crate::types::AffiliateInfo>,
    /// Information about the paid media bought by the user; for `paid_media_payment` transactions only
    pub paid_media: Box<[crate::types::PaidMedia]>,
    /// Bot-specified paid media payload. Can be available only for `paid_media_payment` transactions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media_payload: Option<Box<str>>,
}
impl TransactionPartnerUserPaidMediaPayment {
    /// Creates a new `TransactionPartnerUserPaidMediaPayment`.
    ///
    /// # Arguments
    /// * `user` - Information about the user
    /// * `paid_media` - Information about the paid media bought by the user; for `paid_media_payment` transactions only
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<crate::types::User>,
        T1Item: Into<crate::types::PaidMedia>,
        T1: IntoIterator<Item = T1Item>,
    >(
        user: T0,
        paid_media: T1,
    ) -> Self {
        Self {
            user: Box::new(user.into()),
            affiliate: None,
            paid_media: paid_media.into_iter().map(Into::into).collect(),
            paid_media_payload: None,
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

    /// Information about the paid media bought by the user; for `paid_media_payment` transactions only
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn paid_medias<T: Into<Box<[crate::types::PaidMedia]>>>(mut self, val: T) -> Self {
        self.paid_media = self
            .paid_media
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Information about the paid media bought by the user; for `paid_media_payment` transactions only
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn paid_media<T: Into<crate::types::PaidMedia>>(mut self, val: T) -> Self {
        self.paid_media = self
            .paid_media
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Bot-specified paid media payload. Can be available only for `paid_media_payment` transactions.
    #[must_use]
    pub fn paid_media_payload<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.paid_media_payload = Some(val.into());
        self
    }

    /// Bot-specified paid media payload. Can be available only for `paid_media_payment` transactions.
    #[must_use]
    pub fn paid_media_payload_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.paid_media_payload = val.map(Into::into);
        self
    }
}
