use serde::{Deserialize, Serialize};
/// A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `chat_member` in the list of `allowed_updates` to receive these updates.
/// # Notes
/// This object represents an update from original update field `chat_member`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChatMember {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `chat_member` in the list of `allowed_updates` to receive these updates.
    pub chat_member: crate::types::ChatMemberUpdated,
}
impl UpdateChatMember {
    /// Creates a new `UpdateChatMember`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `chat_member` - A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `chat_member` in the list of `allowed_updates` to receive these updates.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::ChatMemberUpdated>>(
        update_id: T0,
        chat_member: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            chat_member: chat_member.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// A chat member's status was updated in a chat. The bot must be an administrator in the chat and must explicitly specify `chat_member` in the list of `allowed_updates` to receive these updates.
    #[must_use]
    pub fn chat_member<T: Into<crate::types::ChatMemberUpdated>>(mut self, val: T) -> Self {
        self.chat_member = val.into();
        self
    }
}
impl From<UpdateChatMember> for crate::types::ChatMemberUpdated {
    fn from(val: UpdateChatMember) -> Self {
        val.chat_member
    }
}
impl<Client> crate::Extractor<Client> for UpdateChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
