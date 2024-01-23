use super::ChatBoost;

use serde::Deserialize;

/// This object represents a list of boosts added to a chat by a user.
/// # Documentation
/// <https://core.telegram.org/bots/api#userchatboosts>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct UserChatBoosts {
    /// The list of boosts added to the chat by the user
    pub boosts: Box<[ChatBoost]>,
}
