use std::{borrow::Cow, fmt::Debug};
use thiserror;

/// Error that can occur when update type is unknown.
///
/// This error is need to possible handle incorrect update types or unsupported update types,
/// that can be added in the new versions of the Telegram Bot API.
#[derive(thiserror::Error, Debug)]
#[error("Unknown update type error: {msg}")]
pub struct UnknownUpdateType {
    msg: Cow<'static, str>,
}

impl UnknownUpdateType {
    pub fn new<T>(msg: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self { msg: msg.into() }
    }
}
