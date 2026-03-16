use serde::{Deserialize, Serialize};
/// This object represents the content of a service message, sent whenever a user in the chat triggers a proximity alert set by another user.
/// # Documentation
/// <https://core.telegram.org/bots/api#proximityalerttriggered>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: Box<crate::types::User>,
    /// User that set the alert
    pub watcher: Box<crate::types::User>,
    /// The distance between the users
    pub distance: i64,
}
impl ProximityAlertTriggered {
    /// Creates a new `ProximityAlertTriggered`.
    ///
    /// # Arguments
    /// * `traveler` - User that triggered the alert
    /// * `watcher` - User that set the alert
    /// * `distance` - The distance between the users
    #[must_use]
    pub fn new<T0: Into<crate::types::User>, T1: Into<crate::types::User>, T2: Into<i64>>(
        traveler: T0,
        watcher: T1,
        distance: T2,
    ) -> Self {
        Self {
            traveler: Box::new(traveler.into()),
            watcher: Box::new(watcher.into()),
            distance: distance.into(),
        }
    }

    /// User that triggered the alert
    #[must_use]
    pub fn traveler<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.traveler = Box::new(val.into());
        this
    }

    /// User that set the alert
    #[must_use]
    pub fn watcher<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.watcher = Box::new(val.into());
        this
    }

    /// The distance between the users
    #[must_use]
    pub fn distance<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.distance = val.into();
        this
    }
}
