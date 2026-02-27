use serde::{Deserialize, Serialize};
/// New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
/// # Notes
/// This object represents an update from original update field `edited_message`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateEditedMessage {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    pub edited_message: Box<crate::types::Message>,
}
impl UpdateEditedMessage {
    /// Creates a new `UpdateEditedMessage`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `edited_message` - New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Message>>(
        update_id: T0,
        edited_message: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            edited_message: Box::new(edited_message.into()),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// New version of a message that is known to the bot and was edited. This update may at times be triggered by changes to message fields that are either unavailable or not actively used by your bot.
    #[must_use]
    pub fn edited_message<T: Into<crate::types::Message>>(self, val: T) -> Self {
        let mut this = self;
        this.edited_message = Box::new(val.into());
        this
    }
}
impl From<UpdateEditedMessage> for crate::types::Message {
    fn from(val: UpdateEditedMessage) -> Self {
        *val.edited_message
    }
}
impl<Client> crate::Extractor<Client> for UpdateEditedMessage {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
