use serde::{Deserialize, Serialize};
/// A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `message_reaction` in the list of `allowed_updates` to receive these updates. The update isn't received for reactions set by bots.
/// # Notes
/// This object represents an update from original update field `message_reaction`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMessageReaction {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `message_reaction` in the list of `allowed_updates` to receive these updates. The update isn't received for reactions set by bots.
    pub message_reaction: crate::types::MessageReactionUpdated,
}
impl UpdateMessageReaction {
    /// Creates a new `UpdateMessageReaction`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `message_reaction` - A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `message_reaction` in the list of `allowed_updates` to receive these updates. The update isn't received for reactions set by bots.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::MessageReactionUpdated>>(
        update_id: T0,
        message_reaction: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            message_reaction: message_reaction.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// A reaction to a message was changed by a user. The bot must be an administrator in the chat and must explicitly specify `message_reaction` in the list of `allowed_updates` to receive these updates. The update isn't received for reactions set by bots.
    #[must_use]
    pub fn message_reaction<T: Into<crate::types::MessageReactionUpdated>>(
        mut self,
        val: T,
    ) -> Self {
        self.message_reaction = val.into();
        self
    }
}
impl From<UpdateMessageReaction> for crate::types::MessageReactionUpdated {
    fn from(val: UpdateMessageReaction) -> Self {
        val.message_reaction
    }
}
impl<Client> crate::Extractor<Client> for UpdateMessageReaction {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
