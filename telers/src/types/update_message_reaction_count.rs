use serde::{Deserialize, Serialize};
/// Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `message_reaction_count` in the list of `allowed_updates` to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
/// # Notes
/// This object represents an update from original update field `message_reaction_count`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMessageReactionCount {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `message_reaction_count` in the list of `allowed_updates` to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
    pub message_reaction_count: crate::types::MessageReactionCountUpdated,
}
impl UpdateMessageReactionCount {
    /// Creates a new `UpdateMessageReactionCount`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `message_reaction_count` - Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `message_reaction_count` in the list of `allowed_updates` to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::MessageReactionCountUpdated>>(
        update_id: T0,
        message_reaction_count: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            message_reaction_count: message_reaction_count.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// Reactions to a message with anonymous reactions were changed. The bot must be an administrator in the chat and must explicitly specify `message_reaction_count` in the list of `allowed_updates` to receive these updates. The updates are grouped and can be sent with delay up to a few minutes.
    #[must_use]
    pub fn message_reaction_count<T: Into<crate::types::MessageReactionCountUpdated>>(
        self,
        val: T,
    ) -> Self {
        let mut this = self;
        this.message_reaction_count = val.into();
        this
    }
}
impl From<UpdateMessageReactionCount> for crate::types::MessageReactionCountUpdated {
    fn from(val: UpdateMessageReactionCount) -> Self {
        val.message_reaction_count
    }
}
impl<Client> crate::Extractor<Client> for UpdateMessageReactionCount {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
