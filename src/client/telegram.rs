//! Telegram Bot API server configuration.
//!
//! This module contains [`APIServer`] struct, which represents configuration of Telegram Bot API server.
//! By default [`Session`] implementations use [`PRODUCTION`] configuration, but you can use [`TEST`] configuration
//! for testing your bot.
//!
//! You can create [`APIServer`] directly for using local Telegram Bot API server,
//! see example of using local Telegram Bot API server in `examples/local_server.rs`.
//!
//! [`Session`]: crate::client::Session

use once_cell::sync::Lazy;
use std::{
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};

pub trait FilesPathWrapper: Debug + Send + Sync {
    /// Converts a path to a local path
    #[must_use]
    fn to_local(&self, path: &Path) -> PathBuf;

    /// Converts a path to a server path
    #[must_use]
    fn to_server(&self, path: &Path) -> PathBuf;
}

impl<T: ?Sized> FilesPathWrapper for Arc<T>
where
    T: FilesPathWrapper,
{
    fn to_local(&self, path: &Path) -> PathBuf {
        T::to_local(self, path)
    }

    fn to_server(&self, path: &Path) -> PathBuf {
        T::to_server(self, path)
    }
}

/// Bare wrapper for files path in local mode.
///
/// You can use this wrapper for cases, when you have a full path to the file on the server,
/// because this wrapper just return the same path, which you passed to it without any changes.
#[derive(Debug)]
pub struct BareFilesPathWrapper;

impl FilesPathWrapper for BareFilesPathWrapper {
    fn to_local(&self, path: &Path) -> PathBuf {
        path.to_path_buf()
    }

    fn to_server(&self, path: &Path) -> PathBuf {
        path.to_path_buf()
    }
}

/// Simple wrapper for files path in local mode.
///
/// You can use this wrapper for cases, when you want set a base path for files on the server,
/// and a base path for files on the local machine. This wrapper will return a resolved path.
#[derive(Debug)]
pub struct SimpleFilesPathWrapper {
    server_path: PathBuf,
    local_path: PathBuf,
}

impl SimpleFilesPathWrapper {
    #[must_use]
    pub fn new(server_path: PathBuf, local_path: PathBuf) -> Self {
        Self {
            server_path,
            local_path,
        }
    }

    #[must_use]
    fn resolve(base1: &Path, base2: &Path, path: &Path) -> PathBuf {
        let relative = base1.join(path);
        base2.join(relative)
    }
}

impl FilesPathWrapper for SimpleFilesPathWrapper {
    fn to_local(&self, path: &Path) -> PathBuf {
        Self::resolve(&self.server_path, &self.local_path, path)
    }

    fn to_server(&self, path: &Path) -> PathBuf {
        Self::resolve(&self.local_path, &self.server_path, path)
    }
}

/// Configuration of Telegram Bot API server endpoints and local mode
#[derive(Clone, Debug)]
pub struct APIServer {
    /// Base URL for API
    base_url: String,
    /// Files URL
    files_url: String,
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
            base_url: base_url.trim_end_matches('/').to_string(),
            files_url: files_url.trim_end_matches('/').to_string(),
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
    pub fn is_local(&self) -> bool {
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
    pub fn api_url(&self, token: &str, method_name: &str) -> String {
        self.base_url
            .replace("{token}", token)
            .replace("{method_name}", method_name)
    }

    /// Generate URL for downloading file
    /// # Arguments
    /// * `token` - Bot token
    /// * `path` - Path to file
    #[must_use]
    pub fn file_url(&self, token: &str, path: &str) -> String {
        self.files_url
            .replace("{token}", token)
            .replace("{path}", path)
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
            server.api_url(
                "1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11",
                "getUpdates"
            ),
            "https://api.telegram.org/bot1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11/getUpdates"
        );

        let server = APIServer::new(
            "https://api.telegram.org/bot{token}/test/{method_name}",
            "https://api.telegram.org/file/bot{token}/test/{path}",
            false,
            BareFilesPathWrapper,
        );
        assert_eq!(
            server.api_url(
                "1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11",
                "getUpdates"
            ),
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
            server.file_url(
                "1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11",
                "test_path"
            ),
            "https://api.telegram.org/file/bot1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11/test_path"
        );

        let server = APIServer::new(
            "https://api.telegram.org/bot{token}/test/{method_name}",
            "https://api.telegram.org/file/bot{token}/test/{path}",
            false,
            BareFilesPathWrapper,
        );
        assert_eq!(
            server.file_url(
                "1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11",
                "test_path"
            ),
            "https://api.telegram.org/file/bot1234567890:ABC-DEF1234ghIkl-zyx57W2v1u123ew11/test/test_path"
        );
    }
}
