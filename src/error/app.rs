use super::ExtractionError;

use anyhow;
use std::fmt::Debug;
use thiserror;

/// This enum represents all possible errors that can be returned by the library in processing updates.
///
/// All user errors are wrapped in the [`ErrorKind::User`] variant for possible to determine type of errors
/// in higher levels of the application. \
/// For example, [`super::EventError`], which is used to represent
/// all possible errors that can be returned from handlers, converts to [`ErrorKind::User`].
/// So, if you want to handle user errors, you should match [`ErrorKind::User`] variant in middlewares,
/// but if you want to handle extraction errors, you should match [`ErrorKind::Extraction`] variant the same.
#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error(transparent)]
    Extraction(#[from] ExtractionError),
    #[error(transparent)]
    User(#[from] anyhow::Error),
}
