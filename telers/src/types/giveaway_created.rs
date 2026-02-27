use serde::{Deserialize, Serialize};
/// This object represents a service message about the creation of a scheduled giveaway.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaycreated>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GiveawayCreated {
    /// The number of Telegram Stars to be split between giveaway winners; for Telegram Star giveaways only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i64>,
}
impl GiveawayCreated {
    /// Creates a new `GiveawayCreated`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            prize_star_count: None,
        }
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
}
impl Default for GiveawayCreated {
    fn default() -> Self {
        Self::new()
    }
}
