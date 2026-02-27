use crate::client::Bot;
use serde::Serialize;
/// Use this method to edit live location messages. A location can be edited until its `live_period` expires or editing is explicitly disabled by a call to stop[`MessageLiveLocation`]. On success, if the edited message is not an inline message, the edited Message is returned, otherwise `true` is returned.
/// # Documentation
/// <https://core.telegram.org/bots/api#editmessagelivelocation>
/// # Returns
/// - `crate::types::Message`
/// - `bool`
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageLiveLocation {
    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<Box<str>>,
    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<crate::types::ChatIdKind>,
    /// Required if `inline_message_id` is not specified. Identifier of the message to edit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,
    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<Box<str>>,
    /// Latitude of new location
    pub latitude: f64,
    /// Longitude of new location
    pub longitude: f64,
    /// New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current `live_period` by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then `live_period` remains unchanged
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i64>,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<u16>,
    /// The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<u32>,
    /// A JSON-serialized object for a new inline keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::types::InlineKeyboardMarkup>,
}
impl EditMessageLiveLocation {
    /// Creates a new `EditMessageLiveLocation`.
    ///
    /// # Arguments
    /// * `latitude` - Latitude of new location
    /// * `longitude` - Longitude of new location
    ///
    /// # Notes
    /// Use builder methods to set optional fields.
    #[must_use]
    pub fn new<T0: Into<f64>, T1: Into<f64>>(latitude: T0, longitude: T1) -> Self {
        Self {
            business_connection_id: None,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            latitude: latitude.into(),
            longitude: longitude.into(),
            live_period: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[must_use]
    pub fn business_connection_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.business_connection_id = Some(val.into());
        this
    }

    /// Unique identifier of the business connection on behalf of which the message to be edited was sent
    #[must_use]
    pub fn business_connection_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.business_connection_id = val.map(Into::into);
        this
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id<T: Into<crate::types::ChatIdKind>>(self, val: T) -> Self {
        let mut this = self;
        this.chat_id = Some(val.into());
        this
    }

    /// Required if `inline_message_id` is not specified. Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    #[must_use]
    pub fn chat_id_option<T: Into<crate::types::ChatIdKind>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.chat_id = val.map(Into::into);
        this
    }

    /// Required if `inline_message_id` is not specified. Identifier of the message to edit
    #[must_use]
    pub fn message_id<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.message_id = Some(val.into());
        this
    }

    /// Required if `inline_message_id` is not specified. Identifier of the message to edit
    #[must_use]
    pub fn message_id_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.message_id = val.map(Into::into);
        this
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    #[must_use]
    pub fn inline_message_id<T: Into<Box<str>>>(self, val: T) -> Self {
        let mut this = self;
        this.inline_message_id = Some(val.into());
        this
    }

    /// Required if `chat_id` and `message_id` are not specified. Identifier of the inline message
    #[must_use]
    pub fn inline_message_id_option<T: Into<Box<str>>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.inline_message_id = val.map(Into::into);
        this
    }

    /// Latitude of new location
    #[must_use]
    pub fn latitude<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.latitude = val.into();
        this
    }

    /// Longitude of new location
    #[must_use]
    pub fn longitude<T: Into<f64>>(self, val: T) -> Self {
        let mut this = self;
        this.longitude = val.into();
        this
    }

    /// New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current `live_period` by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then `live_period` remains unchanged
    #[must_use]
    pub fn live_period<T: Into<i64>>(self, val: T) -> Self {
        let mut this = self;
        this.live_period = Some(val.into());
        this
    }

    /// New period in seconds during which the location can be updated, starting from the message send date. If 0x7FFFFFFF is specified, then the location can be updated forever. Otherwise, the new value must not exceed the current `live_period` by more than a day, and the live location expiration date must remain within the next 90 days. If not specified, then `live_period` remains unchanged
    #[must_use]
    pub fn live_period_option<T: Into<i64>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.live_period = val.map(Into::into);
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

    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading<T: Into<u16>>(self, val: T) -> Self {
        let mut this = self;
        this.heading = Some(val.into());
        this
    }

    /// Direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    #[must_use]
    pub fn heading_option<T: Into<u16>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.heading = val.map(Into::into);
        this
    }

    /// The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius<T: Into<u32>>(self, val: T) -> Self {
        let mut this = self;
        this.proximity_alert_radius = Some(val.into());
        this
    }

    /// The maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    #[must_use]
    pub fn proximity_alert_radius_option<T: Into<u32>>(self, val: Option<T>) -> Self {
        let mut this = self;
        this.proximity_alert_radius = val.map(Into::into);
        this
    }

    /// A JSON-serialized object for a new inline keyboard.
    #[must_use]
    pub fn reply_markup<T: Into<crate::types::InlineKeyboardMarkup>>(self, val: T) -> Self {
        let mut this = self;
        this.reply_markup = Some(val.into());
        this
    }

    /// A JSON-serialized object for a new inline keyboard.
    #[must_use]
    pub fn reply_markup_option<T: Into<crate::types::InlineKeyboardMarkup>>(
        self,
        val: Option<T>,
    ) -> Self {
        let mut this = self;
        this.reply_markup = val.map(Into::into);
        this
    }
}
impl super::TelegramMethod for EditMessageLiveLocation {
    type Method = Self;
    type Return = crate::Either<crate::types::Message, bool>;

    fn build_request<Client>(self, _bot: &Bot<Client>) -> super::Request<Self::Method> {
        super::Request::new("editMessageLiveLocation", self, None)
    }
}
