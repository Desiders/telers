use serde::{Deserialize, Serialize};
/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the venue.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultvenue>
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: Box<str>,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: Box<str>,
    /// Address of the venue
    pub address: Box<str>,
    /// Foursquare identifier of the venue if known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<Box<str>>,
    /// Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<Box<str>>,
    /// Google Places identifier of the venue
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<Box<str>>,
    /// Google Places type of the venue. (See supported types.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<Box<str>>,
    /// Inline keyboard attached to the message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the venue
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
impl InlineQueryResultVenue {
    /// Creates a new `InlineQueryResultVenue`.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this result, 1-64 Bytes
    /// * `latitude` - Latitude of the venue location in degrees
    /// * `longitude` - Longitude of the venue location in degrees
    /// * `title` - Title of the venue
    /// * `address` - Address of the venue
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<
        T0: Into<Box<str>>,
        T1: Into<f64>,
        T2: Into<f64>,
        T3: Into<Box<str>>,
        T4: Into<Box<str>>,
    >(
        id: T0,
        latitude: T1,
        longitude: T2,
        title: T3,
        address: T4,
    ) -> Self {
        Self {
            id: id.into(),
            latitude: latitude.into(),
            longitude: longitude.into(),
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    /// Unique identifier for this result, 1-64 Bytes
    #[must_use]
    pub fn id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.id = val.into();
        self
    }

    /// Latitude of the venue location in degrees
    #[must_use]
    pub fn latitude<T: Into<f64>>(mut self, val: T) -> Self {
        self.latitude = val.into();
        self
    }

    /// Longitude of the venue location in degrees
    #[must_use]
    pub fn longitude<T: Into<f64>>(mut self, val: T) -> Self {
        self.longitude = val.into();
        self
    }

    /// Title of the venue
    #[must_use]
    pub fn title<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.title = val.into();
        self
    }

    /// Address of the venue
    #[must_use]
    pub fn address<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.address = val.into();
        self
    }

    /// Foursquare identifier of the venue if known
    #[must_use]
    pub fn foursquare_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.foursquare_id = Some(val.into());
        self
    }

    /// Foursquare identifier of the venue if known
    #[must_use]
    pub fn foursquare_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.foursquare_id = val.map(Into::into);
        self
    }

    /// Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[must_use]
    pub fn foursquare_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.foursquare_type = Some(val.into());
        self
    }

    /// Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    #[must_use]
    pub fn foursquare_type_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.foursquare_type = val.map(Into::into);
        self
    }

    /// Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.google_place_id = Some(val.into());
        self
    }

    /// Google Places identifier of the venue
    #[must_use]
    pub fn google_place_id_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.google_place_id = val.map(Into::into);
        self
    }

    /// Google Places type of the venue. (See supported types.)
    #[must_use]
    pub fn google_place_type<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.google_place_type = Some(val.into());
        self
    }

    /// Google Places type of the venue. (See supported types.)
    #[must_use]
    pub fn google_place_type_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.google_place_type = val.map(Into::into);
        self
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }

    /// Inline keyboard attached to the message
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.reply_markup = val.map(Into::into);
        self
    }

    /// Content of the message to be sent instead of the venue
    #[must_use]
    pub fn input_message_content<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: T,
    ) -> Self {
        self.input_message_content = Some(val.into());
        self
    }

    /// Content of the message to be sent instead of the venue
    #[must_use]
    pub fn input_message_content_option<T: Into<crate::types::InputMessageContent>>(
        mut self,
        val: Option<T>,
    ) -> Self {
        self.input_message_content = val.map(Into::into);
        self
    }

    /// Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url<T: Into<Box<str>>>(mut self, val: T) -> Self {
        self.thumbnail_url = Some(val.into());
        self
    }

    /// Url of the thumbnail for the result
    #[must_use]
    pub fn thumbnail_url_option<T: Into<Box<str>>>(mut self, val: Option<T>) -> Self {
        self.thumbnail_url = val.map(Into::into);
        self
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width<T: Into<i64>>(mut self, val: T) -> Self {
        self.thumbnail_width = Some(val.into());
        self
    }

    /// Thumbnail width
    #[must_use]
    pub fn thumbnail_width_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.thumbnail_width = val.map(Into::into);
        self
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height<T: Into<i64>>(mut self, val: T) -> Self {
        self.thumbnail_height = Some(val.into());
        self
    }

    /// Thumbnail height
    #[must_use]
    pub fn thumbnail_height_option<T: Into<i64>>(mut self, val: Option<T>) -> Self {
        self.thumbnail_height = val.map(Into::into);
        self
    }
}
