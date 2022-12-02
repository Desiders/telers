use std::{
    borrow::Cow,
    error::Error as StdError,
    fmt::{self, Debug, Display, Formatter},
};

/// Base error type for telegram api errors
#[allow(clippy::module_name_repetitions)]
pub trait TelegramAPIError: StdError + Send + Sync {
    #[must_use]
    fn message(&self) -> &str;
}

/// Error wrapper for [`TelegramAPIError`]
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
pub enum TelegramAPIErrorKind<'a> {
    NetworkError(TelegramNetworkError<'a>),
    RetryAfter(TelegramRetryAfter<'a>),
    MigrateToChat(TelegramMigrateToChat<'a>),
    BadRequest(TelegramBadRequest<'a>),
    NotFound(TelegramNotFound<'a>),
    ConflictError(TelegramConflictError<'a>),
    Forbidden(TelegramForbidden<'a>),
    Unauthorized(TelegramUnauthorized<'a>),
    ServerError(TelegramServerError<'a>),
    RestartingTelegram(RestartingTelegram<'a>),
    EntityTooLarge(TelegramEntityTooLarge<'a>),
}

impl Display for TelegramAPIErrorKind<'_> {
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
pub struct TelegramNetworkError<'a> {
    message: Cow<'a, str>,
}

impl<'a> TelegramNetworkError<'a> {
    #[must_use]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            message: message.into(),
        }
    }
}

impl Display for TelegramNetworkError<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramNetworkError: {}", self.message)
    }
}

impl StdError for TelegramNetworkError<'_> {}

impl TelegramAPIError for TelegramNetworkError<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramRetryAfter<'a> {
    url: &'static str,
    message: Cow<'a, str>,
    retry_after: i64,
}

impl<'a> TelegramRetryAfter<'a> {
    #[must_use]
    pub fn new<M>(message: M, retry_after: i64) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            url: "https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this",
            message: message.into(),
            retry_after,
        }
    }

    #[must_use]
    pub const fn url(&self) -> &str {
        self.url
    }

    #[must_use]
    pub const fn retry_after(&self) -> i64 {
        self.retry_after
    }
}

impl Display for TelegramRetryAfter<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "TelegramRetryAfter: {} ({} seconds). Url: {}",
            self.message, self.retry_after, self.url,
        )
    }
}

impl StdError for TelegramRetryAfter<'_> {}

impl TelegramAPIError for TelegramRetryAfter<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramMigrateToChat<'a> {
    message: Cow<'a, str>,
    migrate_to_chat_id: i64,
}

impl<'a> TelegramMigrateToChat<'a> {
    #[must_use]
    pub fn new<M>(message: M, migrate_to_chat_id: i64) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            message: message.into(),
            migrate_to_chat_id,
        }
    }

    #[must_use]
    pub fn migrate_to_chat_id(&self) -> i64 {
        self.migrate_to_chat_id
    }
}

impl Display for TelegramMigrateToChat<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "TelegramMigrateToChat: {}. Chat id: {}",
            self.message, self.migrate_to_chat_id
        )
    }
}

impl StdError for TelegramMigrateToChat<'_> {}

impl TelegramAPIError for TelegramMigrateToChat<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramBadRequest<'a> {
    message: Cow<'a, str>,
}

impl<'a> TelegramBadRequest<'a> {
    #[must_use]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            message: message.into(),
        }
    }
}

impl Display for TelegramBadRequest<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramBadRequest: {}", self.message)
    }
}

impl StdError for TelegramBadRequest<'_> {}

impl TelegramAPIError for TelegramBadRequest<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramNotFound<'a> {
    message: Cow<'a, str>,
}

impl<'a> TelegramNotFound<'a> {
    #[must_use]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            message: message.into(),
        }
    }
}

impl Display for TelegramNotFound<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramNotFound: {}", self.message)
    }
}

impl StdError for TelegramNotFound<'_> {}

impl TelegramAPIError for TelegramNotFound<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramConflictError<'a> {
    message: Cow<'a, str>,
}

impl<'a> TelegramConflictError<'a> {
    #[must_use]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            message: message.into(),
        }
    }
}

impl Display for TelegramConflictError<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramConflictError: {}", self.message)
    }
}

impl StdError for TelegramConflictError<'_> {}

impl TelegramAPIError for TelegramConflictError<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramForbidden<'a> {
    message: Cow<'a, str>,
}

impl<'a> TelegramForbidden<'a> {
    #[must_use]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            message: message.into(),
        }
    }
}

impl Display for TelegramForbidden<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramForbidden: {}", self.message)
    }
}

impl StdError for TelegramForbidden<'_> {}

impl TelegramAPIError for TelegramForbidden<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramUnauthorized<'a> {
    message: Cow<'a, str>,
}

impl<'a> TelegramUnauthorized<'a> {
    #[must_use]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            message: message.into(),
        }
    }
}

impl Display for TelegramUnauthorized<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramUnauthorized: {}", self.message)
    }
}

impl StdError for TelegramUnauthorized<'_> {}

impl TelegramAPIError for TelegramUnauthorized<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramServerError<'a> {
    message: Cow<'a, str>,
}

impl<'a> TelegramServerError<'a> {
    #[must_use]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            message: message.into(),
        }
    }
}

impl Display for TelegramServerError<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "TelegramServerError: {}", self.message)
    }
}

impl StdError for TelegramServerError<'_> {}

impl TelegramAPIError for TelegramServerError<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct RestartingTelegram<'a> {
    message: Cow<'a, str>,
}

impl<'a> RestartingTelegram<'a> {
    #[must_use]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            message: message.into(),
        }
    }
}

impl Display for RestartingTelegram<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "RestartingTelegram: {}", self.message)
    }
}

impl StdError for RestartingTelegram<'_> {}

impl TelegramAPIError for RestartingTelegram<'_> {
    fn message(&self) -> &str {
        &self.message
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct TelegramEntityTooLarge<'a> {
    url: &'static str,
    message: Cow<'a, str>,
}

impl<'a> TelegramEntityTooLarge<'a> {
    #[must_use]
    pub fn new<M>(message: M) -> Self
    where
        M: Into<Cow<'a, str>>,
    {
        Self {
            url: "https://core.telegram.org/bots/api#sending-files",
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn url(&self) -> &'static str {
        self.url
    }
}

impl Display for TelegramEntityTooLarge<'_> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "TelegramEntityTooLarge: {}. Url: {}",
            self.message, self.url,
        )
    }
}

impl StdError for TelegramEntityTooLarge<'_> {}

impl TelegramAPIError for TelegramEntityTooLarge<'_> {
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
        let err = TelegramNetworkError::new("test");
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_retry_after() {
        let err = TelegramRetryAfter::new("test", 1);
        assert_eq!(err.message(), "test");
        assert_eq!(
            err.url(),
            "https://core.telegram.org/bots/faq#my-bot-is-hitting-limits-how-do-i-avoid-this"
        );
        assert_eq!(err.retry_after(), 1);
    }

    #[test]
    fn test_telegram_migrate_to_chat() {
        let err = TelegramMigrateToChat::new("test", 1);
        assert_eq!(err.message(), "test");
        assert_eq!(err.migrate_to_chat_id(), 1);
    }

    #[test]
    fn test_telegram_bad_request() {
        let err = TelegramBadRequest::new("test");
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_not_found() {
        let err = TelegramNotFound::new("test");
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_conflict_error() {
        let err = TelegramConflictError::new("test");
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_forbidden() {
        let err = TelegramForbidden::new("test");
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_unauthorized() {
        let err = TelegramUnauthorized::new("test");
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_server_error() {
        let err = TelegramServerError::new("test");
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_restarting_telegram() {
        let err = RestartingTelegram::new("test");
        assert_eq!(err.message(), "test");
    }

    #[test]
    fn test_telegram_entity_too_large() {
        let err = TelegramEntityTooLarge::new("test");
        assert_eq!(err.message(), "test");
        assert_eq!(
            err.url(),
            "https://core.telegram.org/bots/api#sending-files"
        );
    }

    #[test]
    fn test_error() {
        let err = Error::from(TelegramNetworkError::new("test"));
        assert_eq!(err.cause().message(), "test");
    }

    #[test]
    fn test_telegram_api_error_kind() {
        match TelegramAPIErrorKind::RestartingTelegram(RestartingTelegram::new("test")) {
            TelegramAPIErrorKind::RestartingTelegram(RestartingTelegram { message }) => {
                assert_eq!(message, "test");
            }
            _ => unreachable!("Other error"),
        }
    }
}
