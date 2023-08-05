use anyhow;
use thiserror;

/// Error that can occur when processing an inner middleware
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error {
    #[from]
    source: anyhow::Error,
}

impl Error {
    pub fn new(err: impl Into<anyhow::Error>) -> Self {
        Self { source: err.into() }
    }
}
