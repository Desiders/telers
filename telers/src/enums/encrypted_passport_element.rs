use crate::types::EncryptedPassportElement;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display, EnumString, IntoStaticStr};
/// Describes documents or other Telegram Passport elements shared with the bot by the user.
/// Currently, it can be one of
/// - [`crate::types::EncryptedPassportElementAddress`]
/// - [`crate::types::EncryptedPassportElementBankStatement`]
/// - [`crate::types::EncryptedPassportElementDriverLicense`]
/// - [`crate::types::EncryptedPassportElementEmail`]
/// - [`crate::types::EncryptedPassportElementIdentityCard`]
/// - [`crate::types::EncryptedPassportElementInternalPassport`]
/// - [`crate::types::EncryptedPassportElementPassport`]
/// - [`crate::types::EncryptedPassportElementPassportRegistration`]
/// - [`crate::types::EncryptedPassportElementPersonalDetails`]
/// - [`crate::types::EncryptedPassportElementPhoneNumber`]
/// - [`crate::types::EncryptedPassportElementRentalAgreement`]
/// - [`crate::types::EncryptedPassportElementTemporaryRegistration`]
/// - [`crate::types::EncryptedPassportElementUtilityBill`]
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
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
pub enum EncryptedPassportElementType {
    #[strum(serialize = "personal_details")]
    PersonalDetails,
    #[strum(serialize = "passport")]
    Passport,
    #[strum(serialize = "driver_license")]
    DriverLicense,
    #[strum(serialize = "identity_card")]
    IdentityCard,
    #[strum(serialize = "internal_passport")]
    InternalPassport,
    #[strum(serialize = "address")]
    Address,
    #[strum(serialize = "utility_bill")]
    UtilityBill,
    #[strum(serialize = "bank_statement")]
    BankStatement,
    #[strum(serialize = "rental_agreement")]
    RentalAgreement,
    #[strum(serialize = "passport_registration")]
    PassportRegistration,
    #[strum(serialize = "temporary_registration")]
    TemporaryRegistration,
    #[strum(serialize = "phone_number")]
    PhoneNumber,
    #[strum(serialize = "email")]
    Email,
}
impl EncryptedPassportElementType {
    #[must_use]
    pub const fn all() -> [EncryptedPassportElementType; 13usize] {
        [
            EncryptedPassportElementType::PersonalDetails,
            EncryptedPassportElementType::Passport,
            EncryptedPassportElementType::DriverLicense,
            EncryptedPassportElementType::IdentityCard,
            EncryptedPassportElementType::InternalPassport,
            EncryptedPassportElementType::Address,
            EncryptedPassportElementType::UtilityBill,
            EncryptedPassportElementType::BankStatement,
            EncryptedPassportElementType::RentalAgreement,
            EncryptedPassportElementType::PassportRegistration,
            EncryptedPassportElementType::TemporaryRegistration,
            EncryptedPassportElementType::PhoneNumber,
            EncryptedPassportElementType::Email,
        ]
    }
}
impl From<EncryptedPassportElementType> for Box<str> {
    fn from(val: EncryptedPassportElementType) -> Self {
        Into::<&'static str>::into(val).into()
    }
}
impl From<EncryptedPassportElementType> for String {
    fn from(val: EncryptedPassportElementType) -> Self {
        val.as_ref().to_owned()
    }
}
impl<'a> PartialEq<&'a str> for EncryptedPassportElementType {
    fn eq(&self, other: &&'a str) -> bool {
        self.as_ref() == *other
    }
}
impl<'a> From<&'a EncryptedPassportElement> for EncryptedPassportElementType {
    fn from(val: &'a EncryptedPassportElement) -> Self {
        match val {
            EncryptedPassportElement::PersonalDetails(_) => {
                EncryptedPassportElementType::PersonalDetails
            }
            EncryptedPassportElement::Passport(_) => EncryptedPassportElementType::Passport,
            EncryptedPassportElement::DriverLicense(_) => {
                EncryptedPassportElementType::DriverLicense
            }
            EncryptedPassportElement::IdentityCard(_) => EncryptedPassportElementType::IdentityCard,
            EncryptedPassportElement::InternalPassport(_) => {
                EncryptedPassportElementType::InternalPassport
            }
            EncryptedPassportElement::Address(_) => EncryptedPassportElementType::Address,
            EncryptedPassportElement::UtilityBill(_) => EncryptedPassportElementType::UtilityBill,
            EncryptedPassportElement::BankStatement(_) => {
                EncryptedPassportElementType::BankStatement
            }
            EncryptedPassportElement::RentalAgreement(_) => {
                EncryptedPassportElementType::RentalAgreement
            }
            EncryptedPassportElement::PassportRegistration(_) => {
                EncryptedPassportElementType::PassportRegistration
            }
            EncryptedPassportElement::TemporaryRegistration(_) => {
                EncryptedPassportElementType::TemporaryRegistration
            }
            EncryptedPassportElement::PhoneNumber(_) => EncryptedPassportElementType::PhoneNumber,
            EncryptedPassportElement::Email(_) => EncryptedPassportElementType::Email,
        }
    }
}
impl From<EncryptedPassportElement> for EncryptedPassportElementType {
    fn from(val: EncryptedPassportElement) -> Self {
        EncryptedPassportElementType::from(&val)
    }
}
