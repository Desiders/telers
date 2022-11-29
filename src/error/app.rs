use std::{
    convert::Infallible,
    error::Error as StdError,
    fmt::{self, Debug, Display, Formatter},
};

/// Base error type for framework errors
#[allow(clippy::module_name_repetitions)]
pub trait AppError: StdError + Send + Sync {
    #[must_use]
    fn message(&self) -> &str;
}

impl AppError for Infallible {
    fn message(&self) -> &str {
        unimplemented!("Infallible cannot have a message and should never be called")
    }
}

/// Error wrapper for [`AppError`]
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

impl Error {
    #[must_use]
    pub fn cause(&self) -> &dyn AppError {
        self.cause.as_ref()
    }
}

impl StdError for Error {}

#[derive(Debug)]
pub struct ExtractError {
    pub message: String,
}

impl ExtractError {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for ExtractError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "ExtractError: {}", self.message)
    }
}

impl StdError for ExtractError {}

impl AppError for ExtractError {
    fn message(&self) -> &str {
        &self.message
    }
}

impl<T: AppError + 'static> From<T> for Error {
    fn from(err: T) -> Error {
        Error {
            cause: Box::new(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_error() {
        let err = ExtractError::new("test".to_string());
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_error() {
        let err = Error::from(ExtractError::new("test".to_string()));
        assert_eq!(err.cause().message(), "test");
    }
}
