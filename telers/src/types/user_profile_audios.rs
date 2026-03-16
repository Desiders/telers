use serde::{Deserialize, Serialize};
/// This object represents the audios displayed on a user's profile.
/// # Documentation
/// <https://core.telegram.org/bots/api#userprofileaudios>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProfileAudios {
    /// Total number of profile audios for the target user
    pub total_count: i64,
    /// Requested profile audios
    pub audios: Box<[crate::types::Audio]>,
}
impl UserProfileAudios {
    /// Creates a new `UserProfileAudios`.
    ///
    /// # Arguments
    /// * `total_count` - Total number of profile audios for the target user
    /// * `audios` - Requested profile audios
    #[must_use]
    pub fn new<
        T0: Into<i64>,
        T1Item: Into<crate::types::Audio>,
        T1: IntoIterator<Item = T1Item>,
    >(
        total_count: T0,
        audios: T1,
    ) -> Self {
        Self {
            total_count: total_count.into(),
            audios: audios.into_iter().map(Into::into).collect(),
        }
    }

    /// Total number of profile audios for the target user
    #[must_use]
    pub fn total_count<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.total_count = val.into();
        this
    }

    /// Requested profile audios
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn audios<T: Into<Box<[crate::types::Audio]>>>(self, val: T) -> Self {
        let mut this = self;
        this.audios = this
            .audios
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// Requested profile audios
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn audio<T: Into<crate::types::Audio>>(self, val: T) -> Self {
        let mut this = self;
        this.audios = this
            .audios
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
