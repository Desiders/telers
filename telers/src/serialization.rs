//! Serialization types and JSON operations for the selected backend.
//!
//! `deser` selects Deser for the entire library, including FSM storage.
//! Without it, the default `serde` feature selects Serde.
//! Custom types use the selected traits; Deser requires `Sync` for serialization
//! and `Send` for deserialization.

#[cfg(all(not(feature = "deser"), not(feature = "serde")))]
compile_error!("enable either the `serde` or `deser` serialization feature");

#[cfg(feature = "deser")]
pub use deser::{de::DeserializeOwned, Deserialize, Error, Serialize};
#[cfg(feature = "deser")]
pub use deser_json::{from_str, to_string};
#[cfg(feature = "deser")]
pub use deser_value::{to_value, value as json, Value};

#[cfg(not(feature = "deser"))]
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
#[cfg(not(feature = "deser"))]
pub use serde_json::{
    from_str, from_value, json, to_string, to_string_pretty, to_value, Error, Value,
};

/// Deserialize an owned JSON value with the selected backend.
///
/// # Errors
/// Returns an error if the value does not match `T`.
#[cfg(feature = "deser")]
#[allow(
    clippy::needless_pass_by_value,
    reason = "Match the owned-value API of the default backend"
)]
pub fn from_value<T: DeserializeOwned>(value: Value) -> Result<T, Error> {
    deser_value::from_value(&value)
}

/// Serialize JSON with indentation using the selected backend.
///
/// # Errors
/// Returns an error if the value cannot be represented as JSON.
#[cfg(feature = "deser")]
pub fn to_string_pretty<T: Serialize + ?Sized>(value: &T) -> Result<String, Error> {
    deser_json::SerializerConfig::new()
        .pretty(deser_json::Indent::Spaces(2))
        .to_string(&value)
}

/// Borrow a JSON array from a value produced by the selected backend.
#[must_use]
pub fn as_array(value: &Value) -> Option<&[Value]> {
    #[cfg(feature = "deser")]
    {
        value.as_seq().map(|values| values.as_slice())
    }
    #[cfg(not(feature = "deser"))]
    {
        value.as_array().map(Vec::as_slice)
    }
}
