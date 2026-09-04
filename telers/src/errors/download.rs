//! This module contains the [`ErrorKind`] enum, which represents errors that can occur when downloading a file
//! with [`Bot::download`](crate::Bot::download) and related methods.

use super::SessionErrorKind;

use anyhow;
use std::io;
use thiserror;

/// Errors that can occur when downloading a file
/// with [`Bot::download`](crate::Bot::download) and related methods.
#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error(transparent)]
    GetFile(#[from] SessionErrorKind),
    #[error(
        "File path is missing in the `getFile` response, the file is probably too big to be \
         downloaded"
    )]
    MissingFilePath,
    /// The file path cannot be resolved to a local path in [`local mode`](https://core.telegram.org/bots/api#using-a-local-bot-api-server)
    /// by [`FilesPathWrapper`](crate::client::telegram::FilesPathWrapper) of the API server
    #[error("Can't resolve local path for the file path {file_path:?}")]
    LocalPath { file_path: Box<str> },
    #[error(transparent)]
    Client(#[from] anyhow::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}
