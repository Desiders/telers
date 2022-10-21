use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

#[derive(Debug)]
pub enum TelegramAPIError {
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

impl Display for TelegramAPIError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TelegramAPIError::TelegramNetworkError(e) => write!(f, "{}", e),
            TelegramAPIError::TelegramRetryAfter(e) => write!(f, "{}", e),
            TelegramAPIError::TelegramMigrateToChat(e) => write!(f, "{}", e),
            TelegramAPIError::TelegramBadRequest(e) => write!(f, "{}", e),
            TelegramAPIError::TelegramNotFound(e) => write!(f, "{}", e),
            TelegramAPIError::TelegramConflictError(e) => write!(f, "{}", e),
            TelegramAPIError::TelegramForbidden(e) => write!(f, "{}", e),
            TelegramAPIError::TelegramUnauthorized(e) => write!(f, "{}", e),
            TelegramAPIError::TelegramServerError(e) => write!(f, "{}", e),
            TelegramAPIError::RestartingTelegram(e) => write!(f, "{}", e),
            TelegramAPIError::TelegramEntityTooLarge(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug)]
pub struct TelegramNetworkError {
    pub message: String,
}

impl TelegramNetworkError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramNetworkError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramNetworkError: {}", self.message)
    }
}

impl Error for TelegramNetworkError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct TelegramRetryAfter {
    pub url: &'static str,
    pub message: String,
    pub retry_after: i64,
}

impl TelegramRetryAfter {
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

impl Error for TelegramRetryAfter {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct TelegramMigrateToChat {
    pub message: String,
    pub migrate_to_chat_id: i64,
}

impl TelegramMigrateToChat {
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

impl Error for TelegramMigrateToChat {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct TelegramBadRequest {
    pub message: String,
}

impl TelegramBadRequest {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramBadRequest {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramBadRequest: {}", self.message)
    }
}

impl Error for TelegramBadRequest {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct TelegramNotFound {
    pub message: String,
}

impl TelegramNotFound {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramNotFound {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramNotFound: {}", self.message)
    }
}

impl Error for TelegramNotFound {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct TelegramConflictError {
    pub message: String,
}

impl TelegramConflictError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramConflictError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramConflictError: {}", self.message)
    }
}

impl Error for TelegramConflictError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct TelegramForbidden {
    pub message: String,
}

impl TelegramForbidden {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramForbidden {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramForbidden: {}", self.message)
    }
}

impl Error for TelegramForbidden {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct TelegramUnauthorized {
    pub message: String,
}

impl TelegramUnauthorized {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramUnauthorized {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramUnauthorized: {}", self.message)
    }
}

impl Error for TelegramUnauthorized {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct TelegramServerError {
    pub message: String,
}

impl TelegramServerError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for TelegramServerError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramServerError: {}", self.message)
    }
}

impl Error for TelegramServerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct RestartingTelegram {
    pub message: String,
}

impl RestartingTelegram {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for RestartingTelegram {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RestartingTelegram: {}", self.message)
    }
}

impl Error for RestartingTelegram {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub struct TelegramEntityTooLarge {
    pub url: &'static str,
    pub message: String,
}

impl TelegramEntityTooLarge {
    pub fn new(message: String, retry_after: i64) -> Self {
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

impl Error for TelegramEntityTooLarge {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
