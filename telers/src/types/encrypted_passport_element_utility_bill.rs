use serde::{Deserialize, Serialize};
/// This object represents a/an utility bill encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `utility_bill`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedPassportElementUtilityBill {
    /// Array of encrypted files with documents provided by the user; available only for `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    pub files: Box<[crate::types::PassportFile]>,
    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    pub translation: Box<[crate::types::PassportFile]>,
    /// Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    pub hash: Box<str>,
}
impl EncryptedPassportElementUtilityBill {
    /// Creates a new `EncryptedPassportElementUtilityBill`.
    ///
    /// # Arguments
    /// * `files` - Array of encrypted files with documents provided by the user; available only for `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    /// * `translation` - Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    /// * `hash` - Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    #[must_use]
    pub fn new<
        T0Item: Into<crate::types::PassportFile>,
        T0: IntoIterator<Item = T0Item>,
        T1Item: Into<crate::types::PassportFile>,
        T1: IntoIterator<Item = T1Item>,
        T2: Into<Box<str>>,
    >(
        files: T0,
        translation: T1,
        hash: T2,
    ) -> Self {
        Self {
            files: files.into_iter().map(Into::into).collect(),
            translation: translation.into_iter().map(Into::into).collect(),
            hash: hash.into(),
        }
    }

    /// Array of encrypted files with documents provided by the user; available only for `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn files<T: Into<Box<[crate::types::PassportFile]>>>(mut self, val: T) -> Self {
        self.files = self
            .files
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Array of encrypted files with documents provided by the user; available only for `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn file<T: Into<crate::types::PassportFile>>(mut self, val: T) -> Self {
        self.files = self
            .files
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn translations<T: Into<Box<[crate::types::PassportFile]>>>(mut self, val: T) -> Self {
        self.translation = self
            .translation
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn translation<T: Into<crate::types::PassportFile>>(mut self, val: T) -> Self {
        self.translation = self
            .translation
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    #[must_use]
    pub fn hash<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.hash = val.into();
        self
    }
}
