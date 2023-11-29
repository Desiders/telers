use super::PassportFile;

use serde::Deserialize;

/// Describes documents or other Telegram Passport elements shared with the bot by the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EncryptedPassportElement {
    PersonalDetails(PersonalDetails),
    Passport(Passport),
    DriverLicense(DriverLicense),
    IdentityCard(IdentityCard),
    InternalPassport(InternalPassport),
    Address(Address),
    UtilityBill(UtilityBill),
    BankStatement(BankStatement),
    RentalAgreement(RentalAgreement),
    PassportRegistration(PassportRegistration),
    TemporaryRegistration(TemporaryRegistration),
    PhoneNumber(PhoneNumber),
    Email(Email),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct PersonalDetails {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user. Can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub data: Box<str>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Passport {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user. Can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub data: Box<str>,
    /// Encrypted file with the front side of the document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub front_side: PassportFile,
    /// Encrypted file with the selfie of the user holding a document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub selfie: PassportFile,
    /// Array of encrypted files with translated versions of documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Box<[PassportFile]>>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct DriverLicense {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user. Can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub data: Box<str>,
    /// Encrypted file with the front side of the document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub front_side: PassportFile,
    /// Encrypted file with the reverse side of the document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub reverse_side: PassportFile,
    /// Encrypted file with the selfie of the user holding a document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub selfie: PassportFile,
    /// Array of encrypted files with translated versions of documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Box<[PassportFile]>>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct IdentityCard {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user. Can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub data: Box<str>,
    /// Encrypted file with the front side of the document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub front_side: PassportFile,
    /// Encrypted file with the reverse side of the document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub reverse_side: PassportFile,
    /// Encrypted file with the selfie of the user holding a document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub selfie: PassportFile,
    /// Array of encrypted files with translated versions of documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Box<[PassportFile]>>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct InternalPassport {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user. Can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub data: Box<str>,
    /// Encrypted file with the front side of the document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub front_side: PassportFile,
    /// Encrypted file with the selfie of the user holding a document, provided by the user. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub selfie: PassportFile,
    /// Array of encrypted files with translated versions of documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Box<[PassportFile]>>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Address {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user. Can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub data: Box<str>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct UtilityBill {
    /// Array of encrypted files with documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub files: Box<[PassportFile]>,
    /// Array of encrypted files with translated versions of documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Box<[PassportFile]>>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct BankStatement {
    /// Array of encrypted files with documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub files: Box<[PassportFile]>,
    /// Array of encrypted files with translated versions of documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Box<[PassportFile]>>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct RentalAgreement {
    /// Array of encrypted files with documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub files: Box<[PassportFile]>,
    /// Array of encrypted files with translated versions of documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Box<[PassportFile]>>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct PassportRegistration {
    /// Array of encrypted files with documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub files: Box<[PassportFile]>,
    /// Array of encrypted files with translated versions of documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Box<[PassportFile]>>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct TemporaryRegistration {
    /// Array of encrypted files with documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub files: Box<[PassportFile]>,
    /// Array of encrypted files with translated versions of documents provided by the user. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`](crate::types::EncryptedCredentials).
    pub translation: Option<Box<[PassportFile]>>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct PhoneNumber {
    /// User's verified phone number
    #[serde(rename = "phone_number")]
    pub number: Box<str>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct Email {
    /// User's verified email address
    pub email: Box<str>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`](crate::types::PassportElementErrorUnspecified)
    pub hash: Box<str>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_encrypted_passport_element_personal_details() {
        let jsons = [serde_json::json!({
            "type": "personal_details",
            "data": "test",
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_personal_details: PersonalDetails =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::PersonalDetails(
                    encrypted_passport_element_personal_details2,
                ) => {
                    assert_eq!(
                        encrypted_passport_element_personal_details,
                        encrypted_passport_element_personal_details2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_passport() {
        let jsons = [serde_json::json!({
            "type": "passport",
            "data": "test",
            "front_side": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "selfie": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "translation": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_passport: Passport =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::Passport(encrypted_passport_element_passport2) => {
                    assert_eq!(
                        encrypted_passport_element_passport,
                        encrypted_passport_element_passport2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_driver_license() {
        let jsons = [serde_json::json!({
            "type": "driver_license",
            "data": "test",
            "front_side": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "reverse_side": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "selfie": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "translation": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_driver_license: DriverLicense =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::DriverLicense(
                    encrypted_passport_element_driver_license2,
                ) => {
                    assert_eq!(
                        encrypted_passport_element_driver_license,
                        encrypted_passport_element_driver_license2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_identity_card() {
        let jsons = [serde_json::json!({
            "type": "identity_card",
            "data": "test",
            "front_side": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "reverse_side": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "selfie": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "translation": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_identity_card: IdentityCard =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::IdentityCard(
                    encrypted_passport_element_identity_card2,
                ) => {
                    assert_eq!(
                        encrypted_passport_element_identity_card,
                        encrypted_passport_element_identity_card2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_internal_passport() {
        let jsons = [serde_json::json!({
            "type": "internal_passport",
            "data": "test",
            "front_side": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "selfie": {
                "file_id": "file_id",
                "file_unique_id": "file_unique_id",
                "file_size": 1,
                "file_date": 1
            },
            "translation": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_internal_passport: InternalPassport =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::InternalPassport(
                    encrypted_passport_element_internal_passport2,
                ) => {
                    assert_eq!(
                        encrypted_passport_element_internal_passport,
                        encrypted_passport_element_internal_passport2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_address() {
        let jsons = [serde_json::json!({
            "type": "address",
            "data": "test",
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_address: Address =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::Address(encrypted_passport_element_address2) => {
                    assert_eq!(
                        encrypted_passport_element_address,
                        encrypted_passport_element_address2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_utility_bill() {
        let jsons = [serde_json::json!({
            "type": "utility_bill",
            "files": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "translation": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_utility_bill: UtilityBill =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::UtilityBill(encrypted_passport_element_utility_bill2) => {
                    assert_eq!(
                        encrypted_passport_element_utility_bill,
                        encrypted_passport_element_utility_bill2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_bank_statement() {
        let jsons = [serde_json::json!({
            "type": "bank_statement",
            "files": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "translation": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_bank_statement: BankStatement =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::BankStatement(
                    encrypted_passport_element_bank_statement2,
                ) => {
                    assert_eq!(
                        encrypted_passport_element_bank_statement,
                        encrypted_passport_element_bank_statement2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_rental_agreement() {
        let jsons = [serde_json::json!({
            "type": "rental_agreement",
            "files": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "translation": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_rental_agreement: RentalAgreement =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::RentalAgreement(
                    encrypted_passport_element_rental_agreement2,
                ) => {
                    assert_eq!(
                        encrypted_passport_element_rental_agreement,
                        encrypted_passport_element_rental_agreement2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_passport_registration() {
        let jsons = [serde_json::json!({
            "type": "passport_registration",
            "files": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "translation": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_passport_registration: PassportRegistration =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::PassportRegistration(
                    encrypted_passport_element_passport_registration2,
                ) => {
                    assert_eq!(
                        encrypted_passport_element_passport_registration,
                        encrypted_passport_element_passport_registration2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_temporary_registration() {
        let jsons = [serde_json::json!({
            "type": "temporary_registration",
            "files": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "translation": [
                {
                    "file_id": "file_id",
                    "file_unique_id": "file_unique_id",
                    "file_size": 1,
                    "file_date": 1
                }
            ],
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_temporary_registration: TemporaryRegistration =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::TemporaryRegistration(
                    encrypted_passport_element_temporary_registration2,
                ) => {
                    assert_eq!(
                        encrypted_passport_element_temporary_registration,
                        encrypted_passport_element_temporary_registration2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_phone_number() {
        let jsons = [serde_json::json!({
            "type": "phone_number",
            "phone_number": "test",
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_phone_number: PhoneNumber =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::PhoneNumber(encrypted_passport_element_phone_number2) => {
                    assert_eq!(
                        encrypted_passport_element_phone_number,
                        encrypted_passport_element_phone_number2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }

    #[test]
    fn parse_encrypted_passport_element_email() {
        let jsons = [serde_json::json!({
            "type": "email",
            "email": "test",
            "hash": "test"
        })];

        for json in jsons {
            let encrypted_passport_element_email: Email =
                serde_json::from_value(json.clone()).unwrap();
            let encrypted_passport_element: EncryptedPassportElement =
                serde_json::from_value(json).unwrap();

            match encrypted_passport_element {
                EncryptedPassportElement::Email(encrypted_passport_element_email2) => {
                    assert_eq!(
                        encrypted_passport_element_email,
                        encrypted_passport_element_email2
                    );
                }
                _ => panic!("Unexpected result: {encrypted_passport_element:?}"),
            }
        }
    }
}
