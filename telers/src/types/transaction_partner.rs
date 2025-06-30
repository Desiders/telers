use super::{
    TransactionPartnerAffiliateProgram, TransactionPartnerFragment, TransactionPartnerOther,
    TransactionPartnerTelegramAds, TransactionPartnerTelegramApi, TransactionPartnerUser,
};
use crate::types::TransactionPartnerChat;

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
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransactionPartner {
    User(Box<TransactionPartnerUser>),
    Chat(Box<TransactionPartnerChat>),
    AffiliateProgram(TransactionPartnerAffiliateProgram),
    Fragment(TransactionPartnerFragment),
    TelegramAds(TransactionPartnerTelegramAds),
    TelegramApi(TransactionPartnerTelegramApi),
    Other(TransactionPartnerOther),
}

impl From<TransactionPartnerUser> for TransactionPartner {
    fn from(partner: TransactionPartnerUser) -> Self {
        Self::User(Box::new(partner))
    }
}

impl From<TransactionPartnerChat> for TransactionPartner {
    fn from(partner: TransactionPartnerChat) -> Self {
        Self::Chat(Box::new(partner))
    }
}

impl From<TransactionPartnerFragment> for TransactionPartner {
    fn from(partner: TransactionPartnerFragment) -> Self {
        Self::Fragment(partner)
    }
}

impl From<TransactionPartnerTelegramAds> for TransactionPartner {
    fn from(partner: TransactionPartnerTelegramAds) -> Self {
        Self::TelegramAds(partner)
    }
}

impl From<TransactionPartnerTelegramApi> for TransactionPartner {
    fn from(partner: TransactionPartnerTelegramApi) -> Self {
        Self::TelegramApi(partner)
    }
}

impl From<TransactionPartnerOther> for TransactionPartner {
    fn from(partner: TransactionPartnerOther) -> Self {
        Self::Other(partner)
    }
}
