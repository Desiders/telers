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
    pub fn giveaway_message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.giveaway_message_id = val.into();
        this
    }

    /// User that won the prize in the giveaway if any; for Telegram Premium giveaways only
    #[must_use]
    pub fn user<T: Into<crate::types::User>>(self, val: T) -> Self {
        let mut this = self;
        this.user = Some(Box::new(val.into()));
        this
    }

    /// User that won the prize in the giveaway if any; for Telegram Premium giveaways only
    #[must_use]
    pub fn user_option<T: Into<crate::types::User>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.user = val.map(|val| Box::new(val.into()));
        this
    }

    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[must_use]
    pub fn prize_star_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.prize_star_count = Some(val.into());
        this
    }

    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[must_use]
    pub fn prize_star_count_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.prize_star_count = val.map(Into::into);
        this
    }

    /// `true`, if the giveaway was completed, but there was no user to win the prize
    #[must_use]
    pub fn is_unclaimed<T: Into<bool>>(self, val: T) -> Self {
        let mut this = self;
        this.is_unclaimed = Some(val.into());
        this
    }

    /// `true`, if the giveaway was completed, but there was no user to win the prize
    #[must_use]
    pub fn is_unclaimed_option<T: Into<bool>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.is_unclaimed = val.map(Into::into);
        this
    }
}
