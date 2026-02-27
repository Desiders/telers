use serde::{Deserialize, Serialize};
/// Describes Telegram Passport data shared with the bot by the user.
/// # Documentation
/// <https://core.telegram.org/bots/api#passportdata>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub data: Box<[crate::types::EncryptedPassportElement]>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: crate::types::EncryptedCredentials,
}
impl PassportData {
    /// Creates a new `PassportData`.
    ///
    /// # Arguments
    /// * `data` - Array with information about documents and other Telegram Passport elements that was shared with the bot
    /// * `credentials` - Encrypted credentials required to decrypt the data
    #[must_use]
    pub fn new<
        T0Item: Into<crate::types::EncryptedPassportElement>,
        T0: IntoIterator<Item = T0Item>,
        T1: Into<crate::types::EncryptedCredentials>,
    >(
        data: T0,
        credentials: T1,
    ) -> Self {
        Self {
            data: data.into_iter().map(Into::into).collect(),
            credentials: credentials.into(),
        }
    }

    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn data<T: Into<Box<[crate::types::EncryptedPassportElement]>>>(self, val: T) -> Self {
        let mut this = self;
        this.data = this.data.into_vec().into_iter().chain(val.into()).collect();
        this
    }

    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn datum<T: Into<crate::types::EncryptedPassportElement>>(self, val: T) -> Self {
        let mut this = self;
        this.data = this
            .data
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Encrypted credentials required to decrypt the data
    #[must_use]
    pub fn credentials<T: Into<crate::types::EncryptedCredentials>>(self, val: T) -> Self {
        let mut this = self;
        this.credentials = val.into();
        this
    }
}
