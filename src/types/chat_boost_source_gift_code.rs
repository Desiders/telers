use super::User;

use serde::Deserialize;

/// The boost was obtained by the creation of Telegram Premium gift codes to boost a chat. Each such code boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsourcegiftcode>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct ChatBoostSourceGiftCode {
    /// User for which the gift code was created
    pub user: User,
}
