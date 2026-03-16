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
    pub fn total_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.total_count = val.into();
        this
    }

    /// The list of gifts
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn gifts<T: Into<Box<[crate::types::OwnedGift]>>>(self, val: T) -> Self {
        let mut this = self;
        this.gifts = this
            .gifts
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// The list of gifts
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn gift<T: Into<crate::types::OwnedGift>>(self, val: T) -> Self {
        let mut this = self;
        this.gifts = this
            .gifts
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }

    /// Offset for the next request. If empty, then there are no more results
    #[must_use]
    pub fn next_offset<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.next_offset = Some(val.into());
        this
    }

    /// Offset for the next request. If empty, then there are no more results
    #[must_use]
    pub fn next_offset_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.next_offset = val.map(Into::into);
        this
    }
}
