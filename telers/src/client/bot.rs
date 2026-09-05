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

use secrecy::SecretString;
use std::{
    env,
    fmt::{self, Debug, Display, Formatter},
    sync::Arc,
    time::Duration,
};
use tracing::{event, Level};

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

    /// Use this method to send requests to Telegram API and retry them after [`RetryAfter`] errors
    ///
    /// [`RetryAfter`]: crate::errors::TelegramErrorKind::RetryAfter
    /// # Arguments
    /// * `method` - Telegram API method
    /// * `max_attempts` - Maximum number of request attempts (including the first one), so `1` means no retries
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    /// # Notes
    /// This method uses default timeout for requests, which is 30 seconds.
    /// If you want to use custom timeout, use [`Bot::send_with_timeout_and_retry`] method.
    ///
    /// Only [`RetryAfter`] errors are retried: the method waits for the number of seconds
    /// returned by Telegram API and then sends the same request again.
    /// Other errors are returned immediately without retries.
    /// If all attempts are exhausted, the last error is returned.
    ///
    /// [`RetryAfter`]: crate::errors::TelegramErrorKind::RetryAfter
    pub async fn send_with_retry<T>(
        &self,
        method: T,
        max_attempts: u32,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Clone + Send + Sync,
        T::Method: Send + Sync,
    {
        self.send_with_retry_inner(method, None, max_attempts).await
    }

    /// Use this method to send requests to Telegram API with timeout
    /// and retry them after [`RetryAfter`] errors
    ///
    /// [`RetryAfter`]: crate::errors::TelegramErrorKind::RetryAfter
    /// # Arguments
    /// * `method` - Telegram API method
    /// * `request_timeout` - Request timeout
    /// * `max_attempts` - Maximum number of request attempts (including the first one), so `1` means no retries
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    /// # Notes
    /// This method uses passed timeout for requests.
    /// If you want to use default timeout, use [`Bot::send_with_retry`] method.
    ///
    /// Only [`RetryAfter`] errors are retried: the method waits for the number of seconds
    /// returned by Telegram API and then sends the same request again.
    /// Other errors are returned immediately without retries.
    /// If all attempts are exhausted, the last error is returned.
    ///
    /// [`RetryAfter`]: crate::errors::TelegramErrorKind::RetryAfter
    pub async fn send_with_timeout_and_retry<T>(
        &self,
        method: T,
        request_timeout: f32,
        max_attempts: u32,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Clone + Send + Sync,
        T::Method: Send + Sync,
    {
        self.send_with_retry_inner(method, Some(request_timeout), max_attempts)
            .await
    }

    async fn send_with_retry_inner<T>(
        &self,
        method: T,
        timeout: Option<f32>,
        max_attempts: u32,
    ) -> Result<T::Return, SessionErrorKind>
    where
        T: TelegramMethod + Clone + Send + Sync,
        T::Method: Send + Sync,
    {
        for attempt in 1..=max_attempts {
            match self
                .client
                .make_request_and_get_result(self, method.clone(), timeout)
                .await
            {
                Err(SessionErrorKind::Telegram(TelegramErrorKind::RetryAfter {
                    retry_after,
                    ..
                })) if attempt < max_attempts => {
                    event!(
                        Level::WARN,
                        attempt,
                        max_attempts,
                        retry_after,
                        "Request was rate limited, retrying after retry_after seconds"
                    );

                    tokio::time::sleep(Duration::from_secs(retry_after.max(0) as u64)).await;
                }
                result => return result,
            }
        }

        unreachable!("the loop returns the result on the last attempt")
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

    const OK_RESPONSE: &str = r#"{"ok":true,"result":42}"#;
    const RATE_LIMITED_RESPONSE: &str = r#"{"ok":false,"description":"Too Many Requests","error_code":429,"parameters":{"retry_after":0}}"#;
    const SERVER_ERROR_RESPONSE: &str =
        r#"{"ok":false,"description":"Internal Server Error","error_code":500}"#;

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
    async fn retry_after_stops_after_max_attempts() {
        let session = MockSession::new(&[
            (RATE_LIMITED_STATUS_CODE, RATE_LIMITED_RESPONSE),
            (RATE_LIMITED_STATUS_CODE, RATE_LIMITED_RESPONSE),
        ]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_retry(TestMethod, 2).await;

        match result {
            Err(SessionErrorKind::Telegram(TelegramErrorKind::RetryAfter {
                retry_after, ..
            })) => {
                assert_eq!(retry_after, 0);
            }
            _ => panic!("Expected RetryAfter error"),
        }

        assert_eq!(attempts(&bot), 2);
    }

    #[tokio::test]
    async fn other_errors_are_not_retried() {
        let session = MockSession::new(&[(SERVER_ERROR_STATUS_CODE, SERVER_ERROR_RESPONSE)]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_retry(TestMethod, 5).await;

        assert!(matches!(
            result,
            Err(SessionErrorKind::Telegram(
                TelegramErrorKind::ServerError { .. }
            ))
        ));
        assert_eq!(attempts(&bot), 1);
    }

    #[tokio::test]
    async fn single_attempt_does_not_retry() {
        let session = MockSession::new(&[(RATE_LIMITED_STATUS_CODE, RATE_LIMITED_RESPONSE)]);
        let bot = Bot::with_client("123:token", session);

        let result = bot.send_with_retry(TestMethod, 1).await;

        assert!(matches!(
            result,
            Err(SessionErrorKind::Telegram(
                TelegramErrorKind::RetryAfter { .. }
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
