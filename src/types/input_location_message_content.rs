use serde::{Deserialize, Serialize};

/// Represents the `content <https://core.telegram.org/bots/api#inputmessagecontent>` of a location message to be sent as the result of an inline query.
/// <https://core.telegram.org/bots/api#inputlocationmessagecontent>
#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputLocationMessageContent {
    /// Latitude of the location in degrees
    pub latitude: f64,
    /// Longitude of the location in degrees
    pub longitude: f64,
    /// *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// *Optional*. Period in seconds for which the location can be updated, should be between 60 and 86400.
    pub live_period: Option<i64>,
    /// *Optional*. For live locations, a direction in which the user is moving, in degrees. Must be between 1 and 360 if specified.
    pub heading: Option<i64>,
    /// *Optional*. For live locations, a maximum distance for proximity alerts about approaching another chat member, in meters. Must be between 1 and 100000 if specified.
    pub proximity_alert_radius: Option<i64>,
}
