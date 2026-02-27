use serde::{Deserialize, Serialize};
/// This object represents a/an driver license encrypted passport element.
/// # Notes
/// This object represents an encrypted passport element from original field `driver_license`.
/// # Documentation
/// <https://core.telegram.org/bots/api#encryptedpassportelement>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptedPassportElementDriverLicense {
    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    pub data: Box<str>,
    /// Encrypted file with the front side of the document, provided by the user; available only for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    pub front_side: crate::types::PassportFile,
    /// Encrypted file with the reverse side of the document, provided by the user; available only for `driver_license` and `identity_card`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    pub reverse_side: crate::types::PassportFile,
    /// Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    pub selfie: crate::types::PassportFile,
    /// Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    pub translation: Box<[crate::types::PassportFile]>,
    /// Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    pub hash: Box<str>,
}
impl EncryptedPassportElementDriverLicense {
    /// Creates a new `EncryptedPassportElementDriverLicense`.
    ///
    /// # Arguments
    /// * `data` - Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    /// * `front_side` - Encrypted file with the front side of the document, provided by the user; available only for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    /// * `reverse_side` - Encrypted file with the reverse side of the document, provided by the user; available only for `driver_license` and `identity_card`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    /// * `selfie` - Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    /// * `translation` - Array of encrypted files with translated versions of documents provided by the user; available if requested for `passport`, `driver_license`, `identity_card`, `internal_passport`, `utility_bill`, `bank_statement`, `rental_agreement`, `passport_registration` and `temporary_registration` types. Files can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    /// * `hash` - Base64-encoded element hash for using in [`PassportElementErrorUnspecified`]
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::PassportFile>,
        T2: Into<crate::types::PassportFile>,
        T3: Into<crate::types::PassportFile>,
        T4Item: Into<crate::types::PassportFile>,
        T4: IntoIterator<Item = T4Item>,
        T5: Into<Box<str>>,
    >(
        data: T0,
        front_side: T1,
        reverse_side: T2,
        selfie: T3,
        translation: T4,
        hash: T5,
    ) -> Self {
        Self {
            data: data.into(),
            front_side: front_side.into(),
            reverse_side: reverse_side.into(),
            selfie: selfie.into(),
            translation: translation.into_iter().map(Into::into).collect(),
            hash: hash.into(),
        }
    }

    /// Base64-encoded encrypted Telegram Passport element data provided by the user; available only for `personal_details`, `passport`, `driver_license`, `identity_card`, `internal_passport` and `address` types. Can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn data<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.data = val.into();
        this
    }

    /// Encrypted file with the front side of the document, provided by the user; available only for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn front_side<T: Into<crate::types::PassportFile>>(self, val: T) -> Self {
        let mut this = self;
        this.front_side = val.into();
        this
    }

    /// Encrypted file with the reverse side of the document, provided by the user; available only for `driver_license` and `identity_card`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn reverse_side<T: Into<crate::types::PassportFile>>(self, val: T) -> Self {
        let mut this = self;
        this.reverse_side = val.into();
        this
    }

    /// Encrypted file with the selfie of the user holding a document, provided by the user; available if requested for `passport`, `driver_license`, `identity_card` and `internal_passport`. The file can be decrypted and verified using the accompanying [`EncryptedCredentials`].
    #[must_use]
    pub fn selfie<T: Into<crate::types::PassportFile>>(self, val: T) -> Self {
        let mut this = self;
        this.selfie = val.into();
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
