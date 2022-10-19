use super::User;

use serde::{Deserialize, Serialize};

/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
/// <https://core.telegram.org/bots/api#proximityalerttriggered>_
#[derive(Default, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: i64,
}
