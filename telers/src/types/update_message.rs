use serde::{Deserialize, Serialize};
/// New incoming message of any kind - text, photo, sticker, etc.
/// # Notes
/// This object represents an update from original update field `message`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMessage {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming message of any kind - text, photo, sticker, etc.
    pub message: Box<crate::types::Message>,
}
impl UpdateMessage {
    /// Creates a new `UpdateMessage`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `message` - New incoming message of any kind - text, photo, sticker, etc.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Message>>(update_id: T0, message: T1) -> Self {
        Self {
            update_id: update_id.into(),
            message: Box::new(message.into()),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// New incoming message of any kind - text, photo, sticker, etc.
    #[must_use]
    pub fn message<T: Into<crate::types::Message>>(mut self, val: T) -> Self {
        self.message = Box::new(val.into());
        self
    }
}
impl From<UpdateMessage> for crate::types::Message {
    fn from(val: UpdateMessage) -> Self {
        *val.message
    }
}
impl<Client> crate::Extractor<Client> for UpdateMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
