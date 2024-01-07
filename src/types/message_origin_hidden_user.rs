use serde::Deserialize;

/// The message was originally sent by an unknown user.
/// # Documentation
/// <https://core.telegram.org/bots/api#messageoriginhiddenuser>
#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct MessageOriginHiddenUser {
    /// Date the message was sent originally in Unix time
    pub date: i64,
    /// Name of the user that sent the message originally
    pub sender_user_name: Box<str>,
}
