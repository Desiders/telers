use serde::{Deserialize, Serialize};

/// This object represents a point on the map.
/// <https://core.telegram.org/bots/api#location>_
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: f64,
    /// Latitude as defined by sender
    pub latitude: f64,
    /// *Optional*. The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,
    /// *Optional*. Time relative to the message sending date, during which the location can be updated; in seconds. For active live locations only.
    pub live_period: Option<i64>,
    /// *Optional*. The direction in which user is moving, in degrees; 1-360. For active live locations only.
    pub heading: Option<i64>,
    /// *Optional*. The maximum distance for proximity alerts about approaching another chat member, in meters. For sent live locations only."""
    pub proximity_alert_radius: Option<i64>,
}
