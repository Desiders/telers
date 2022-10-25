use std::{
    self,
    fmt::{self, Debug, Display, Formatter},
};

#[allow(clippy::module_name_repetitions)]
pub trait TelegramAPIError: Debug + Display + std::error::Error {}

pub struct Error {
    cause: Box<dyn TelegramAPIError>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.cause, f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.cause, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum TelegramAPIErrorKind {
    TelegramNetworkError(TelegramNetworkError),
    TelegramRetryAfter(TelegramRetryAfter),
    TelegramMigrateToChat(TelegramMigrateToChat),
    TelegramBadRequest(TelegramBadRequest),
    TelegramNotFound(TelegramNotFound),
    TelegramConflictError(TelegramConflictError),
    TelegramForbidden(TelegramForbidden),
    TelegramUnauthorized(TelegramUnauthorized),
    TelegramServerError(TelegramServerError),
    RestartingTelegram(RestartingTelegram),
    TelegramEntityTooLarge(TelegramEntityTooLarge),
}

impl Display for TelegramAPIErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TelegramAPIErrorKind::TelegramNetworkError(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::TelegramRetryAfter(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::TelegramMigrateToChat(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::TelegramBadRequest(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::TelegramNotFound(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::TelegramConflictError(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::TelegramForbidden(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::TelegramUnauthorized(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::TelegramServerError(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::RestartingTelegram(e) => write!(f, "{}", e),
            TelegramAPIErrorKind::TelegramEntityTooLarge(e) => write!(f, "{}", e),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramNetworkError {
    pub message: String,
}

impl TelegramNetworkError {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramNetworkError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramNetworkError: {}", self.message)
    }
}

impl std::error::Error for TelegramNetworkError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramNetworkError {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramRetryAfter {
    pub url: &'static str,
    pub message: String,
    pub retry_after: i64,
}

impl TelegramRetryAfter {
    #[must_use]
    pub fn new(message: String, retry_after: i64) -> Self {
        Self {
            url: "https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this",
            message,
            retry_after,
        }
    }
}

impl Display for TelegramRetryAfter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "TelegramRetryAfter: {} ({} seconds). Url: {}",
            self.message, self.retry_after, self.url,
        )
    }
}

impl std::error::Error for TelegramRetryAfter {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramRetryAfter {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramMigrateToChat {
    pub message: String,
    pub migrate_to_chat_id: i64,
}

impl TelegramMigrateToChat {
    #[must_use]
    pub fn new(message: String, migrate_to_chat_id: i64) -> Self {
        Self {
            message,
            migrate_to_chat_id,
        }
    }
}

impl Display for TelegramMigrateToChat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "TelegramMigrateToChat: {}. Chat id: {}",
            self.message, self.migrate_to_chat_id
        )
    }
}

impl std::error::Error for TelegramMigrateToChat {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramMigrateToChat {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramBadRequest {
    pub message: String,
}

impl TelegramBadRequest {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramBadRequest {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramBadRequest: {}", self.message)
    }
}

impl std::error::Error for TelegramBadRequest {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramBadRequest {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramNotFound {
    pub message: String,
}

impl TelegramNotFound {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramNotFound {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramNotFound: {}", self.message)
    }
}

impl std::error::Error for TelegramNotFound {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramNotFound {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramConflictError {
    pub message: String,
}

impl TelegramConflictError {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramConflictError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramConflictError: {}", self.message)
    }
}

impl std::error::Error for TelegramConflictError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramConflictError {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramForbidden {
    pub message: String,
}

impl TelegramForbidden {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramForbidden {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramForbidden: {}", self.message)
    }
}

impl std::error::Error for TelegramForbidden {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramForbidden {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramUnauthorized {
    pub message: String,
}

impl TelegramUnauthorized {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramUnauthorized {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramUnauthorized: {}", self.message)
    }
}

impl std::error::Error for TelegramUnauthorized {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramUnauthorized {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramServerError {
    pub message: String,
}

impl TelegramServerError {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramServerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramServerError: {}", self.message)
    }
}

impl std::error::Error for TelegramServerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramServerError {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct RestartingTelegram {
    pub message: String,
}

impl RestartingTelegram {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for RestartingTelegram {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RestartingTelegram: {}", self.message)
    }
}

impl std::error::Error for RestartingTelegram {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for RestartingTelegram {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramEntityTooLarge {
    pub url: &'static str,
    pub message: String,
}

impl TelegramEntityTooLarge {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self {
            url: "https://core.telegram.org/bots/api#sending-files",
            message,
        }
    }
}

impl Display for TelegramEntityTooLarge {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "TelegramEntityTooLarge: {}. Url: {}",
            self.message, self.url,
        )
    }
}

impl std::error::Error for TelegramEntityTooLarge {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl TelegramAPIError for TelegramEntityTooLarge {}

impl<T: TelegramAPIError + 'static> From<T> for Error {
    fn from(err: T) -> Error {
        Error {
            cause: Box::new(err),
        }
    }
}
