use serde::{Deserialize, Serialize};
/// Describes the paid media added to a message.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediainfo>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaidMediaInfo {
    /// The number of Telegram Stars that must be paid to buy access to the media
    pub star_count: i64,
    /// Information about the paid media
    pub paid_media: Box<[crate::types::PaidMedia]>,
}
impl PaidMediaInfo {
    /// Creates a new `PaidMediaInfo`.
    ///
    /// # Arguments
    /// * `star_count` - The number of Telegram Stars that must be paid to buy access to the media
    /// * `paid_media` - Information about the paid media
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1Item: Into<crate::types::PaidMedia>,
        T1: IntoIterator<Item = T1Item>,
    >(
        star_count: T0,
        paid_media: T1,
    ) -> Self {
        Self {
            star_count: star_count.into(),
            paid_media: paid_media.into_iter().map(Into::into).collect(),
        }
    }

    /// The number of Telegram Stars that must be paid to buy access to the media
    #[must_use]
    pub fn star_count<T: Into<i64>>(mut self, val: T) -> Self {
        self.star_count = val.into();
        self
    }

    /// Information about the paid media
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn paid_medias<T: Into<Box<[crate::types::PaidMedia]>>>(mut self, val: T) -> Self {
        self.paid_media = self
            .paid_media
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        self
    }

    /// Information about the paid media
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn paid_media<T: Into<crate::types::PaidMedia>>(mut self, val: T) -> Self {
        self.paid_media = self
            .paid_media
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        self
    }
}
