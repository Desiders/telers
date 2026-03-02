use serde::{Deserialize, Serialize};
/// Describes documents or other Telegram Passport elements shared with the bot by the user.
/// Currently, it can be one of
/// - [`EncryptedPassportElementAddress`]
/// - [`EncryptedPassportElementBankStatement`]
/// - [`EncryptedPassportElementDriverLicense`]
/// - [`EncryptedPassportElementEmail`]
/// - [`EncryptedPassportElementIdentityCard`]
/// - [`EncryptedPassportElementInternalPassport`]
/// - [`EncryptedPassportElementPassport`]
/// - [`EncryptedPassportElementPassportRegistration`]
/// - [`EncryptedPassportElementPersonalDetails`]
/// - [`EncryptedPassportElementPhoneNumber`]
/// - [`EncryptedPassportElementRentalAgreement`]
/// - [`EncryptedPassportElementTemporaryRegistration`]
/// - [`EncryptedPassportElementUtilityBill`]
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EncryptedPassportElement {
    PersonalDetails(crate::types::EncryptedPassportElementPersonalDetails),
    Passport(crate::types::EncryptedPassportElementPassport),
    DriverLicense(crate::types::EncryptedPassportElementDriverLicense),
    IdentityCard(crate::types::EncryptedPassportElementIdentityCard),
    InternalPassport(crate::types::EncryptedPassportElementInternalPassport),
    Address(crate::types::EncryptedPassportElementAddress),
    UtilityBill(crate::types::EncryptedPassportElementUtilityBill),
    BankStatement(crate::types::EncryptedPassportElementBankStatement),
    RentalAgreement(crate::types::EncryptedPassportElementRentalAgreement),
    PassportRegistration(crate::types::EncryptedPassportElementPassportRegistration),
    TemporaryRegistration(crate::types::EncryptedPassportElementTemporaryRegistration),
    PhoneNumber(crate::types::EncryptedPassportElementPhoneNumber),
    Email(crate::types::EncryptedPassportElementEmail),
}
impl EncryptedPassportElement {
    /// Helper method for field `data`.
    ///
    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn data(&self) -> Option<&str> {
        match self {
            Self::PersonalDetails(val) => Some(val.data.as_ref()),
            Self::Passport(val) => Some(val.data.as_ref()),
            Self::DriverLicense(val) => Some(val.data.as_ref()),
            Self::IdentityCard(val) => Some(val.data.as_ref()),
            Self::InternalPassport(val) => Some(val.data.as_ref()),
            Self::Address(val) => Some(val.data.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `email`.
    ///
    /// User's verified email address; available only for `email` type
    #[must_use]
    pub fn email(&self) -> Option<&str> {
        match self {
            Self::Email(val) => Some(val.email.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `files`.
    ///
    /// Array of encrypted files with documents provided by the user; available only for `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn files(&self) -> Option<&[crate::types::PassportFile]> {
        match self {
            Self::UtilityBill(val) => Some(val.files.as_ref()),
            Self::BankStatement(val) => Some(val.files.as_ref()),
            Self::RentalAgreement(val) => Some(val.files.as_ref()),
            Self::PassportRegistration(val) => Some(val.files.as_ref()),
            Self::TemporaryRegistration(val) => Some(val.files.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `front_side`.
    ///
    /// Encrypted file with the front side of the document, provided by the user; available only for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn front_side(&self) -> Option<&crate::types::PassportFile> {
        match self {
            Self::Passport(val) => Some(&val.front_side),
            Self::DriverLicense(val) => Some(&val.front_side),
            Self::IdentityCard(val) => Some(&val.front_side),
            Self::InternalPassport(val) => Some(&val.front_side),
            _ => None,
        }
    }

    /// Helper method for field `hash`.
    ///
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    #[must_use]
    pub fn hash(&self) -> &str {
        match self {
            Self::PersonalDetails(val) => val.hash.as_ref(),
            Self::Passport(val) => val.hash.as_ref(),
            Self::DriverLicense(val) => val.hash.as_ref(),
            Self::IdentityCard(val) => val.hash.as_ref(),
            Self::InternalPassport(val) => val.hash.as_ref(),
            Self::Address(val) => val.hash.as_ref(),
            Self::UtilityBill(val) => val.hash.as_ref(),
            Self::BankStatement(val) => val.hash.as_ref(),
            Self::RentalAgreement(val) => val.hash.as_ref(),
            Self::PassportRegistration(val) => val.hash.as_ref(),
            Self::TemporaryRegistration(val) => val.hash.as_ref(),
            Self::PhoneNumber(val) => val.hash.as_ref(),
            Self::Email(val) => val.hash.as_ref(),
        }
    }

    /// Helper method for field `phone_number`.
    ///
    /// User's verified phone number; available only for `phone_number` type
    #[must_use]
    pub fn phone_number(&self) -> Option<&str> {
        match self {
            Self::PhoneNumber(val) => Some(val.phone_number.as_ref()),
            _ => None,
        }
    }

    /// Helper method for field `reverse_side`.
    ///
    /// Encrypted file with the reverse side of the document, provided by the user; available only for `driver_license` and `identity_card`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn reverse_side(&self) -> Option<&crate::types::PassportFile> {
        match self {
            Self::DriverLicense(val) => Some(&val.reverse_side),
            Self::IdentityCard(val) => Some(&val.reverse_side),
            _ => None,
        }
    }

    /// Helper method for field `selfie`.
    ///
    /// Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn selfie(&self) -> Option<&crate::types::PassportFile> {
        match self {
            Self::Passport(val) => Some(&val.selfie),
            Self::DriverLicense(val) => Some(&val.selfie),
            Self::IdentityCard(val) => Some(&val.selfie),
            Self::InternalPassport(val) => Some(&val.selfie),
            _ => None,
        }
    }

    /// Helper method for field `translation`.
    ///
    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn translation(&self) -> Option<&[crate::types::PassportFile]> {
        match self {
            Self::Passport(val) => Some(val.translation.as_ref()),
            Self::DriverLicense(val) => Some(val.translation.as_ref()),
            Self::IdentityCard(val) => Some(val.translation.as_ref()),
            Self::InternalPassport(val) => Some(val.translation.as_ref()),
            Self::UtilityBill(val) => Some(val.translation.as_ref()),
            Self::BankStatement(val) => Some(val.translation.as_ref()),
            Self::RentalAgreement(val) => Some(val.translation.as_ref()),
            Self::PassportRegistration(val) => Some(val.translation.as_ref()),
            Self::TemporaryRegistration(val) => Some(val.translation.as_ref()),
            _ => None,
        }
    }
}
impl From<crate::types::EncryptedPassportElementPersonalDetails> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementPersonalDetails) -> Self {
        Self::PersonalDetails(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementPersonalDetails {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::PersonalDetails(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementPersonalDetails),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementPassport> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementPassport) -> Self {
        Self::Passport(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementPassport {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::Passport(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementPassport),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementDriverLicense> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementDriverLicense) -> Self {
        Self::DriverLicense(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementDriverLicense {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::DriverLicense(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementDriverLicense),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementIdentityCard> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementIdentityCard) -> Self {
        Self::IdentityCard(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementIdentityCard {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::IdentityCard(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementIdentityCard),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementInternalPassport> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementInternalPassport) -> Self {
        Self::InternalPassport(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementInternalPassport {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::InternalPassport(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementInternalPassport),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementAddress> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementAddress) -> Self {
        Self::Address(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementAddress {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::Address(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementAddress),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementUtilityBill> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementUtilityBill) -> Self {
        Self::UtilityBill(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementUtilityBill {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::UtilityBill(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementUtilityBill),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementBankStatement> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementBankStatement) -> Self {
        Self::BankStatement(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementBankStatement {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::BankStatement(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementBankStatement),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementRentalAgreement> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementRentalAgreement) -> Self {
        Self::RentalAgreement(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementRentalAgreement {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::RentalAgreement(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementRentalAgreement),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementPassportRegistration> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementPassportRegistration) -> Self {
        Self::PassportRegistration(val)
    }
}
impl TryFrom<EncryptedPassportElement>
    for crate::types::EncryptedPassportElementPassportRegistration
{
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::PassportRegistration(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementPassportRegistration),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementTemporaryRegistration>
    for EncryptedPassportElement
{
    fn from(val: crate::types::EncryptedPassportElementTemporaryRegistration) -> Self {
        Self::TemporaryRegistration(val)
    }
}
impl TryFrom<EncryptedPassportElement>
    for crate::types::EncryptedPassportElementTemporaryRegistration
{
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::TemporaryRegistration(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementTemporaryRegistration),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementPhoneNumber> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementPhoneNumber) -> Self {
        Self::PhoneNumber(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementPhoneNumber {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::PhoneNumber(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementPhoneNumber),
            ))
        }
    }
}
impl From<crate::types::EncryptedPassportElementEmail> for EncryptedPassportElement {
    fn from(val: crate::types::EncryptedPassportElementEmail) -> Self {
        Self::Email(val)
    }
}
impl TryFrom<EncryptedPassportElement> for crate::types::EncryptedPassportElementEmail {
    type Error = crate::errors::ConvertToTypeError;

    fn try_from(val: EncryptedPassportElement) -> Result<Self, Self::Error> {
        if let EncryptedPassportElement::Email(inner) = val {
            Ok(inner)
        } else {
            Err(Self::Error::new(
                stringify!(EncryptedPassportElement),
                stringify!(EncryptedPassportElementEmail),
            ))
        }
    }
}
