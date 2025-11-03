use super::{Update, UpdateKind, User};

use crate::{errors::ConvertToTypeError, types::BusinessBotRights, FromEvent};

use serde::{Deserialize, Serialize};

/// Describes the connection of the bot with a business account.
/// # Documentation
/// <https://core.telegram.org/bots/api#businessconnection>
#[derive(Debug, Default, Clone, Hash, PartialEq, Eq, Deserialize, Serialize, FromEvent)]
#[event(try_from = Update)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection
    pub id: Box<str>,
    /// Business account user that created the business connection
    pub user: User,
    /// Identifier of a private chat with the user who created the business connection. This number may have more than 32 significant bits and some programming languages may have difficulty/silent defects in interpreting it. But it has at most 52 significant bits, so a 64-bit integer or double-precision float type are safe for storing this identifier.
    pub user_chat_id: i64,
    /// Date the connection was established in Unix time
    pub date: i64,
    /// Rights of the business bot
    pub rights: BusinessBotRights,
    /// `true`, if the connection is active
    pub is_enabled: bool,
}

impl TryFrom<Update> for BusinessConnection {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::BusinessConnection(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "BusinessConnection")),
        }
    }
}
