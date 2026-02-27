use serde::{Deserialize, Serialize};
/// This object represents a/an temporary registration encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `temporary_registration`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedPassportElementTemporaryRegistration {
    /// Array of encrypted files with documents provided by the user; available only for `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    pub files: Box<[crate::types::PassportFile]>,
    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    pub translation: Box<[crate::types::PassportFile]>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    pub hash: Box<str>,
}
impl EncryptedPassportElementTemporaryRegistration {
    /// Creates a new `EncryptedPassportElementTemporaryRegistration`.
    ///
    /// # Arguments
    /// * `files` - Array of encrypted files with documents provided by the user; available only for `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    /// * `translation` - Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    /// * `hash` - Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
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

    /// Array of encrypted files with documents provided by the user; available only for `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn files<T: Into<Box<[crate::types::PassportFile]>>>(self, val: T) -> Self {
        let mut this = self;
        this.files = this
            .files
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// Array of encrypted files with documents provided by the user; available only for `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn file<T: Into<crate::types::PassportFile>>(self, val: T) -> Self {
        let mut this = self;
        this.files = this
            .files
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn translations<T: Into<Box<[crate::types::PassportFile]>>>(self, val: T) -> Self {
        let mut this = self;
        this.translation = this
            .translation
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn translation<T: Into<crate::types::PassportFile>>(self, val: T) -> Self {
        let mut this = self;
        this.translation = this
            .translation
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    #[must_use]
    pub fn hash<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.hash = val.into();
        this
    }
}
