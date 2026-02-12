use bytes::{Bytes, BytesMut};
use futures_util::{Stream, TryFutureExt as _, TryStreamExt as _};
use serde::{Serialize, Serializer};
use std::{
    ffi::OsStr,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    io, mem,
    path::PathBuf,
};
use tokio_util::codec::{BytesCodec, FramedRead};
use uuid::Uuid;

const ATTACH_PREFIX: &str = "attach://";

pub const DEFAULT_CAPACITY: usize = 64 * 1024; // 64 KiB

/// This object represents the contents of a file to be uploaded.
/// # Notes
/// You can use instead of [`InputFile`] any type that implements [`Into<InputFile>`]:
/// - [`FileId`] (for example `FileId::new(file_id)`)
/// - [`UrlFile`] (for example `UrlFile::new(url)`)
/// - [`FSFile`] (for example `FSFile::new(path)`)
/// - [`BufferedFile`] (for example `BufferedFile::new(bytes)`)
/// - [`StreamFile`] (for example `StreamFile::new(stream)`)
/// # Documentation
/// <https://core.telegram.org/bots/api#inputfile>
#[derive(Debug, Hash, PartialEq)]
pub enum InputFile {
    Id(FileId),
    Url(UrlFile),
    FS(FSFile),
    Buffered(BufferedFile),
    Stream(StreamFile),
}

impl InputFile {
    /// Creates a new [`InputFile`] with [`FileId`]
    #[must_use]
    pub fn id(id: impl Into<String>) -> Self {
        Self::Id(FileId::new(id))
    }

    /// Creates a new [`InputFile`] with [`UrlFile`]
    #[must_use]
    pub fn url(url: impl Into<String>) -> Self {
        Self::Url(UrlFile::new(url))
    }

    /// Creates a new [`InputFile`] with [`FSFile`]
    #[must_use]
    pub fn fs(path: impl Into<PathBuf>) -> Self {
        Self::FS(FSFile::new(path))
    }

    /// Creates a new [`InputFile`] with [`FSFile`] and specified filename
    #[must_use]
    pub fn fs_with_name(path: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        Self::FS(FSFile::new_with_name(path, name))
    }

    /// Creates a new [`InputFile`] with [`BufferedFile`]
    #[must_use]
    pub fn buffered(bytes: impl Into<Bytes>) -> Self {
        Self::Buffered(BufferedFile::new(bytes))
    }

    /// Creates a new [`InputFile`] with [`BufferedFile`] and specified filename
    #[must_use]
    pub fn buffered_with_name(bytes: impl Into<Bytes>, name: impl Into<String>) -> Self {
        Self::Buffered(BufferedFile::new_with_name(bytes, name))
    }

    /// Creates a new [`InputFile`] with [`StreamFile`]
    /// # Warning
    /// If stream is taken, default client implementation raises an error,
    /// so you need to use [`StreamFile::set_stream`] to set stream again.
    /// This need because the stream can't be restored after it was taken.
    ///
    /// Check [`StreamFile::take_stream`] and [`StreamFile::set_stream`] for more information.
    #[must_use]
    pub fn stream(
        stream: impl Stream<Item = Result<Bytes, io::Error>> + Unpin + Send + Sync + 'static,
    ) -> Self {
        Self::Stream(StreamFile::new(stream))
    }

    /// Creates a new [`InputFile`] with [`StreamFile`] and specified filename
    #[must_use]
    pub fn stream_with_name(
        stream: impl Stream<Item = Result<Bytes, io::Error>> + Unpin + Send + Sync + 'static,
        name: impl Into<String>,
    ) -> Self {
        Self::Stream(StreamFile::new_with_name(stream, name))
    }
}

impl InputFile {
    /// Some variants can be uploaded in `multipart/form-data` format,
    /// others can be uploaded as URL or path (depends on [`InputFile`]).
    /// If the file in `multipart/form-data` format,
    /// then `str_to_file` will indicate "path" to data in form (because `multipart/form-data` format),
    /// otherwise it will be just string, which itself indicate "path" to data (because URL and telegram file id).
    /// # Returns
    /// If this file should be uploaded in `multipart/form-data` format, returns `attach://{id}`.
    /// Otherwise returns string as URL or path (depends on [`InputFile`]).
    #[must_use]
    pub fn str_to_file(&self) -> &str {
        match self {
            Self::Id(file) => file.str_to_file(),
            Self::Url(file) => file.str_to_file(),
            Self::FS(file) => file.str_to_file(),
            Self::Buffered(file) => file.str_to_file(),
            Self::Stream(file) => file.str_to_file(),
        }
    }

    /// Some variants can be uploaded in `multipart/form-data` format,
    /// others can be uploaded as URL or path (depends on [`InputFile`]).
    /// # Returns
    /// If this file should be uploaded in `multipart/form-data` format, returns `true`.
    /// Otherwise returns `false` and file [`InputFile`] may be uploaded in any way (URL and telegram file id).
    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        match self {
            Self::Id(file) => file.is_require_multipart(),
            Self::Url(file) => file.is_require_multipart(),
            Self::FS(file) => file.is_require_multipart(),
            Self::Buffered(file) => file.is_require_multipart(),
            Self::Stream(file) => file.is_require_multipart(),
        }
    }

    #[must_use]
    pub fn take(&mut self) -> Self {
        match self {
            Self::Id(file) => file.take().into(),
            Self::Url(file) => file.take().into(),
            Self::FS(file) => file.take().into(),
            Self::Buffered(file) => file.take().into(),
            Self::Stream(file) => file.take().into(),
        }
    }
}

impl Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.str_to_file().serialize(serializer)
    }
}

impl From<FileId> for InputFile {
    fn from(file: FileId) -> Self {
        Self::Id(file)
    }
}

impl From<UrlFile> for InputFile {
    fn from(file: UrlFile) -> Self {
        Self::Url(file)
    }
}

impl From<FSFile> for InputFile {
    fn from(file: FSFile) -> Self {
        Self::FS(file)
    }
}

impl From<BufferedFile> for InputFile {
    fn from(file: BufferedFile) -> Self {
        Self::Buffered(file)
    }
}

impl From<StreamFile> for InputFile {
    fn from(file: StreamFile) -> Self {
        Self::Stream(file)
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct FileId {
    pub(crate) id: String,
}

impl FileId {
    #[must_use]
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        false
    }

    #[must_use]
    pub fn take(&mut self) -> Self {
        Self {
            id: self.id.clone(),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct UrlFile {
    pub(crate) url: String,
}

impl UrlFile {
    #[must_use]
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }

    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.url
    }

    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        false
    }

    #[must_use]
    pub fn take(&mut self) -> Self {
        Self {
            url: self.url.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct FSFile {
    pub(crate) id: Uuid,
    file_name: Option<String>,
    pub(crate) path: PathBuf,
    pub(crate) str_to_file: String,
}

impl FSFile {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let id = Uuid::new_v4();
        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: None,
            path: path.into(),
            str_to_file,
        }
    }

    #[must_use]
    pub fn new_with_name(path: impl Into<PathBuf>, name: impl Into<String>) -> Self {
        let id = Uuid::new_v4();
        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: Some(name.into()),
            path: path.into(),
            str_to_file,
        }
    }

    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        true
    }

    #[must_use]
    pub const fn id(&self) -> &Uuid {
        &self.id
    }

    /// Gets passed filename or filename by path
    /// # Returns
    /// If filename is not passed and filename by path is not valid Unicode, returns `None`
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.file_name
            .as_deref()
            .or(self.path.file_name().and_then(OsStr::to_str))
    }

    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.str_to_file
    }

    #[must_use]
    pub fn take(&mut self) -> Self {
        Self {
            id: mem::take(&mut self.id),
            file_name: mem::take(&mut self.file_name),
            path: mem::take(&mut self.path),
            str_to_file: self.str_to_file.clone(),
        }
    }
}

impl FSFile {
    /// Opens a file and returns a stream of its bytes with a specified capacity for the underlying buffer
    /// # Errors
    /// Returns an error if the file cannot be opened or read
    pub fn stream_with_capacity(
        self,
        capacity: usize,
    ) -> impl Stream<Item = Result<Bytes, io::Error>> {
        tokio::fs::File::open(self.path)
            .map_ok(move |file| {
                FramedRead::with_capacity(file, BytesCodec::new(), capacity)
                    .map_ok(BytesMut::freeze)
            })
            .try_flatten_stream()
    }

    /// Opens a file and returns a stream of its bytes with a default capacity for the underlying buffer
    /// # Notes
    /// If you want to specify the capacity, use `stream_with_capacity` method instead.
    ///
    /// The default capacity is [`DEFAULT_CAPACITY`].
    /// # Errors
    /// Returns an error if the file cannot be opened or read
    pub fn stream(self) -> impl Stream<Item = Result<Bytes, io::Error>> {
        self.stream_with_capacity(DEFAULT_CAPACITY)
    }
}

impl Hash for FSFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(PartialEq, Eq)]
pub struct BufferedFile {
    pub(crate) id: Uuid,
    pub(crate) bytes: Bytes,
    pub(crate) file_name: Option<String>,
    pub(crate) str_to_file: String,
}

impl BufferedFile {
    #[must_use]
    pub fn new(bytes: impl Into<Bytes>) -> Self {
        let id = Uuid::new_v4();
        let bytes = bytes.into();

        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            bytes,
            file_name: None,
            str_to_file,
        }
    }

    /// Creates a new [`BufferedFile`] with specified filename
    #[must_use]
    pub fn new_with_name(bytes: impl Into<Bytes>, name: impl Into<String>) -> Self {
        let id = Uuid::new_v4();
        let bytes = bytes.into();

        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            bytes,
            file_name: Some(name.into()),
            str_to_file,
        }
    }

    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        true
    }

    #[must_use]
    pub const fn id(&self) -> &Uuid {
        &self.id
    }

    /// Gets string to file as path in format `attach://{id}`
    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.str_to_file
    }

    #[must_use]
    pub fn take(&mut self) -> Self {
        Self {
            id: mem::take(&mut self.id),
            bytes: mem::take(&mut self.bytes),
            file_name: mem::take(&mut self.file_name),
            str_to_file: self.str_to_file.clone(),
        }
    }
}

impl Debug for BufferedFile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("BufferedFile")
            .field("id", &self.id)
            .field("file_name", &self.file_name)
            .field("str_to_file", &self.str_to_file)
            .finish()
    }
}

impl Hash for BufferedFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub struct StreamFile {
    pub(crate) id: Uuid,
    pub(crate) file_name: Option<String>,
    pub(crate) stream:
        Option<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin>>,
    pub(crate) str_to_file: String,
}

impl StreamFile {
    #[must_use]
    pub fn new(
        stream: impl Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin + 'static,
    ) -> Self {
        let id = Uuid::new_v4();

        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: None,
            stream: Some(Box::new(stream)),
            str_to_file,
        }
    }

    #[must_use]
    pub fn new_with_name(
        stream: impl Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin + 'static,
        name: impl Into<String>,
    ) -> Self {
        let id = Uuid::new_v4();
        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: Some(name.into()),
            stream: Some(Box::new(stream)),
            str_to_file,
        }
    }

    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        true
    }

    #[must_use]
    pub const fn id(&self) -> &Uuid {
        &self.id
    }

    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.str_to_file
    }

    #[must_use]
    pub fn take(&mut self) -> Self {
        Self {
            id: mem::take(&mut self.id),
            file_name: mem::take(&mut self.file_name),
            stream: mem::take(&mut self.stream),
            str_to_file: self.str_to_file.clone(),
        }
    }
}

impl Debug for StreamFile {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("StreamFile")
            .field("id", &self.id)
            .field("file_name", &self.file_name)
            .field("str_to_file", &self.str_to_file)
            .finish()
    }
}

impl Hash for StreamFile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for StreamFile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
