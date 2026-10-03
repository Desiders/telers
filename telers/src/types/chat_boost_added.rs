/// This object represents a service message about a user boosting a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostadded>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct ChatBoostAdded {
    /// Number of boosts added by the user
    pub boost_count: i64,
}
impl ChatBoostAdded {
    /// Creates a new `ChatBoostAdded`.
    ///
    /// # Arguments
    /// * `boost_count` - Number of boosts added by the user
    #[must_use]
    pub fn new<T0: Into<i64>>(boost_count: T0) -> Self {
        Self {
            boost_count: boost_count.into(),
        }
    }

    /// Number of boosts added by the user
    #[must_use]
    pub fn boost_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.boost_count = val.into();
        self
    }
}
