use std::{borrow::Cow, fmt::Debug};
use thiserror;

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
