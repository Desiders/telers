use once_cell::sync::Lazy;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub trait FilesPathWrapper: Send + Sync {
    /// Converts a path to a local path
    #[must_use]
    fn to_local(&self, path: &Path) -> PathBuf;

    /// Converts a path to a server path
    #[must_use]
    fn to_server(&self, path: &Path) -> PathBuf;
}

pub struct BareFilesPathWrapper;

impl FilesPathWrapper for BareFilesPathWrapper {
    fn to_local(&self, path: &Path) -> PathBuf {
        path.to_path_buf()
    }

    fn to_server(&self, path: &Path) -> PathBuf {
        path.to_path_buf()
    }
}

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
    fn resolve(&self, base1: &Path, base2: &Path, path: &Path) -> PathBuf {
        let relative = base1.join(path);
        base2.join(relative)
    }
}

impl FilesPathWrapper for SimpleFilesPathWrapper {
    fn to_local(&self, path: &Path) -> PathBuf {
        self.resolve(&self.server_path, &self.local_path, path)
    }

    fn to_server(&self, path: &Path) -> PathBuf {
        self.resolve(&self.local_path, &self.server_path, path)
    }
}

/// Base config for API Endpoints
pub struct TelegramAPIServer {
    /// Base URL for API
    base: String,
    /// Files URL
    file: String,
    /// Mark this server is in `local mode <https://core.telegram.org/bots/api#using-a-local-bot-api-server>`_
    is_local: bool,
    /// Path wrapper for files in local mode
    files_path_wrapper: Arc<dyn FilesPathWrapper>,
}

impl TelegramAPIServer {
    /// Create a new TelegramAPIServer
    /// # Arguments
    /// * `base` - Base URL for API
    /// * `file` - Files URL
    /// * `is_local` - Mark this server is in `local mode <https://core.telegram.org/bots/api#using-a-local-bot-api-server>`_
    /// * `files_path_wrapper` - Path wrapper for files in local mode
    #[must_use]
    pub fn new<T, W>(base: &str, file: &str, is_local: bool, files_path_wrapper: W) -> Self
    where
        T: FilesPathWrapper + 'static,
        W: Into<Arc<T>>,
    {
        Self {
            base: base.trim_end_matches('/').to_string(),
            file: file.trim_end_matches('/').to_string(),
            is_local,
            files_path_wrapper: files_path_wrapper.into(),
        }
    }

    /// Get base URL for API
    #[must_use]
    pub fn base(&self) -> &str {
        &self.base
    }

    /// Get file URL
    #[must_use]
    pub fn file(&self) -> &str {
        &self.file
    }

    /// Check if this server is in `local mode <https://core.telegram.org/bots/api#using-a-local-bot-api-server>`_
    #[must_use]
    pub fn is_local(&self) -> bool {
        self.is_local
    }

    /// Get path wrapper for files in local mode
    #[must_use]
    pub fn files_path_wrapper(&self) -> &Arc<dyn FilesPathWrapper> {
        &self.files_path_wrapper
    }

    /// Generate URL for API methods
    /// # Arguments
    /// * `token` - Bot token
    /// * `method` - API method name (case insensitive)
    #[must_use]
    pub fn api_url(&self, token: &str, method: &str) -> String {
        self.base
            .replace("{token}", token)
            .replace("{method}", method)
    }

    /// Generate URL for downloading files
    /// # Arguments
    /// * `token` - Bot token
    /// * `path` - Path to file
    #[must_use]
    pub fn file_url(&self, token: &str, path: &str) -> String {
        self.file.replace("{token}", token).replace("{path}", path)
    }
}

impl Default for TelegramAPIServer {
    fn default() -> Self {
        Self::new(
            "https://api.telegram.org/bot{token}/{method}",
            "https://api.telegram.org/file/bot{token}/{path}",
            false,
            BareFilesPathWrapper,
        )
    }
}

pub static PRODUCTION: Lazy<TelegramAPIServer> = Lazy::new(|| TelegramAPIServer::default());

pub static TEST: Lazy<TelegramAPIServer> = Lazy::new(|| {
    TelegramAPIServer::new(
        "https://api.telegram.org/bot{token}/test/{method}",
        "https://api.telegram.org/file/bot{token}/test/{path}",
        false,
        BareFilesPathWrapper,
    )
});
