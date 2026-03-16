use serde::{Deserialize, Serialize};
/// The paid media is a photo.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediaphoto>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaidMediaPhoto {
    /// The photo
    pub photo: Box<[crate::types::PhotoSize]>,
}
impl PaidMediaPhoto {
    /// Creates a new `PaidMediaPhoto`.
    ///
    /// # Arguments
    /// * `photo` - The photo
    #[must_use]
    pub fn new<T0Item: Into<crate::types::PhotoSize>, T0: IntoIterator<Item = T0Item>>(
        photo: T0,
    ) -> Self {
        Self {
            photo: photo.into_iter().map(Into::into).collect(),
        }
    }

    /// The photo
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn photos<T: Into<Box<[crate::types::PhotoSize]>>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = this
            .photo
            .into_vec()
            .into_iter()
            .chain(val.into())
            .collect();
        this
    }

    /// The photo
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn photo<T: Into<crate::types::PhotoSize>>(self, val: T) -> Self {
        let mut this = self;
        this.photo = this
            .photo
            .into_vec()
            .into_iter()
            .chain(Some(val.into()))
            .collect();
        this
    }
}
