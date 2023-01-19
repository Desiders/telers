use bytes::Bytes;
use serde::{Serialize, Serializer};
use std::{borrow::Cow, io, path::PathBuf};
use tokio::{self, io::AsyncReadExt as _};
use uuid::Uuid;

const ATTACH_PREFIX: &str = "attach://";
const DEFAULT_CHUNK_SIZE: usize = 64 * 1024; // 64 KiB

/// This object represents the contents of a file to be uploaded.
/// Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
/// <https://core.telegram.org/bots/api#inputfile>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InputFile<'a>(FileKind<'a>);

impl<'a> InputFile<'a> {
    /// Creates a new `InputFile` from a file id
    #[must_use]
    pub fn id<T: Into<Cow<'a, str>>>(id: T) -> Self {
        Self(FileKind::Id(FileId::new(id)))
    }

    /// Creates a new `InputFile` from a url
    #[must_use]
    pub fn url<T: Into<Cow<'a, str>>>(url: T) -> Self {
        Self(FileKind::Url(UrlFile::new(url)))
    }

    /// Creates a new `InputFile` from a file system path
    #[must_use]
    pub fn fs<P, F>(path: P, filename: Option<F>) -> Self
    where
        P: Into<PathBuf>,
        F: Into<Cow<'a, str>>,
    {
        let id = Uuid::new_v4();

        Self(FileKind::FS(FSFile::new(id, path, filename)))
    }

    /// Alias to [`InputFile::fs`] method
    #[must_use]
    pub fn path<P, F>(path: P, filename: Option<F>) -> Self
    where
        P: Into<PathBuf>,
        F: Into<Cow<'a, str>>,
    {
        Self::fs(path, filename)
    }

    /// Alias to [`InputFile::fs`] method
    #[must_use]
    pub fn file_path<P, F>(path: P, filename: Option<F>) -> Self
    where
        P: Into<PathBuf>,
        F: Into<Cow<'a, str>>,
    {
        Self::fs(path, filename)
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FileKind<'a> {
    Id(FileId<'a>),
    Url(UrlFile<'a>),
    FS(FSFile<'a>),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FileId<'a> {
    id: Cow<'a, str>,
}

impl<'a> FileId<'a> {
    #[must_use]
    pub fn new<T: Into<Cow<'a, str>>>(id: T) -> Self {
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
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UrlFile<'a> {
    url: Cow<'a, str>,
}

impl<'a> UrlFile<'a> {
    #[must_use]
    pub fn new<T: Into<Cow<'a, str>>>(url: T) -> Self {
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
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FSFile<'a> {
    id: Uuid,
    filename: Option<Cow<'a, str>>,
    path: PathBuf,
    string_to_file: String,
}

impl<'a> FSFile<'a> {
    #[must_use]
    pub fn new<P, F>(id: Uuid, path: P, filename: Option<F>) -> Self
    where
        P: Into<PathBuf>,
        F: Into<Cow<'a, str>>,
    {
        let string_to_file = format!("{ATTACH_PREFIX}{id}");

        Self {
            id,
            filename: filename.map(Into::into),
            path: path.into(),
            string_to_file,
        }
    }

    #[must_use]
    pub fn str_to_file(&self) -> &str {
        &self.string_to_file
    }

    #[must_use]
    pub const fn is_require_multipart(&self) -> bool {
        true
    }

    #[must_use]
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns file name
    /// # Returns
    /// - If file name was set by [`InputFile::fs`], returns it
    /// - Otherwise returns file name from path if it exists and is valid Unicode, otherwise returns `None`
    #[must_use]
    pub fn file_name(&self) -> Option<&str> {
        if let Some(filename) = &self.filename {
            return Some(filename.as_ref());
        }

        self.path.file_name()?.to_str()
    }

    /// Reads file from filesystem and returns it as a vector of bytes
    /// # Errors
    /// If file can't be read or if path doesn't already exist
    pub async fn read(&self) -> Result<Bytes, io::Error> {
        let mut file = tokio::fs::File::open(&self.path).await?;
        let mut buffer = [0; DEFAULT_CHUNK_SIZE];

        let mut result = Vec::new();
        while let Ok(size) = file.read(&mut buffer).await {
            if size == 0 {
                break;
            }

            result.extend(buffer);
        }

        Ok(Bytes::from(result))
    }
}
