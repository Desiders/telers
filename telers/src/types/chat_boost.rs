use serde::{Deserialize, Serialize};
/// This object contains information about a chat boost.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboost>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatBoost {
    /// Unique identifier of the boost
    pub boost_id: Box<str>,
    /// Point in time (Unix timestamp) when the chat was boosted
    pub add_date: i64,
    /// Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged
    pub expiration_date: i64,
    /// Source of the added boost
    pub source: crate::types::ChatBoostSource,
}
impl ChatBoost {
    /// Creates a new `ChatBoost`.
    ///
    /// # Arguments
    /// * `boost_id` - Unique identifier of the boost
    /// * `add_date` - Point in time (Unix timestamp) when the chat was boosted
    /// * `expiration_date` - Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged
    /// * `source` - Source of the added boost
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3: Into<crate::types::ChatBoostSource>,
    >(
        boost_id: T0,
        add_date: T1,
        expiration_date: T2,
        source: T3,
    ) -> Self {
        Self {
            boost_id: boost_id.into(),
            add_date: add_date.into(),
            expiration_date: expiration_date.into(),
            source: source.into(),
        }
    }

    /// Unique identifier of the boost
    #[must_use]
    pub fn boost_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.boost_id = val.into();
        this
    }

    /// Point in time (Unix timestamp) when the chat was boosted
    #[must_use]
    pub fn add_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.add_date = val.into();
        this
    }

    /// Point in time (Unix timestamp) when the boost will automatically expire, unless the booster's Telegram Premium subscription is prolonged
    #[must_use]
    pub fn expiration_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.expiration_date = val.into();
        this
    }

    /// Source of the added boost
    #[must_use]
    pub fn source<T: Into<crate::types::ChatBoostSource>>(self, val: T) -> Self {
        let mut this = self;
        this.source = val.into();
        this
    }
}
