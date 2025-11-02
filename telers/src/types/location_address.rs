use serde::Serialize;
use serde_with::skip_serializing_none;

/// Describes the physical address of a location.
/// # Documentation
/// <https://core.telegram.org/bots/api#locationaddress>
#[skip_serializing_none]
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize)]
pub struct LocationAddress {
    /// The two-letter ISO 3166-1 alpha-2 country code of the country where the location is located
    pub country_code: String,
    /// State of the location
    pub state: Option<String>,
    /// City of the location
    pub city: Option<String>,
    /// Street address of the location
    pub street: Option<String>,
}
