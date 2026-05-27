use serde::{Deserialize, Serialize};
/// This object represents a service message about the completion of a giveaway without public winners.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaycompleted>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway
    pub winner_count: i64,
    /// Number of undistributed prizes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i64>,
    /// Message with the giveaway that was completed, if it wasn't deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_message: Option<Box<crate::types::Message>>,
    /// `true`, if the giveaway is a Telegram Star giveaway. Otherwise, currently, the giveaway is a Telegram Premium giveaway.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_star_giveaway: Option<bool>,
}
impl GiveawayCompleted {
    /// Creates a new `GiveawayCompleted`.
    ///
    /// # Arguments
    /// * `winner_count` - Number of winners in the giveaway
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(winner_count: T0) -> Self {
        Self {
            winner_count: winner_count.into(),
            unclaimed_prize_count: None,
            giveaway_message: None,
            is_star_giveaway: None,
        }
    }

    /// Number of winners in the giveaway
    #[must_use]
    pub fn winner_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.winner_count = val.into();
        self
    }

    /// Number of undistributed prizes
    #[must_use]
    pub fn unclaimed_prize_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.unclaimed_prize_count = Some(val.into());
        self
    }

    /// Number of undistributed prizes
    #[must_use]
    pub fn unclaimed_prize_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.unclaimed_prize_count = val.map(Into::into);
        self
    }

    /// Message with the giveaway that was completed, if it wasn't deleted
    #[must_use]
    pub fn giveaway_message<T: Into<crate::types::Message>>(mut self, val: T) -> Self {
        self.giveaway_message = Some(Box::new(val.into()));
        self
    }

    /// Message with the giveaway that was completed, if it wasn't deleted
    #[must_use]
    pub fn giveaway_message_option<T: Into<crate::types::Message>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.giveaway_message = val.map(|val| Box::new(val.into()));
        self
    }

    /// `true`, if the giveaway is a Telegram Star giveaway. Otherwise, currently, the giveaway is a Telegram Premium giveaway.
    #[must_use]
    pub fn is_star_giveaway<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_star_giveaway = Some(val.into());
        self
    }

    /// `true`, if the giveaway is a Telegram Star giveaway. Otherwise, currently, the giveaway is a Telegram Premium giveaway.
    #[must_use]
    pub fn is_star_giveaway_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_star_giveaway = val.map(Into::into);
        self
    }
}
