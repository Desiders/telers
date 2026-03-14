use crate::types::TransactionPartner;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// This object describes the source of a transaction, or its recipient for outgoing transactions. Currently, it can be one of
/// - [`crate::types::TransactionPartnerUser`]
/// - [`crate::types::TransactionPartnerChat`]
/// - [`crate::types::TransactionPartnerAffiliateProgram`]
/// - [`crate::types::TransactionPartnerFragment`]
/// - [`crate::types::TransactionPartnerTelegramAds`]
/// - [`crate::types::TransactionPartnerTelegramApi`]
/// - [`crate::types::TransactionPartnerOther`]
/// # Documentation
/// <https://core.telegram.org/bots/api#transactionpartner>
#[derive(
    Debug,
    Display,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    EnumString,
    AsRefStr,
    IntoStaticStr,
    Deserialize,
    Serialize,
)]
pub enum TransactionPartnerType {
    #[strum(serialize = "user")]
    User,
    #[strum(serialize = "chat")]
    Chat,
    #[strum(serialize = "affiliate_program")]
    AffiliateProgram,
    #[strum(serialize = "fragment")]
    Fragment,
    #[strum(serialize = "telegram_ads")]
    TelegramAds,
    #[strum(serialize = "telegram_api")]
    TelegramApi,
    #[strum(serialize = "other")]
    Other,
}
impl TransactionPartnerType {
    #[must_use]
    pub const fn all() -> [TransactionPartnerType; 7usize] {
        [
            TransactionPartnerType::User,
            TransactionPartnerType::Chat,
            TransactionPartnerType::AffiliateProgram,
            TransactionPartnerType::Fragment,
            TransactionPartnerType::TelegramAds,
            TransactionPartnerType::TelegramApi,
            TransactionPartnerType::Other,
        ]
    }
}
impl From<TransactionPartnerType> for Box<str> {
    fn from(val: TransactionPartnerType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<TransactionPartnerType> for String {
    fn from(val: TransactionPartnerType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for TransactionPartnerType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a TransactionPartner> for TransactionPartnerType {
    fn from(val: &'a TransactionPartner) -> Self {
        match val {
            TransactionPartner::User(_) => TransactionPartnerType::User,
            TransactionPartner::Chat(_) => TransactionPartnerType::Chat,
            TransactionPartner::AffiliateProgram(_) => TransactionPartnerType::AffiliateProgram,
            TransactionPartner::Fragment(_) => TransactionPartnerType::Fragment,
            TransactionPartner::TelegramAds(_) => TransactionPartnerType::TelegramAds,
            TransactionPartner::TelegramApi(_) => TransactionPartnerType::TelegramApi,
            TransactionPartner::Other(_) => TransactionPartnerType::Other,
        }
    }
}
impl From<TransactionPartner> for TransactionPartnerType {
    fn from(val: TransactionPartner) -> Self {
        TransactionPartnerType::from(&val)
    }
}
