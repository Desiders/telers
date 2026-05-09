use serde::{Deserialize, Serialize};
/// Messages were deleted from a connected business account
/// # Notes
/// This object represents an update from original update field `deleted_business_messages`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateDeletedBusinessMessages {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// Messages were deleted from a connected business account
    pub deleted_business_messages: crate::types::BusinessMessagesDeleted,
}
impl UpdateDeletedBusinessMessages {
    /// Creates a new `UpdateDeletedBusinessMessages`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `deleted_business_messages` - Messages were deleted from a connected business account
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::BusinessMessagesDeleted>>(
        update_id: T0,
        deleted_business_messages: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            deleted_business_messages: deleted_business_messages.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// Messages were deleted from a connected business account
    #[must_use]
    pub fn deleted_business_messages<T: Into<crate::types::BusinessMessagesDeleted>>(
        mut self,
        val: T,
    ) -> Self {
        self.deleted_business_messages = val.into();
        self
    }
}
impl From<UpdateDeletedBusinessMessages> for crate::types::BusinessMessagesDeleted {
    fn from(val: UpdateDeletedBusinessMessages) -> Self {
        val.deleted_business_messages
    }
}
impl<Client> crate::Extractor<Client> for UpdateDeletedBusinessMessages {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
