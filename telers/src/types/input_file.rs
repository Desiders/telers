use bytes::{Bytes, BytesMut};
use futures::{Stream, TryFutureExt as _, TryStreamExt as _};
use serde::{Serialize, Serializer};
use std::{
    borrow::Cow,
    ffi::OsStr,
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    io,
    path::{Path, PathBuf},
    sync::Arc,
};
use takecell::TakeOwnCell;
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
///
/// This struct is useful for fast and easy creation of any of these types,
/// but if you want to use methods of specific type (for example [`FSFile::stream`] or [`StreamFile::set_stream`]),
/// you need to use specific type.
/// # Warning
/// If you [`Clone`] file, you will get a new file with the same ID for [`FileId`], [`UrlFile`], [`FSFile`], [`BufferedFile`].
/// So several parts will refer to the same data. It can be useful to minimize the amount of data uploaded.
/// If case of [`StreamFile`] you will get a new file with a new ID,
/// this is done to avoid problems when several parts with different streams refer to the first part and respectively to the first stream.
/// # Documentation
/// <https://core.telegram.org/bots/api#inputfile>
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum InputFile<'a> {
    Id(FileId<'a>),
    Url(UrlFile<'a>),
    FS(FSFile<'a>),
    Buffered(BufferedFile<'a>),
    Stream(StreamFile<'a>),
}

impl<'a> InputFile<'a> {
    /// Creates a new [`InputFile`] with [`FileId`]
    #[must_use]
    pub fn id(id: impl Into<Cow<'a, str>>) -> Self {
        Self::Id(FileId::new(id))
    }

    /// Creates a new [`InputFile`] with [`UrlFile`]
    #[must_use]
    pub fn url(url: impl Into<Cow<'a, str>>) -> Self {
        Self::Url(UrlFile::new(url))
    }

    /// Creates a new [`InputFile`] with [`FSFile`]
    #[must_use]
    pub fn fs(path: impl AsRef<Path>) -> Self {
        Self::FS(FSFile::new(path))
    }

    /// Creates a new [`InputFile`] with [`FSFile`] and specified filename
    #[must_use]
    pub fn fs_with_name(path: impl AsRef<Path>, name: impl Into<Cow<'a, str>>) -> Self {
        Self::FS(FSFile::new_with_name(path, name))
    }

    /// Creates a new [`InputFile`] with [`BufferedFile`]
    #[must_use]
    pub fn buffered(bytes: impl Into<Bytes>) -> Self {
        Self::Buffered(BufferedFile::new(bytes))
    }

    /// Creates a new [`InputFile`] with [`BufferedFile`] and specified filename
    #[must_use]
    pub fn buffered_with_name(bytes: impl Into<Bytes>, name: impl Into<Cow<'a, str>>) -> Self {
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
        name: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self::Stream(StreamFile::new_with_name(stream, name))
    }
}

impl<'a> InputFile<'a> {
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
        Self::Id(file_id)
    }
}

impl<'a> From<UrlFile<'a>> for InputFile<'a> {
    fn from(url_file: UrlFile<'a>) -> Self {
        Self::Url(url_file)
    }
}

impl<'a> From<FSFile<'a>> for InputFile<'a> {
    fn from(fs_file: FSFile<'a>) -> Self {
        Self::FS(fs_file)
    }
}

impl<'a> From<BufferedFile<'a>> for InputFile<'a> {
    fn from(buffered_file: BufferedFile<'a>) -> Self {
        Self::Buffered(buffered_file)
    }
}

impl<'a> From<StreamFile<'a>> for InputFile<'a> {
    fn from(stream_file: StreamFile<'a>) -> Self {
        Self::Stream(stream_file)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FSFile<'a> {
    id: Uuid,
    file_name: Option<Cow<'a, str>>,
    path: PathBuf,
    str_to_file: String,
}

impl<'a> FSFile<'a> {
    #[must_use]
    pub fn new(path: impl AsRef<Path>) -> Self {
        let id = Uuid::new_v4();

        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: None,
            path: path.as_ref().to_owned(),
            str_to_file,
        }
    }

    /// Creates a new [`FSFile`] with specified filename
    #[must_use]
    pub fn new_with_name(path: impl AsRef<Path>, name: impl Into<Cow<'a, str>>) -> Self {
        let id = Uuid::new_v4();
        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: Some(name.into()),
            path: path.as_ref().to_owned(),
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

impl Hash for FSFile<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct BufferedFile<'a> {
    id: Uuid,
    bytes: Bytes,
    file_name: Option<Cow<'a, str>>,
    str_to_file: String,
}

impl<'a> BufferedFile<'a> {
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
    pub fn new_with_name(bytes: impl Into<Bytes>, name: impl Into<Cow<'a, str>>) -> Self {
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

    /// Gets passed filename
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_deref()
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

impl Debug for BufferedFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("BufferedFile")
            .field("id", &self.id)
            .field("file_name", &self.file_name)
            .field("bytes", &"...")
            .field("str_to_file", &self.str_to_file)
            .finish()
    }
}

impl Hash for BufferedFile<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

type SharedStream =
    Arc<TakeOwnCell<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin>>>;

/// # Warning
/// If stream is taken, default client implementation raises an error,
/// so you need to use [`StreamFile::set_stream`] to set stream again.
/// This need because the stream can't be restored after it was taken.
///
/// Check [`StreamFile::take_stream`] and [`StreamFile::set_stream`] for more information.
pub struct StreamFile<'a> {
    id: Uuid,
    file_name: Option<Cow<'a, str>>,
    stream: SharedStream,
    str_to_file: String,
}

impl<'a> StreamFile<'a> {
    #[must_use]
    pub fn new(
        stream: impl Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin + 'static,
    ) -> Self {
        let id = Uuid::new_v4();

        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: None,
            stream: Arc::new(TakeOwnCell::new(Box::new(stream))),
            str_to_file,
        }
    }

    /// Creates a new [`FSFile`] with specified filename
    #[must_use]
    pub fn new_with_name(
        stream: impl Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin + 'static,
        name: impl Into<Cow<'a, str>>,
    ) -> Self {
        let id = Uuid::new_v4();
        let str_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            file_name: Some(name.into()),
            stream: Arc::new(TakeOwnCell::new(Box::new(stream))),
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
        self.file_name.as_deref()
    }

    /// Takes stream.
    /// # Warning
    /// If stream is taken, default client implementation raises an error,
    /// so you need to use [`StreamFile::set_stream`] to set stream again.
    /// # Returns
    /// After this function once returns `Some(_)` all consequtive calls before [`StreamFile::set_stream`]
    /// will return `None` as the value is already taken
    #[must_use]
    pub fn take_stream(
        &self,
    ) -> Option<Box<dyn Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin>> {
        self.stream.take()
    }

    /// Sets stream unconditionally.
    /// You need to use this method if you want to use [`StreamFile`] again for another request,
    /// because after [`StreamFile::take_stream`] was called, stream is taken and cannot be used again.
    /// # Notes
    /// If stream is taken, this method sets stream anyway.
    ///
    /// If you want to set stream only if stream is taken, use [`StreamFile::set_stream_if_taken`].
    /// # Returns
    pub fn set_stream(
        &mut self,
        stream: impl Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin + 'static,
    ) {
        self.stream = Arc::new(TakeOwnCell::new(Box::new(stream)));
    }

    /// Sets stream if stream is taken.
    /// You need to use this method if you want to use [`StreamFile`] again for another request,
    /// because after [`StreamFile::take_stream`] was called, stream is taken and cannot be used again.
    /// # Notes
    /// If stream is taken, this method does nothing.
    ///
    /// If you want to set stream unconditionally, use [`StreamFile::set_stream`].
    /// # Returns
    /// If stream is taken returns `false`, otherwise returns `true` and sets stream.
    pub fn set_stream_if_taken(
        &mut self,
        stream: impl Stream<Item = Result<Bytes, io::Error>> + Send + Sync + Unpin + 'static,
    ) -> bool {
        if self.stream.is_taken() {
            return false;
        }

        self.stream = Arc::new(TakeOwnCell::new(Box::new(stream)));

        true
    }

    /// Gets string to file as path in format `attach://{id}`
    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.str_to_file
    }
}

impl Debug for StreamFile<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("StreamFile")
            .field("id", &self.id)
            .field("file_name", &self.file_name)
            .field("stream", &"...")
            .field("str_to_file", &self.str_to_file)
            .finish()
    }
}

impl Clone for StreamFile<'_> {
    fn clone(&self) -> Self {
        let id = Uuid::new_v4();

        Self {
            id,
            file_name: self.file_name.clone(),
            stream: self.stream.clone(),
            str_to_file: format!("{ATTACH_PREFIX}{id}"),
        }
    }
}

impl Hash for StreamFile<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for StreamFile<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
