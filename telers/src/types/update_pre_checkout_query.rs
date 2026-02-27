use serde::{Deserialize, Serialize};
/// New incoming pre-checkout query. Contains full information about checkout
/// # Notes
/// This object represents an update from original update field `pre_checkout_query`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdatePreCheckoutQuery {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming pre-checkout query. Contains full information about checkout
    pub pre_checkout_query: crate::types::PreCheckoutQuery,
}
impl UpdatePreCheckoutQuery {
    /// Creates a new `UpdatePreCheckoutQuery`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `pre_checkout_query` - New incoming pre-checkout query. Contains full information about checkout
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::PreCheckoutQuery>>(
        update_id: T0,
        pre_checkout_query: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            pre_checkout_query: pre_checkout_query.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.update_id = val.into();
        this
    }

    /// New incoming pre-checkout query. Contains full information about checkout
    #[must_use]
    pub fn pre_checkout_query<T: Into<crate::types::PreCheckoutQuery>>(self, val: T) -> Self {
        let mut this = self;
        this.pre_checkout_query = val.into();
        this
    }
}
impl From<UpdatePreCheckoutQuery> for crate::types::PreCheckoutQuery {
    fn from(val: UpdatePreCheckoutQuery) -> Self {
        val.pre_checkout_query
    }
}
impl<Client> crate::Extractor<Client> for UpdatePreCheckoutQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
