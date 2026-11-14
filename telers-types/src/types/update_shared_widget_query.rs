/// New incoming query from a forwarded message with a shared widget
/// # Notes
/// This object represents an update from original update field `shared_widget_query`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSharedWidgetQuery {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming query from a forwarded message with a shared widget
    pub shared_widget_query: crate::types::SharedWidgetQuery,
}
impl UpdateSharedWidgetQuery {
    /// Creates a new `UpdateSharedWidgetQuery`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `shared_widget_query` - New incoming query from a forwarded message with a shared widget
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::SharedWidgetQuery>>(
        update_id: T0,
        shared_widget_query: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            shared_widget_query: shared_widget_query.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// New incoming query from a forwarded message with a shared widget
    #[must_use]
    pub fn shared_widget_query<T: Into<crate::types::SharedWidgetQuery>>(mut self, val: T) -> Self {
        self.shared_widget_query = val.into();
        self
    }
}
impl From<UpdateSharedWidgetQuery> for crate::types::SharedWidgetQuery {
    fn from(val: UpdateSharedWidgetQuery) -> Self {
        val.shared_widget_query
    }
}
