use serde::{Deserialize, Serialize};
/// New incoming inline query
/// # Notes
/// This object represents an update from original update field `inline_query`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInlineQuery {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming inline query
    pub inline_query: crate::types::InlineQuery,
}
impl UpdateInlineQuery {
    /// Creates a new `UpdateInlineQuery`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `inline_query` - New incoming inline query
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::InlineQuery>>(
        update_id: T0,
        inline_query: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            inline_query: inline_query.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// New incoming inline query
    #[must_use]
    pub fn inline_query<T: Into<crate::types::InlineQuery>>(mut self, val: T) -> Self {
        self.inline_query = val.into();
        self
    }
}
impl From<UpdateInlineQuery> for crate::types::InlineQuery {
    fn from(val: UpdateInlineQuery) -> Self {
        val.inline_query
    }
}
impl<Client> crate::Extractor<Client> for UpdateInlineQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
