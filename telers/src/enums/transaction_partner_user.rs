use crate::types::TransactionPartnerUser;
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// Describes a transaction with a user.
/// Currently, it can be one of
/// - [`TransactionPartnerUserBusinessAccountTransfer`]
/// - [`TransactionPartnerUserGiftPurchase`]
/// - [`TransactionPartnerUserInvoicePayment`]
/// - [`TransactionPartnerUserPaidMediaPayment`]
/// - [`TransactionPartnerUserPremiumPurchase`]
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, IntoStaticStr)]
pub enum TransactionPartnerUserType {
    #[strum(serialize = "invoice_payment")]
    InvoicePayment,
    #[strum(serialize = "paid_media_payment")]
    PaidMediaPayment,
    #[strum(serialize = "gift_purchase")]
    GiftPurchase,
    #[strum(serialize = "premium_purchase")]
    PremiumPurchase,
    #[strum(serialize = "business_account_transfer")]
    BusinessAccountTransfer,
}
impl TransactionPartnerUserType {
    #[must_use]
    pub const fn all() -> [TransactionPartnerUserType; 5usize] {
        [
            TransactionPartnerUserType::InvoicePayment,
            TransactionPartnerUserType::PaidMediaPayment,
            TransactionPartnerUserType::GiftPurchase,
            TransactionPartnerUserType::PremiumPurchase,
            TransactionPartnerUserType::BusinessAccountTransfer,
        ]
    }
}
impl From<TransactionPartnerUserType> for Box<str> {
    fn from(val: TransactionPartnerUserType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<TransactionPartnerUserType> for String {
    fn from(val: TransactionPartnerUserType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for TransactionPartnerUserType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a TransactionPartnerUser> for TransactionPartnerUserType {
    fn from(val: &'a TransactionPartnerUser) -> Self {
        match val {
            TransactionPartnerUser::InvoicePayment(_) => TransactionPartnerUserType::InvoicePayment,
            TransactionPartnerUser::PaidMediaPayment(_) => {
                TransactionPartnerUserType::PaidMediaPayment
            }
            TransactionPartnerUser::GiftPurchase(_) => TransactionPartnerUserType::GiftPurchase,
            TransactionPartnerUser::PremiumPurchase(_) => {
                TransactionPartnerUserType::PremiumPurchase
            }
            TransactionPartnerUser::BusinessAccountTransfer(_) => {
                TransactionPartnerUserType::BusinessAccountTransfer
            }
        }
    }
}
impl From<TransactionPartnerUser> for TransactionPartnerUserType {
    fn from(val: TransactionPartnerUser) -> Self {
        TransactionPartnerUserType::from(&val)
    }
}
