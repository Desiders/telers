use serde::{Deserialize, Serialize};
/// Describes an interval of time during which a business is open.
/// # Documentation
/// <https://core.telegram.org/bots/api#businessopeninghoursinterval>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BusinessOpeningHoursInterval {
    /// The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 * 24 * 60
    pub opening_minute: i64,
    /// The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 * 24 * 60
    pub closing_minute: i64,
}
impl BusinessOpeningHoursInterval {
    /// Creates a new `BusinessOpeningHoursInterval`.
    ///
    /// # Arguments
    /// * `opening_minute` - The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 * 24 * 60
    /// * `closing_minute` - The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 * 24 * 60
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<i64>>(opening_minute: T0, closing_minute: T1) -> Self {
        Self {
            opening_minute: opening_minute.into(),
            closing_minute: closing_minute.into(),
        }
    }

    /// The minute's sequence number in a week, starting on Monday, marking the start of the time interval during which the business is open; 0 - 7 * 24 * 60
    #[must_use]
    pub fn opening_minute<T: Into<i64>>(mut self, val: T) -> Self {
        self.opening_minute = val.into();
        self
    }

    /// The minute's sequence number in a week, starting on Monday, marking the end of the time interval during which the business is open; 0 - 8 * 24 * 60
    #[must_use]
    pub fn closing_minute<T: Into<i64>>(mut self, val: T) -> Self {
        self.closing_minute = val.into();
        self
    }
}
