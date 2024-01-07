use super::base::{Request, TelegramMethod};

use crate::{
    client::Bot,
    types::{ChatIdKind, Message, ReplyMarkup, ReplyParameters},
};

use serde::Serialize;
use serde_with::skip_serializing_none;

/// Use this method to send information about a venue.
/// # Documentation
/// <https://core.telegram.org/bots/api#sendvenue>
/// # Returns
/// On success, the sent [`Message`] is returned
#[skip_serializing_none]
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SendVenue {
    /// Unique identifier for the target chat or username of the target channel (in the format `@channelusername`)
    pub chat_id: ChatIdKind,
    /// Unique identifier for the target message thread (topic) of the forum; for forum supergroups only
    pub message_thread_id: Option<i64>,
    /// Latitude of the venue
    pub longitude: f64,
    /// Latitude of the venue
    pub latitude: f64,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue, if known. (For example, `arts_entertainment/default`, `arts_entertainment/aquarium` or `food/icecream`.)
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue. (See [supported types](https://developers.google.com/places/web-service/supported_types).)
    pub google_place_type: Option<String>,
    /// Sends the message [silently](https://telegram.org/blog/channels-2-0#silent-messages). Users will receive a notification with no sound
    pub disable_notification: Option<bool>,
    /// Protects the contents of the sent message from forwarding and saving
    pub protect_content: Option<bool>,
    /// Description of the message to reply to
    pub reply_parameters: Option<ReplyParameters>,
    /// Additional interface options. A JSON-serialized object for an [inline keyboard](https://core.telegram.org/bots/features#inline-keyboards), [custom reply keyboard](https://core.telegram.org/bots/features#keyboards), instructions to remove reply keyboard or to force a reply from the user.
    pub reply_markup: Option<ReplyMarkup>,
}

impl SendVenue {
    #[must_use]
    pub fn new(
        chat_id: impl Into<ChatIdKind>,
        longitude: f64,
        latitude: f64,
        title: impl Into<String>,
        address: impl Into<String>,
    ) -> Self {
        Self {
            chat_id: chat_id.into(),
            message_thread_id: None,
            longitude,
            latitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            disable_notification: None,
            protect_content: None,
            reply_parameters: None,
            reply_markup: None,
        }
    }

    #[must_use]
    pub fn chat_id(self, val: impl Into<ChatIdKind>) -> Self {
        Self {
            chat_id: val.into(),
            ..self
        }
    }

    #[must_use]
    pub fn message_thread_id(self, val: i64) -> Self {
        Self {
            message_thread_id: Some(val),
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
    pub fn latitude(self, val: f64) -> Self {
        Self {
            latitude: val,
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
    pub fn disable_notification(self, val: bool) -> Self {
        Self {
            disable_notification: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn protect_content(self, val: bool) -> Self {
        Self {
            protect_content: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn reply_parameters(self, val: ReplyParameters) -> Self {
        Self {
            reply_parameters: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup(self, val: impl Into<ReplyMarkup>) -> Self {
        Self {
            reply_markup: Some(val.into()),
            ..self
        }
    }
}

impl SendVenue {
    #[must_use]
    pub fn message_thread_id_option(self, val: Option<i64>) -> Self {
        Self {
            message_thread_id: val,
            ..self
        }
    }

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
    pub fn disable_notification_option(self, val: Option<bool>) -> Self {
        Self {
            disable_notification: val,
            ..self
        }
    }

    #[must_use]
    pub fn protect_content_option(self, val: Option<bool>) -> Self {
        Self {
            protect_content: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_parameters_option(self, val: Option<ReplyParameters>) -> Self {
        Self {
            reply_parameters: val,
            ..self
        }
    }

    #[must_use]
    pub fn reply_markup_option(self, val: Option<impl Into<ReplyMarkup>>) -> Self {
        Self {
            reply_markup: val.map(Into::into),
            ..self
        }
    }
}

impl TelegramMethod for SendVenue {
    type Method = Self;
    type Return = Message;

    fn build_request<Client>(&self, _bot: &Bot<Client>) -> Request<Self::Method> {
        Request::new("sendVenue", self, None)
    }
}

impl AsRef<SendVenue> for SendVenue {
    fn as_ref(&self) -> &Self {
        self
    }
}
