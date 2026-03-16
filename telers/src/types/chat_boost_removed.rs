use serde::{Deserialize, Serialize};
/// This object represents a boost removed from a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#chatboostremoved>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatBoostRemoved {
    /// Chat which was boosted
    pub chat: Box<crate::types::Chat>,
    /// Unique identifier of the boost
    pub boost_id: Box<str>,
    /// Point in time (Unix timestamp) when the boost was removed
    pub remove_date: i64,
    /// Source of the removed boost
    pub source: crate::types::ChatBoostSource,
}
impl ChatBoostRemoved {
    /// Creates a new `ChatBoostRemoved`.
    ///
    /// # Arguments
    /// * `chat` - Chat which was boosted
    /// * `boost_id` - Unique identifier of the boost
    /// * `remove_date` - Point in time (Unix timestamp) when the boost was removed
    /// * `source` - Source of the removed boost
    #[must_use]
    pub fn new<
        T0: Into<crate::types::Chat>,
        T1: Into<Box<str>>,
        T2: Into<i64>,
        T3: Into<crate::types::ChatBoostSource>,
    >(
        chat: T0,
        boost_id: T1,
        remove_date: T2,
        source: T3,
    ) -> Self {
        Self {
            chat: Box::new(chat.into()),
            boost_id: boost_id.into(),
            remove_date: remove_date.into(),
            source: source.into(),
        }
    }

    /// Chat which was boosted
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(self, val: T) -> Self {
        let mut this = self;
        this.chat = Box::new(val.into());
        this
    }

    /// Unique identifier of the boost
    #[must_use]
    pub fn boost_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.boost_id = val.into();
        this
    }

    /// Point in time (Unix timestamp) when the boost was removed
    #[must_use]
    pub fn remove_date<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.remove_date = val.into();
        this
    }

    /// Source of the removed boost
    #[must_use]
    pub fn source<T: Into<crate::types::ChatBoostSource>>(self, val: T) -> Self {
        let mut this = self;
        this.source = val.into();
        this
    }
}
