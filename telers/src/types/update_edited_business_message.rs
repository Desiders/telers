use serde::{Deserialize, Serialize};
/// New version of a message from a connected business account
/// # Notes
/// This object represents an update from original update field `edited_business_message`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEditedBusinessMessage {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New version of a message from a connected business account
    pub edited_business_message: Box<crate::types::Message>,
}
impl UpdateEditedBusinessMessage {
    /// Creates a new `UpdateEditedBusinessMessage`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `edited_business_message` - New version of a message from a connected business account
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Message>>(
        update_id: T0,
        edited_business_message: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            edited_business_message: Box::new(edited_business_message.into()),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// New version of a message from a connected business account
    #[must_use]
    pub fn edited_business_message<T: Into<crate::types::Message>>(self, val: T) -> Self {
        let mut this = self;
        this.edited_business_message = Box::new(val.into());
        this
    }
}
impl From<UpdateEditedBusinessMessage> for crate::types::Message {
    fn from(val: UpdateEditedBusinessMessage) -> Self {
        *val.edited_business_message
    }
}
impl<Client> crate::Extractor<Client> for UpdateEditedBusinessMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
