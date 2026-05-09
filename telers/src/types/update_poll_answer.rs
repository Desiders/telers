use serde::{Deserialize, Serialize};
/// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
/// # Notes
/// This object represents an update from original update field `poll_answer`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePollAnswer {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    pub poll_answer: crate::types::PollAnswer,
}
impl UpdatePollAnswer {
    /// Creates a new `UpdatePollAnswer`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `poll_answer` - A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::PollAnswer>>(
        update_id: T0,
        poll_answer: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            poll_answer: poll_answer.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// A user changed their answer in a non-anonymous poll. Bots receive new votes only in polls that were sent by the bot itself.
    #[must_use]
    pub fn poll_answer<T: Into<crate::types::PollAnswer>>(mut self, val: T) -> Self {
        self.poll_answer = val.into();
        self
    }
}
impl From<UpdatePollAnswer> for crate::types::PollAnswer {
    fn from(val: UpdatePollAnswer) -> Self {
        val.poll_answer
    }
}
impl<Client> crate::Extractor<Client> for UpdatePollAnswer {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
