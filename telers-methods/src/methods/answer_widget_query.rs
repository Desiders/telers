/// Use this method to send an answer to a query from a widget. The query must be answered within 15 seconds. The answer is received only by the widget that sent the query and doesn't change data of the widget; use [`crate::methods::EditMessageWidgetData`] or [`crate::methods::EditSharedWidgetData`] to change it. On success, `true` is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#answerwidgetquery>
/// # Returns
/// - `bool`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct AnswerWidgetQuery {
    /// Unique identifier for the query to be answered
    pub widget_query_id: Box<str>,
    /// Data to be returned to the widget, 0-1024 bytes
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub data: Option<Box<str>>,
}
impl AnswerWidgetQuery {
    /// Creates a new `AnswerWidgetQuery`.
    ///
    /// # Arguments
    /// * `widget_query_id` - Unique identifier for the query to be answered
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(widget_query_id: T0) -> Self {
        Self {
            widget_query_id: widget_query_id.into(),
            data: None,
        }
    }

    /// Unique identifier for the query to be answered
    #[must_use]
    pub fn widget_query_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.widget_query_id = val.into();
        self
    }

    /// Data to be returned to the widget, 0-1024 bytes
    #[must_use]
    pub fn data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.data = Some(val.into());
        self
    }

    /// Data to be returned to the widget, 0-1024 bytes
    #[must_use]
    pub fn data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.data = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for AnswerWidgetQuery {
    type Method = Self;
    type Return = bool;

    fn build_request(self) -> super::Request<Self::Method> {
        super::Request::new("answerWidgetQuery", self, None)
    }
}
