use std::{
    error::Error as StdError,
    fmt::{self, Debug, Display, Formatter},
};

#[allow(clippy::module_name_repetitions)]
pub trait TelegramAPIError: StdError {
    fn message(&self) -> &str;
}

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

impl Error {
    #[must_use]
    pub fn cause(&self) -> &dyn TelegramAPIError {
        self.cause.as_ref()
    }
}

impl StdError for Error {}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub enum TelegramAPIErrorKind {
    NetworkError(TelegramNetworkError),
    RetryAfter(TelegramRetryAfter),
    MigrateToChat(TelegramMigrateToChat),
    BadRequest(TelegramBadRequest),
    NotFound(TelegramNotFound),
    ConflictError(TelegramConflictError),
    Forbidden(TelegramForbidden),
    Unauthorized(TelegramUnauthorized),
    ServerError(TelegramServerError),
    RestartingTelegram(RestartingTelegram),
    EntityTooLarge(TelegramEntityTooLarge),
}

impl Display for TelegramAPIErrorKind {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            TelegramAPIErrorKind::NetworkError(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::RetryAfter(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::MigrateToChat(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::BadRequest(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::NotFound(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::ConflictError(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::Forbidden(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::Unauthorized(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::ServerError(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::RestartingTelegram(e) => write!(f, "{e}"),
            TelegramAPIErrorKind::EntityTooLarge(e) => write!(f, "{e}"),
        }
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramNetworkError {
    message: String,
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

impl StdError for TelegramNetworkError {}

impl TelegramAPIError for TelegramNetworkError {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramRetryAfter {
    url: &'static str,
    message: String,
    retry_after: i64,
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

    #[must_use]
    pub const fn url(&self) -> &'static str {
        self.url
    }

    #[must_use]
    pub const fn retry_after(&self) -> i64 {
        self.retry_after
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

impl StdError for TelegramRetryAfter {}

impl TelegramAPIError for TelegramRetryAfter {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramMigrateToChat {
    message: String,
    migrate_to_chat_id: i64,
}

impl TelegramMigrateToChat {
    #[must_use]
    pub fn new(message: String, migrate_to_chat_id: i64) -> Self {
        Self {
            message,
            migrate_to_chat_id,
        }
    }

    #[must_use]
    pub fn migrate_to_chat_id(&self) -> i64 {
        self.migrate_to_chat_id
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

impl StdError for TelegramMigrateToChat {}

impl TelegramAPIError for TelegramMigrateToChat {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramBadRequest {
    message: String,
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

impl StdError for TelegramBadRequest {}

impl TelegramAPIError for TelegramBadRequest {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramNotFound {
    message: String,
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

impl StdError for TelegramNotFound {}

impl TelegramAPIError for TelegramNotFound {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramConflictError {
    message: String,
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

impl StdError for TelegramConflictError {}

impl TelegramAPIError for TelegramConflictError {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramForbidden {
    message: String,
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

impl StdError for TelegramForbidden {}

impl TelegramAPIError for TelegramForbidden {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramUnauthorized {
    message: String,
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

impl StdError for TelegramUnauthorized {}

impl TelegramAPIError for TelegramUnauthorized {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramServerError {
    message: String,
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

impl StdError for TelegramServerError {}

impl TelegramAPIError for TelegramServerError {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct RestartingTelegram {
    message: String,
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

impl StdError for RestartingTelegram {}

impl TelegramAPIError for RestartingTelegram {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramEntityTooLarge {
    url: &'static str,
    message: String,
}

impl TelegramEntityTooLarge {
    #[must_use]
    pub fn new(message: String) -> Self {
        Self {
            url: "https://core.telegram.org/bots/api#sending-files",
            message,
        }
    }

    #[must_use]
    pub const fn url(&self) -> &'static str {
        self.url
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

impl StdError for TelegramEntityTooLarge {}

impl TelegramAPIError for TelegramEntityTooLarge {
    fn message(&self) -> &str {
        &self.message
    }
}

impl<T: TelegramAPIError + 'static> From<T> for Error {
    fn from(err: T) -> Error {
        Error {
            cause: Box::new(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telegram_network_error() {
        let err = TelegramNetworkError::new("test".to_string());
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_retry_after() {
        let err = TelegramRetryAfter::new("test".to_string(), 1);
        assert_eq!(err.message(), "test");
        assert_eq!(
            err.url(),
            "https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this"
        );
        assert_eq!(err.retry_after(), 1);
    }

    #[test]
    fn test_telegram_migrate_to_chat() {
        let err = TelegramMigrateToChat::new("test".to_string(), 1);
        assert_eq!(err.message(), "test");
        assert_eq!(err.migrate_to_chat_id(), 1);
    }

    #[test]
    fn test_telegram_bad_request() {
        let err = TelegramBadRequest::new("test".to_string());
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_not_found() {
        let err = TelegramNotFound::new("test".to_string());
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_conflict_error() {
        let err = TelegramConflictError::new("test".to_string());
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_forbidden() {
        let err = TelegramForbidden::new("test".to_string());
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_unauthorized() {
        let err = TelegramUnauthorized::new("test".to_string());
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_server_error() {
        let err = TelegramServerError::new("test".to_string());
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_restarting_telegram() {
        let err = RestartingTelegram::new("test".to_string());
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_entity_too_large() {
        let err = TelegramEntityTooLarge::new("test".to_string());
        assert_eq!(err.message(), "test");
        assert_eq!(
            err.url(),
            "https://core.telegram.org/bots/api#sending-files"
        );
    }

    #[test]
    fn test_error() {
        let err = Error::from(TelegramNetworkError::new("test".to_string()));
        assert_eq!(err.cause().message(), "test");
    }

    #[test]
    fn test_telegram_api_error_kind() {
        match TelegramAPIErrorKind::RestartingTelegram(RestartingTelegram::new("test".to_string()))
        {
            TelegramAPIErrorKind::RestartingTelegram(RestartingTelegram { message }) => {
                assert_eq!(message, "test".to_string());
            }
            _ => unimplemented!("Other error"),
        }
    }
}
