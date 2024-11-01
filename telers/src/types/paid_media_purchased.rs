use super::{Update, UpdateKind, User};

use crate::{errors::ConvertToTypeError, FromEvent};

use serde::{Deserialize, Serialize};

/// This object contains information about a paid media purchase.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediapurchased>
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, FromEvent)]
#[event(try_from = Update)]
pub struct PaidMediaPurchased {
    /// User who purchased the media
    pub from: User,
    /// Bot-specified paid media payload
    pub paid_media_payload: Box<str>,
}

impl TryFrom<Update> for PaidMediaPurchased {
    type Error = ConvertToTypeError;

    fn try_from(update: Update) -> Result<Self, Self::Error> {
        match update.kind {
            UpdateKind::PurchasedPaidMedia(val) => Ok(val),
            _ => Err(ConvertToTypeError::new("Update", "PaidMediaPurchased")),
        }
    }
}
