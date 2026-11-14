/// This object describes an HTML widget.
/// # Documentation
/// <https://core.telegram.org/bots/api#widget>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct Widget {
    /// Unique identifier of the shared widget, which can be used to send the widget again using the method [`telers_methods::methods::SendSharedWidget`](https://docs.rs/telers-methods/latest/telers_methods/methods/send_shared_widget/struct.SendSharedWidget.html); omitted if the widget isn't shared
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub id: Option<Box<str>>,
    /// Width of the widget in logical pixels; omitted if the widget must occupy the whole width of the message bubble
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub width: Option<i64>,
    /// Height of the widget in logical pixels; omitted if the height of the widget isn't fixed
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub height: Option<i64>,
    /// HTML code of the widget
    pub html: Box<str>,
    /// Public data of the widget, which is available wherever the widget is shown. All messages with a shared widget have the same public data.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub public_data: Option<Box<str>>,
    /// Data of the widget, which is specific to the message
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub chat_data: Option<Box<str>>,
}
impl Widget {
    /// Creates a new `Widget`.
    ///
    /// # Arguments
    /// * `html` - HTML code of the widget
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(html: T0) -> Self {
        Self {
            id: None,
            width: None,
            height: None,
            html: html.into(),
            public_data: None,
            chat_data: None,
        }
    }

    /// Unique identifier of the shared widget, which can be used to send the widget again using the method [`telers_methods::methods::SendSharedWidget`](https://docs.rs/telers-methods/latest/telers_methods/methods/send_shared_widget/struct.SendSharedWidget.html); omitted if the widget isn't shared
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = Some(val.into());
        self
    }

    /// Unique identifier of the shared widget, which can be used to send the widget again using the method [`telers_methods::methods::SendSharedWidget`](https://docs.rs/telers-methods/latest/telers_methods/methods/send_shared_widget/struct.SendSharedWidget.html); omitted if the widget isn't shared
    #[must_use]
    pub fn id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.id = val.map(Into::into);
        self
    }

    /// Width of the widget in logical pixels; omitted if the widget must occupy the whole width of the message bubble
    #[must_use]
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = Some(val.into());
        self
    }

    /// Width of the widget in logical pixels; omitted if the widget must occupy the whole width of the message bubble
    #[must_use]
    pub fn width_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.width = val.map(Into::into);
        self
    }

    /// Height of the widget in logical pixels; omitted if the height of the widget isn't fixed
    #[must_use]
    pub fn height<T: Into<i64>>(mut self, val: T) -> Self {
        self.height = Some(val.into());
        self
    }

    /// Height of the widget in logical pixels; omitted if the height of the widget isn't fixed
    #[must_use]
    pub fn height_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.height = val.map(Into::into);
        self
    }

    /// HTML code of the widget
    #[must_use]
    pub fn html<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.html = val.into();
        self
    }

    /// Public data of the widget, which is available wherever the widget is shown. All messages with a shared widget have the same public data.
    #[must_use]
    pub fn public_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.public_data = Some(val.into());
        self
    }

    /// Public data of the widget, which is available wherever the widget is shown. All messages with a shared widget have the same public data.
    #[must_use]
    pub fn public_data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.public_data = val.map(Into::into);
        self
    }

    /// Data of the widget, which is specific to the message
    #[must_use]
    pub fn chat_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.chat_data = Some(val.into());
        self
    }

    /// Data of the widget, which is specific to the message
    #[must_use]
    pub fn chat_data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.chat_data = val.map(Into::into);
        self
    }
}
