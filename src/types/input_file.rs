use bytes::Bytes;
use serde::{Serialize, Serializer};
use std::{borrow::Cow, io, path::PathBuf};
use tokio::{self, io::AsyncReadExt};
use uuid::Uuid;

const ATTACH_PREFIX: &str = "attach://";
const DEFAULT_CHUNK_SIZE: usize = 64 * 1024; // 64 KiB

/// This object represents the contents of a file to be uploaded.
/// Must be posted using multipart/form-data in the usual way that files are uploaded via the browser.
/// <https://core.telegram.org/bots/api#inputfile>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct InputFile(InputFileKind);

impl InputFile {
    /// Creates a new `InputFile` from a file id
    pub fn id(id: impl Into<String>) -> Self {
        Self(InputFileKind::Id(FileId { id: id.into() }))
    }

    /// Creates a new `InputFile` from a url
    pub fn url(url: impl Into<Cow<'static, str>>) -> Self {
        Self(InputFileKind::Url(UrlFile { url: url.into() }))
    }

    /// Creates a new `InputFile` from a file system path
    pub fn fs(path: impl Into<PathBuf>, filename: Option<impl Into<Cow<'static, str>>>) -> Self {
        Self(InputFileKind::FS(FSFile {
            id: Uuid::new_v4(),
            filename: filename.map(|f| f.into()),
            path: path.into(),
        }))
    }

    /// Alias to [`InputFile::fs`] method
    pub fn file(path: impl Into<PathBuf>, filename: Option<impl Into<Cow<'static, str>>>) -> Self {
        Self::fs(path, filename)
    }

    /// Alias to [`InputFile::fs`] method
    pub fn path(path: impl Into<PathBuf>, filename: Option<impl Into<Cow<'static, str>>>) -> Self {
        Self::fs(path, filename)
    }

    /// Alias to [`InputFile::fs`] method
    pub fn file_path(
        path: impl Into<PathBuf>,
        filename: Option<impl Into<Cow<'static, str>>>,
    ) -> Self {
        Self::fs(path, filename)
    }
}

impl InputFile {
    /// Some variants can be uploaded in `multipart/form-data` format,
    /// others can be uploaded as URL or path (depends on [`InputFileKind`]).
    /// If the file in `multipart/form-data` format,
    /// then [`InputFile::string_to_file`] will indicate "path" to data in form (because `multipart/form-data` format),
    /// otherwise it will be just string, which itself indicate "path" to data (because URL and telegram file id).
    /// # Returns
    /// If this file should be uploaded in `multipart/form-data` format, returns `attach://{id}`.
    /// Otherwise returns string as URL or path (depends on [`InputFileKind`]).
    pub fn string_to_file(&self) -> String {
        match &self.0 {
            InputFileKind::Id(file) => file.string_to_file(),
            InputFileKind::Url(file) => file.string_to_file(),
            InputFileKind::FS(file) => file.string_to_file(),
        }
    }

    /// Some variants can be uploaded in `multipart/form-data` format,
    /// others can be uploaded as URL or path (depends on [`InputFileKind`]).
    /// # Returns
    /// If this file should be uploaded in `multipart/form-data` format, returns `true`.
    /// Otherwise returns `false` and file [`InputFile`] may be uploaded in any way (URL and telegram file id).
    pub const fn is_require_multipart(&self) -> bool {
        match &self.0 {
            InputFileKind::Id(file) => file.is_require_multipart(),
            InputFileKind::Url(file) => file.is_require_multipart(),
            InputFileKind::FS(file) => file.is_require_multipart(),
        }
    }

    pub fn kind(&self) -> &InputFileKind {
        &self.0
    }
}

impl Serialize for InputFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.string_to_file().serialize(serializer)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputFileKind {
    Id(FileId),
    Url(UrlFile),
    FS(FSFile),
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FileId {
    id: String,
}

impl FileId {
    pub fn string_to_file(&self) -> String {
        self.id.clone()
    }

    pub const fn is_require_multipart(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct UrlFile {
    url: Cow<'static, str>,
}

impl UrlFile {
    pub fn string_to_file(&self) -> String {
        self.url.to_string()
    }

    pub const fn is_require_multipart(&self) -> bool {
        false
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FSFile {
    id: Uuid,
    filename: Option<Cow<'static, str>>,
    path: PathBuf,
}

impl FSFile {
    pub fn string_to_file(&self) -> String {
        format!("{}{}", ATTACH_PREFIX, self.id)
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub const fn is_require_multipart(&self) -> bool {
        true
    }

    /// Reads file from filesystem and returns it as a vector of bytes
    pub async fn read(&self) -> Result<Bytes, io::Error> {
        let mut file = tokio::fs::File::open(&self.path).await?;
        let mut buffer = [0; DEFAULT_CHUNK_SIZE];

        let mut result = Vec::new();
        while let Ok(size) = file.read(&mut buffer).await {
            if size == 0 {
                break;
            }

            result.extend(&buffer[..size])
        }

        Ok(Bytes::from(result))
    }

    pub fn file_name(&self) -> Option<Cow<'static, str>> {
        if let Some(filename) = &self.filename {
            return Some(filename.clone());
        }

        Some(self.path.file_name()?.to_string_lossy().into_owned().into())
    }
}
