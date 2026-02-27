use serde::{Deserialize, Serialize};
/// A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
/// # Notes
/// This object represents an update from original update field `chat_boost`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChatBoost {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    pub chat_boost: crate::types::ChatBoostUpdated,
}
impl UpdateChatBoost {
    /// Creates a new `UpdateChatBoost`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `chat_boost` - A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::ChatBoostUpdated>>(
        update_id: T0,
        chat_boost: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            chat_boost: chat_boost.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// A chat boost was added or changed. The bot must be an administrator in the chat to receive these updates.
    #[must_use]
    pub fn chat_boost<T: Into<crate::types::ChatBoostUpdated>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_boost = val.into();
        this
    }
}
impl From<UpdateChatBoost> for crate::types::ChatBoostUpdated {
    fn from(val: UpdateChatBoost) -> Self {
        val.chat_boost
    }
}
impl<Client> crate::Extractor<Client> for UpdateChatBoost {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
