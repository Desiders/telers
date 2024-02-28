use super::{InlineKeyboardMarkup, InputMessageContent};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

/// Represents a venue. By default, the venue will be sent by the user. Alternatively, you can use `input_message_content` to send a message with the specified content instead of the venue.
/// # Documentation
/// <https://core.telegram.org/bots/api#inlinequeryresultvenue>
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result, 1-64 Bytes
    pub id: String,
    /// Latitude of the venue location in degrees
    pub latitude: f64,
    /// Longitude of the venue location in degrees
    pub longitude: f64,
    /// Title of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue if known
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, 'arts_entertainment/default', 'arts_entertainment/aquarium' or 'food/icecream'.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See [`supported types`](https://developers.google.com/places/web-service/supported_types).)
    pub google_place_type: Option<String>,
    /// [`Inline keyboard`](https://core.telegram.org/bots/features#inline-keyboards) attached to the message
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// Content of the message to be sent instead of the venue
    pub input_message_content: Option<InputMessageContent>,
    /// Url of the thumbnail for the result
    pub thumbnail_url: Option<String>,
    /// Thumbnail width
    pub thumbnail_width: Option<i64>,
    /// Thumbnail height
    pub thumbnail_height: Option<i64>,
}

impl InlineQueryResultVenue {
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        latitude: f64,
        longitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            latitude,
            longitude,
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

    #[must_use]
    pub fn id(self, val: impl Into<String>) -> Self {
        Self {
            id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn latitude(self, val: f64) -> Self {
        Self {
            latitude: val,
            ..self
        }
    }

    #[must_use]
    pub fn longitude(self, val: f64) -> Self {
        Self {
            longitude: val,
            ..self
        }
    }

    #[must_use]
    pub fn title(self, val: impl Into<String>) -> Self {
        Self {
            title: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn address(self, val: impl Into<String>) -> Self {
        Self {
            address: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn foursquare_id(self, val: impl Into<String>) -> Self {
        Self {
            foursquare_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn foursquare_type(self, val: impl Into<String>) -> Self {
        Self {
            foursquare_type: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn google_place_id(self, val: impl Into<String>) -> Self {
        Self {
            google_place_id: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn google_place_type(self, val: impl Into<String>) -> Self {
        Self {
            google_place_type: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<InlineKeyboardMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn input_message_content(self, val: impl Into<InputMessageContent>) -> Self {
        Self {
            input_message_content: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_url(self, val: impl Into<String>) -> Self {
        Self {
            thumbnail_url: Some(val.into()),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_width(self, val: i64) -> Self {
        Self {
            thumbnail_width: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_height(self, val: i64) -> Self {
        Self {
            thumbnail_height: Some(val),
            ..self
        }
    }
}

impl InlineQueryResultVenue {
    #[must_use]
    pub fn foursquare_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            foursquare_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn foursquare_type_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            foursquare_type: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn google_place_id_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            google_place_id: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn google_place_type_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            google_place_type: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<InlineKeyboardMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn input_message_content_option(self, val: Option<impl Into<InputMessageContent>>) -> Self {
        Self {
            input_message_content: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_url_option(self, val: Option<impl Into<String>>) -> Self {
        Self {
            thumbnail_url: val.map(Into::into),
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_width_option(self, val: Option<i64>) -> Self {
        Self {
            thumbnail_width: val,
            ..self
        }
    }

    #[must_use]
    pub fn thumbnail_height_option(self, val: Option<i64>) -> Self {
        Self {
            thumbnail_height: val,
            ..self
        }
    }
}
