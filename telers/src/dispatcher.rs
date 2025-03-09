//! [`Dispatcher`] is the main part of the library, which contains functionality for handling updates and dispatching them to the router.
//! You can create [`Dispatcher`] using [`Builder`].
//!
//! Components of the dispatcher:
//! * [`Bot`]:
//!     Bot is used for sending requests to the Telegram API and receiving updates from the Telegram API.
//!     Usually you need only one bot and one dispatcher, but you can pass multiple bots to the dispatcher and it will work with all of them
//!     with own polling processes.
//! * `Propagator`:
//!     Propagator is abstract component, which is used for propagating events, usually it's [`Router`].
//!     Router combines services and observers and propagates events to them and allows creating complex event handling logic.
//!     See [`router module`] for more information (**recommended**).
//! * `Polling timeout`:
//!     Timeout in seconds for long polling.
//!     By default, it's 30 seconds, but you can change it with [`Builder::polling_timeout`] method.
//!     Polling sends [`GetUpdates`] request to the Telegram API and will wait for `polling_timeout` seconds.
//!     If there are no updates, it will send the same request again, so often as you set it in [`Builder::backoff`] method.
//! * [`ExponentialBackoff`]:
//!     Backoff used for handling server-side errors and network errors (like connection reset or telegram server is down, etc.)
//!     and set timeout between requests to telegram server.
//! * `Allowed updates`:
//!     List the types of updates you want your bot to receive.
//!     For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types.
//!     See [`UpdateType`] for a complete list of available update types.
//!     By default, all update types except [`ChatMember`] are enabled.
//!
//! Dispatcher supports startup and shutdown events.
//! You can register handlers for these observers (startup and shutdown) in the main router and handle them (see [`router module`]).
//! When you call long polling with [`Dispatcher::run_polling`] method, it will emit main router startup event
//! and shutdown event when polling is stopped by signal (**SIGINT** and **SIGTERM** in Unix; **CTRL-C** and **CTRL-BREAK** in Windows).
//! Also, you can emit these events manually with [`Dispatcher::emit_startup`] and [`Dispatcher::emit_shutdown`] methods.
//! See [`Dispatcher::run_polling_without_startup_and_shutdown`] method if you don't need emitting these events.
//!
//! Use [`Dispatcher::feed_update`] and [`Dispatcher::feed_update_with_context`] methods for feeding updates to the dispatcher manually.
//! These methods are useful for testing or if you want to use your own update source.
//! Second method allows you to pass [`Context`] with own data, which will be used in the handlers, middlewares, etc. (see [`context module`] for more information).
//!
//! Check out the examples directory for usage examples.
//!
//! [`Router`]: telers::router::Router
//! [`UpdateType`]: telers::enums::UpdateType
//! [`ChatMember`]: telers::enums::UpdateType::ChatMember
//! [`router module`]: telers::router
//! [`context module`]: telers::context
//! [`Dispatcher::new`]: Dispatcher#method.new
//! [`Builder::polling_timeout`]: Builder#method.polling_timeout
//! [`Builder::backoff`]: Builder#method.backoff
//! [`Dispatcher::run_polling`]: Service#method.run_polling
//! [`Dispatcher::emit_startup`]: Service#method.emit_startup
//! [`Dispatcher::emit_shutdown`]: Service#method.emit_shutdown
//! [`Dispatcher::run_polling_without_startup_and_shutdown`]: Service#method.run_polling_without_startup_and_shutdown
//! [`Dispatcher::feed_update`]: Service#method.feed_update
//! [`Dispatcher::feed_update_with_context`]: Service#method.feed_update_with_context

use super::router::{PropagateEvent, Response};
use crate::{
    client::{Bot, Session},
    context::Context,
    enums::UpdateType,
    errors::{EventErrorKind, HandlerError},
    methods::GetUpdates,
    types::Update,
    Extensions, Request,
};

use backoff::{backoff::Backoff, exponential::ExponentialBackoff, SystemClock};
use futures_util::future::BoxFuture;
use std::{
    future::{Future, IntoFuture},
    sync::Arc,
};
use tokio::{
    select,
    sync::{mpsc, watch},
};
use tracing::{event, field, instrument, Level, Span};

const GET_UPDATES_SIZE: i64 = 100;
const CHANNEL_UPDATES_SIZE: usize = 100;

pub const DEFAULT_POLLING_TIMEOUT: i64 = 30;

/// Dispatcher using to dispatch incoming updates to the main router
#[derive(Clone)]
pub struct Dispatcher<Client, Propagator, BackoffType = ExponentialBackoff<SystemClock>> {
    propagator: Propagator,
    bots: Vec<Bot<Client>>,
    extensions: Extensions,
    context: Context,
    polling_timeout: Option<i64>,
    backoff: BackoffType,
    allowed_updates: Vec<UpdateType>,
}

impl<Client, Propagator> Dispatcher<Client, Propagator>
where
    Propagator: Default,
{
    #[must_use]
    pub fn builder() -> Builder<Client, Propagator> {
        Builder::default()
    }
}

pub struct Builder<Client, Propagator, BackoffType = ExponentialBackoff<SystemClock>> {
    propagator: Propagator,
    bots: Vec<Bot<Client>>,
    context: Context,
    extensions: Extensions,
    polling_timeout: Option<i64>,
    backoff: BackoffType,
    allowed_updates: Vec<UpdateType>,
}

impl<Client, Propagator> Default for Builder<Client, Propagator>
where
    Propagator: Default,
{
    /// Creates a new dispatcher builder with default values
    #[must_use]
    fn default() -> Self {
        Self {
            propagator: Propagator::default(),
            bots: vec![],
            context: Context::new(),
            extensions: Extensions::new(),
            polling_timeout: Some(DEFAULT_POLLING_TIMEOUT),
            backoff: ExponentialBackoff::default(),
            allowed_updates: vec![],
        }
    }
}

impl<Client, Propagator, BackoffType> Builder<Client, Propagator, BackoffType> {
    #[must_use]
    pub fn with_backoff(mut self, backoff: BackoffType) -> Self {
        self.backoff = backoff;
        self
    }
}

impl<Client, Propagator, BackoffType> Builder<Client, Propagator, BackoffType> {
    /// Main router, whose service will propagate updates to the other routers and its observers
    #[must_use]
    pub fn main_router(self, val: Propagator) -> Self
    where
        Propagator: PropagateEvent<Client>,
    {
        Self {
            propagator: val,
            ..self
        }
    }

    /// Main router, whose service will propagate updates to the other routers and its observers
    /// # Notes
    /// Alias to [`Builder::main_router`] method
    #[must_use]
    pub fn router(self, val: Propagator) -> Self
    where
        Propagator: PropagateEvent<Client>,
    {
        self.main_router(val)
    }

    /// Bots that will be used for getting updates and sending requests.
    /// All bots use the same dispatcher, but each bot has the own polling process.
    /// Polling process gets updates and propagates them to the main propagator.
    /// # Notes
    /// You can add multiple bots using [`Builder::bots`] method
    #[must_use]
    pub fn bot(self, val: Bot<Client>) -> Self {
        Self {
            bots: self.bots.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    /// Bots that will be used for getting updates and sending requests.
    /// All bots use the same dispatcher, but each bot has the own polling process.
    /// Polling process gets updates and propagates them to the main propagator.
    /// # Notes
    /// You can add sinlge bot using [`Builder::bot`] method
    #[must_use]
    pub fn bots(self, val: impl IntoIterator<Item = Bot<Client>>) -> Self {
        Self {
            bots: self.bots.into_iter().chain(val).collect(),
            ..self
        }
    }

    /// Insert a type into this [`Extensions`]
    #[must_use]
    pub fn context<T>(mut self, key: &'static str, val: T) -> Self
    where
        T: Clone + Send + Sync + 'static,
    {
        self.context.insert(key, val);
        self
    }

    /// Extend context of dispatcher
    #[must_use]
    pub fn context_extend(mut self, val: Context) -> Self {
        self.context.extend(val);
        self
    }

    /// Insert a type into this [`Extensions`]
    #[must_use]
    pub fn extension<T>(mut self, val: T) -> Self
    where
        T: Clone + Send + Sync + 'static,
    {
        self.extensions.insert(val);
        self
    }

    /// Extend extensions of dispatcher
    #[must_use]
    pub fn extensions_extend(mut self, val: Extensions) -> Self {
        self.extensions.extend(val);
        self
    }

    /// Timeout in seconds for long polling
    /// # Default
    /// [`DEFAULT_POLLING_TIMEOUT`]
    #[must_use]
    pub fn polling_timeout(self, val: i64) -> Self {
        Self {
            polling_timeout: Some(val),
            ..self
        }
    }

    /// Backoff used for handling server-side errors and network errors (like connection reset or telegram server is down, etc.)
    /// and set timeout between requests to telegram server
    #[must_use]
    pub fn backoff(self, val: BackoffType) -> Self {
        Self {
            backoff: val,
            ..self
        }
    }

    /// Update type you want your bot to receive.
    /// For example, specify [`UpdateType::Message`] to only receive this update type.
    /// # Notes
    /// You can add multiple update types using [`Builder::allowed_updates`] method
    #[must_use]
    pub fn allowed_update(self, val: UpdateType) -> Self {
        Self {
            allowed_updates: self.allowed_updates.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    /// List the types of updates you want your bot to receive.
    /// For example, specify [`UpdateType::Message`], [`UpdateType::EditedChannelPost`], [`UpdateType::CallbackQuery`]
    /// to only receive updates of these types.
    /// # Notes
    /// You can add single update type using [`Builder::allowed_update`] method
    #[must_use]
    pub fn allowed_updates(self, val: impl IntoIterator<Item = UpdateType>) -> Self {
        Self {
            allowed_updates: self.allowed_updates.into_iter().chain(val).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> Dispatcher<Client, Propagator, BackoffType> {
        Dispatcher {
            propagator: self.propagator,
            bots: self.bots,
            extensions: self.extensions,
            context: self.context,
            polling_timeout: self.polling_timeout,
            backoff: self.backoff,
            allowed_updates: self.allowed_updates,
        }
    }
}

impl<Client, Propagator, BackoffType> Dispatcher<Client, Propagator, BackoffType> {
    /// Main entry point for incoming updates.
    /// This method will propagate update to the main router.
    #[instrument(skip_all, fields(update_id = update.id, update_type))]
    pub async fn feed_update(
        &mut self,
        bot: Bot<Client>,
        update: Arc<Update>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + Clone + 'static,
        Propagator: PropagateEvent<Client>,
    {
        let update_type = UpdateType::from(update.as_ref());

        Span::current().record("update_type", field::display(&update_type));

        self.propagator
            .propagate_event(
                update_type,
                Request {
                    bot,
                    update,
                    context: self.context.clone(),
                    extensions: self.extensions.clone(),
                },
            )
            .await
    }

    /// Start listening updates for the bot.
    /// [`Update`] is sent to the [`Sender`] channel.
    /// # Errors
    /// If sender channel is disconnected
    #[instrument(skip_all)]
    async fn listen_updates(
        bot: Bot<Client>,
        polling_timeout: Option<i64>,
        allowed_updates: Vec<UpdateType>,
        update_tx: mpsc::Sender<Update>,
        mut backoff: BackoffType,
    ) -> mpsc::error::SendError<Update>
    where
        Client: Session,
        BackoffType: Backoff,
    {
        event!(Level::TRACE, "Start listening updates");

        let mut method = GetUpdates::new()
            .limit(GET_UPDATES_SIZE)
            .timeout_option(polling_timeout)
            .allowed_updates(allowed_updates.iter().map(AsRef::as_ref));

        // Flag for handling connection errors.
        // If it's `true`, we will use backoff algorithm to next backoff.
        // If it's `false`, we will use default backoff algorithm.
        let mut failed = false;

        loop {
            event!(
                Level::TRACE,
                "Send `getUpdates` request to the Telegram server",
            );

            let updates = match bot.send(&method).await {
                Ok(updates) => {
                    // Get last update id to set offset or skip updates if it's empty
                    let Some(Update { id, .. }) = updates.last() else {
                        event!(Level::TRACE, "No updates received");

                        continue;
                    };

                    event!(
                        Level::TRACE,
                        updates_len = updates.len(),
                        last_update_id = id,
                        "Received updates from the Telegram server",
                    );

                    // The `getUpdates` method returns the earliest 100 unconfirmed updates.
                    // To confirm an update, use the offset parameter when calling `getUpdates`.
                    // All updates with `update_id` less than or equal to `offset` will be marked.
                    // as confirmed on the server and will no longer be returned.
                    // So we need to set offset to the last update `id` + 1
                    method.offset = Some(id + 1);

                    updates
                }
                Err(err) => {
                    event!(Level::ERROR, %err, "Failed to fetch updates");

                    // If we failed to fetch updates, we will sleep for a while and try again
                    failed = true;

                    if let Some(duration) = backoff.next_backoff() {
                        event!(
                            Level::WARN,
                            "Sleep for {duration:?} seconds and try again..."
                        );

                        tokio::time::sleep(duration).await;
                    }
                    continue;
                }
            };

            for update in updates {
                event!(Level::TRACE, "Send update to the listener",);

                if let Err(err) = update_tx.send(update).await {
                    return err;
                }
            }

            // If we successfully connected to the server, we will reset backoff config
            if failed {
                event!(Level::INFO, "Connection established successfully");

                backoff.reset();

                // Reset failed flag, because we successfully connected to the server and don't need to use backoff algorithm
                failed = false;
            }
        }
    }

    /// Internal polling process.
    /// Start listening updates for the bot and propagate them to the main router.
    /// # Returns
    /// Guard just should be dropped to stop polling
    #[instrument(skip_all, fields(bot_id = bot.id))]
    fn polling(&self, bot: Bot<Client>) -> impl Drop
    where
        Client: Session + Clone + 'static,
        Propagator: PropagateEvent<Client> + Clone,
        BackoffType: Backoff + Send + Sync + Clone + 'static,
    {
        let (signal_tx, signal_rx) = watch::channel(());
        let (update_tx, mut update_rx) = mpsc::channel(CHANNEL_UPDATES_SIZE);

        let hidden_token = bot.hidden_token.clone();

        tokio::spawn({
            let fut = Self::listen_updates(
                bot.clone(),
                self.polling_timeout,
                self.allowed_updates.clone(),
                update_tx,
                self.backoff.clone(),
            );

            async move {
                select! {
                    _ = signal_tx.closed() => event!(Level::TRACE, "Select signal branch"),
                    _ = fut => event!(Level::TRACE, "Select future branch"),
                };
                event!(Level::WARN, "Graceful shutdown signal received");
            }
        });
        tokio::spawn({
            let dispatcher = self.clone();

            async move {
                while let Some(update) = update_rx.recv().await {
                    event!(
                        Level::TRACE,
                        update_id = update.id,
                        "Received update from the listener"
                    );

                    let update = Arc::new(update);
                    let bot = bot.clone();
                    let mut dispatcher = dispatcher.clone();

                    tokio::spawn(async move { dispatcher.feed_update(bot, update).await });
                }
            }
        });

        event!(Level::INFO, token = hidden_token, "Started");

        signal_rx
    }

    /// External polling process runner for multiple bots and emit startup and shutdown observers
    /// # Errors
    /// - If any startup observer returns error
    /// - If any shutdown observer returns error
    /// # Panics
    /// - If failed to register exit signal handlers
    /// - If bots is empty
    pub fn run_polling(self) -> Serve<Client, Propagator, BackoffType>
    where
        Client: Session + Clone + 'static,
        Propagator: PropagateEvent<Client> + 'static,
        BackoffType: Backoff + Send + Sync + Clone + 'static,
    {
        assert!(
            self.bots.len() > 0,
            "You must add at least one bot to the dispatcher",
        );

        Serve::new(self)
    }
}

pub struct Serve<Client, Propagator, BackoffType> {
    dispatcher: Dispatcher<Client, Propagator, BackoffType>,
}

impl<Client, Propagator, BackoffType> Serve<Client, Propagator, BackoffType> {
    pub const fn new(dispatcher: Dispatcher<Client, Propagator, BackoffType>) -> Self {
        Self { dispatcher }
    }

    pub fn with_graceful_shutdown<Signal>(
        self,
        signal: Signal,
    ) -> ServeWithGracefulShutdown<Client, Propagator, BackoffType, Signal>
    where
        Signal: Future + Send + 'static,
        Signal::Output: Send,
    {
        ServeWithGracefulShutdown::new(self.dispatcher, signal)
    }
}

impl<Client, Propagator, BackoffType> IntoFuture for Serve<Client, Propagator, BackoffType>
where
    Client: Session + Clone + 'static,
    Propagator: PropagateEvent<Client>,
    BackoffType: Backoff + Send + Sync + Clone + 'static,
{
    type Output = Result<(), HandlerError>;
    type IntoFuture = BoxFuture<'static, Self::Output>;

    #[cfg(feature = "default_signal")]
    fn into_future(self) -> Self::IntoFuture {
        use crate::utils::shutdown_signal;

        self.with_graceful_shutdown(shutdown_signal()).into_future()
    }

    #[cfg(not(feature = "default_signal"))]
    fn into_future(self) -> Self::IntoFuture {
        if self.dispatcher.propagator.shutdown_handlers_len() != 0 {
            event!(
                // I'm use target instead of name here because: https://github.com/tokio-rs/tracing/discussions/1587#discussioncomment-1370883
                target: "telers:dispatcher:into_future",
                Level::WARN,
                "Shutdown observer can't be called without graceful shutdow. \
                You can off this log by `telers:dispatcher:into_future=off`.",
            );
        }

        self.with_graceful_shutdown(std::future::pending::<Self::Output>())
            .into_future()
    }
}

pub struct ServeWithGracefulShutdown<Client, Propagator, BackoffType, Signal> {
    dispatcher: Dispatcher<Client, Propagator, BackoffType>,
    signal: Signal,
}

impl<Client, Propagator, BackoffType, Signal>
    ServeWithGracefulShutdown<Client, Propagator, BackoffType, Signal>
{
    pub const fn new(
        dispatcher: Dispatcher<Client, Propagator, BackoffType>,
        signal: Signal,
    ) -> Self {
        Self { dispatcher, signal }
    }
}

impl<Client, Propagator, BackoffType, Signal> IntoFuture
    for ServeWithGracefulShutdown<Client, Propagator, BackoffType, Signal>
where
    Client: Session + Clone + 'static,
    Signal: Future + Send + 'static,
    Signal::Output: Send,
    Propagator: PropagateEvent<Client>,
    BackoffType: Backoff + Send + Sync + Clone + 'static,
{
    type Output = Result<(), HandlerError>;
    type IntoFuture = BoxFuture<'static, Self::Output>;

    fn into_future(mut self) -> Self::IntoFuture {
        Box::pin(async move {
            self.dispatcher.propagator.emit_startup().await?;

            let mut pollings = Vec::with_capacity(self.dispatcher.bots.len());
            for bot in self.dispatcher.bots.clone() {
                pollings.push(self.dispatcher.polling(bot));
            }

            self.signal.await;

            self.dispatcher.propagator.emit_shutdown().await?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        event::bases::{EventReturn, PropagateEventResult},
        router::Router,
    };

    use std::convert::Infallible;
    use tokio;

    #[tokio::test]
    async fn test_feed_update() {
        let bot = Bot::<Reqwest>::default();
        let update = Arc::new(Update::default());

        let router = Router::new("main");
        let mut dispatcher = Dispatcher::builder()
            .main_router(router.configure_default())
            .build();

        let response = dispatcher
            .feed_update(bot.clone(), update.clone())
            .await
            .unwrap();

        // Event shouldn't be handled, because there is no any handler registered
        match response.propagate_result {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        let mut router = Router::new("main");
        router
            .message
            .register(|| async { Ok::<_, Infallible>(EventReturn::Finish) });

        let mut dispatcher = Dispatcher::builder()
            .main_router(router.configure_default())
            .build();

        let response = dispatcher.feed_update(bot.clone(), update).await.unwrap();

        // Event should be handled
        match response.propagate_result {
            PropagateEventResult::Handled(_) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[derive(Clone)]
    struct Test1;

    #[derive(Clone)]
    struct Test2;

    #[derive(Clone)]
    struct Test3;

    #[test]
    fn test_builder() {
        let bot = Bot::<Reqwest>::default();

        let dispatcher = Dispatcher::builder()
            .main_router(Router::new("main").configure_default())
            .bot(bot.clone())
            .bots([bot])
            .extension(Test1)
            .extension(Test2)
            .extensions_extend({
                let mut extensions = Extensions::new();
                extensions.insert(Test3);
                extensions
            })
            .context("1", Test1)
            .context("2", Test2)
            .context_extend({
                let mut context = Context::new();
                context.insert("3", Test3);
                context
            })
            .polling_timeout(123)
            .allowed_update(UpdateType::Message)
            .allowed_updates([UpdateType::InlineQuery, UpdateType::ChosenInlineResult])
            .build();

        assert_eq!(dispatcher.bots.len(), 2);
        assert_eq!(dispatcher.extensions.len(), 3);
        assert_eq!(dispatcher.context.len(), 3);
        assert_eq!(dispatcher.polling_timeout, Some(123));
        assert_eq!(dispatcher.allowed_updates.len(), 3);
    }
}
