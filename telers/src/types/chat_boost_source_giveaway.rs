use serde::{Deserialize, Serialize};
/// The boost was obtained by the creation of a Telegram Premium or a Telegram Star giveaway. This boosts the chat 4 times for the duration of the corresponding Telegram Premium subscription for Telegram Premium giveaways and `prize_star_count` / 500 times for one year for Telegram Star giveaways.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostsourcegiveaway>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatBoostSourceGiveaway {
    /// Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    pub giveaway_message_id: i64,
    /// User that won the prize in the giveaway if any; for Telegram Premium giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<crate::types::User>>,
    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
    /// `true`, if the giveaway was completed, but there was no user to win the prize
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unclaimed: Option<bool>,
}
impl ChatBoostSourceGiveaway {
    /// Creates a new `ChatBoostSourceGiveaway`.
    ///
    /// # Arguments
    /// * `giveaway_message_id` - Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>>(giveaway_message_id: T0) -> Self {
        Self {
            giveaway_message_id: giveaway_message_id.into(),
            user: None,
            prize_star_count: None,
            is_unclaimed: None,
        }
    }

    /// Identifier of a message in the chat with the giveaway; the message could have been deleted already. May be 0 if the message isn't sent yet.
    #[must_use]
    pub fn giveaway_message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.giveaway_message_id = val.into();
        self
    }

    /// User that won the prize in the giveaway if any; for Telegram Premium giveaways only
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.user = Some(Box::new(val.into()));
        self
    }

    /// User that won the prize in the giveaway if any; for Telegram Premium giveaways only
    #[must_use]
    pub fn user_option<T: Into<crate::types::User>>(mut self, val: Option<T>) -> Self {
        self.user = val.map(|val| Box::new(val.into()));
        self
    }

    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[must_use]
    pub fn prize_star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.prize_star_count = Some(val.into());
        self
    }

    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[must_use]
    pub fn prize_star_count_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.prize_star_count = val.map(Into::into);
        self
    }

    /// `true`, if the giveaway was completed, but there was no user to win the prize
    #[must_use]
    pub fn is_unclaimed<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_unclaimed = Some(val.into());
        self
    }

    /// `true`, if the giveaway was completed, but there was no user to win the prize
    #[must_use]
    pub fn is_unclaimed_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_unclaimed = val.map(Into::into);
        self
    }
}
