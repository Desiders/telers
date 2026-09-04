//! This module contains the [`FileDownload`] stream and the [`Bot::download`] family of methods for downloading files
//! from the Telegram Bot API file server.
//!
//! [`Bot::download`] accepts a file ID or any object that implements [`FileIdGetter`]
//! (for example [`PhotoSize`], [`Document`] or [`File`]), calls `getFile` to resolve the file path
//! and starts downloading the file. [`Bot::download_file`] does the same, but accepts an already known file path
//! (for example from a previous `getFile` call) and doesn't call `getFile` again.
//!
//! Both return a [`FileDownload`], which is a [`Stream`] of chunks of the file. You can consume it as a stream
//! or use one of the helpers: [`FileDownload::bytes`] to read the file into memory,
//! [`FileDownload::to_writer`] to write it to any [`AsyncWrite`] or [`FileDownload::to_path`] to save it to the file system.
//!
//! # Local mode
//!
//! If the API server is in [`local mode`](https://core.telegram.org/bots/api#using-a-local-bot-api-server),
//! files are read directly from the file system instead of being downloaded over HTTP.
//! The file path returned by `getFile` is resolved to a local path with the
//! [`FilesPathWrapper`] of the [`APIServer`].
//!
//! # Examples
//! ```rust
//! use telers::{errors::DownloadErrorKind, types::PhotoSize, Bot};
//!
//! async fn save_photo(bot: Bot, photo: &PhotoSize) -> Result<(), DownloadErrorKind> {
//!     // Save the file to the file system
//!     bot.download(photo).await?.to_path("photo.jpg").await?;
//!
//!     // Read the file into memory
//!     let bytes = bot.download(photo).await?.bytes().await?;
//!     println!("Downloaded {} bytes", bytes.len());
//!
//!     Ok(())
//! }
//!
//! async fn download_by_id(bot: Bot, file_id: &str) -> Result<(), DownloadErrorKind> {
//!     let bytes = bot.download(file_id).await?.bytes().await?;
//!     println!("Downloaded {} bytes", bytes.len());
//!
//!     Ok(())
//! }
//! ```
//!
//! [`PhotoSize`]: telers::types::PhotoSize
//! [`Document`]: telers::types::Document
//! [`File`]: telers::types::File
//! [`FilesPathWrapper`]: telers::client::telegram::FilesPathWrapper
//! [`APIServer`]: telers::client::telegram::APIServer

use super::Bot;

use crate::{
    client::session::base::{ClientStreamResponse, ContentStream, Session},
    errors::DownloadErrorKind,
    methods::GetFile,
    types::{FileIdGetter, DEFAULT_CAPACITY},
};

use bytes::{Bytes, BytesMut};
use futures_util::{Stream, TryStreamExt as _};
use secrecy::ExposeSecret as _;
use std::{
    fmt::{self, Debug, Formatter},
    io,
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncWrite, AsyncWriteExt as _};
use tokio_util::codec::{BytesCodec, FramedRead};
use tracing::{event, instrument, Level, Span};

type LocalStream = Pin<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send>>;

enum Inner {
    Remote(ContentStream),
    Local(LocalStream),
}

/// A file being downloaded from the Telegram Bot API file server (or read from the file system in local mode).
///
/// It's a [`Stream`] of chunks of the file. Use [`FileDownload::bytes`], [`FileDownload::to_writer`]
/// or [`FileDownload::to_path`] helpers to consume it, or consume it as a stream directly.
///
/// Check [module docs](crate::client::bot::download) for examples.
#[must_use = "the file isn't downloaded until the stream is consumed"]
pub struct FileDownload {
    inner: Inner,
}

impl FileDownload {
    fn remote(stream: ContentStream) -> Self {
        Self {
            inner: Inner::Remote(stream),
        }
    }

    fn local(stream: LocalStream) -> Self {
        Self {
            inner: Inner::Local(stream),
        }
    }

    /// Reads the whole file into memory
    /// # Errors
    /// If a chunk of the file cannot be read
    pub async fn bytes(mut self) -> Result<Bytes, DownloadErrorKind> {
        let mut buf = BytesMut::new();
        while let Some(chunk) = self.try_next().await? {
            buf.extend_from_slice(&chunk);
        }
        Ok(buf.freeze())
    }

    /// Writes the file to the given writer and flushes it
    /// # Errors
    /// - If a chunk of the file cannot be read
    /// - If the writer fails
    pub async fn to_writer<W>(mut self, writer: &mut W) -> Result<(), DownloadErrorKind>
    where
        W: AsyncWrite + Unpin + ?Sized,
    {
        while let Some(chunk) = self.try_next().await? {
            writer.write_all(&chunk).await?;
        }
        writer.flush().await?;
        Ok(())
    }

    /// Saves the file to the given path, creating or truncating the destination file
    /// # Notes
    /// Parent directories aren't created.
    /// # Errors
    /// - If a chunk of the file cannot be read
    /// - If the destination file cannot be created or written
    pub async fn to_path(self, path: impl AsRef<Path>) -> Result<(), DownloadErrorKind> {
        let mut file = tokio::fs::File::create(path).await?;
        self.to_writer(&mut file).await
    }
}

impl Stream for FileDownload {
    type Item = Result<Bytes, DownloadErrorKind>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut self.get_mut().inner {
            Inner::Remote(stream) => stream
                .as_mut()
                .poll_next(cx)
                .map_err(DownloadErrorKind::Client),
            Inner::Local(stream) => stream.as_mut().poll_next(cx).map_err(DownloadErrorKind::Io),
        }
    }
}

impl Debug for FileDownload {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let kind = match self.inner {
            Inner::Remote(_) => "Remote",
            Inner::Local(_) => "Local",
        };
        f.debug_struct("FileDownload")
            .field("kind", &kind)
            .finish_non_exhaustive()
    }
}

impl<Client: Session> Bot<Client> {
    /// Use this method to download a file by its ID or by an object that represents a file
    /// (for example [`PhotoSize`](crate::types::PhotoSize) or [`Document`](crate::types::Document)).
    ///
    /// It calls `getFile` to resolve the file path and starts downloading the file.
    /// If you already know the file path, use [`Bot::download_file`] instead.
    /// # Arguments
    /// * `file` - File ID or any object that implements [`FileIdGetter`]
    /// # Errors
    /// - If the `getFile` request fails
    /// - If `getFile` returns no file path (the file is too big to be downloaded)
    /// - If the download request cannot be sent or the file server returns an error
    /// - In local mode, if the file path cannot be resolved to a local path or the file cannot be opened
    /// # Notes
    /// This method uses default timeout of the client for requests.
    /// If you want to use custom timeout, use [`Bot::download_with_timeout`] method.
    ///
    /// Check [module docs](crate::client::bot::download) for examples.
    pub async fn download(
        &self,
        file: impl FileIdGetter,
    ) -> Result<FileDownload, DownloadErrorKind> {
        self.download_inner(file.file_id(), None).await
    }

    /// Use this method to download a file by its ID or by an object that represents a file with timeout
    /// # Arguments
    /// * `file` - File ID or any object that implements [`FileIdGetter`]
    /// * `request_timeout` - Timeout for the `getFile` request and for the download request
    /// # Errors
    /// Same as [`Bot::download`]
    /// # Notes
    /// This method uses passed timeout for requests.
    /// If you want to use default timeout, use [`Bot::download`] method.
    pub async fn download_with_timeout(
        &self,
        file: impl FileIdGetter,
        request_timeout: f32,
    ) -> Result<FileDownload, DownloadErrorKind> {
        self.download_inner(file.file_id(), Some(request_timeout))
            .await
    }

    /// Use this method to download a file by its path, which can be got from [`File`](crate::types::File)
    /// returned by `getFile`.
    ///
    /// Unlike [`Bot::download`], this method doesn't call `getFile`.
    /// # Arguments
    /// * `file_path` - File path on the Telegram Bot API server
    /// # Errors
    /// - If the download request cannot be sent or the file server returns an error
    /// - In local mode, if the file path cannot be resolved to a local path or the file cannot be opened
    /// # Notes
    /// This method uses default timeout of the client for requests.
    /// If you want to use custom timeout, use [`Bot::download_file_with_timeout`] method.
    pub async fn download_file(&self, file_path: &str) -> Result<FileDownload, DownloadErrorKind> {
        self.download_file_inner(file_path, None).await
    }

    /// Use this method to download a file by its path with timeout
    /// # Arguments
    /// * `file_path` - File path on the Telegram Bot API server
    /// * `request_timeout` - Timeout for the download request
    /// # Errors
    /// Same as [`Bot::download_file`]
    /// # Notes
    /// This method uses passed timeout for requests.
    /// If you want to use default timeout, use [`Bot::download_file`] method.
    pub async fn download_file_with_timeout(
        &self,
        file_path: &str,
        request_timeout: f32,
    ) -> Result<FileDownload, DownloadErrorKind> {
        self.download_file_inner(file_path, Some(request_timeout))
            .await
    }

    #[instrument(name = "download", skip_all, fields(file_id = file_id))]
    async fn download_inner(
        &self,
        file_id: &str,
        timeout: Option<f32>,
    ) -> Result<FileDownload, DownloadErrorKind> {
        let file = self
            .client
            .make_request_and_get_result(self, GetFile::new(file_id), timeout)
            .await?;
        let file_path = file.file_path.ok_or(DownloadErrorKind::MissingFilePath)?;

        self.download_file_inner(&file_path, timeout).await
    }

    #[instrument(name = "download_file", skip_all, fields(file_path = file_path, local))]
    async fn download_file_inner(
        &self,
        file_path: &str,
        timeout: Option<f32>,
    ) -> Result<FileDownload, DownloadErrorKind> {
        let api = self.client.api();

        Span::current().record("local", api.is_local());

        if api.is_local() {
            let local_path = api
                .files_path_wrapper()
                .to_local(Path::new(file_path))
                .ok_or_else(|| DownloadErrorKind::LocalPath {
                    file_path: file_path.into(),
                })?;

            let file = tokio::fs::File::open(&local_path)
                .await
                .inspect_err(|err| {
                    event!(
                        Level::ERROR,
                        error = %err,
                        path = %local_path.display(),
                        "Cannot open a local file",
                    );
                })?;
            let stream = FramedRead::with_capacity(file, BytesCodec::new(), DEFAULT_CAPACITY)
                .map_ok(BytesMut::freeze);

            return Ok(FileDownload::local(Box::pin(stream)));
        }

        let url = api.file_url(self.token().expose_secret(), file_path);
        let response = self.client.stream_content(&url, timeout).await?;

        check_stream_response(response).map(FileDownload::remote)
    }
}

/// Checks the status code of a download response and returns its content stream if it's successful
fn check_stream_response(
    response: ClientStreamResponse,
) -> Result<ContentStream, DownloadErrorKind> {
    let ClientStreamResponse {
        status_code,
        content,
    } = response;

    if status_code.is_success() {
        return Ok(content);
    }

    event!(
        Level::ERROR,
        %status_code,
        "Cannot download a file: error response",
    );

    Err(DownloadErrorKind::Client(anyhow::Error::msg(format!(
        "Cannot download a file: status code {status_code}"
    ))))
}
