use anyhow;
use std::{borrow::Cow, convert::Infallible, fmt::Debug};
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error("Extract error: {0}")]
    ExtractError(Cow<'static, str>),
    #[error("Update type error: {0}")]
    UpdateTypeError(String),
    #[error(transparent)]
    UserError(#[from] anyhow::Error),
}

impl From<Infallible> for ErrorKind {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}
