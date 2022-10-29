use std::{
    convert::Infallible,
    error::Error as StdError,
    fmt::{self, Debug, Display, Formatter},
};

#[allow(clippy::module_name_repetitions)]
pub trait AppError: StdError {}

impl AppError for Infallible {}

pub struct Error {
    cause: Box<dyn AppError>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.cause, f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.cause, f)
    }
}

impl StdError for Error {}

#[derive(Debug)]
pub struct ExtractError {
    pub message: String,
}

impl Display for ExtractError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ExtractError: {}", self.message)
    }
}

impl StdError for ExtractError {}

impl AppError for ExtractError {}

impl<T: AppError + 'static> From<T> for Error {
    fn from(err: T) -> Error {
        Error {
            cause: Box::new(err),
        }
    }
}
