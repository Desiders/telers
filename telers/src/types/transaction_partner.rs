use super::{
    TransactionPartnerAffiliateProgram, TransactionPartnerFragment, TransactionPartnerOther,
    TransactionPartnerTelegramAds, TransactionPartnerTelegramApi, TransactionPartnerUser,
};
use crate::types::TransactionPartnerChat;

use serde::Deserialize;

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
#[derive(Debug, Clone, PartialEq, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_chat() {
        let jsons = [serde_json::json!({
            "type": "chat",
            "chat": {
                "id": -1,
                "title": "test",
                "type": "channel",
            },
        })];

        for json in jsons {
            let transaction_chat: TransactionPartnerChat =
                serde_json::from_value(json.clone()).unwrap();
            let transaction = serde_json::from_value(json).unwrap();

            match transaction {
                TransactionPartner::Chat(transaction) => assert_eq!(*transaction, transaction_chat),
                _ => panic!("Unexpected transaction type: {transaction:?}"),
            }
        }
    }

    #[test]
    fn deserialize_user() {
        let jsons = [serde_json::json!({
            "type": "user",
            "transaction_type": "invoice_payment",
            "user": {
                "id": -1,
                "is_bot": false,
                "first_name": "test",
            },
        })];

        for json in jsons {
            let transaction_user: TransactionPartnerUser =
                serde_json::from_value(json.clone()).unwrap();
            let transaction = serde_json::from_value(json).unwrap();

            match transaction {
                TransactionPartner::User(transaction) => assert_eq!(*transaction, transaction_user),
                _ => panic!("Unexpected transaction type: {transaction:?}"),
            }
        }
    }
}
