use serde::{Deserialize, Serialize};
/// This object describes the rating of a user based on their Telegram Star spendings.
/// # Documentation
/// <https://core.telegram.org/bots/api#userrating>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserRating {
    /// Current level of the user, indicating their reliability when purchasing digital goods and services. A higher level suggests a more trustworthy customer; a negative level is likely reason for concern.
    pub level: i64,
    /// Numerical value of the user's rating; the higher the rating, the better
    pub rating: i64,
    /// The rating value required to get the current level
    pub current_level_rating: i64,
    /// The rating value required to get to the next level; omitted if the maximum level was reached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_level_rating: Option<i64>,
}
impl UserRating {
    /// Creates a new `UserRating`.
    ///
    /// # Arguments
    /// * `level` - Current level of the user, indicating their reliability when purchasing digital goods and services. A higher level suggests a more trustworthy customer; a negative level is likely reason for concern.
    /// * `rating` - Numerical value of the user's rating; the higher the rating, the better
    /// * `current_level_rating` - The rating value required to get the current level
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>, T2: Into<i64>>(
        level: T0,
        rating: T1,
        current_level_rating: T2,
    ) -> Self {
        Self {
            level: level.into(),
            rating: rating.into(),
            current_level_rating: current_level_rating.into(),
            next_level_rating: None,
        }
    }

    /// Current level of the user, indicating their reliability when purchasing digital goods and services. A higher level suggests a more trustworthy customer; a negative level is likely reason for concern.
    #[must_use]
    pub fn level<T: Into<i64>>(mut self, val: T) -> Self {
        self.level = val.into();
        self
    }

    /// Numerical value of the user's rating; the higher the rating, the better
    #[must_use]
    pub fn rating<T: Into<i64>>(mut self, val: T) -> Self {
        self.rating = val.into();
        self
    }

    /// The rating value required to get the current level
    #[must_use]
    pub fn current_level_rating<T: Into<i64>>(mut self, val: T) -> Self {
        self.current_level_rating = val.into();
        self
    }

    /// The rating value required to get to the next level; omitted if the maximum level was reached
    #[must_use]
    pub fn next_level_rating<T: Into<i64>>(mut self, val: T) -> Self {
        self.next_level_rating = Some(val.into());
        self
    }

    /// The rating value required to get to the next level; omitted if the maximum level was reached
    #[must_use]
    pub fn next_level_rating_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.next_level_rating = val.map(Into::into);
        self
    }
}
