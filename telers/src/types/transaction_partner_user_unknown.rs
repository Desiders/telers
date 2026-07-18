use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a [`crate::types::TransactionPartnerUser`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionPartnerUserUnknown {
    /// Raw `transaction_type` value of the variant unknown to this version of the library
    pub transaction_type: Box<str>,
    /// Type of the transaction partner, always `user`
    pub r#type: Box<str>,
    /// Information about the user
    pub user: Box<crate::types::User>,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl TransactionPartnerUserUnknown {
    /// Creates a new `TransactionPartnerUserUnknown`.
    ///
    /// # Arguments
    /// * `transaction_type` - Raw `transaction_type` value of the variant unknown to this version of the library
    /// * `type` - Type of the transaction partner, always `user`
    /// * `user` - Information about the user
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>, T2: Into<crate::types::User>>(
        transaction_type: T0,
        r#type: T1,
        user: T2,
    ) -> Self {
        Self {
            transaction_type: transaction_type.into(),
            r#type: r#type.into(),
            user: Box::new(user.into()),
            extra: BTreeMap::new(),
        }
    }

    /// Raw `transaction_type` value of the variant unknown to this version of the library
    #[must_use]
    pub fn transaction_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.transaction_type = val.into();
        self
    }

    /// Type of the transaction partner, always `user`
    #[must_use]
    pub fn r#type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Information about the user
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Box::new(val.into());
        self
    }
}
