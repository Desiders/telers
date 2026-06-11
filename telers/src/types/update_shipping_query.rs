use serde::{Deserialize, Serialize};
/// New incoming shipping query. Only for invoices with flexible price.
/// # Notes
/// This object represents an update from original update field `shipping_query`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateShippingQuery {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming shipping query. Only for invoices with flexible price.
    pub shipping_query: crate::types::ShippingQuery,
}
impl UpdateShippingQuery {
    /// Creates a new `UpdateShippingQuery`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `shipping_query` - New incoming shipping query. Only for invoices with flexible price.
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::ShippingQuery>>(
        update_id: T0,
        shipping_query: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            shipping_query: shipping_query.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// New incoming shipping query. Only for invoices with flexible price.
    #[must_use]
    pub fn shipping_query<T: Into<crate::types::ShippingQuery>>(mut self, val: T) -> Self {
        self.shipping_query = val.into();
        self
    }
}
impl From<UpdateShippingQuery> for crate::types::ShippingQuery {
    fn from(val: UpdateShippingQuery) -> Self {
        val.shipping_query
    }
}
impl<Client> crate::Extractor<Client> for UpdateShippingQuery {
    type Error = crate::errors::ConvertToTypeError;

    fn extract(
        request: &crate::Request<Client>,
    ) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send {
        let val = TryFrom::try_from((*request.update).clone());
        async move { val }
    }
}
