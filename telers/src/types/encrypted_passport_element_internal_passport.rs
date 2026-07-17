use serde::{Deserialize, Serialize};
/// This object represents a/an internal passport encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `internal_passport`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedPassportElementInternalPassport {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    pub data: Box<str>,
    /// Encrypted file with the front side of the document, provided by the user; available only for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    pub front_side: crate::types::PassportFile,
    /// Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<crate::types::PassportFile>,
    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Box<[crate::types::PassportFile]>>,
    /// Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    pub hash: Box<str>,
}
impl EncryptedPassportElementInternalPassport {
    /// Creates a new `EncryptedPassportElementInternalPassport`.
    ///
    /// # Arguments
    /// * `data` - Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    /// * `front_side` - Encrypted file with the front side of the document, provided by the user; available only for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    /// * `hash` - Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<crate::types::PassportFile>, T2: Into<Box<str>>>(
        data: T0,
        front_side: T1,
        hash: T2,
    ) -> Self {
        Self {
            data: data.into(),
            front_side: front_side.into(),
            selfie: None,
            translation: None,
            hash: hash.into(),
        }
    }

    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    #[must_use]
    pub fn data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.data = val.into();
        self
    }

    /// Encrypted file with the front side of the document, provided by the user; available only for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    #[must_use]
    pub fn front_side<T: Into<crate::types::PassportFile>>(mut self, val: T) -> Self {
        self.front_side = val.into();
        self
    }

    /// Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    #[must_use]
    pub fn selfie<T: Into<crate::types::PassportFile>>(mut self, val: T) -> Self {
        self.selfie = Some(val.into());
        self
    }

    /// Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    #[must_use]
    pub fn selfie_option<T: Into<crate::types::PassportFile>>(mut self, val: Option<T>) -> Self {
        self.selfie = val.map(Into::into);
        self
    }

    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn translations<T: Into<Box<[crate::types::PassportFile]>>>(mut self, val: T) -> Self {
        self.translation = Some(
            self.translation
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn translation<T: Into<crate::types::PassportFile>>(mut self, val: T) -> Self {
        self.translation = Some(
            self.translation
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`crate::types::EncryptedCredentials`].
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn translation_option<T: Into<Box<[crate::types::PassportFile]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.translation = val.map(Into::into);
        self
    }

    /// Base64-encoded element hash for using in [`crate::types::PassportElementErrorUnspecified`]
    #[must_use]
    pub fn hash<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.hash = val.into();
        self
    }
}
