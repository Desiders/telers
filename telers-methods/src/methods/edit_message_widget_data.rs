/// Use this method to edit data of a widget in a message sent by the bot or via the bot (for inline bots). Public data of a shared widget can be changed at most once per second. On success, `true` is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagewidgetdata>
/// # Returns
/// - `bool`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct EditMessageWidgetData {
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target supergroup in the format @username.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub chat_id: Option<crate::types::ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message with the widget.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub inline_message_id: Option<Box<str>>,
    /// New public data of the widget, 0-1024 bytes; pass an empty string to remove the public data. If omitted, then the public data isn't changed.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub public_data: Option<Box<str>>,
    /// New data of the widget, which is specific to the message, 0-1024 bytes; pass an empty string to remove the data. If omitted, then the data isn't changed.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub chat_data: Option<Box<str>>,
}
impl EditMessageWidgetData {
    /// Creates a new `EditMessageWidgetData`.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new() -> Self {
        Self {
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            public_data: None,
            chat_data: None,
        }
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target supergroup in the format @username.
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(mut self, val: T) -> Self {
        self.chat_id = Some(val.into());
        self
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target supergroup in the format @username.
    #[must_use]
    pub fn chat_id_option<T: Into<crate::types::ChatIdKind>>(mut self, val: Option<T>) -> Self {
        self.chat_id = val.map(Into::into);
        self
    }

    /// Required if `inline_message_id` is not specified. Identifier of the message with the widget.
    #[must_use]
    pub fn message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.message_id = Some(val.into());
        self
    }

    /// Required if `inline_message_id` is not specified. Identifier of the message with the widget.
    #[must_use]
    pub fn message_id_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.message_id = val.map(Into::into);
        self
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message.
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.inline_message_id = Some(val.into());
        self
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message.
    #[must_use]
    pub fn inline_message_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.inline_message_id = val.map(Into::into);
        self
    }

    /// New public data of the widget, 0-1024 bytes; pass an empty string to remove the public data. If omitted, then the public data isn't changed.
    #[must_use]
    pub fn public_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.public_data = Some(val.into());
        self
    }

    /// New public data of the widget, 0-1024 bytes; pass an empty string to remove the public data. If omitted, then the public data isn't changed.
    #[must_use]
    pub fn public_data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.public_data = val.map(Into::into);
        self
    }

    /// New data of the widget, which is specific to the message, 0-1024 bytes; pass an empty string to remove the data. If omitted, then the data isn't changed.
    #[must_use]
    pub fn chat_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.chat_data = Some(val.into());
        self
    }

    /// New data of the widget, which is specific to the message, 0-1024 bytes; pass an empty string to remove the data. If omitted, then the data isn't changed.
    #[must_use]
    pub fn chat_data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.chat_data = val.map(Into::into);
        self
    }
}
impl Default for EditMessageWidgetData {
    fn default() -> Self {
        Self::new()
    }
}
impl super::TelegramMethod for EditMessageWidgetData {
    type Method = Self;
    type Return = bool;

    fn build_request(self) -> super::Request<Self::Method> {
        super::Request::new("editMessageWidgetData", self, None)
    }
}
