use super::{AffiliateInfo, Gift, PaidMedia, User};

use serde::Deserialize;

/// Describes a transaction with a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "transaction_type", rename_all = "snake_case")]
pub enum TransactionPartnerUser {
    InvoicePayment(TransactionPartnerUserInvoicePayment),
    PaidMediaPayment(TransactionPartnerUserPaidMediaPayment),
    GiftPurchase(TransactionPartnerUserGiftPurchase),
    PremiumPurchase(TransactionPartnerUserPremiumPurchase),
    BusinessAccountTransfer(TransactionPartnerUserBusinessAccountTransfer),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TransactionPartnerUserInvoicePayment {
    /// Information about the user
    pub user: User,
    /// Information about the affiliate that received a commission via this transaction
    pub affiliate: Option<AffiliateInfo>,
    /// Bot-specified invoice payload
    pub invoice_payload: Option<Box<str>>,
    /// The duration of the paid subscription
    pub subscription_period: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TransactionPartnerUserPaidMediaPayment {
    /// Information about the user
    pub user: User,
    /// Information about the affiliate that received a commission via this transaction
    pub affiliate: Option<AffiliateInfo>,
    /// Information about the paid media bought by the user
    pub paid_media: Box<[PaidMedia]>,
    /// Bot-specified paid media payload
    pub paid_media_payload: Option<Box<str>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TransactionPartnerUserGiftPurchase {
    /// Information about the user
    pub user: User,
    /// The gift sent to the user by the bot;
    pub gift: Gift,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TransactionPartnerUserPremiumPurchase {
    /// Information about the user
    pub user: User,
    /// Number of months the gifted Telegram Premium subscription will be active for
    pub premium_subscription_duration: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TransactionPartnerUserBusinessAccountTransfer {
    /// Information about the user
    pub user: User,
}

impl TransactionPartnerUser {
    #[must_use]
    pub const fn user(&self) -> &User {
        match self {
            Self::InvoicePayment(TransactionPartnerUserInvoicePayment { user, .. })
            | Self::PaidMediaPayment(TransactionPartnerUserPaidMediaPayment { user, .. })
            | Self::GiftPurchase(TransactionPartnerUserGiftPurchase { user, .. })
            | Self::PremiumPurchase(TransactionPartnerUserPremiumPurchase { user, .. })
            | Self::BusinessAccountTransfer(TransactionPartnerUserBusinessAccountTransfer {
                user,
                ..
            }) => user,
        }
    }

    #[must_use]
    pub const fn affiliate(&self) -> Option<&AffiliateInfo> {
        match self {
            Self::InvoicePayment(TransactionPartnerUserInvoicePayment { affiliate, .. })
            | Self::PaidMediaPayment(TransactionPartnerUserPaidMediaPayment {
                affiliate, ..
            }) => affiliate.as_ref(),
            _ => None,
        }
    }

    #[must_use]
    pub const fn gift(&self) -> Option<&Gift> {
        match self {
            Self::GiftPurchase(TransactionPartnerUserGiftPurchase { gift, .. }) => Some(gift),
            _ => None,
        }
    }

    #[must_use]
    pub const fn premium_subscription_duration(&self) -> Option<i64> {
        match self {
            Self::PremiumPurchase(TransactionPartnerUserPremiumPurchase {
                premium_subscription_duration,
                ..
            }) => Some(*premium_subscription_duration),
            _ => None,
        }
    }

    #[must_use]
    pub const fn paid_media(&self) -> Option<&[PaidMedia]> {
        match self {
            Self::PaidMediaPayment(TransactionPartnerUserPaidMediaPayment {
                paid_media, ..
            }) => Some(&**paid_media),
            _ => None,
        }
    }

    #[must_use]
    pub const fn paid_media_payload(&self) -> Option<&str> {
        match self {
            Self::PaidMediaPayment(TransactionPartnerUserPaidMediaPayment {
                paid_media_payload: Some(paid_media_payload),
                ..
            }) => Some(&**paid_media_payload),
            _ => None,
        }
    }

    #[must_use]
    pub const fn invoice_payload(&self) -> Option<&str> {
        match self {
            Self::InvoicePayment(TransactionPartnerUserInvoicePayment {
                invoice_payload: Some(invoice_payload),
                ..
            }) => Some(&**invoice_payload),
            _ => None,
        }
    }

    #[must_use]
    pub const fn subscription_period(&self) -> Option<i64> {
        match self {
            Self::InvoicePayment(TransactionPartnerUserInvoicePayment {
                subscription_period,
                ..
            }) => *subscription_period,
            _ => None,
        }
    }
}
