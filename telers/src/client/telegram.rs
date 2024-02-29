//! This module contains the [`APIServer`] struct, which represents the configuration of the Telegram Bot API server
//! and the [`FilesPathWrapper`] trait, which is used for resolving paths.
//!
//! You can use global variables [`PRODUCTION`] and [`TEST`] for using default configurations of Telegram Bot API server
//! for production and testing.
//!
//! [`FilesPathWrapper`] is used for resolving paths for files.
//! By default, [`BareFilesPathWrapper`] should be used, which just returns the same path, which you passed to it without any changes.
//! You can use [`FilesDiffPathWrapper`] for resolving paths with different server and local paths.
//! This can be useful for local Telegram Bot API server.

use once_cell::sync::Lazy;
use pathdiff::diff_paths;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};

pub trait FilesPathWrapper: Debug + Send + Sync {
    /// Converts a path to a local path
    #[must_use]
    fn to_local(&self, path: &Path) -> Option<PathBuf>;

    /// Converts a path to a server path
    #[must_use]
    fn to_server(&self, path: &Path) -> Option<PathBuf>;
}

impl<T: ?Sized> FilesPathWrapper for Arc<T>
where
    T: FilesPathWrapper,
{
    fn to_local(&self, path: &Path) -> Option<PathBuf> {
        T::to_local(self, path)
    }

    fn to_server(&self, path: &Path) -> Option<PathBuf> {
        T::to_server(self, path)
    }
}

/// Bare wrapper for server and local paths.
///
/// This wrapper just return the same path, which you passed to it without any changes.
#[derive(Debug, Clone, Copy, Default)]
pub struct BareFilesPathWrapper;

impl BareFilesPathWrapper {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl FilesPathWrapper for BareFilesPathWrapper {
    /// # Returns
    /// Always `Some(path)`
    fn to_local(&self, path: &Path) -> Option<PathBuf> {
        Some(path.to_path_buf())
    }

    /// # Returns
    /// Always `Some(path)`
    fn to_server(&self, path: &Path) -> Option<PathBuf> {
        Some(path.to_path_buf())
    }
}

/// Wrapper for files path with different server and local paths.
///
/// This wrapper resolve local path from server path and vice versa.
/// # Notes
/// It uses [`pathdiff::diff_paths`] for resolving paths.
#[derive(Debug)]
pub struct FilesDiffPathWrapper {
    server_path: PathBuf,
    local_path: PathBuf,
}

impl FilesDiffPathWrapper {
    #[must_use]
    pub fn new(server_path: PathBuf, local_path: PathBuf) -> Self {
        Self {
            server_path,
            local_path,
        }
    }
}

impl FilesPathWrapper for FilesDiffPathWrapper {
    /// # Warning
    /// `..` in `path` and similar will not be resolved,
    /// for example,`/etc/telegram-bot-api/data/../data/test_path` will be resolved to `/opt/app/data/../data/test_path`
    fn to_local(&self, path: &Path) -> Option<PathBuf> {
        diff_paths(path, &self.server_path).map(|relative_path| self.local_path.join(relative_path))
    }

    /// # Warning
    /// `..` in `path` and similar will not be resolved,
    /// for example,`/opt/app/data/../data/test_path` will be resolved to `/etc/telegram-bot-api/data/../data/test_path`
    fn to_server(&self, path: &Path) -> Option<PathBuf> {
        diff_paths(path, &self.local_path).map(|relative_path| self.server_path.join(relative_path))
    }
}

/// Configuration of Telegram Bot API server endpoints and local mode
#[derive(Debug, Clone)]
pub struct APIServer {
    /// Base URL for API
    base_url: Box<str>,
    /// Files URL
    files_url: Box<str>,
    /// Mark this server is in [`local mode`](https://core.telegram.org/bots/api#using-a-local-bot-api-server)
    is_local: bool,
    /// Path wrapper for files in local mode
    files_path_wrapper: Arc<dyn FilesPathWrapper>,
}

impl APIServer {
    #[must_use]
    pub fn new<T>(base_url: &str, files_url: &str, is_local: bool, files_path_wrapper: T) -> Self
    where
        T: FilesPathWrapper + 'static,
    {
        Self {
            base_url: base_url.trim_end_matches('/').into(),
            files_url: files_url.trim_end_matches('/').into(),
            is_local,
            files_path_wrapper: Arc::new(files_path_wrapper),
        }
    }

    /// Get base URL for API
    #[must_use]
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Get files URL
    #[must_use]
    pub fn files_url(&self) -> &str {
        &self.files_url
    }

    /// Check if this server is in [`local mode`](https://core.telegram.org/bots/api#using-a-local-bot-api-server)
    #[must_use]
    pub const fn is_local(&self) -> bool {
        self.is_local
    }

    /// Get path wrapper for files in local mode
    #[must_use]
    pub fn files_path_wrapper(&self) -> &dyn FilesPathWrapper {
        &*self.files_path_wrapper
    }

    /// Generate URL for API method
    /// # Arguments
    /// * `token` - Bot token
    /// * `method_name` - API method name (case insensitive)
    #[must_use]
    pub fn api_url(&self, token: &str, method_name: &str) -> Box<str> {
        self.base_url
            .replace("{token}", token)
            .replace("{method_name}", method_name)
            .into()
    }

    /// Generate URL for downloading file
    /// # Arguments
    /// * `token` - Bot token
    /// * `path` - Path to file
    #[must_use]
    pub fn file_url(&self, token: &str, path: &str) -> Box<str> {
        self.files_url
            .replace("{token}", token)
            .replace("{path}", path)
            .into()
    }
}

impl Default for APIServer {
    #[must_use]
    fn default() -> Self {
        Self::new(
            "https://api.telegram.org/bot{token}/{method_name}",
            "https://api.telegram.org/file/bot{token}/{path}",
            false,
            BareFilesPathWrapper,
        )
    }
}

pub static PRODUCTION: Lazy<APIServer> = Lazy::new(APIServer::default);
pub static TEST: Lazy<APIServer> = Lazy::new(|| {
    APIServer::new(
        "https://api.telegram.org/bot{token}/test/{method_name}",
        "https://api.telegram.org/file/bot{token}/test/{path}",
        false,
        BareFilesPathWrapper,
    )
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_url() {
        let server = APIServer::new(
            "https://api.telegram.org/bot{token}/{method_name}",
            "https://api.telegram.org/file/bot{token}/{path}",
            false,
            BareFilesPathWrapper,
        );
        assert_eq!(
            server
                .api_url(
                    "1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11",
                    "getUpdates"
                )
                .as_ref(),
            "https://api.telegram.org/bot1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11/getUpdates"
        );

        let server = APIServer::new(
            "https://api.telegram.org/bot{token}/test/{method_name}",
            "https://api.telegram.org/file/bot{token}/test/{path}",
            false,
            BareFilesPathWrapper,
        );
        assert_eq!(
            server
                .api_url("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11", "getUpdates")
                .as_ref(),
            "https://api.telegram.org/bot1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11/test/getUpdates"
        );
    }

    #[test]
    fn test_file_url() {
        let server = APIServer::new(
            "https://api.telegram.org/bot{token}/{method_name}",
            "https://api.telegram.org/file/bot{token}/{path}",
            false,
            BareFilesPathWrapper,
        );
        assert_eq!(
            server
                .file_url("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11", "test_path")
                .as_ref(),
            "https://api.telegram.org/file/bot1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11/test_path"
        );

        let server = APIServer::new(
            "https://api.telegram.org/bot{token}/test/{method_name}",
            "https://api.telegram.org/file/bot{token}/test/{path}",
            false,
            BareFilesPathWrapper,
        );
        assert_eq!(
            server
                .file_url("1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11", "test_path")
                .as_ref(),
            "https://api.telegram.org/file/bot1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11/test/test_path"
        );
    }

    #[test]
    fn test_bare_files_path_wrapper() {
        let wrapper = BareFilesPathWrapper;

        assert_eq!(
            wrapper.to_local(Path::new("test_path")),
            Some(PathBuf::from("test_path")),
        );
        assert_eq!(
            wrapper.to_server(Path::new("test_path")),
            Some(PathBuf::from("test_path")),
        );
    }

    #[test]
    fn test_files_diff_path_wrapper() {
        let wrapper = FilesDiffPathWrapper::new(
            PathBuf::from("/etc/telegram-bot-api/data"),
            PathBuf::from("/opt/app/data"),
        );

        assert_eq!(
            wrapper.to_local(Path::new("/etc/telegram-bot-api/data/test_path")),
            Some(PathBuf::from("/opt/app/data/test_path")),
        );
        assert_eq!(
            wrapper.to_server(Path::new("/opt/app/data/test_path")),
            Some(PathBuf::from("/etc/telegram-bot-api/data/test_path")),
        );
    }
}
