use std::borrow::Cow;
use thiserror;

/// Error that can occur when update type is unknown.
///
/// This error is need to possible handle incorrect update types or unsupported update types,
/// that can be added in the new versions of the Telegram Bot API.
#[derive(Debug, thiserror::Error)]
#[error("Unknown update type `{raw_type}`")]
pub struct UnknownUpdateType {
    raw_type: Cow<'static, str>,
}

impl UnknownUpdateType {
    pub fn new<T>(raw_type: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self {
            raw_type: raw_type.into(),
        }
    }
}
