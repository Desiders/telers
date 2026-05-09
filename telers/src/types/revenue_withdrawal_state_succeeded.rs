use serde::{Deserialize, Serialize};
/// The withdrawal succeeded.
/// # Documentation
/// <https://core.telegram.org/bots/api#revenuewithdrawalstatesucceeded>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RevenueWithdrawalStateSucceeded {
    /// Date the withdrawal was completed in Unix time
    pub date: i64,
    /// An HTTPS URL that can be used to see transaction details
    pub url: Box<str>,
}
impl RevenueWithdrawalStateSucceeded {
    /// Creates a new `RevenueWithdrawalStateSucceeded`.
    ///
    /// # Arguments
    /// * `date` - Date the withdrawal was completed in Unix time
    /// * `url` - An HTTPS URL that can be used to see transaction details
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<Box<str>>>(date: T0, url: T1) -> Self {
        Self {
            date: date.into(),
            url: url.into(),
        }
    }

    /// Date the withdrawal was completed in Unix time
    #[must_use]
    pub fn date<T: Into<i64>>(mut self, val: T) -> Self {
        self.date = val.into();
        self
    }

    /// An HTTPS URL that can be used to see transaction details
    #[must_use]
    pub fn url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.url = val.into();
        self
    }
}
