use serde::{Deserialize, Serialize};
/// Media is a venue, information about the venue
/// # Notes
/// This object represents a poll media from original field `venue`.
/// # Documentation
/// <https://core.telegram.org/bots/api#pollmedia>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PollMediaVenue {
    /// Media is a venue, information about the venue
    pub venue: Box<crate::types::Venue>,
}
impl PollMediaVenue {
    /// Creates a new `PollMediaVenue`.
    ///
    /// # Arguments
    /// * `venue` - Media is a venue, information about the venue
    #[must_use]
    pub fn new<T0: Into<crate::types::Venue>>(venue: T0) -> Self {
        Self {
            venue: Box::new(venue.into()),
        }
    }

    /// Media is a venue, information about the venue
    #[must_use]
    pub fn venue<T: Into<crate::types::Venue>>(mut self, val: T) -> Self {
        self.venue = Box::new(val.into());
        self
    }
}
