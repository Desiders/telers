use std::{borrow::Cow, fmt::Debug};
use thiserror;

/// To possible handle incorrect update types or unsupported update types,
/// that can be added in the new versions of the Telegram API.
/// This is necessary to support the "old" API if the "new" API has released a new update type.
#[derive(thiserror::Error, Debug)]
#[error("Unknown update type error: {msg}")]
pub struct UnknownUpdateType {
    msg: Cow<'static, str>,
}

impl UnknownUpdateType {
    pub fn new<T: Into<Cow<'static, str>>>(msg: T) -> Self {
        Self { msg: msg.into() }
    }
}
