use serde::{Deserialize, Serialize};
/// A request to join the chat has been sent. The bot must have the `can_invite_users` administrator right in the chat to receive these updates.
/// # Notes
/// This object represents an update from original update field `chat_join_request`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChatJoinRequest {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A request to join the chat has been sent. The bot must have the `can_invite_users` administrator right in the chat to receive these updates.
    pub chat_join_request: crate::types::ChatJoinRequest,
}
impl UpdateChatJoinRequest {
    /// Creates a new `UpdateChatJoinRequest`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `chat_join_request` - A request to join the chat has been sent. The bot must have the `can_invite_users` administrator right in the chat to receive these updates.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::ChatJoinRequest>>(
        update_id: T0,
        chat_join_request: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            chat_join_request: chat_join_request.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// A request to join the chat has been sent. The bot must have the `can_invite_users` administrator right in the chat to receive these updates.
    #[must_use]
    pub fn chat_join_request<T: Into<crate::types::ChatJoinRequest>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_join_request = val.into();
        this
    }
}
impl From<UpdateChatJoinRequest> for crate::types::ChatJoinRequest {
    fn from(val: UpdateChatJoinRequest) -> Self {
        val.chat_join_request
    }
}
impl<Client> crate::Extractor<Client> for UpdateChatJoinRequest {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
