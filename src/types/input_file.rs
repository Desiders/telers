use bytes::{Bytes, BytesMut};
use futures::{Stream, TryFutureExt as _, TryStreamExt as _};
use serde::{Serialize, Serializer};
use std::{
    borrow::Cow,
    ffi::OsStr,
    io,
    path::{Path, PathBuf},
};
use tokio_util::codec::{BytesCodec, FramedRead};
use uuid::Uuid;

const ATTACH_PREFIX: &str = "attach://";

pub const DEFAULT_CAPACITY: usize = 64 * 1024; // 64 KiB

/// This object represents the contents of a file to be uploaded.
/// Must be posted using `multipart/form-data` in the usual way that files are uploaded via the browser.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputfile>
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct InputFile<'a>(FileKind<'a>);

impl<'a> InputFile<'a> {
    /// Creates a new [`InputFile`] with [`FileKind::Id`]
    #[must_use]
    pub fn id(id: impl Into<Cow<'a, str>>) -> Self {
        Self(FileKind::Id(FileId::new(id)))
    }

    /// Creates a new [`InputFile`] with [`FileKind::Url`]
    #[must_use]
    pub fn url(url: impl Into<Cow<'a, str>>) -> Self {
        Self(FileKind::Url(UrlFile::new(url)))
    }

    /// Creates a new [`InputFile`] with [`FileKind::FS`]
    #[must_use]
    pub fn fs(path: impl Into<PathBuf>, file_name: Option<&'a str>) -> Self {
        Self(FileKind::FS(FSFile::new(path, file_name)))
    }

    /// Creates a new [`InputFile`] with [`FileKind::Buffered`]
    #[must_use]
    pub fn buffered(bytes: impl Into<Bytes>, file_name: Option<&'a str>) -> Self {
        Self(FileKind::Buffered(BufferedFile::new(bytes, file_name)))
    }
}

impl<'a> InputFile<'a> {
    /// Some variants can be uploaded in `multipart/form-data` format,
    /// others can be uploaded as URL or path (depends on [`FileKind`]).
    /// If the file in `multipart/form-data` format,
    /// then [`InputFile::str_to_file`] will indicate "path" to data in form (because `multipart/form-data` format),
    /// otherwise it will be just string, which itself indicate "path" to data (because URL and telegram file id).
    /// # Returns
    /// If this file should be uploaded in `multipart/form-data` format, returns `attach://{id}`.
    /// Otherwise returns string as URL or path (depends on [`FileKind`]).
    #[must_use]
    pub fn str_to_file(&self) -> &str {
        match &self.0 {
            FileKind::Id(file) => file.str_to_file(),
            FileKind::Url(file) => file.str_to_file(),
            FileKind::FS(file) => file.str_to_file(),
            FileKind::Buffered(file) => file.str_to_file(),
        }
    }

    /// Some variants can be uploaded in `multipart/form-data` format,
    /// others can be uploaded as URL or path (depends on [`FileKind`]).
    /// # Returns
    /// If this file should be uploaded in `multipart/form-data` format, returns `true`.
    /// Otherwise returns `false` and file [`InputFile`] may be uploaded in any way (URL and telegram file id).
    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        match &self.0 {
            FileKind::Id(file) => file.is_require_multipart(),
            FileKind::Url(file) => file.is_require_multipart(),
            FileKind::FS(file) => file.is_require_multipart(),
            FileKind::Buffered(file) => file.is_require_multipart(),
        }
    }

    #[must_use]
    pub fn kind(&self) -> &FileKind {
        &self.0
    }
}

impl Serialize for InputFile<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.str_to_file().serialize(serializer)
    }
}

impl<'a> From<FileId<'a>> for InputFile<'a> {
    fn from(file_id: FileId<'a>) -> Self {
        Self(FileKind::Id(file_id))
    }
}

impl<'a> From<UrlFile<'a>> for InputFile<'a> {
    fn from(url_file: UrlFile<'a>) -> Self {
        Self(FileKind::Url(url_file))
    }
}

impl<'a> From<FSFile<'a>> for InputFile<'a> {
    fn from(fs_file: FSFile<'a>) -> Self {
        Self(FileKind::FS(fs_file))
    }
}

impl<'a> From<BufferedFile<'a>> for InputFile<'a> {
    fn from(buffered_file: BufferedFile<'a>) -> Self {
        Self(FileKind::Buffered(buffered_file))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FileKind<'a> {
    Id(FileId<'a>),
    Url(UrlFile<'a>),
    FS(FSFile<'a>),
    Buffered(BufferedFile<'a>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FileId<'a> {
    id: Cow<'a, str>,
}

impl<'a> FileId<'a> {
    #[must_use]
    pub fn new(id: impl Into<Cow<'a, str>>) -> Self {
        Self { id: id.into() }
    }

    /// Gets string to file as ID
    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.id
    }

    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UrlFile<'a> {
    url: Cow<'a, str>,
}

impl<'a> UrlFile<'a> {
    #[must_use]
    pub fn new(url: impl Into<Cow<'a, str>>) -> Self {
        Self { url: url.into() }
    }

    /// Gets string to file as URL
    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.url
    }

    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FSFile<'a> {
    id: Uuid,
    file_name: Option<&'a str>,
    path: PathBuf,
    str_to_file: String,
}

impl<'a> FSFile<'a> {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>, file_name: Option<&'a str>) -> Self {
        let id = Uuid::new_v4();

        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: file_name.map(Into::into),
            path: path.into(),
            str_to_file,
        }
    }

    /// Creates a new [`FSFile`] with specified ID
    /// # Notes
    /// If you want to create a new [`FSFile`] with random ID, use [`FSFile::new`] method instead.
    #[must_use]
    pub fn new_with_id(
        id: impl Into<Uuid>,
        path: impl Into<PathBuf>,
        file_name: Option<&'a str>,
    ) -> Self {
        let id = id.into();
        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: file_name.map(Into::into),
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
            .or(self.path.file_name().and_then(OsStr::to_str))
    }

    /// Gets path to file
    #[must_use]
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// Gets string to file as path in format `attach://{id}`
    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.str_to_file
    }
}

impl<'a> FSFile<'a> {
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BufferedFile<'a> {
    id: Uuid,
    bytes: Bytes,
    file_name: Option<&'a str>,
    str_to_file: String,
}

impl<'a> BufferedFile<'a> {
    #[must_use]
    pub fn new(bytes: impl Into<Bytes>, file_name: Option<&'a str>) -> Self {
        let id = Uuid::new_v4();
        let bytes = bytes.into();

        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            bytes,
            file_name: file_name.map(Into::into),
            str_to_file,
        }
    }

    /// Creates a new [`BufferedFile`] with specified ID
    /// # Notes
    /// If you want to create a new [`BufferedFile`] with random ID, use [`BufferedFile::new`] method instead
    #[must_use]
    pub fn new_with_id(
        id: impl Into<Uuid>,
        bytes: impl Into<Bytes>,
        file_name: Option<&'a str>,
    ) -> Self {
        let id = id.into();
        let bytes = bytes.into();

        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            bytes,
            file_name: file_name.map(Into::into),
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

    /// Gets passed filename
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.file_name
    }

    /// Gets bytes of file
    #[must_use]
    pub fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Gets string to file as path in format `attach://{id}`
    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.str_to_file
    }
}
