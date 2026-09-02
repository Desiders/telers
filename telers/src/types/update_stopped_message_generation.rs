use serde::{Deserialize, Serialize};
/// A user asked the bot to stop the generation of a message
/// # Notes
/// This object represents an update from original update field `stopped_message_generation`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateStoppedMessageGeneration {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A user asked the bot to stop the generation of a message
    pub stopped_message_generation: crate::types::MessageGenerationStopped,
}
impl UpdateStoppedMessageGeneration {
    /// Creates a new `UpdateStoppedMessageGeneration`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `stopped_message_generation` - A user asked the bot to stop the generation of a message
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::MessageGenerationStopped>>(
        update_id: T0,
        stopped_message_generation: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            stopped_message_generation: stopped_message_generation.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// A user asked the bot to stop the generation of a message
    #[must_use]
    pub fn stopped_message_generation<T: Into<crate::types::MessageGenerationStopped>>(
        mut self,
        val: T,
    ) -> Self {
        self.stopped_message_generation = val.into();
        self
    }
}
impl From<UpdateStoppedMessageGeneration> for crate::types::MessageGenerationStopped {
    fn from(val: UpdateStoppedMessageGeneration) -> Self {
        val.stopped_message_generation
    }
}
impl<Client> crate::Extractor<Client> for UpdateStoppedMessageGeneration {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
