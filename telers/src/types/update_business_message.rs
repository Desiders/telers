use serde::{Deserialize, Serialize};
/// New message from a connected business account
/// # Notes
/// This object represents an update from original update field `business_message`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateBusinessMessage {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New message from a connected business account
    pub business_message: Box<crate::types::Message>,
}
impl UpdateBusinessMessage {
    /// Creates a new `UpdateBusinessMessage`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `business_message` - New message from a connected business account
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Message>>(
        update_id: T0,
        business_message: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            business_message: Box::new(business_message.into()),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// New message from a connected business account
    #[must_use]
    pub fn business_message<T: Into<crate::types::Message>>(mut self, val: T) -> Self {
        self.business_message = Box::new(val.into());
        self
    }
}
impl From<UpdateBusinessMessage> for crate::types::Message {
    fn from(val: UpdateBusinessMessage) -> Self {
        *val.business_message
    }
}
impl<Client> crate::Extractor<Client> for UpdateBusinessMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
