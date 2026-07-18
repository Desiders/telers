use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a Giveaway unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveaway>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GiveawayUnknown {
    /// The list of chats which the user must join to participate in the giveaway
    pub chats: Box<[crate::types::Chat]>,
    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    pub winners_selection_date: i64,
    /// The number of users which are supposed to be selected as winners of the giveaway
    pub winner_count: i64,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl GiveawayUnknown {
    /// Creates a new `GiveawayUnknown`.
    ///
    /// # Arguments
    /// * `chats` - The list of chats which the user must join to participate in the giveaway
    /// * `winners_selection_date` - Point in time (Unix timestamp) when winners of the giveaway will be selected
    /// * `winner_count` - The number of users which are supposed to be selected as winners of the giveaway
    #[must_use]
    pub fn new<
        T0Item: Into<crate::types::Chat>,
        T0: IntoIterator<Item = T0Item>,
        T1: Into<i64>,
        T2: Into<i64>,
    >(
        chats: T0,
        winners_selection_date: T1,
        winner_count: T2,
    ) -> Self {
        Self {
            chats: chats.into_iter().map(Into::into).collect(),
            winners_selection_date: winners_selection_date.into(),
            winner_count: winner_count.into(),
            extra: BTreeMap::new(),
        }
    }

    /// The list of chats which the user must join to participate in the giveaway
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn chats<T: Into<Box<[crate::types::Chat]>>>(mut self, val: T) -> Self {
        self.chats = self
            .chats
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// The list of chats which the user must join to participate in the giveaway
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chats = self
            .chats
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Point in time (Unix timestamp) when winners of the giveaway will be selected
    #[must_use]
    pub fn winners_selection_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.winners_selection_date = val.into();
        self
    }

    /// The number of users which are supposed to be selected as winners of the giveaway
    #[must_use]
    pub fn winner_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.winner_count = val.into();
        self
    }
}
