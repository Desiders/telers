use serde::{Deserialize, Serialize};
/// Contains the list of gifts received and owned by a user or a chat.
/// # Documentation
/// <https://core.telegram.org/bots/api#ownedgifts>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OwnedGifts {
    /// The total number of gifts owned by the user or the chat
    pub total_count: i64,
    /// The list of gifts
    pub gifts: Box<[crate::types::OwnedGift]>,
    /// Offset for the next request. If empty, then there are no more results
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<Box<str>>,
}
impl OwnedGifts {
    /// Creates a new `OwnedGifts`.
    ///
    /// # Arguments
    /// * `total_count` - The total number of gifts owned by the user or the chat
    /// * `gifts` - The list of gifts
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1Item: Into<crate::types::OwnedGift>,
        T1: IntoIterator<Item = T1Item>,
    >(
        total_count: T0,
        gifts: T1,
    ) -> Self {
        Self {
            total_count: total_count.into(),
            gifts: gifts.into_iter().map(Into::into).collect(),
            next_offset: None,
        }
    }

    /// The total number of gifts owned by the user or the chat
    #[must_use]
    pub fn total_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.total_count = val.into();
        self
    }

    /// The list of gifts
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn gifts<T: Into<Box<[crate::types::OwnedGift]>>>(mut self, val: T) -> Self {
        self.gifts = self
            .gifts
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// The list of gifts
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn gift<T: Into<crate::types::OwnedGift>>(mut self, val: T) -> Self {
        self.gifts = self
            .gifts
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }

    /// Offset for the next request. If empty, then there are no more results
    #[must_use]
    pub fn next_offset<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.next_offset = Some(val.into());
        self
    }

    /// Offset for the next request. If empty, then there are no more results
    #[must_use]
    pub fn next_offset_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.next_offset = val.map(Into::into);
        self
    }
}
