use serde::{Deserialize, Serialize};
/// User payment subscription has changed
/// # Notes
/// This object represents an update from original update field `subscription`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateSubscription {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// User payment subscription has changed
    pub subscription: crate::types::BotSubscriptionUpdated,
}
impl UpdateSubscription {
    /// Creates a new `UpdateSubscription`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `subscription` - User payment subscription has changed
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::BotSubscriptionUpdated>>(
        update_id: T0,
        subscription: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            subscription: subscription.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// User payment subscription has changed
    #[must_use]
    pub fn subscription<T: Into<crate::types::BotSubscriptionUpdated>>(mut self, val: T) -> Self {
        self.subscription = val.into();
        self
    }
}
impl From<UpdateSubscription> for crate::types::BotSubscriptionUpdated {
    fn from(val: UpdateSubscription) -> Self {
        val.subscription
    }
}
impl<Client> crate::Extractor<Client> for UpdateSubscription {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
