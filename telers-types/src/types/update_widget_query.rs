/// New incoming query from a widget in a message sent by the bot or via the bot
/// # Notes
/// This object represents an update from original update field `widget_query`.
/// # Documentation
/// <https://core.telegram.org/bots/api#update>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateWidgetQuery {
    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    pub update_id: i64,
    /// New incoming query from a widget in a message sent by the bot or via the bot
    pub widget_query: crate::types::WidgetQuery,
}
impl UpdateWidgetQuery {
    /// Creates a new `UpdateWidgetQuery`.
    ///
    /// # Arguments
    /// * `update_id` - The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    /// * `widget_query` - New incoming query from a widget in a message sent by the bot or via the bot
    #[must_use]
    pub fn new<T0: Into<i64>, T1: Into<crate::types::WidgetQuery>>(
        update_id: T0,
        widget_query: T1,
    ) -> Self {
        Self {
            update_id: update_id.into(),
            widget_query: widget_query.into(),
        }
    }

    /// The update's unique identifier. Update identifiers start from a certain positive number and increase sequentially. This identifier becomes especially handy if you're using webhooks, since it allows you to ignore repeated updates or to restore the correct update sequence, should they get out of order. If there are no new updates for at least a week, then identifier of the next update will be chosen randomly instead of sequentially.
    #[must_use]
    pub fn update_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.update_id = val.into();
        self
    }

    /// New incoming query from a widget in a message sent by the bot or via the bot
    #[must_use]
    pub fn widget_query<T: Into<crate::types::WidgetQuery>>(mut self, val: T) -> Self {
        self.widget_query = val.into();
        self
    }
}
impl From<UpdateWidgetQuery> for crate::types::WidgetQuery {
    fn from(val: UpdateWidgetQuery) -> Self {
        val.widget_query
    }
}
