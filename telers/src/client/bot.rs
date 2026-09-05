//! This module contains the [`Bot`] structure that represents a bot with its token and ID,
//! it also contains client for sending requests to Telegram API.
//!
//! You can use [`Bot::send`] method, which accepts any type that implements [`TelegramMethod`].
//! Methods from [`methods`] module are implemented with builders, so you don't need to pass all parameters to it,
//! only the required ones, and set optional ones using builder methods. Builders can also have some useful shortcuts.
//!
//! # Notes
//!
//! This structure is cheap to clone: the token is shared behind an [`Arc`] and the id is an
//! [`i64`]. The default client is [`Reqwest`], which is also cheap to clone.
//!
//! You can use a custom client by using the [`Bot::with_client`] method.
//!
//! # Examples
//! ```rust
//! use telers::{methods::SendMessage, Bot};
//!
//! async fn call_method(bot: Bot) {
//!     let chat_id = 1;
//!     let text = "Hello, world!";
//!
//!     let _ = bot.send(SendMessage::new(chat_id, text)).await;
//! }
//! ```
//!
//! You can also use the [`Bot::send_with_timeout`] method to send requests with a timeout:
//!
//! ```rust
//! use telers::{methods::SendMessage, Bot};
//!
//! async fn call_method(bot: Bot) {
//!     let chat_id = 1;
//!     let text = "Hello, world!";
//!     let timeout = 10.0; // 10 seconds
//!
//!     let _ = bot
//!         .send_with_timeout(SendMessage::new(chat_id, text), timeout)
//!         .await;
//! }
//! ```
//!
//! Files can be downloaded with the [`Bot::download`] method, which accepts a file ID or any object
//! that represents a file, for example [`PhotoSize`](crate::types::PhotoSize).
//! Check the [`download`] module docs for more information:
//!
//! ```rust
//! use telers::{types::PhotoSize, Bot};
//!
//! async fn save_photo(bot: Bot, photo: &PhotoSize) {
//!     let _ = bot
//!         .download(photo)
//!         .await
//!         .unwrap()
//!         .to_path("photo.jpg")
//!         .await;
//! }
//! ```
//!
//! More production examples can be found in the [`examples`] directory.
//!
//! [`examples`]: https://github.com/Desiders/telers/tree/dev-1.x/examples
//! [`methods`]: telers::methods

pub mod download;

use super::{session::base::Session, Reqwest};

use crate::{
    errors::{SessionErrorKind, TelegramErrorKind},
    methods::TelegramMethod,
    utils::token,
};

use backoff::{future::retry, Error as BackoffError, ExponentialBackoff};

use secrecy::SecretString;
use std::{
    env,
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
    time::Duration,
};
use tracing::{event, Level};

/// Retry policy used by [`Bot::send_with_retry`] and [`Bot::send_with_timeout_and_retry`].
///
/// - `max_retries` limits the number of retries for transient server errors
///   ([`ServerError`](crate::errors::TelegramErrorKind::ServerError) and
///   [`MigrateToChat`](crate::errors::TelegramErrorKind::MigrateToChat))
/// - `backoff` controls the delays between retries and the elapsed time budget for
///   [`RetryAfter`](crate::errors::TelegramErrorKind::RetryAfter) errors,
///   for which Telegram API dictates the wait time itself
///
/// # Examples
///
/// Customizing the backoff algorithm:
///
/// ```rust
/// use telers::client::RetryPolicy;
///
/// use backoff::ExponentialBackoff;
/// use std::time::Duration;
///
/// let mut backoff = ExponentialBackoff::default();
/// backoff.max_elapsed_time = Some(Duration::from_secs(30));
///
/// let policy = RetryPolicy {
///     max_retries: 3,
///     backoff,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retries for transient server errors
    /// ([`ServerError`](crate::errors::TelegramErrorKind::ServerError) and
    /// [`MigrateToChat`](crate::errors::TelegramErrorKind::MigrateToChat)).
    /// [`RetryAfter`](crate::errors::TelegramErrorKind::RetryAfter) errors are not
    /// limited by this number, but by the elapsed time budget of [`Self::backoff`] instead
    pub max_retries: u32,
    /// Backoff algorithm that provides delays between retries.
    /// Defaults to `ExponentialBackoff::default()` with a 15 minutes elapsed time budget.
    /// Customize it directly, for example with `ExponentialBackoffBuilder`
    pub backoff: ExponentialBackoff,
}

impl RetryPolicy {
    /// Creates a new retry policy with the given maximum number of retries
    /// and the default exponential backoff
    #[must_use]
    pub fn new(max_retries: u32) -> Self {
        Self {
            max_retries,
            backoff: ExponentialBackoff::default(),
        }
    }
}

impl Default for RetryPolicy {
    /// Returns a policy without transient server error retries and the default backoff
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<u32> for RetryPolicy {
    fn from(max_retries: u32) -> Self {
        Self::new(max_retries)
    }
}

/// Represents a bot with its token and ID, also contains client for sending requests to Telegram API.
/// # Notes
/// This structure is cheap to clone, because the token is shared behind an [`Arc`] and the id is an
/// [`i64`].
///
/// Default client is [`Reqwest`], which also is cheap to clone.
///
/// You can use a custom client by using the [`Bot::with_client`] method.
///
/// Check [module docs](crate::client::bot) for examples.
#[derive(Clone)]
pub struct Bot<Client = Reqwest> {
    /// Bot token, which is used to receive updates and send requests to the Telegram API.
    ///
    /// Wrapped in a [`SecretString`] so it is redacted in `Debug`/`Display` output and zeroized when
    /// the last clone is dropped. Read it with [`Bot::token`].
    token: Arc<SecretString>,
    /// Bot id, extracted from the token
    pub id: i64,
    /// Client for sending requests to Telegram API
    client: Client,
}

impl<Client> Bot<Client> {
    /// # Panics
    /// Panics if the token is invalid
    #[must_use]
    pub fn with_client(token: impl Into<String>, client: Client) -> Self {
        let token = token.into();
        let id = token::extract_bot_id(&token).expect(
            "This bot token is invalid, please check it. If you test your bot, and you don't have \
             a token, use `Bot::default` method instead of `Bot::new`.",
        );

        Self {
            token: Arc::new(SecretString::from(token.into_boxed_str())),
            id,
            client,
        }
    }

    /// Bot token, which is used to receive updates and send requests to the Telegram API.
    ///
    /// # Notes
    /// The token is a [`SecretString`], so it is redacted in `Debug`/`Display` output and zeroized
    /// once the last [`Bot`] clone is dropped. Reading it is deliberate and auditable — call
    /// [`ExposeSecret::expose_secret`] on the returned value:
    ///
    /// ```rust
    /// use telers::{client::ExposeSecret as _, Bot};
    ///
    /// let bot = Bot::new("123:token");
    ///
    /// assert_eq!(bot.token().expose_secret(), "123:token");
    /// ```
    ///
    /// [`ExposeSecret::expose_secret`]: secrecy::ExposeSecret::expose_secret
    #[must_use]
    pub fn token(&self) -> &SecretString {
        &self.token
    }
}

impl<Client: Default> Default for Bot<Client> {
    /// Creates a bot with an empty token and a zero id, for tests and other cases where no real
    /// token is available
    fn default() -> Self {
        Self {
            token: Arc::new(SecretString::from(Box::<str>::from(""))),
            id: 0,
            client: Client::default(),
        }
    }
}

impl Bot<Reqwest> {
    /// # Panics
    /// Panics if the token is invalid
    #[must_use]
    pub fn new(token: impl Into<String>) -> Self {
        Self::with_client(token, Reqwest::default())
    }

    /// # Notes
    /// This method uses custom environment variable to get the token.
    /// If you want to use default environment variable, use [`Bot::from_env`] method instead.
    /// If you want to pass the token directly, use [`Bot::new`] method instead.
    /// # Panics
    /// Panics if the token is invalid or unset in the environment variables
    #[must_use]
    pub fn from_env_by_key(name: impl AsRef<str>) -> Self {
        Self::new(env::var(name.as_ref()).expect("This env variable is not set!"))
    }

    /// # Notes
    /// This method uses `BOT_TOKEN` environment variable to get the token.
    /// If you want to use custom environment variable, use [`Bot::from_env_by_key`] method instead.
    /// If you want to pass the token directly, use [`Bot::new`] method instead.
    /// # Panics
    /// Panics if the token is invalid or unset in the environment variables
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_by_key("BOT_TOKEN")
    }
}

impl<Client> Debug for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Bot")
            .field("token", &self.token)
            .field("bot_id", &self.id)
            .finish_non_exhaustive()
    }
}

impl<Client> Display for Bot<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Bot {{ bot_id: {}, token: {:?} }}", self.id, self.token)
    }
}

impl<Client: Session> Bot<Client> {
    /// Use this method to send requests to Telegram API
    /// # Arguments
    /// * `method` - Telegram API method
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    /// # Notes
    /// This method uses default timeout for requests, which is 30 seconds.
    /// If you want to use custom timeout, use [`Bot::send_with_timeout`] method.
    pub async fn send<T>(&self, method: T) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        self.client
            .make_request_and_get_result(self, method, None)
            .await
    }

    /// Use this method to send requests to Telegram API with timeout
    /// # Arguments
    /// * `method` - Telegram API method
    /// * `request_timeout` - Request timeout
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    /// # Notes
    /// This method uses passed timeout for requests.
    /// If you want to use default timeout, use [`Bot::send`] method.
    pub async fn send_with_timeout<T>(
        &self,
        method: T,
        request_timeout: f32,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        self.client
            .make_request_and_get_result(self, method, Some(request_timeout))
            .await
    }

    /// Use this method to send requests to Telegram API and retry them after [`RetryAfter`],
    /// [`ServerError`] and [`MigrateToChat`] errors
    ///
    /// [`RetryAfter`]: crate::errors::TelegramErrorKind::RetryAfter
    /// [`ServerError`]: crate::errors::TelegramErrorKind::ServerError
    /// [`MigrateToChat`]: crate::errors::TelegramErrorKind::MigrateToChat
    /// # Arguments
    /// * `method` - Telegram API method
    /// * `retry_policy` - Retry policy, check [`RetryPolicy`] for more information.
    ///   Can be constructed from a number of retries: `bot.send_with_retry(method, 3.into())`
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    /// # Notes
    /// This method uses default timeout for requests, which is 30 seconds.
    /// If you want to use custom timeout, use [`Bot::send_with_timeout_and_retry`] method.
    ///
    /// Retryable errors:
    /// - [`RetryAfter`] — the method waits for the number of seconds returned by Telegram API
    ///   and then sends the same request again. These retries are limited by the elapsed time
    ///   budget of the backoff algorithm (15 minutes by default) instead of the number of retries
    /// - [`ServerError`] and [`MigrateToChat`] — the method retries them with an exponential
    ///   backoff, up to `max_retries` retries from the retry policy
    ///
    /// Other errors are returned immediately without retries.
    /// If all retries are exhausted, the last error is returned.
    pub async fn send_with_retry<T>(
        &self,
        method: T,
        retry_policy: impl Into<RetryPolicy>,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Clone + Send + Sync,
        T::Method: Send + Sync,
    {
        self.send_with_retry_inner(method, None, retry_policy.into())
            .await
    }

    /// Use this method to send requests to Telegram API with timeout
    /// and retry them after [`RetryAfter`], [`ServerError`] and [`MigrateToChat`] errors
    ///
    /// [`RetryAfter`]: crate::errors::TelegramErrorKind::RetryAfter
    /// [`ServerError`]: crate::errors::TelegramErrorKind::ServerError
    /// [`MigrateToChat`]: crate::errors::TelegramErrorKind::MigrateToChat
    /// # Arguments
    /// * `method` - Telegram API method
    /// * `request_timeout` - Request timeout
    /// * `retry_policy` - Retry policy, check [`RetryPolicy`] for more information.
    ///   Can be constructed from a number of retries: `bot.send_with_timeout_and_retry(method, timeout, 3.into())`
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    /// # Notes
    /// This method uses passed timeout for requests.
    /// If you want to use default timeout, use [`Bot::send_with_retry`] method.
    ///
    /// Retryable errors:
    /// - [`RetryAfter`] — the method waits for the number of seconds returned by Telegram API
    ///   and then sends the same request again. These retries are limited by the elapsed time
    ///   budget of the backoff algorithm (15 minutes by default) instead of the number of retries
    /// - [`ServerError`] and [`MigrateToChat`] — the method retries them with an exponential
    ///   backoff, up to `max_retries` retries from the retry policy
    ///
    /// Other errors are returned immediately without retries.
    /// If all retries are exhausted, the last error is returned.
    pub async fn send_with_timeout_and_retry<T>(
        &self,
        method: T,
        request_timeout: f32,
        retry_policy: impl Into<RetryPolicy>,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Clone + Send + Sync,
        T::Method: Send + Sync,
    {
        self.send_with_retry_inner(method, Some(request_timeout), retry_policy.into())
            .await
    }

    async fn send_with_retry_inner<T>(
        &self,
        method: T,
        timeout: Option<f32>,
        retry_policy: RetryPolicy,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Clone + Send + Sync,
        T::Method: Send + Sync,
    {
        let RetryPolicy {
            max_retries,
            backoff,
        } = retry_policy;
        let mut transient_retries = 0_u32;

        retry(backoff, || {
            let method = method.clone();
            let attempt = transient_retries;
            transient_retries += 1;

            async move {
                match self
                    .client
                    .make_request_and_get_result(self, method, timeout)
                    .await
                {
                    Err(SessionErrorKind::Telegram(
                        error @ TelegramErrorKind::RetryAfter {
                            retry_after, ..
                        },
                    )) => {
                        event!(
                            Level::WARN,
                            retry_after,
                            "Request was rate limited, retrying after retry_after seconds"
                        );

                        Err(BackoffError::retry_after(
                            SessionErrorKind::Telegram(error),
                            Duration::from_secs(retry_after.max(0) as u64),
                        ))
                    }
                    Err(
                        error @ SessionErrorKind::Telegram(
                            TelegramErrorKind::ServerError {
                                ..
                            }
                            | TelegramErrorKind::MigrateToChat {
                                ..
                            },
                        ),
                    ) => {
                        if attempt < max_retries {
                            event!(
                                Level::WARN,
                                attempt = attempt + 1,
                                max_retries,
                                "Request failed with retryable server error, retrying with backoff"
                            );

                            Err(BackoffError::transient(error))
                        } else {
                            Err(BackoffError::permanent(error))
                        }
                    }
                    result => result.map_err(BackoffError::permanent),
                }
            }
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        client::{
            session::base::{ClientResponse, ClientStreamResponse},
            telegram::APIServer,
        },
        methods::Request,
    };

    use std::{
        collections::VecDeque,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Mutex, OnceLock,
        },
    };

    use anyhow;

    const OK_STATUS_CODE: u16 = 200;
    const RATE_LIMITED_STATUS_CODE: u16 = 429;
    const SERVER_ERROR_STATUS_CODE: u16 = 500;
    const MIGRATE_STATUS_CODE: u16 = 400;

    const OK_RESPONSE: &str = r#"{"ok":true,"result":42}"#;
    const RATE_LIMITED_RESPONSE: &str = r#"{"ok":false,"description":"Too Many Requests","error_code":429,"parameters":{"retry_after":0}}"#;
    const SERVER_ERROR_RESPONSE: &str =
        r#"{"ok":false,"description":"Internal Server Error","error_code":500}"#;
    const MIGRATE_RESPONSE: &str = r#"{"ok":false,"description":"Group migrated to a supergroup chat","error_code":400,"parameters":{"migrate_to_chat_id":-1001234567890}}"#;
    const BAD_REQUEST_RESPONSE: &str =
        r#"{"ok":false,"description":"Bad Request: chat not found","error_code":400}"#;

    /// Returns a response from the prepared queue and counts attempts
    struct MockSession {
        responses: Mutex<VecDeque<Result<ClientResponse, anyhow::Error>>>,
        attempts: AtomicUsize,
    }

    impl MockSession {
        fn new(responses: &[(u16, &str)]) -> Self {
            Self {
                responses: Mutex::new(
                    responses
                        .iter()
                        .map(|(status_code, content)| {
                            Ok(ClientResponse::new(*status_code, *content))
                        })
                        .collect(),
                ),
                attempts: AtomicUsize::new(0),
            }
        }
    }

    impl Session for MockSession {
        fn api(&self) -> &APIServer {
            static API: OnceLock<APIServer> = OnceLock::new();

            API.get_or_init(APIServer::default)
        }

        async fn send_request<Client, T>(
            &self,
            _bot: &Bot<Client>,
            _method: T,
            _timeout: Option<f32>,
        ) -> Result<ClientResponse, anyhow::Error>
        where
            Client: Session,
            T: TelegramMethod + Send + Sync,
            T::Method: Send + Sync,
        {
            self.attempts.fetch_add(1, Ordering::SeqCst);

            self.responses
                .lock()
                .expect("Mutex should not be poisoned")
                .pop_front()
                .expect("No more prepared responses")
        }

        async fn stream_content(
            &self,
            _url: &str,
            _timeout: Option<f32>,
        ) -> Result<ClientStreamResponse, anyhow::Error> {
            anyhow::bail!("Not implemented in mock session")
        }
    }

    #[derive(Clone)]
    struct TestMethod;

    impl TelegramMethod for TestMethod {
        type Method = ();
        type Return = i64;

        fn build_request<Client>(self, _bot: &Bot<Client>) -> Request<Self::Method> {
            Request::new("test", (), None)
        }
    }

    fn attempts(bot: &Bot<MockSession>) -> usize {
        bot.client.attempts.load(Ordering::SeqCst)
    }

    #[tokio::test]
    async fn retry_after_is_retried_until_success() {
        let session = MockSession::new(&[
            (RATE_LIMITED_STATUS_CODE, RATE_LIMITED_RESPONSE),
            (OK_STATUS_CODE, OK_RESPONSE),
        ]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_retry(TestMethod, 2).await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts(&bot), 2);
    }

    #[tokio::test]
    async fn retry_after_is_not_limited_by_max_retries() {
        let session = MockSession::new(&[
            (RATE_LIMITED_STATUS_CODE, RATE_LIMITED_RESPONSE),
            (RATE_LIMITED_STATUS_CODE, RATE_LIMITED_RESPONSE),
            (OK_STATUS_CODE, OK_RESPONSE),
        ]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_retry(TestMethod, 0).await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts(&bot), 3);
    }

    #[tokio::test]
    async fn server_error_is_retried_until_success() {
        let session = MockSession::new(&[
            (SERVER_ERROR_STATUS_CODE, SERVER_ERROR_RESPONSE),
            (SERVER_ERROR_STATUS_CODE, SERVER_ERROR_RESPONSE),
            (OK_STATUS_CODE, OK_RESPONSE),
        ]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_retry(TestMethod, 3).await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts(&bot), 3);
    }

    #[tokio::test]
    async fn server_error_stops_after_max_retries() {
        let session = MockSession::new(&[
            (SERVER_ERROR_STATUS_CODE, SERVER_ERROR_RESPONSE),
            (SERVER_ERROR_STATUS_CODE, SERVER_ERROR_RESPONSE),
        ]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_retry(TestMethod, 1).await;

        assert!(matches!(
            result,
            Err(SessionErrorKind::Telegram(
                TelegramErrorKind::ServerError { .. }
            ))
        ));
        assert_eq!(attempts(&bot), 2);
    }

    #[tokio::test]
    async fn migrate_to_chat_is_retried() {
        let session = MockSession::new(&[
            (MIGRATE_STATUS_CODE, MIGRATE_RESPONSE),
            (OK_STATUS_CODE, OK_RESPONSE),
        ]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_retry(TestMethod, 2).await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts(&bot), 2);
    }

    #[tokio::test]
    async fn other_errors_are_not_retried() {
        let session = MockSession::new(&[(MIGRATE_STATUS_CODE, BAD_REQUEST_RESPONSE)]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_retry(TestMethod, 5).await;

        assert!(matches!(
            result,
            Err(SessionErrorKind::Telegram(
                TelegramErrorKind::BadRequest { .. }
            ))
        ));
        assert_eq!(attempts(&bot), 1);
    }

    #[tokio::test]
    async fn timeout_and_retry_works() {
        let session = MockSession::new(&[
            (RATE_LIMITED_STATUS_CODE, RATE_LIMITED_RESPONSE),
            (OK_STATUS_CODE, OK_RESPONSE),
        ]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_timeout_and_retry(TestMethod, 10.0, 2).await;

        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempts(&bot), 2);
    }
}
