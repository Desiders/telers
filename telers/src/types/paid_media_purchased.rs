use serde::{Deserialize, Serialize};
/// This object contains information about a paid media purchase.
/// # Documentation
/// <https://core.telegram.org/bots/api#paidmediapurchased>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaidMediaPurchased {
    /// User who purchased the media
    pub from: Box<crate::types::User>,
    /// Bot-specified paid media payload
    pub paid_media_payload: Box<str>,
}
impl PaidMediaPurchased {
    /// Creates a new `PaidMediaPurchased`.
    ///
    /// # Arguments
    /// * `from` - User who purchased the media
    /// * `paid_media_payload` - Bot-specified paid media payload
    #[must_use]
    pub fn new<T0: Into<crate::types::User>, T1: Into<Box<str>>>(
        from: T0,
        paid_media_payload: T1,
    ) -> Self {
        Self {
            from: Box::new(from.into()),
            paid_media_payload: paid_media_payload.into(),
        }
    }

    /// User who purchased the media
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.from = Box::new(val.into());
        self
    }

    /// Bot-specified paid media payload
    #[must_use]
    pub fn paid_media_payload<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.paid_media_payload = val.into();
        self
    }
}
