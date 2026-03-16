use serde::{Deserialize, Serialize};
/// Represents a location on a map. By default, the location will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the location.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultlocation>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: Box<str>,
    /// Location latitude in degrees
    pub latitude: f64,
    /// Location longitude in degrees
    pub longitude: f64,
    /// Location title
    pub title: Box<str>,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<u32>,
    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<crate::types::InputMessageContent>,
    /// Url of the thumbnail for the result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<Box<str>>,
    /// Thumbnail width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail_height: Option<i64>,
}
impl InlineQueryResultLocation {
    /// Creates a new `InlineQueryResultLocation`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 Bytes
    /// * `latitude` - Location latitude in degrees
    /// * `longitude` - Location longitude in degrees
    /// * `title` - Location title
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<Box<str>>, T1: Into<f64>, T2: Into<f64>, T3: Into<Box<str>>>(
        id: T0,
        latitude: T1,
        longitude: T2,
        title: T3,
    ) -> Self {
        Self {
            id: id.into(),
            latitude: latitude.into(),
            longitude: longitude.into(),
            title: title.into(),
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    /// Unique identifier for this result, 1-64 Bytes
    #[must_use]
    pub fn id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.id = val.into();
        this
    }

    /// Location latitude in degrees
    #[must_use]
    pub fn latitude<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.latitude = val.into();
        this
    }

    /// Location longitude in degrees
    #[must_use]
    pub fn longitude<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.longitude = val.into();
        this
    }

    /// Location title
    #[must_use]
    pub fn title<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.title = val.into();
        this
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.horizontal_accuracy = Some(val.into());
        this
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[must_use]
    pub fn horizontal_accuracy_option<T: Into<f64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.horizontal_accuracy = val.map(Into::into);
        this
    }

    /// Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[must_use]
    pub fn live_period<T: Into<u32>>(self, val: T) -> Self {
        let mut this = self;
        this.live_period = Some(val.into());
        this
    }

    /// Period in seconds during which the location can be updated, should be between 60 and 86400, or 0x7FFFFFFF for live locations that can be edited indefinitely.
    #[must_use]
    pub fn live_period_option<T: Into<u32>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.live_period = val.map(Into::into);
        this
    }

    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading<T: Into<u16>>(self, val: T) -> Self {
        let mut this = self;
        this.heading = Some(val.into());
        this
    }

    /// For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading_option<T: Into<u16>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.heading = val.map(Into::into);
        this
    }

    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius<T: Into<u32>>(self, val: T) -> Self {
        let mut this = self;
        this.proximity_alert_radius = Some(val.into());
        this
    }

    /// For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius_option<T: Into<u32>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.proximity_alert_radius = val.map(Into::into);
        this
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_markup = Some(val.into());
        this
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_markup = val.map(Into::into);
        this
    }

    /// Content of the message to be sent instead of the location
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(self, val: T) -> Self {
        let mut this = self;
        this.input_message_content = Some(val.into());
        this
    }

    /// Content of the message to be sent instead of the location
    #[must_use]
    pub fn input_message_content_option<T: Into<crate::types::InputMessageContent>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.input_message_content = val.map(Into::into);
        this
    }

    /// Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail_url = Some(val.into());
        this
    }

    /// Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail_url = val.map(Into::into);
        this
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail_width = Some(val.into());
        this
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail_width = val.map(Into::into);
        this
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.thumbnail_height = Some(val.into());
        this
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.thumbnail_height = val.map(Into::into);
        this
    }
}
