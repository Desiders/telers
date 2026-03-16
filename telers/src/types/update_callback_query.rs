use serde::{Deserialize, Serialize};
/// New incoming callback query
/// # Notes
/// This object represents an update from original update field `callback_query`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateCallbackQuery {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming callback query
    pub callback_query: crate::types::CallbackQuery,
}
impl UpdateCallbackQuery {
    /// Creates a new `UpdateCallbackQuery`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `callback_query` - New incoming callback query
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::CallbackQuery>>(
        update_id: T0,
        callback_query: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            callback_query: callback_query.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// New incoming callback query
    #[must_use]
    pub fn callback_query<T: Into<crate::types::CallbackQuery>>(self, val: T) -> Self {
        let mut this = self;
        this.callback_query = val.into();
        this
    }
}
impl From<UpdateCallbackQuery> for crate::types::CallbackQuery {
    fn from(val: UpdateCallbackQuery) -> Self {
        val.callback_query
    }
}
impl<Client> crate::Extractor<Client> for UpdateCallbackQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
