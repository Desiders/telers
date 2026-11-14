/// Use this method to edit public data of a shared widget created by the bot. A shared widget can be edited at most once per second. On success, `true` is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#editsharedwidgetdata>
/// # Returns
/// - `bool`
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize))]
pub struct EditSharedWidgetData {
    /// Unique identifier of the shared widget
    pub widget_id: Box<str>,
    /// New public data of the widget, 0-1024 bytes; pass an empty string to remove the public data
    pub public_data: Box<str>,
    /// The maximum amount of time in seconds that the public data of the widget may be cached client-side; 5-3600. If omitted, then the current value is kept, which is 30 by default.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub cache_time: Option<u16>,
}
impl EditSharedWidgetData {
    /// Creates a new `EditSharedWidgetData`.
    ///
    /// # Arguments
    /// * `widget_id` - Unique identifier of the shared widget
    /// * `public_data` - New public data of the widget, 0-1024 bytes; pass an empty string to remove the public data
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<Box<str>>>(widget_id: T0, public_data: T1) -> Self {
        Self {
            widget_id: widget_id.into(),
            public_data: public_data.into(),
            cache_time: None,
        }
    }

    /// Unique identifier of the shared widget
    #[must_use]
    pub fn widget_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.widget_id = val.into();
        self
    }

    /// New public data of the widget, 0-1024 bytes; pass an empty string to remove the public data
    #[must_use]
    pub fn public_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.public_data = val.into();
        self
    }

    /// The maximum amount of time in seconds that the public data of the widget may be cached client-side; 5-3600. If omitted, then the current value is kept, which is 30 by default.
    #[must_use]
    pub fn cache_time<T: Into<u16>>(mut self, val: T) -> Self {
        self.cache_time = Some(val.into());
        self
    }

    /// The maximum amount of time in seconds that the public data of the widget may be cached client-side; 5-3600. If omitted, then the current value is kept, which is 30 by default.
    #[must_use]
    pub fn cache_time_option<T: Into<u16>>(mut self, val: Option<T>) -> Self {
        self.cache_time = val.map(Into::into);
        self
    }
}
impl super::TelegramMethod for EditSharedWidgetData {
    type Method = Self;
    type Return = bool;

    fn build_request(self) -> super::Request<Self::Method> {
        super::Request::new("editSharedWidgetData", self, None)
    }
}
