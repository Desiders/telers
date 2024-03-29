//! [`Dispatcher`] is the main part of the library, which contains functionality for handling updates and dispatching them to the router.
//! You can create [`Dispatcher`] with [`Dispatcher::new`] method or using [`Builder`] (**recommended**).
//!
//! Components of the dispatcher:
//! * [`Bot`]:
//! Bot is used for sending requests to the Telegram API and receiving updates from the Telegram API.
//! Usually you need only one bot and one dispatcher, but you can pass multiple bots to the dispatcher and it will work with all of them
//! with own polling processes.
//! * `Propagator`:
//! Propagator is abstract component, which is used for propagating events, usually it's [`Router`].
//! Router combines services and observers and propagates events to them and allows creating complex event handling logic.
//! See [`router module`] for more information (**recommended**).
//! * `Polling timeout`:
//! Timeout in seconds for long polling.
//! By default, it's 30 seconds, but you can change it with [`Builder::polling_timeout`] method.
//! Polling sends [`GetUpdates`] request to the Telegram API and will wait for `polling_timeout` seconds.
//! If there are no updates, it will send the same request again, so often as you set it in [`Builder::backoff`] method.
//! * [`ExponentialBackoff`]:
//! Backoff used for handling server-side errors and network errors (like connection reset or telegram server is down, etc.)
//! and set timeout between requests to telegram server.
//! * `Allowed updates`:
//! List the types of updates you want your bot to receive.
//! For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types.
//! See [`UpdateType`] for a complete list of available update types.
//! By default, all update types except [`ChatMember`] are enabled.
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
//! [`Router`]: crate::router::Router
//! [`UpdateType`]: crate::enums::UpdateType
//! [`ChatMember`]: crate::enums::UpdateType::ChatMember
//! [`router module`]: crate::router
//! [`context module`]: crate::context
//! [`Dispatcher::new`]: Dispatcher#method.new
//! [`Builder::polling_timeout`]: Builder#method.polling_timeout
//! [`Builder::backoff`]: Builder#method.backoff
//! [`Dispatcher::run_polling`]: Service#method.run_polling
//! [`Dispatcher::emit_startup`]: Service#method.emit_startup
//! [`Dispatcher::emit_shutdown`]: Service#method.emit_shutdown
//! [`Dispatcher::run_polling_without_startup_and_shutdown`]: Service#method.run_polling_without_startup_and_shutdown
//! [`Dispatcher::feed_update`]: Service#method.feed_update
//! [`Dispatcher::feed_update_with_context`]: Service#method.feed_update_with_context

use super::router::{PropagateEvent, Request, Response};

use crate::{
    client::{Bot, Session},
    context::Context,
    enums::UpdateType,
    errors::EventErrorKind,
    event::{
        service::{ServiceProvider, ToServiceProvider},
        simple::HandlerResult as SimpleHandlerResult,
    },
    methods::GetUpdates,
    types::Update,
};

use backoff::{backoff::Backoff, exponential::ExponentialBackoff, SystemClock};
use std::sync::Arc;
use thiserror;
use tokio::sync::mpsc::{channel as mspc_channel, error::SendError, Sender};
use tracing::{event, field, instrument, Level, Span};

const GET_UPDATES_SIZE: i64 = 100;
const CHANNEL_UPDATES_SIZE: usize = 100;

pub const DEFAULT_POLLING_TIMEOUT: i64 = 30;

#[derive(Debug, thiserror::Error)]
enum ListenerError<T> {
    #[error(transparent)]
    SendError(#[from] SendError<T>),
}

#[derive(Debug, thiserror::Error)]
enum PollingError {
    #[error("Polling was aborted by signal")]
    Aborted,
}

/// Dispatcher using to dispatch incoming updates to the main router
pub struct Dispatcher<Client, Propagator, BackoffType = ExponentialBackoff<SystemClock>> {
    main_router: Propagator,
    bots: Box<[Bot<Client>]>,
    polling_timeout: Option<i64>,
    backoff: BackoffType,
    allowed_updates: Box<[UpdateType]>,
}

impl<Client, Propagator, BackoffType> Dispatcher<Client, Propagator, BackoffType> {
    /// Creates new dispatcher
    /// # Arguments
    /// * `main_router` -
    /// Main router, whose service will propagate updates to the other routers and its observers
    /// * `bots` -
    /// Bots that will be used for getting updates and sending requests.
    /// All bots use the same dispatcher, but each bot has the own polling process.
    /// Polling process gets updates and propagates them to the main propagator.
    /// * `polling_timeout` -
    /// Timeout in seconds for long polling
    /// * `backoff` -
    /// Backoff used for handling server-side errors and network errors (like connection reset or telegram server is down, etc.)
    /// and set timeout between requests to telegram server
    /// * `allowed_updates` -
    /// List the types of updates you want your bot to receive.
    /// For example, specify [`UpdateType::Message`], [`UpdateType::EditedChannelPost`], [`UpdateType::CallbackQuery`]
    /// to only receive updates of these types.
    #[must_use]
    pub fn new<Cfg, PropagatorService, InitError>(
        main_router: Propagator,
        bots: impl IntoIterator<Item = Bot<Client>>,
        polling_timeout: Option<i64>,
        backoff: BackoffType,
        allowed_updates: impl IntoIterator<Item = UpdateType>,
    ) -> Self
    where
        BackoffType: Backoff,
        Propagator: ToServiceProvider<
            Config = Cfg,
            ServiceProvider = PropagatorService,
            InitError = InitError,
        >,
        PropagatorService: PropagateEvent<Client>,
    {
        Self {
            main_router,
            bots: bots.into_iter().collect(),
            polling_timeout,
            backoff,
            allowed_updates: allowed_updates.into_iter().collect(),
        }
    }
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

impl<Client, Propagator, BackoffType> Dispatcher<Client, Propagator, BackoffType>
where
    Propagator: Default,
{
    #[must_use]
    pub fn builder_with_backoff(val: BackoffType) -> Builder<Client, Propagator, BackoffType> {
        Builder::default_with_backoff(val)
    }
}

pub struct Builder<Client, Propagator, BackoffType = ExponentialBackoff<SystemClock>> {
    main_router: Propagator,
    bots: Vec<Bot<Client>>,
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
            main_router: Propagator::default(),
            bots: vec![],
            polling_timeout: Some(DEFAULT_POLLING_TIMEOUT),
            backoff: ExponentialBackoff::default(),
            allowed_updates: vec![],
        }
    }
}

impl<Client, Propagator, BackoffType> Builder<Client, Propagator, BackoffType>
where
    Propagator: Default,
{
    #[must_use]
    pub fn default_with_backoff(backoff: BackoffType) -> Self {
        Self {
            main_router: Propagator::default(),
            bots: vec![],
            polling_timeout: Some(DEFAULT_POLLING_TIMEOUT),
            backoff,
            allowed_updates: vec![],
        }
    }
}

impl<Client, Propagator, BackoffType> Builder<Client, Propagator, BackoffType> {
    /// Main router, whose service will propagate updates to the other routers and its observers
    #[must_use]
    pub fn main_router<Cfg, PropagatorService, InitError>(self, val: Propagator) -> Self
    where
        Propagator: ToServiceProvider<
            Config = Cfg,
            ServiceProvider = PropagatorService,
            InitError = InitError,
        >,
        PropagatorService: PropagateEvent<Client>,
    {
        Self {
            main_router: val,
            ..self
        }
    }

    /// Main router, whose service will propagate updates to the other routers and its observers
    /// # Notes
    /// Alias to [`Builder::main_router`] method
    #[must_use]
    pub fn router<Cfg, PropagatorService, InitError>(self, val: Propagator) -> Self
    where
        Propagator: ToServiceProvider<
            Config = Cfg,
            ServiceProvider = PropagatorService,
            InitError = InitError,
        >,
        PropagatorService: PropagateEvent<Client>,
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
            main_router: self.main_router,
            bots: self.bots.into(),
            polling_timeout: self.polling_timeout,
            backoff: self.backoff,
            allowed_updates: self.allowed_updates.into_iter().collect(),
        }
    }
}

/// This converts all dependencies to [`ServiceProvider`] and creates [`Arc<Service>`]
/// that contains converted [`ServiceProvider`]s.
impl<Client, BackoffType, PropagatorService, Propagator, Cfg, InitError> ToServiceProvider
    for Dispatcher<Client, Propagator, BackoffType>
where
    Client: Send + Sync + 'static,
    Propagator:
        ToServiceProvider<Config = Cfg, ServiceProvider = PropagatorService, InitError = InitError>,
{
    type Config = Cfg;
    type ServiceProvider = Arc<Service<Client, PropagatorService, BackoffType>>;
    type InitError = InitError;

    fn to_service_provider(
        self,
        config: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        Ok(Arc::new(Service {
            main_router: self.main_router.to_service_provider(config)?,
            bots: self.bots,
            polling_timeout: self.polling_timeout,
            backoff: self.backoff,
            allowed_updates: self.allowed_updates,
        }))
    }
}

pub struct Service<Client, PropagatorService, BackoffType> {
    main_router: PropagatorService,
    bots: Box<[Bot<Client>]>,
    polling_timeout: Option<i64>,
    backoff: BackoffType,
    allowed_updates: Box<[UpdateType]>,
}

impl<Client, PropagatorService, BackoffType> ServiceProvider
    for Service<Client, PropagatorService, BackoffType>
{
}

impl<Client, PropagatorService, BackoffType> Service<Client, PropagatorService, BackoffType> {
    /// Main entry point for incoming updates.
    /// This method will propagate update to the main router.
    #[instrument(skip(self, bot, update))]
    pub async fn feed_update(
        self: Arc<Self>,
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static,
        PropagatorService: PropagateEvent<Client>,
    {
        self.feed_update_with_context(bot, update, Arc::new(Context::default()))
            .await
    }

    /// Main entry point for incoming updates with user context.
    /// This method will propagate update to the main router.
    #[instrument(
        skip(self, bot, update, context),
        fields(bot_id, update_id, update_type)
    )]
    pub async fn feed_update_with_context(
        self: Arc<Self>,
        bot: Arc<Bot<Client>>,
        update: Arc<Update>,
        context: Arc<Context>,
    ) -> Result<Response<Client>, EventErrorKind>
    where
        Client: Send + Sync + 'static,
        PropagatorService: PropagateEvent<Client>,
    {
        let update_type = UpdateType::from(update.as_ref());

        Span::current()
            .record("bot_id", bot.bot_id)
            .record("update_id", update.id)
            .record("update_type", field::debug(&update_type));

        self.main_router
            .propagate_event(update_type, Request::new(bot, update, context))
            .await
    }

    /// Start listening updates for the bot.
    /// [`Update`] is sent to the [`Sender`] channel.
    /// # Errors
    /// If sender channel is disconnected
    #[instrument(skip(bot, polling_timeout, allowed_updates, update_sender, backoff))]
    async fn listen_updates(
        bot: Arc<Bot<Client>>,
        polling_timeout: Option<i64>,
        allowed_updates: Box<[UpdateType]>,
        update_sender: Sender<Update>,
        mut backoff: BackoffType,
    ) -> Result<(), ListenerError<Update>>
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
                    // `unwrap` is safe here, because we checked that updates isn't empty
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

                // `Box` is used to avoid stack overflow, because `Update` is a big struct
                update_sender.send(update).await?;
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
    /// Wait exit signal to stop polling.
    /// # Panics
    /// If failed to register exit signal handlers
    #[instrument(skip(self, bot), fields(bot_id = bot.bot_id))]
    async fn polling(self: Arc<Self>, bot: Bot<Client>) -> PollingError
    where
        Client: Session + 'static,
        PropagatorService: PropagateEvent<Client> + 'static,
        BackoffType: Backoff + Send + Sync + Clone + 'static,
    {
        let bot = Arc::new(bot);

        let (sender_update, mut receiver_update) = mspc_channel(CHANNEL_UPDATES_SIZE);

        let listen_updates_handle = tokio::spawn(Self::listen_updates(
            Arc::clone(&bot),
            self.polling_timeout,
            self.allowed_updates.clone(),
            sender_update,
            self.backoff.clone(),
        ));

        let receiver_updates_handle = tokio::spawn(async move {
            while let Some(update) = receiver_update.recv().await {
                event!(
                    Level::TRACE,
                    update_id = update.id,
                    "Received update from the listener"
                );

                let dispatcher = Arc::clone(&self);
                let bot = Arc::clone(&bot);

                tokio::spawn(dispatcher.feed_update(bot, Arc::new(update)));
            }
        });

        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};

            let mut sigint =
                signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");
            let mut sigterm =
                signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

            tokio::select! {
                _ = sigint.recv() => {
                    event!(Level::WARN, "SIGINT signal received");
                },
                _ = sigterm.recv() => {
                    event!(Level::WARN, "SIGTERM signal received");
                },
            }
        }
        #[cfg(windows)]
        {
            use tokio::signal::windows::{ctrl_break, ctrl_c};

            let mut ctrl_c = ctrl_c().expect("Failed to register CTRL+C handler");
            let mut ctrl_break = ctrl_break().expect("Failed to register CTRL+BREAK handler");

            tokio::select! {
                _ = ctrl_c.recv() => {
                    event!(Level::WARN, "CTRL+C signal received");
                },
                _ = ctrl_break.recv() => {
                    event!(Level::WARN,  "CTRL+BREAK signal received");
                },
            }
        }

        #[cfg(any(unix, windows))]
        {
            listen_updates_handle.abort();
            receiver_updates_handle.abort();

            PollingError::Aborted
        }
        #[cfg(not(any(unix, windows)))]
        {
            event!(
                Level::WARN,
                "Exit signals of this platform are not supported, \
                so polling process will never stop by signal and shutdown events will never be emitted.",
            );

            listen_updates_handle.await;
            receiver_updates_handle.await;

            unimplemented!("Exit signals of this platform are not supported");
        }
    }

    /// External polling process runner for multiple bots and emit startup and shutdown observers
    /// # Errors
    /// - If any startup observer returns error
    /// - If any shutdown observer returns error
    /// # Panics
    /// - If failed to register exit signal handlers
    /// - If bots is empty
    #[instrument(skip(self))]
    pub async fn run_polling(self: Arc<Self>) -> Result<(), EventErrorKind>
    where
        Client: Session + Clone + 'static,
        PropagatorService: PropagateEvent<Client> + 'static,
        BackoffType: Backoff + Send + Sync + Clone + 'static,
    {
        event!(Level::TRACE, "Start emit startup observers");

        if let Err(err) = self.main_router.emit_startup().await {
            event!(Level::ERROR, error = %err, "Error while emit startup");

            return Err(err.into());
        }

        let dispatcher = Arc::clone(&self);
        dispatcher.run_polling_without_startup_and_shutdown().await;

        event!(Level::TRACE, "Start emit shutdown observers");

        self.emit_shutdown().await.map_err(|err| {
            event!(Level::ERROR, error = %err, "Error while emit shutdown");

            err.into()
        })
    }

    /// External polling process runner for multiple bots
    /// # Panics
    /// If bots is empty
    #[instrument(skip(self))]
    pub async fn run_polling_without_startup_and_shutdown(self: Arc<Self>)
    where
        Client: Session + Clone + 'static,
        PropagatorService: PropagateEvent<Client> + 'static,
        BackoffType: Backoff + Send + Sync + Clone + 'static,
    {
        let bots = self.bots.clone();
        let bots_len = bots.len();

        assert!(
            bots_len > 0,
            "You must add at least one bot to the dispatcher",
        );

        let mut handles = Vec::with_capacity(bots_len);
        for bot in bots.into_vec() {
            let dispatcher = Arc::clone(&self);

            event!(Level::INFO, bot = %bot, "Polling is started for bot");

            handles.push(tokio::spawn(dispatcher.polling(bot)));
        }

        for handle in handles {
            if let Err(err) = handle.await {
                event!(Level::ERROR, error = %err);
            }
        }

        if bots_len == 1 {
            event!(Level::WARN, "Polling is finished for the bot");
        } else {
            event!(Level::WARN, "Polling is finished for the bots");
        }
    }

    /// Emit startup events.
    /// Use this method if you want to emit startup events manually
    /// # Notes
    /// This method is called automatically in `run_polling` method,
    /// but not in `run_polling_without_startup_and_shutdown` method
    /// # Errors
    /// If any startup observer returns error
    #[instrument(skip(self))]
    pub async fn emit_startup(&self) -> SimpleHandlerResult
    where
        PropagatorService: PropagateEvent<Client>,
    {
        self.main_router.emit_startup().await
    }

    /// Emit shutdown events.
    /// Use this method if you want to emit shutdown events manually
    /// # Notes
    /// This method is called automatically in `run_polling` method,
    /// but not in `run_polling_without_startup_and_shutdown` method
    /// # Errors
    /// If any shutdown observer returns error
    #[instrument(skip(self))]
    pub async fn emit_shutdown(&self) -> SimpleHandlerResult
    where
        PropagatorService: PropagateEvent<Client>,
    {
        self.main_router.emit_shutdown().await
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

    use tokio;

    #[tokio::test]
    async fn test_feed_update() {
        let bot = Arc::new(Bot::<Reqwest>::default());
        let update = Arc::new(Update::default());

        let router = Router::new("main");
        let dispatcher = Dispatcher::builder()
            .main_router(router)
            .build()
            .to_service_provider_default()
            .unwrap();

        let response = dispatcher
            .feed_update(Arc::clone(&bot), Arc::clone(&update))
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
            .register(|| async { Ok(EventReturn::Finish) });

        let dispatcher = Dispatcher::builder()
            .main_router(router)
            .build()
            .to_service_provider_default()
            .unwrap();

        let response = Arc::clone(&dispatcher)
            .feed_update(Arc::clone(&bot), update)
            .await
            .unwrap();

        // Event should be handled
        match response.propagate_result {
            PropagateEventResult::Handled(_) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_builder() {
        let bot = Bot::<Reqwest>::default();

        let dispatcher = Dispatcher::builder()
            .main_router(Router::new("main"))
            .bot(bot.clone())
            .bots([bot])
            .polling_timeout(123)
            .allowed_update(UpdateType::Message)
            .allowed_updates([UpdateType::InlineQuery, UpdateType::ChosenInlineResult])
            .build();

        assert_eq!(dispatcher.bots.len(), 2);
        assert_eq!(dispatcher.polling_timeout, Some(123));
        assert_eq!(dispatcher.allowed_updates.len(), 3);
    }
}
