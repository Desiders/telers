use serde::{Deserialize, Serialize};
/// New incoming channel post of any kind - text, photo, sticker, etc.
/// # Notes
/// This object represents an update from original update field `channel_post`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateChannelPost {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming channel post of any kind - text, photo, sticker, etc.
    pub channel_post: Box<crate::types::Message>,
}
impl UpdateChannelPost {
    /// Creates a new `UpdateChannelPost`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `channel_post` - New incoming channel post of any kind - text, photo, sticker, etc.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::Message>>(
        update_id: T0,
        channel_post: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            channel_post: Box::new(channel_post.into()),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// New incoming channel post of any kind - text, photo, sticker, etc.
    #[must_use]
    pub fn channel_post<T: Into<crate::types::Message>>(self, val: T) -> Self {
        let mut this = self;
        this.channel_post = Box::new(val.into());
        this
    }
}
impl From<UpdateChannelPost> for crate::types::Message {
    fn from(val: UpdateChannelPost) -> Self {
        *val.channel_post
    }
}
impl<Client> crate::Extractor<Client> for UpdateChannelPost {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
