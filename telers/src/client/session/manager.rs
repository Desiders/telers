//! This module contains [`SessionManager`], a [`Session`] with outgoing middlewares around it,
//! and the [`Middleware`] trait they are implemented with.
//!
//! Check [module docs](crate::client::session) for more information.

use crate::{
    client::{
        session::base::{ClientResponse, ClientStreamResponse, Session},
        telegram::APIServer,
        Bot,
    },
    errors::SessionErrorKind,
    methods::{Response, TelegramMethod},
};

use futures_util::future::BoxFuture;
use std::sync::Arc;
use tracing::debug_span;

pub mod logging;

pub use self::logging::Logging;

/// The middlewares and the inner session at the end
pub type Next<'a> =
    Box<dyn FnOnce() -> BoxFuture<'a, Result<ClientResponse, anyhow::Error>> + Send + 'a>;

/// An outgoing middleware behind an [`Arc`], so the chain can be stored and cloned as a slice
pub type BoxedMiddleware = Arc<dyn Middleware>;

/// Outgoing middlewares, called around every request a bot sends to the Telegram API.
///
/// A middleware sees the id of the bot the request is made on behalf of, the name of the called
/// method and the response before it is parsed, so logging, metrics and short-circuiting have
/// something to work with. It does not see the parameters of the method.
///
/// A middleware takes `self` by [`Arc`], so a closure cannot be one: a closure is callable, and
/// `Arc<F>` is not. Implement [`Middleware`] for a named type instead.
///
/// # Notes
/// A middleware cannot retry the call itself, because [`Next`] can only be called once. Retrying
/// would need a second copy of the method, and building one a second time is not possible for
/// methods that take files. Use [`Bot::send_with_retry`] instead, it calls the chain again for
/// every attempt.
///
/// Downloading a file is not covered: the method that resolves the file path goes through the
/// chain, but the bytes are streamed by [`Session::stream_content`] straight from the inner
/// session.
///
/// A middleware that sends a request from inside the chain re-enters it, so a middleware that
/// reacts to failures has to not react to its own.
///
/// A middleware sees the response before it is parsed, so a Telegram error arrives as a successful
/// result with an error status code in it. Reading the description or the error code means parsing
/// [`ClientResponse::content`] by hand.
///
/// [`Bot::send_with_retry`]: crate::Bot::send_with_retry
pub trait Middleware: Send + Sync + 'static {
    /// Call the next middleware in the chain or the inner session if this is the last one
    fn call<'a>(
        self: Arc<Self>,
        next: Next<'a>,
        bot_id: i64,
        method_name: &'static str,
    ) -> BoxFuture<'a, Result<ClientResponse, anyhow::Error>>;
}

/// A [`Session`] with outgoing middlewares around it.
///
/// The middlewares live here and not in [`Session`], so a session implementation only has to send
/// a request, and every session gets the same chain.
#[derive(Clone)]
pub struct SessionManager<S> {
    /// The session that sends the request at the end of the chain
    session: S,
    /// The chain, outermost middleware first
    middlewares: Arc<[BoxedMiddleware]>,
}

impl<S> SessionManager<S> {
    /// Wrap a session without any middleware
    #[must_use]
    pub fn new(session: S) -> Self {
        Self {
            session,
            middlewares: Arc::from(Vec::new()),
        }
    }

    /// Add a middleware to the end of the chain
    #[must_use]
    pub fn on_middleware<M>(self, middleware: M) -> Self
    where
        M: Middleware + Send + Sync + 'static,
    {
        let mut middlewares = self.middlewares.to_vec();
        middlewares.push(Arc::new(middleware) as BoxedMiddleware);

        Self {
            session: self.session,
            middlewares: Arc::from(middlewares),
        }
    }
}

impl<S: Session> SessionManager<S> {
    /// Get configuration of Telegram Bot API server endpoints and local mode
    #[must_use]
    pub fn api(&self) -> &APIServer {
        self.session.api()
    }

    /// Get the inner session
    #[cfg(test)]
    #[must_use]
    pub(crate) fn session(&self) -> &S {
        &self.session
    }

    /// Makes a request to Telegram API, calling the middlewares around it
    /// # Arguments
    /// * `bot` - Bot instance for building and sending request, it is mainly used for getting bot token
    /// * `method` - Telegram method for building and sending request
    /// * `timeout` - Request timeout.
    ///   If `None`, then client timeout will be used, which is
    ///   [`DEFAULT_TIMEOUT`](crate::client::session::base::DEFAULT_TIMEOUT) by default.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    pub async fn make_request<Client, T>(
        &self,
        bot: &Bot<Client>,
        method: T,
        timeout: Option<f32>,
    ) -> Result<Response<T::Return>, SessionErrorKind>
    where
        Client: Session,
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        let method_name = method.method_name();
        let send: Next<'_> =
            Box::new(move || Box::pin(self.session.send_request(bot, method, timeout)));

        let next = wrap_to_next(&self.middlewares, bot.id, method_name, send);
        let response = next().await?;

        debug_span!("response", status_code = response.status_code.as_u16()).in_scope(|| {
            let resp = T::build_response(&response.content)?;
            self.session.check_response(&resp, response.status_code)?;
            Ok(resp)
        })
    }

    /// Makes a request to Telegram API and get result from it
    /// # Arguments
    /// * `bot` - Bot instance for building and sending request, it is mainly used for getting bot token
    /// * `method` - Telegram method for building and sending request
    /// * `timeout` - Request timeout.
    ///   If `None`, then client timeout will be used, which is
    ///   [`DEFAULT_TIMEOUT`](crate::client::session::base::DEFAULT_TIMEOUT) by default.
    /// # Errors
    /// - If the request cannot be send or decoded
    /// - If the response cannot be parsed
    /// - If the response represents an Telegram API error
    /// # Panics
    /// Never in practice. The result is unwrapped only after [`Session::check_response`] has
    /// already rejected a response that carried none.
    pub async fn make_request_and_get_result<Client, T>(
        &self,
        bot: &Bot<Client>,
        method: T,
        timeout: Option<f32>,
    ) -> Result<T::Return, SessionErrorKind>
    where
        Client: Session,
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        let response = self.make_request(bot, method, timeout).await?;

        // Unwrap safe because `Session::check_response` checked it
        Ok(response.result.unwrap())
    }

    /// Streams the content of a file from the Telegram Bot API file server
    /// # Arguments
    /// * `url` - URL of the file, usually built by [`APIServer::file_url`]
    /// * `timeout` - Request timeout.
    ///   If `None`, then client timeout will be used, which is [`Session`] default by default.
    /// # Errors
    /// If the request cannot be sent
    pub async fn stream_content(
        &self,
        url: &str,
        timeout: Option<f32>,
    ) -> Result<ClientStreamResponse, anyhow::Error> {
        self.session.stream_content(url, timeout).await
    }

    /// Close the underlying session
    /// # Errors
    /// If the session cannot be closed
    pub async fn close(&self) -> Result<(), anyhow::Error> {
        self.session.close().await
    }
}

/// Wrap the middlewares and the inner session into the [`Next`] function
fn wrap_to_next<'a>(
    middlewares: &'a [BoxedMiddleware],
    bot_id: i64,
    method_name: &'static str,
    send: Next<'a>,
) -> Next<'a> {
    let mut next = send;

    // Wrapping from the end puts the first middleware outermost
    for middleware in middlewares.iter().rev() {
        let middleware = Arc::clone(middleware);
        let inner = next;

        next = Box::new(move || middleware.call(inner, bot_id, method_name));
    }

    next
}
