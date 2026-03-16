use serde::{Deserialize, Serialize};
/// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
/// # Notes
/// This object represents an update from original update field `purchased_paid_media`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePurchasedPaidMedia {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    pub purchased_paid_media: crate::types::PaidMediaPurchased,
}
impl UpdatePurchasedPaidMedia {
    /// Creates a new `UpdatePurchasedPaidMedia`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `purchased_paid_media` - A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::PaidMediaPurchased>>(
        update_id: T0,
        purchased_paid_media: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            purchased_paid_media: purchased_paid_media.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// A user purchased paid media with a non-empty payload sent by the bot in a non-channel chat
    #[must_use]
    pub fn purchased_paid_media<T: Into<crate::types::PaidMediaPurchased>>(self, val: T) -> Self {
        let mut this = self;
        this.purchased_paid_media = val.into();
        this
    }
}
impl From<UpdatePurchasedPaidMedia> for crate::types::PaidMediaPurchased {
    fn from(val: UpdatePurchasedPaidMedia) -> Self {
        val.purchased_paid_media
    }
}
impl<Client> crate::Extractor<Client> for UpdatePurchasedPaidMedia {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
