use serde::{Deserialize, Serialize};
/// Describes a transaction with a user.
/// Currently, it can be one of
/// - [`TransactionPartnerUserBusinessAccountTransfer`]
/// - [`TransactionPartnerUserGiftPurchase`]
/// - [`TransactionPartnerUserInvoicePayment`]
/// - [`TransactionPartnerUserPaidMediaPayment`]
/// - [`TransactionPartnerUserPremiumPurchase`]
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartneruser>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "transaction_type", rename_all = "snake_case")]
pub enum TransactionPartnerUser {
    InvoicePayment(crate::types::TransactionPartnerUserInvoicePayment),
    PaidMediaPayment(crate::types::TransactionPartnerUserPaidMediaPayment),
    GiftPurchase(crate::types::TransactionPartnerUserGiftPurchase),
    PremiumPurchase(crate::types::TransactionPartnerUserPremiumPurchase),
    BusinessAccountTransfer(crate::types::TransactionPartnerUserBusinessAccountTransfer),
}
impl TransactionPartnerUser {
    /// Helper method for field `affiliate`.
    ///
    /// Information about the affiliate that received a commission via this transaction. Can be available only for `invoice_payment` and `paid_media_payment` transactions.
    #[must_use]
    pub fn affiliate(&self) -> Option<&crate::types::AffiliateInfo> {
        match self {
            Self::InvoicePayment(val) => val.affiliate.as_ref(),
            Self::PaidMediaPayment(val) => val.affiliate.as_ref(),
            _ => None,
        }
    }

    /// Helper method for field `gift`.
    ///
    /// The gift sent to the user by the bot; for `gift_purchase` transactions only
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::Gift> {
        match self {
            Self::GiftPurchase(val) => Some(val.gift.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `invoice_payload`.
    ///
    /// Bot-specified invoice payload. Can be available only for `invoice_payment` transactions.
    #[must_use]
    pub fn invoice_payload(&self) -> Option<&str> {
        match self {
            Self::InvoicePayment(val) => val.invoice_payload.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `paid_media`.
    ///
    /// Information about the paid media bought by the user; for `paid_media_payment` transactions only
    #[must_use]
    pub fn paid_media(&self) -> Option<&[crate::types::PaidMedia]> {
        match self {
            Self::PaidMediaPayment(val) => Some(val.paid_media.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `paid_media_payload`.
    ///
    /// Bot-specified paid media payload. Can be available only for `paid_media_payment` transactions.
    #[must_use]
    pub fn paid_media_payload(&self) -> Option<&str> {
        match self {
            Self::PaidMediaPayment(val) => val.paid_media_payload.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `premium_subscription_duration`.
    ///
    /// Number of months the gifted Telegram Premium subscription will be active for; for `premium_purchase` transactions only
    #[must_use]
    pub fn premium_subscription_duration(&self) -> Option<i64> {
        match self {
            Self::PremiumPurchase(val) => Some(val.premium_subscription_duration),
            _ => None,
        }
    }

    /// Helper method for field `subscription_period`.
    ///
    /// The duration of the paid subscription. Can be available only for `invoice_payment` transactions.
    #[must_use]
    pub fn subscription_period(&self) -> Option<i64> {
        match self {
            Self::InvoicePayment(val) => val.subscription_period,
            _ => None,
        }
    }

    /// Helper method for field `user`.
    ///
    /// Information about the user
    #[must_use]
    pub fn user(&self) -> &crate::types::User {
        match self {
            Self::InvoicePayment(val) => val.user.as_ref(),
            Self::PaidMediaPayment(val) => val.user.as_ref(),
            Self::GiftPurchase(val) => val.user.as_ref(),
            Self::PremiumPurchase(val) => val.user.as_ref(),
            Self::BusinessAccountTransfer(val) => val.user.as_ref(),
        }
    }

    /// Helper method for nested field `added_to_attachment_menu`.
    #[must_use]
    pub fn added_to_attachment_menu(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.added_to_attachment_menu
        }
    }

    /// Helper method for nested field `affiliate_chat`.
    #[must_use]
    pub fn affiliate_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::InvoicePayment(val) => val
                .affiliate
                .as_ref()
                .and_then(|inner| inner.affiliate_chat.as_deref()),
            Self::PaidMediaPayment(val) => val
                .affiliate
                .as_ref()
                .and_then(|inner| inner.affiliate_chat.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `affiliate_user`.
    #[must_use]
    pub fn affiliate_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::InvoicePayment(val) => val
                .affiliate
                .as_ref()
                .and_then(|inner| inner.affiliate_user.as_deref()),
            Self::PaidMediaPayment(val) => val
                .affiliate
                .as_ref()
                .and_then(|inner| inner.affiliate_user.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `allows_users_to_create_topics`.
    #[must_use]
    pub fn allows_users_to_create_topics(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.allows_users_to_create_topics
        }
    }

    /// Helper method for nested field `amount`.
    #[must_use]
    pub fn amount(&self) -> Option<i64> {
        match self {
            Self::InvoicePayment(val) => val.affiliate.as_ref().map(|inner| inner.amount),
            Self::PaidMediaPayment(val) => val.affiliate.as_ref().map(|inner| inner.amount),
            _ => None,
        }
    }

    /// Helper method for nested field `background`.
    #[must_use]
    pub fn background(&self) -> Option<&crate::types::GiftBackground> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                inner.background.as_ref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `can_connect_to_business`.
    #[must_use]
    pub fn can_connect_to_business(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.can_connect_to_business
        }
    }

    /// Helper method for nested field `can_join_groups`.
    #[must_use]
    pub fn can_join_groups(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.can_join_groups
        }
    }

    /// Helper method for nested field `can_read_all_group_messages`.
    #[must_use]
    pub fn can_read_all_group_messages(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.can_read_all_group_messages
        }
    }

    /// Helper method for nested field `commission_per_mille`.
    #[must_use]
    pub fn commission_per_mille(&self) -> Option<i64> {
        match self {
            Self::InvoicePayment(val) => val
                .affiliate
                .as_ref()
                .map(|inner| inner.commission_per_mille),
            Self::PaidMediaPayment(val) => val
                .affiliate
                .as_ref()
                .map(|inner| inner.commission_per_mille),
            _ => None,
        }
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> &str {
        {
            let inner = self.user();
            inner.first_name.as_ref()
        }
    }

    /// Helper method for nested field `has_colors`.
    #[must_use]
    pub fn has_colors(&self) -> Option<bool> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                inner.has_colors
            }
            _ => None,
        }
    }

    /// Helper method for nested field `has_main_web_app`.
    #[must_use]
    pub fn has_main_web_app(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.has_main_web_app
        }
    }

    /// Helper method for nested field `has_topics_enabled`.
    #[must_use]
    pub fn has_topics_enabled(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.has_topics_enabled
        }
    }

    /// Helper method for nested field `is_bot`.
    #[must_use]
    pub fn is_bot(&self) -> bool {
        {
            let inner = self.user();
            inner.is_bot
        }
    }

    /// Helper method for nested field `language_code`.
    #[must_use]
    pub fn language_code(&self) -> Option<&str> {
        {
            let inner = self.user();
            inner.language_code.as_deref()
        }
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        {
            let inner = self.user();
            inner.last_name.as_deref()
        }
    }

    /// Helper method for nested field `nanostar_amount`.
    #[must_use]
    pub fn nanostar_amount(&self) -> Option<i32> {
        match self {
            Self::InvoicePayment(val) => val
                .affiliate
                .as_ref()
                .and_then(|inner| inner.nanostar_amount),
            Self::PaidMediaPayment(val) => val
                .affiliate
                .as_ref()
                .and_then(|inner| inner.nanostar_amount),
            _ => None,
        }
    }

    /// Helper method for nested field `personal_remaining_count`.
    #[must_use]
    pub fn personal_remaining_count(&self) -> Option<i64> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                inner.personal_remaining_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `personal_total_count`.
    #[must_use]
    pub fn personal_total_count(&self) -> Option<i64> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                inner.personal_total_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `publisher_chat`.
    #[must_use]
    pub fn publisher_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                inner.publisher_chat.as_deref()
            }
            _ => None,
        }
    }

    /// Helper method for nested field `remaining_count`.
    #[must_use]
    pub fn remaining_count(&self) -> Option<i64> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                inner.remaining_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `star_count`.
    #[must_use]
    pub fn star_count(&self) -> Option<i64> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                Some(inner.star_count)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `sticker`.
    #[must_use]
    pub fn sticker(&self) -> Option<&crate::types::Sticker> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                Some(inner.sticker.as_ref())
            }
            _ => None,
        }
    }

    /// Helper method for nested field `supports_inline_queries`.
    #[must_use]
    pub fn supports_inline_queries(&self) -> Option<bool> {
        {
            let inner = self.user();
            inner.supports_inline_queries
        }
    }

    /// Helper method for nested field `total_count`.
    #[must_use]
    pub fn total_count(&self) -> Option<i64> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                inner.total_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `unique_gift_variant_count`.
    #[must_use]
    pub fn unique_gift_variant_count(&self) -> Option<i64> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                inner.unique_gift_variant_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `upgrade_star_count`.
    #[must_use]
    pub fn upgrade_star_count(&self) -> Option<i64> {
        match self {
            Self::GiftPurchase(val) => {
                let inner = val.gift.as_ref();
                inner.upgrade_star_count
            }
            _ => None,
        }
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        {
            let inner = self.user();
            inner.username.as_deref()
        }
    }
}
impl From<crate::types::TransactionPartnerUserInvoicePayment> for TransactionPartnerUser {
    fn from(val: crate::types::TransactionPartnerUserInvoicePayment) -> Self {
        Self::InvoicePayment(val)
    }
}
impl TryFrom<TransactionPartnerUser> for crate::types::TransactionPartnerUserInvoicePayment {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartnerUser) -> Result<Self, Self::Error> {
        if let TransactionPartnerUser::InvoicePayment(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartnerUser),
                stringify!(TransactionPartnerUserInvoicePayment),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerUserPaidMediaPayment> for TransactionPartnerUser {
    fn from(val: crate::types::TransactionPartnerUserPaidMediaPayment) -> Self {
        Self::PaidMediaPayment(val)
    }
}
impl TryFrom<TransactionPartnerUser> for crate::types::TransactionPartnerUserPaidMediaPayment {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartnerUser) -> Result<Self, Self::Error> {
        if let TransactionPartnerUser::PaidMediaPayment(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartnerUser),
                stringify!(TransactionPartnerUserPaidMediaPayment),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerUserGiftPurchase> for TransactionPartnerUser {
    fn from(val: crate::types::TransactionPartnerUserGiftPurchase) -> Self {
        Self::GiftPurchase(val)
    }
}
impl TryFrom<TransactionPartnerUser> for crate::types::TransactionPartnerUserGiftPurchase {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartnerUser) -> Result<Self, Self::Error> {
        if let TransactionPartnerUser::GiftPurchase(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartnerUser),
                stringify!(TransactionPartnerUserGiftPurchase),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerUserPremiumPurchase> for TransactionPartnerUser {
    fn from(val: crate::types::TransactionPartnerUserPremiumPurchase) -> Self {
        Self::PremiumPurchase(val)
    }
}
impl TryFrom<TransactionPartnerUser> for crate::types::TransactionPartnerUserPremiumPurchase {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartnerUser) -> Result<Self, Self::Error> {
        if let TransactionPartnerUser::PremiumPurchase(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartnerUser),
                stringify!(TransactionPartnerUserPremiumPurchase),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerUserBusinessAccountTransfer> for TransactionPartnerUser {
    fn from(val: crate::types::TransactionPartnerUserBusinessAccountTransfer) -> Self {
        Self::BusinessAccountTransfer(val)
    }
}
impl TryFrom<TransactionPartnerUser>
    for crate::types::TransactionPartnerUserBusinessAccountTransfer
{
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartnerUser) -> Result<Self, Self::Error> {
        if let TransactionPartnerUser::BusinessAccountTransfer(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartnerUser),
                stringify!(TransactionPartnerUserBusinessAccountTransfer),
            ))
        }
    }
}
