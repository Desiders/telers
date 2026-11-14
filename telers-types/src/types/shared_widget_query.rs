/// This object represents an incoming query from a forwarded message with a shared widget created by the bot. Use [`telers_methods::methods::AnswerWidgetQuery`](https://docs.rs/telers-methods/latest/telers_methods/methods/answer_widget_query/struct.AnswerWidgetQuery.html) to answer the query. Each user can send at most 5 queries per 5 seconds to the bot from all its widgets.
/// # Documentation
/// <https://core.telegram.org/bots/api#sharedwidgetquery>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct SharedWidgetQuery {
    /// Unique identifier for this query
    pub id: Box<str>,
    /// Sender
    pub from: Box<crate::types::User>,
    /// Type of the query
    pub r#type: crate::types::WidgetQueryType,
    /// Data of the query, 1-64 bytes
    pub data: Box<str>,
    /// Unique identifier of the shared widget, which sent the query
    pub widget_id: Box<str>,
}
impl SharedWidgetQuery {
    /// Creates a new `SharedWidgetQuery`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this query
    /// * `from` - Sender
    /// * `type` - Type of the query
    /// * `data` - Data of the query, 1-64 bytes
    /// * `widget_id` - Unique identifier of the shared widget, which sent the query
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::User>,
        T2: Into<crate::types::WidgetQueryType>,
        T3: Into<Box<str>>,
        T4: Into<Box<str>>,
    >(
        id: T0,
        from: T1,
        r#type: T2,
        data: T3,
        widget_id: T4,
    ) -> Self {
        Self {
            id: id.into(),
            from: Box::new(from.into()),
            r#type: r#type.into(),
            data: data.into(),
            widget_id: widget_id.into(),
        }
    }

    /// Unique identifier for this query
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Sender
    #[must_use]
    pub fn from<T: Into<crate::types::User>>(mut self, val: T) -> Self {
        self.from = Box::new(val.into());
        self
    }

    /// Type of the query
    #[must_use]
    pub fn r#type<T: Into<crate::types::WidgetQueryType>>(mut self, val: T) -> Self {
        self.r#type = val.into();
        self
    }

    /// Data of the query, 1-64 bytes
    #[must_use]
    pub fn data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.data = val.into();
        self
    }

    /// Unique identifier of the shared widget, which sent the query
    #[must_use]
    pub fn widget_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.widget_id = val.into();
        self
    }
}
