/// This object represents an incoming query from a widget in a message sent by the bot or via the bot. Use [`telers_methods::methods::AnswerWidgetQuery`](https://docs.rs/telers-methods/latest/telers_methods/methods/answer_widget_query/struct.AnswerWidgetQuery.html) to answer the query. Each user can send at most 5 queries per 5 seconds to the bot from all its widgets.
/// # Documentation
/// <https://core.telegram.org/bots/api#widgetquery>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct WidgetQuery {
    /// Unique identifier for this query
    pub id: Box<str>,
    /// Sender
    pub from: Box<crate::types::User>,
    /// Chat with the message containing the widget; omitted if the widget is in an inline message
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub chat: Option<Box<crate::types::Chat>>,
    /// Identifier of the message containing the widget; omitted if the widget is in an inline message
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub message_id: Option<i64>,
    /// Identifier of the inline message containing the widget
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub inline_message_id: Option<Box<str>>,
    /// Global identifier, uniquely corresponding to the chat to which the message with the widget was sent
    pub chat_instance: Box<str>,
    /// For widgets in inline messages, type of the chat to which the message was sent. Can be either `sender` for a private chat with the inline query sender, `private`, `group`, `supergroup`, or `channel`.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub chat_type: Option<Box<str>>,
    /// Type of the query
    pub r#type: crate::types::WidgetQueryType,
    /// Data of the query, 1-64 bytes
    pub data: Box<str>,
    /// Unique identifier of the shared widget, which sent the query; omitted if the widget isn't shared
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub widget_id: Option<Box<str>>,
}
impl WidgetQuery {
    /// Creates a new `WidgetQuery`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this query
    /// * `from` - Sender
    /// * `chat_instance` - Global identifier, uniquely corresponding to the chat to which the message with the widget was sent
    /// * `type` - Type of the query
    /// * `data` - Data of the query, 1-64 bytes
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<crate::types::User>,
        T2: Into<Box<str>>,
        T3: Into<crate::types::WidgetQueryType>,
        T4: Into<Box<str>>,
    >(
        id: T0,
        from: T1,
        chat_instance: T2,
        r#type: T3,
        data: T4,
    ) -> Self {
        Self {
            id: id.into(),
            from: Box::new(from.into()),
            chat: None,
            message_id: None,
            inline_message_id: None,
            chat_instance: chat_instance.into(),
            chat_type: None,
            r#type: r#type.into(),
            data: data.into(),
            widget_id: None,
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

    /// Chat with the message containing the widget; omitted if the widget is in an inline message
    #[must_use]
    pub fn chat<T: Into<crate::types::Chat>>(mut self, val: T) -> Self {
        self.chat = Some(Box::new(val.into()));
        self
    }

    /// Chat with the message containing the widget; omitted if the widget is in an inline message
    #[must_use]
    pub fn chat_option<T: Into<crate::types::Chat>>(mut self, val: Option<T>) -> Self {
        self.chat = val.map(|val| Box::new(val.into()));
        self
    }

    /// Identifier of the message containing the widget; omitted if the widget is in an inline message
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = Some(val.into());
        self
    }

    /// Identifier of the message containing the widget; omitted if the widget is in an inline message
    #[must_use]
    pub fn message_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_id = val.map(Into::into);
        self
    }

    /// Identifier of the inline message containing the widget
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    /// Identifier of the inline message containing the widget
    #[must_use]
    pub fn inline_message_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.inline_message_id = val.map(Into::into);
        self
    }

    /// Global identifier, uniquely corresponding to the chat to which the message with the widget was sent
    #[must_use]
    pub fn chat_instance<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.chat_instance = val.into();
        self
    }

    /// For widgets in inline messages, type of the chat to which the message was sent. Can be either `sender` for a private chat with the inline query sender, `private`, `group`, `supergroup`, or `channel`.
    #[must_use]
    pub fn chat_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.chat_type = Some(val.into());
        self
    }

    /// For widgets in inline messages, type of the chat to which the message was sent. Can be either `sender` for a private chat with the inline query sender, `private`, `group`, `supergroup`, or `channel`.
    #[must_use]
    pub fn chat_type_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.chat_type = val.map(Into::into);
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

    /// Unique identifier of the shared widget, which sent the query; omitted if the widget isn't shared
    #[must_use]
    pub fn widget_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.widget_id = Some(val.into());
        self
    }

    /// Unique identifier of the shared widget, which sent the query; omitted if the widget isn't shared
    #[must_use]
    pub fn widget_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.widget_id = val.map(Into::into);
        self
    }
}
