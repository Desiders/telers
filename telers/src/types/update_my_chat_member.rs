use serde::{Deserialize, Serialize};
/// The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
/// # Notes
/// This object represents an update from original update field `my_chat_member`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateMyChatMember {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    pub my_chat_member: crate::types::ChatMemberUpdated,
}
impl UpdateMyChatMember {
    /// Creates a new `UpdateMyChatMember`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `my_chat_member` - The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::ChatMemberUpdated>>(
        update_id: T0,
        my_chat_member: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            my_chat_member: my_chat_member.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// The bot's chat member status was updated in a chat. For private chats, this update is received only when the bot is blocked or unblocked by the user.
    #[must_use]
    pub fn my_chat_member<T: Into<crate::types::ChatMemberUpdated>>(mut self, val: T) -> Self {
        self.my_chat_member = val.into();
        self
    }
}
impl From<UpdateMyChatMember> for crate::types::ChatMemberUpdated {
    fn from(val: UpdateMyChatMember) -> Self {
        val.my_chat_member
    }
}
impl<Client> crate::Extractor<Client> for UpdateMyChatMember {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
