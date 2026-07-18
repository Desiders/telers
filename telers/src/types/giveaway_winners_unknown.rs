use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// This object represents a [`crate::types::GiveawayWinners`] unknown to this version of the library.
/// # Notes
/// Fields shared by all known variants are parsed as usual; everything else is kept in `extra`, so the object can be inspected and reserialized without data loss.
/// # Documentation
/// <https://core.telegram.org/bots/api#giveawaywinners>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GiveawayWinnersUnknown {
    /// The chat that created the giveaway
    pub chat: Box<crate::types::Chat>,
    /// Identifier of the message with the giveaway in the chat
    pub giveaway_message_id: i64,
    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    pub winners_selection_date: i64,
    /// Total number of winners in the giveaway
    pub winner_count: i64,
    /// List of up to 100 winners of the giveaway
    pub winners: Box<[crate::types::User]>,
    #[serde(flatten)]
    pub extra: BTreeMap<Box<str>, serde_json::Value>,
}
impl GiveawayWinnersUnknown {
    /// Creates a new `GiveawayWinnersUnknown`.
    ///
    /// # Arguments
    /// * `chat` - The chat that created the giveaway
    /// * `giveaway_message_id` - Identifier of the message with the giveaway in the chat
    /// * `winners_selection_date` - Point in time (Unix timestamp) when winners of the giveaway were selected
    /// * `winner_count` - Total number of winners in the giveaway
    /// * `winners` - List of up to 100 winners of the giveaway
    #[must_use]
    pub fn new<
        T0: Into<crate::types::Chat>,
        T1: Into<i64>,
        T2: Into<i64>,
        T3: Into<i64>,
        T4Item: Into<crate::types::User>,
        T4: IntoIterator<Item = T4Item>,
    >(
        chat: T0,
        giveaway_message_id: T1,
        winners_selection_date: T2,
        winner_count: T3,
        winners: T4,
    ) -> Self {
        Self {
            chat: Box::new(chat.into()),
            giveaway_message_id: giveaway_message_id.into(),
            winners_selection_date: winners_selection_date.into(),
            winner_count: winner_count.into(),
            winners: winners.into_iter().map(Into::into).collect(),
            extra: BTreeMap::new(),
        }
    }

    /// The chat that created the giveaway
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Box::new(val.into());
        self
    }

    /// Identifier of the message with the giveaway in the chat
    #[must_use]
    pub fn giveaway_message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.giveaway_message_id = val.into();
        self
    }

    /// Point in time (Unix timestamp) when winners of the giveaway were selected
    #[must_use]
    pub fn winners_selection_date<T: Into<i64>>(mut self, val: T) -> Self {
        self.winners_selection_date = val.into();
        self
    }

    /// Total number of winners in the giveaway
    #[must_use]
    pub fn winner_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.winner_count = val.into();
        self
    }

    /// List of up to 100 winners of the giveaway
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn winners<T: Into<Box<[crate::types::User]>>>(mut self, val: T) -> Self {
        self.winners = self
            .winners
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// List of up to 100 winners of the giveaway
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn winner<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.winners = self
            .winners
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
