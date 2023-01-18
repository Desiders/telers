use super::{EncryptedCredentials, EncryptedPassportElement};

use serde::{Deserialize};

/// Describes Telegram Passport data shared with the bot by the user.
/// <https://core.telegram.org/bots/api#passportdata>
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
pub struct PassportData {
    /// Array with information about documents and other Telegram Passport elements that was shared with the bot
    pub data: Vec<EncryptedPassportElement>,
    /// Encrypted credentials required to decrypt the data
    pub credentials: EncryptedCredentials,
}
