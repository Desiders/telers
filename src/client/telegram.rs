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

/// Base config for API Endpoints
#[derive(Clone, Debug)]
pub struct APIServer {
    /// Base URL for API
    base_url: String,
    /// Files URL
    files_url: String,
    /// Mark this server is in `local mode <https://core.telegram.org/bots/api#using-a-local-bot-api-server>`_
    is_local: bool,
    /// Path wrapper for files in local mode
    files_path_wrapper: Arc<dyn FilesPathWrapper>,
}

impl APIServer {
    /// Create a new `APIServer`
    /// # Arguments
    /// * `base_url` - Base URL for API
    /// * `files_url` - Files URL
    /// * `is_local` - Mark this server is in `local mode <https://core.telegram.org/bots/api#using-a-local-bot-api-server>`_
    /// * `files_path_wrapper` - Path wrapper for files in local mode
    #[must_use]
    pub fn new<T, W>(base_url: &str, files_url: &str, is_local: bool, files_path_wrapper: W) -> Self
    where
        T: FilesPathWrapper + 'static,
        W: Into<Arc<T>>,
    {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            files_url: files_url.trim_end_matches('/').to_string(),
            is_local,
            files_path_wrapper: files_path_wrapper.into(),
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

    /// Check if this server is in `local mode <https://core.telegram.org/bots/api#using-a-local-bot-api-server>`_
    #[must_use]
    pub fn is_local(&self) -> bool {
        self.is_local
    }

    /// Get path wrapper for files in local mode
    #[must_use]
    pub fn files_path_wrapper(&self) -> Arc<dyn FilesPathWrapper> {
        Arc::clone(&self.files_path_wrapper)
    }

    /// Generate URL for API method
    /// # Arguments
    /// * `token` - Bot token
    /// * `method` - API method name (case insensitive)
    #[must_use]
    pub fn api_url(&self, token: &str, method: &str) -> String {
        self.base_url
            .replace("{token}", token)
            .replace("{method}", method)
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
    fn default() -> Self {
        Self::new(
            "https://api.telegram.org/bot{token}/{method}",
            "https://api.telegram.org/file/bot{token}/{path}",
            false,
            BareFilesPathWrapper,
        )
    }
}

pub static PRODUCTION: Lazy<APIServer> = Lazy::new(APIServer::default);
pub static TEST: Lazy<APIServer> = Lazy::new(|| {
    APIServer::new(
        "https://api.telegram.org/bot{token}/test/{method}",
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
            "https://api.telegram.org/bot{token}/{method}",
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
            "https://api.telegram.org/bot{token}/test/{method}",
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
            "https://api.telegram.org/bot{token}/{method}",
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
            "https://api.telegram.org/bot{token}/test/{method}",
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
