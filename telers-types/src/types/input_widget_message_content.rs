/// Represents the content of a message with an HTML widget to be sent as the result of an inline query. Currently, widgets can't be sent in response to inline queries in the production environment.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputwidgetmessagecontent>
#[derive(Clone, Debug)]
#[cfg_attr(feature = "deser", derive(deser::Serialize, deser::Deserialize))]
#[cfg_attr(not(feature = "deser"), derive(serde::Serialize, serde::Deserialize))]
pub struct InputWidgetMessageContent {
    /// HTML code of the widget, 1-32768 bytes. The code must not exceed 4096 bytes when compressed by zlib with the maximum compression level.
    pub html: Box<str>,
    /// Width of the widget in logical pixels; omit if the widget must occupy the whole width of the message bubble. The width is clamped by the app to the range from 200 to the maximum width of the message bubble.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub width: Option<i64>,
    /// Height of the widget in logical pixels; omit if the height of the widget isn't fixed. The height is clamped by the app to the range 50-600.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub height: Option<u16>,
    /// Public data of the widget, 0-1024 bytes, which will be available wherever the widget is shown. If the widget is shared, then all messages with it will have the same public data.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub public_data: Option<Box<str>>,
    /// Data of the widget, which is specific to the message, 0-1024 bytes
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub chat_data: Option<Box<str>>,
    /// Widget caption, 0-1024 characters after entities parsing
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub caption: Option<Box<str>>,
    /// Mode for parsing entities in the widget caption. See formatting options for more details.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub parse_mode: Option<Box<str>>,
    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub caption_entities: Option<Box<[crate::types::MessageEntity]>>,
    /// Pass `true` if the caption must be shown above the widget
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub show_caption_above_media: Option<bool>,
    /// Pass `true` to allow the widget to send queries to the bot
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub allow_queries: Option<bool>,
    /// Pass `true` to create a shared widget. Each message sent from the result creates a separate shared widget, whose identifier is received by the bot only in the field `widget_id` of queries from the widget. To send an existing shared widget in response to an inline query, use [`crate::types::InputSharedWidgetMessageContent`] instead.
    #[cfg_attr(not(feature = "deser"), serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "deser", deser(skip_serializing_if = Option::is_none))]
    pub is_shared: Option<bool>,
}
impl InputWidgetMessageContent {
    /// Creates a new `InputWidgetMessageContent`.
    ///
    /// # Arguments
    /// * `html` - HTML code of the widget, 1-32768 bytes. The code must not exceed 4096 bytes when compressed by zlib with the maximum compression level.
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>>(html: T0) -> Self {
        Self {
            html: html.into(),
            width: None,
            height: None,
            public_data: None,
            chat_data: None,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: None,
            allow_queries: None,
            is_shared: None,
        }
    }

    /// HTML code of the widget, 1-32768 bytes. The code must not exceed 4096 bytes when compressed by zlib with the maximum compression level.
    #[must_use]
    pub fn html<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.html = val.into();
        self
    }

    /// Width of the widget in logical pixels; omit if the widget must occupy the whole width of the message bubble. The width is clamped by the app to the range from 200 to the maximum width of the message bubble.
    #[must_use]
    pub fn width<T: Into<i64>>(mut self, val: T) -> Self {
        self.width = Some(val.into());
        self
    }

    /// Width of the widget in logical pixels; omit if the widget must occupy the whole width of the message bubble. The width is clamped by the app to the range from 200 to the maximum width of the message bubble.
    #[must_use]
    pub fn width_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.width = val.map(Into::into);
        self
    }

    /// Height of the widget in logical pixels; omit if the height of the widget isn't fixed. The height is clamped by the app to the range 50-600.
    #[must_use]
    pub fn height<T: Into<u16>>(mut self, val: T) -> Self {
        self.height = Some(val.into());
        self
    }

    /// Height of the widget in logical pixels; omit if the height of the widget isn't fixed. The height is clamped by the app to the range 50-600.
    #[must_use]
    pub fn height_option<T: Into<u16>>(mut self, val: Option<T>) -> Self {
        self.height = val.map(Into::into);
        self
    }

    /// Public data of the widget, 0-1024 bytes, which will be available wherever the widget is shown. If the widget is shared, then all messages with it will have the same public data.
    #[must_use]
    pub fn public_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.public_data = Some(val.into());
        self
    }

    /// Public data of the widget, 0-1024 bytes, which will be available wherever the widget is shown. If the widget is shared, then all messages with it will have the same public data.
    #[must_use]
    pub fn public_data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.public_data = val.map(Into::into);
        self
    }

    /// Data of the widget, which is specific to the message, 0-1024 bytes
    #[must_use]
    pub fn chat_data<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.chat_data = Some(val.into());
        self
    }

    /// Data of the widget, which is specific to the message, 0-1024 bytes
    #[must_use]
    pub fn chat_data_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.chat_data = val.map(Into::into);
        self
    }

    /// Widget caption, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.caption = Some(val.into());
        self
    }

    /// Widget caption, 0-1024 characters after entities parsing
    #[must_use]
    pub fn caption_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.caption = val.map(Into::into);
        self
    }

    /// Mode for parsing entities in the widget caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    /// Mode for parsing entities in the widget caption. See formatting options for more details.
    #[must_use]
    pub fn parse_mode_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.parse_mode = val.map(Into::into);
        self
    }

    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds multiple elements.
    #[must_use]
    pub fn caption_entities<T: Into<Box<[crate::types::MessageEntity]>>>(mut self, val: T) -> Self {
        self.caption_entities = Some(
            self.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(val.into())
                .collect(),
        );
        self
    }

    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entity<T: Into<crate::types::MessageEntity>>(mut self, val: T) -> Self {
        self.caption_entities = Some(
            self.caption_entities
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
        );
        self
    }

    /// List of special entities that appear in the caption, which can be specified instead of `parse_mode`
    ///
    /// # Notes
    /// Adds a single element.
    #[must_use]
    pub fn caption_entities_option<T: Into<Box<[crate::types::MessageEntity]>>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.caption_entities = val.map(Into::into);
        self
    }

    /// Pass `true` if the caption must be shown above the widget
    #[must_use]
    pub fn show_caption_above_media<T: Into<bool>>(mut self, val: T) -> Self {
        self.show_caption_above_media = Some(val.into());
        self
    }

    /// Pass `true` if the caption must be shown above the widget
    #[must_use]
    pub fn show_caption_above_media_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.show_caption_above_media = val.map(Into::into);
        self
    }

    /// Pass `true` to allow the widget to send queries to the bot
    #[must_use]
    pub fn allow_queries<T: Into<bool>>(mut self, val: T) -> Self {
        self.allow_queries = Some(val.into());
        self
    }

    /// Pass `true` to allow the widget to send queries to the bot
    #[must_use]
    pub fn allow_queries_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.allow_queries = val.map(Into::into);
        self
    }

    /// Pass `true` to create a shared widget. Each message sent from the result creates a separate shared widget, whose identifier is received by the bot only in the field `widget_id` of queries from the widget. To send an existing shared widget in response to an inline query, use [`crate::types::InputSharedWidgetMessageContent`] instead.
    #[must_use]
    pub fn is_shared<T: Into<bool>>(mut self, val: T) -> Self {
        self.is_shared = Some(val.into());
        self
    }

    /// Pass `true` to create a shared widget. Each message sent from the result creates a separate shared widget, whose identifier is received by the bot only in the field `widget_id` of queries from the widget. To send an existing shared widget in response to an inline query, use [`crate::types::InputSharedWidgetMessageContent`] instead.
    #[must_use]
    pub fn is_shared_option<T: Into<bool>>(mut self, val: Option<T>) -> Self {
        self.is_shared = val.map(Into::into);
        self
    }
}
