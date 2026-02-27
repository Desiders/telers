use serde::{Deserialize, Serialize};
/// New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot
/// # Notes
/// This object represents an update from original update field `poll`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePoll {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot
    pub poll: Box<crate::types::Poll>,
}
impl UpdatePoll {
    /// Creates a new `UpdatePoll`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `poll` - New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Poll>>(update_id: T0, poll: T1) -> Self {
        Self {
            update_id: update_id.into(),
            poll: Box::new(poll.into()),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// New poll state. Bots receive only updates about manually stopped polls and polls, which are sent by the bot
    #[must_use]
    pub fn poll<T: Into<crate::types::Poll>>(self, val: T) -> Self {
        let mut this = self;
        this.poll = Box::new(val.into());
        this
    }
}
impl From<UpdatePoll> for crate::types::Poll {
    fn from(val: UpdatePoll) -> Self {
        *val.poll
    }
}
impl<Client> crate::Extractor<Client> for UpdatePoll {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
