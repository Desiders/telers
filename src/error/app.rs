use super::ExtractionError;

use anyhow;
use std::fmt::Debug;
use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum ErrorKind {
    #[error(transparent)]
    Extraction(#[from] ExtractionError),
    #[error(transparent)]
    User(#[from] anyhow::Error),
}
