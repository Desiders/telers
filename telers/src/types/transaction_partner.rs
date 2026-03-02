use serde::{Deserialize, Serialize};
/// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of
/// - [`TransactionPartnerUser`]
/// - [`TransactionPartnerChat`]
/// - [`TransactionPartnerAffiliateProgram`]
/// - [`TransactionPartnerFragment`]
/// - [`TransactionPartnerTelegramAds`]
/// - [`TransactionPartnerTelegramApi`]
/// - [`TransactionPartnerOther`]
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartner>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionPartner {
    User(crate::types::TransactionPartnerUser),
    Chat(crate::types::TransactionPartnerChat),
    AffiliateProgram(crate::types::TransactionPartnerAffiliateProgram),
    Fragment(crate::types::TransactionPartnerFragment),
    TelegramAds(crate::types::TransactionPartnerTelegramAds),
    TelegramApi(crate::types::TransactionPartnerTelegramApi),
    Other(crate::types::TransactionPartnerOther),
}
impl TransactionPartner {
    /// Helper method for field `chat`.
    ///
    /// Information about the chat
    #[must_use]
    pub fn chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Chat(val) => Some(val.chat.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `commission_per_mille`.
    ///
    /// The number of Telegram Stars received by the bot for each 1000 Telegram Stars received by the affiliate program sponsor from referred users
    #[must_use]
    pub fn commission_per_mille(&self) -> Option<i64> {
        match self {
            Self::AffiliateProgram(val) => Some(val.commission_per_mille),
            _ => None,
        }
    }

    /// Helper method for field `gift`.
    ///
    /// The gift sent to the chat by the bot
    #[must_use]
    pub fn gift(&self) -> Option<&crate::types::Gift> {
        match self {
            Self::Chat(val) => val.gift.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `request_count`.
    ///
    /// The number of successful requests that exceeded regular limits and were therefore billed
    #[must_use]
    pub fn request_count(&self) -> Option<i64> {
        match self {
            Self::TelegramApi(val) => Some(val.request_count),
            _ => None,
        }
    }

    /// Helper method for field `sponsor_user`.
    ///
    /// Information about the bot that sponsored the affiliate program
    #[must_use]
    pub fn sponsor_user(&self) -> Option<&crate::types::User> {
        match self {
            Self::AffiliateProgram(val) => val.sponsor_user.as_deref(),
            _ => None,
        }
    }

    /// Helper method for field `withdrawal_state`.
    ///
    /// State of the transaction if the transaction is outgoing
    #[must_use]
    pub fn withdrawal_state(&self) -> Option<&crate::types::RevenueWithdrawalState> {
        match self {
            Self::Fragment(val) => val.withdrawal_state.as_ref(),
            _ => None,
        }
    }

    /// Helper method for nested field `added_to_attachment_menu`.
    #[must_use]
    pub fn added_to_attachment_menu(&self) -> Option<bool> {
        match self {
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.added_to_attachment_menu),
            _ => None,
        }
    }

    /// Helper method for nested field `allows_users_to_create_topics`.
    #[must_use]
    pub fn allows_users_to_create_topics(&self) -> Option<bool> {
        match self {
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.allows_users_to_create_topics),
            _ => None,
        }
    }

    /// Helper method for nested field `background`.
    #[must_use]
    pub fn background(&self) -> Option<&crate::types::GiftBackground> {
        match self {
            Self::Chat(val) => val
                .gift
                .as_deref()
                .and_then(|inner| inner.background.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `can_connect_to_business`.
    #[must_use]
    pub fn can_connect_to_business(&self) -> Option<bool> {
        match self {
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.can_connect_to_business),
            _ => None,
        }
    }

    /// Helper method for nested field `can_join_groups`.
    #[must_use]
    pub fn can_join_groups(&self) -> Option<bool> {
        match self {
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.can_join_groups),
            _ => None,
        }
    }

    /// Helper method for nested field `can_read_all_group_messages`.
    #[must_use]
    pub fn can_read_all_group_messages(&self) -> Option<bool> {
        match self {
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.can_read_all_group_messages),
            _ => None,
        }
    }

    /// Helper method for nested field `date`.
    #[must_use]
    pub fn date(&self) -> Option<i64> {
        match self {
            Self::Fragment(val) => val
                .withdrawal_state
                .as_ref()
                .and_then(crate::types::RevenueWithdrawalState::date),
            _ => None,
        }
    }

    /// Helper method for nested field `first_name`.
    #[must_use]
    pub fn first_name(&self) -> Option<&str> {
        match self {
            Self::Chat(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::first_name(inner)
            }
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .map(|inner| inner.first_name.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `has_colors`.
    #[must_use]
    pub fn has_colors(&self) -> Option<bool> {
        match self {
            Self::Chat(val) => val.gift.as_deref().and_then(|inner| inner.has_colors),
            _ => None,
        }
    }

    /// Helper method for nested field `has_main_web_app`.
    #[must_use]
    pub fn has_main_web_app(&self) -> Option<bool> {
        match self {
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.has_main_web_app),
            _ => None,
        }
    }

    /// Helper method for nested field `has_topics_enabled`.
    #[must_use]
    pub fn has_topics_enabled(&self) -> Option<bool> {
        match self {
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.has_topics_enabled),
            _ => None,
        }
    }

    /// Helper method for nested field `is_bot`.
    #[must_use]
    pub fn is_bot(&self) -> Option<bool> {
        match self {
            Self::AffiliateProgram(val) => val.sponsor_user.as_deref().map(|inner| inner.is_bot),
            _ => None,
        }
    }

    /// Helper method for nested field `is_direct_messages`.
    #[must_use]
    pub fn is_direct_messages(&self) -> Option<bool> {
        match self {
            Self::Chat(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::is_direct_messages(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_forum`.
    #[must_use]
    pub fn is_forum(&self) -> Option<bool> {
        match self {
            Self::Chat(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::is_forum(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `is_premium`.
    #[must_use]
    pub fn is_premium(&self) -> Option<bool> {
        match self {
            Self::Chat(val) => val.gift.as_deref().and_then(|inner| inner.is_premium),
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.is_premium),
            _ => None,
        }
    }

    /// Helper method for nested field `language_code`.
    #[must_use]
    pub fn language_code(&self) -> Option<&str> {
        match self {
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.language_code.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `last_name`.
    #[must_use]
    pub fn last_name(&self) -> Option<&str> {
        match self {
            Self::Chat(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::last_name(inner)
            }
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.last_name.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `personal_remaining_count`.
    #[must_use]
    pub fn personal_remaining_count(&self) -> Option<i64> {
        match self {
            Self::Chat(val) => val
                .gift
                .as_deref()
                .and_then(|inner| inner.personal_remaining_count),
            _ => None,
        }
    }

    /// Helper method for nested field `personal_total_count`.
    #[must_use]
    pub fn personal_total_count(&self) -> Option<i64> {
        match self {
            Self::Chat(val) => val
                .gift
                .as_deref()
                .and_then(|inner| inner.personal_total_count),
            _ => None,
        }
    }

    /// Helper method for nested field `publisher_chat`.
    #[must_use]
    pub fn publisher_chat(&self) -> Option<&crate::types::Chat> {
        match self {
            Self::Chat(val) => val
                .gift
                .as_deref()
                .and_then(|inner| inner.publisher_chat.as_deref()),
            _ => None,
        }
    }

    /// Helper method for nested field `remaining_count`.
    #[must_use]
    pub fn remaining_count(&self) -> Option<i64> {
        match self {
            Self::Chat(val) => val.gift.as_deref().and_then(|inner| inner.remaining_count),
            _ => None,
        }
    }

    /// Helper method for nested field `star_count`.
    #[must_use]
    pub fn star_count(&self) -> Option<i64> {
        match self {
            Self::Chat(val) => val.gift.as_deref().map(|inner| inner.star_count),
            _ => None,
        }
    }

    /// Helper method for nested field `sticker`.
    #[must_use]
    pub fn sticker(&self) -> Option<&crate::types::Sticker> {
        match self {
            Self::Chat(val) => val.gift.as_deref().map(|inner| inner.sticker.as_ref()),
            _ => None,
        }
    }

    /// Helper method for nested field `supports_inline_queries`.
    #[must_use]
    pub fn supports_inline_queries(&self) -> Option<bool> {
        match self {
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.supports_inline_queries),
            _ => None,
        }
    }

    /// Helper method for nested field `title`.
    #[must_use]
    pub fn title(&self) -> Option<&str> {
        match self {
            Self::Chat(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::title(inner)
            }
            _ => None,
        }
    }

    /// Helper method for nested field `total_count`.
    #[must_use]
    pub fn total_count(&self) -> Option<i64> {
        match self {
            Self::Chat(val) => val.gift.as_deref().and_then(|inner| inner.total_count),
            _ => None,
        }
    }

    /// Helper method for nested field `unique_gift_variant_count`.
    #[must_use]
    pub fn unique_gift_variant_count(&self) -> Option<i64> {
        match self {
            Self::Chat(val) => val
                .gift
                .as_deref()
                .and_then(|inner| inner.unique_gift_variant_count),
            _ => None,
        }
    }

    /// Helper method for nested field `upgrade_star_count`.
    #[must_use]
    pub fn upgrade_star_count(&self) -> Option<i64> {
        match self {
            Self::Chat(val) => val
                .gift
                .as_deref()
                .and_then(|inner| inner.upgrade_star_count),
            _ => None,
        }
    }

    /// Helper method for nested field `url`.
    #[must_use]
    pub fn url(&self) -> Option<&str> {
        match self {
            Self::Fragment(val) => val
                .withdrawal_state
                .as_ref()
                .and_then(crate::types::RevenueWithdrawalState::url),
            _ => None,
        }
    }

    /// Helper method for nested field `username`.
    #[must_use]
    pub fn username(&self) -> Option<&str> {
        match self {
            Self::Chat(val) => {
                let inner = val.chat.as_ref();
                crate::types::Chat::username(inner)
            }
            Self::AffiliateProgram(val) => val
                .sponsor_user
                .as_deref()
                .and_then(|inner| inner.username.as_deref()),
            _ => None,
        }
    }
}
impl From<crate::types::TransactionPartnerUser> for TransactionPartner {
    fn from(val: crate::types::TransactionPartnerUser) -> Self {
        Self::User(val)
    }
}
impl TryFrom<TransactionPartner> for crate::types::TransactionPartnerUser {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartner) -> Result<Self, Self::Error> {
        if let TransactionPartner::User(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartner),
                stringify!(TransactionPartnerUser),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerChat> for TransactionPartner {
    fn from(val: crate::types::TransactionPartnerChat) -> Self {
        Self::Chat(val)
    }
}
impl TryFrom<TransactionPartner> for crate::types::TransactionPartnerChat {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartner) -> Result<Self, Self::Error> {
        if let TransactionPartner::Chat(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartner),
                stringify!(TransactionPartnerChat),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerAffiliateProgram> for TransactionPartner {
    fn from(val: crate::types::TransactionPartnerAffiliateProgram) -> Self {
        Self::AffiliateProgram(val)
    }
}
impl TryFrom<TransactionPartner> for crate::types::TransactionPartnerAffiliateProgram {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartner) -> Result<Self, Self::Error> {
        if let TransactionPartner::AffiliateProgram(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartner),
                stringify!(TransactionPartnerAffiliateProgram),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerFragment> for TransactionPartner {
    fn from(val: crate::types::TransactionPartnerFragment) -> Self {
        Self::Fragment(val)
    }
}
impl TryFrom<TransactionPartner> for crate::types::TransactionPartnerFragment {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartner) -> Result<Self, Self::Error> {
        if let TransactionPartner::Fragment(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartner),
                stringify!(TransactionPartnerFragment),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerTelegramAds> for TransactionPartner {
    fn from(val: crate::types::TransactionPartnerTelegramAds) -> Self {
        Self::TelegramAds(val)
    }
}
impl TryFrom<TransactionPartner> for crate::types::TransactionPartnerTelegramAds {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartner) -> Result<Self, Self::Error> {
        if let TransactionPartner::TelegramAds(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartner),
                stringify!(TransactionPartnerTelegramAds),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerTelegramApi> for TransactionPartner {
    fn from(val: crate::types::TransactionPartnerTelegramApi) -> Self {
        Self::TelegramApi(val)
    }
}
impl TryFrom<TransactionPartner> for crate::types::TransactionPartnerTelegramApi {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartner) -> Result<Self, Self::Error> {
        if let TransactionPartner::TelegramApi(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartner),
                stringify!(TransactionPartnerTelegramApi),
            ))
        }
    }
}
impl From<crate::types::TransactionPartnerOther> for TransactionPartner {
    fn from(val: crate::types::TransactionPartnerOther) -> Self {
        Self::Other(val)
    }
}
impl TryFrom<TransactionPartner> for crate::types::TransactionPartnerOther {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: TransactionPartner) -> Result<Self, Self::Error> {
        if let TransactionPartner::Other(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(TransactionPartner),
                stringify!(TransactionPartnerOther),
            ))
        }
    }
}
