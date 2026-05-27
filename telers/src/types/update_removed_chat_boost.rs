use serde::{Deserialize, Serialize};
/// A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
/// # Notes
/// This object represents an update from original update field `removed_chat_boost`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateRemovedChatBoost {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    pub removed_chat_boost: crate::types::ChatBoostRemoved,
}
impl UpdateRemovedChatBoost {
    /// Creates a new `UpdateRemovedChatBoost`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `removed_chat_boost` - A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::ChatBoostRemoved>>(
        update_id: T0,
        removed_chat_boost: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            removed_chat_boost: removed_chat_boost.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// A boost was removed from a chat. The bot must be an administrator in the chat to receive these updates.
    #[must_use]
    pub fn removed_chat_boost<T: Into<crate::types::ChatBoostRemoved>>(mut self, val: T) -> Self {
        self.removed_chat_boost = val.into();
        self
    }
}
impl From<UpdateRemovedChatBoost> for crate::types::ChatBoostRemoved {
    fn from(val: UpdateRemovedChatBoost) -> Self {
        val.removed_chat_boost
    }
}
impl<Client> crate::Extractor<Client> for UpdateRemovedChatBoost {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
