//! [`Dispatcher`] is the main part of the library, which contains functionality for handling updates and dispatching them to the router.
//! You can create [`Dispatcher`] with [`Dispatcher::new`] method or using [`DispatcherBuilder`] (**recommended**).
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
//! By default, it's 30 seconds, but you can change it with [`DispatcherBuilder::polling_timeout`] method.
//! Polling sends [`GetUpdates`] request to the Telegram API and will wait for `polling_timeout` seconds.
//! If there are no updates, it will send the same request again, so often as you set it in [`DispatcherBuilder::backoff`] method.
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
//! [`DispatcherBuilder::polling_timeout`]: DispatcherBuilder#method.polling_timeout
//! [`DispatcherBuilder::backoff`]: DispatcherBuilder#method.backoff
//! [`Dispatcher::run_polling`]: DispatcherService#method.run_polling
//! [`Dispatcher::emit_startup`]: DispatcherService#method.emit_startup
//! [`Dispatcher::emit_shutdown`]: DispatcherService#method.emit_shutdown
//! [`Dispatcher::run_polling_without_startup_and_shutdown`]: DispatcherService#method.run_polling_without_startup_and_shutdown
//! [`Dispatcher::feed_update`]: DispatcherService#method.feed_update
//! [`Dispatcher::feed_update_with_context`]: DispatcherService#method.feed_update_with_context

use super::router::{PropagateEvent, Request, Response};

use crate::{
    client::{Bot, Session},
    context::Context,
    errors::{EventErrorKind, UnknownUpdateTypeError},
    event::{
        service::{ServiceProvider, ToServiceProvider},
        simple::HandlerResult as SimpleHandlerResult,
    },
    methods::GetUpdates,
    types::Update,
};

use backoff::{backoff::Backoff as _, exponential::ExponentialBackoff, SystemClock};
use log::{error, info, warn};
use std::sync::Arc;
use thiserror;
use tokio::sync::mpsc::{channel as mspc_channel, error::SendError, Sender};

const GET_UPDATES_SIZE: i64 = 100;
const DEFAULT_POLLING_TIMEOUT: i64 = 30;

#[derive(thiserror::Error, Debug)]
enum ListenerError<T> {
    #[error(transparent)]
    SendError(#[from] SendError<T>),
}

#[derive(thiserror::Error, Debug)]
enum PollingError {
    #[error("Polling was aborted by signal")]
    Aborted,
}

/// Dispatcher using to dispatch incoming updates to the main router
pub struct Dispatcher<Client, Propagator> {
    main_router: Propagator,
    bots: Vec<Bot<Client>>,
    polling_timeout: Option<i64>,
    backoff: ExponentialBackoff<SystemClock>,
    allowed_updates: Vec<String>,
}

impl<Client, Propagator> Dispatcher<Client, Propagator> {
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
    /// For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types.
    /// See [`crate::enums::UpdateType`] for a complete list of available update types.
    /// By default, all update types except [`crate::enums::UpdateType::ChatMember`] are enabled.
    #[must_use]
    pub fn new<Cfg, PropagatorService, InitError>(
        main_router: Propagator,
        bots: Vec<Bot<Client>>,
        polling_timeout: Option<i64>,
        backoff: ExponentialBackoff<SystemClock>,
        allowed_updates: Vec<impl Into<String>>,
    ) -> Self
    where
        Propagator: ToServiceProvider<
            Config = Cfg,
            ServiceProvider = PropagatorService,
            InitError = InitError,
        >,
        PropagatorService: PropagateEvent<Client>,
    {
        Self {
            main_router,
            bots,
            polling_timeout,
            backoff,
            allowed_updates: allowed_updates.into_iter().map(Into::into).collect(),
        }
    }
}

impl<Client, Propagator> Dispatcher<Client, Propagator>
where
    Propagator: Default,
{
    /// Creates dispatcher builder with default values
    #[must_use]
    pub fn builder() -> DispatcherBuilder<Client, Propagator> {
        DispatcherBuilder::default()
    }
}

impl<Client, Propagator> Default for Dispatcher<Client, Propagator>
where
    Propagator: Default,
{
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

#[allow(clippy::module_name_repetitions)]
pub struct DispatcherBuilder<Client, Propagator> {
    main_router: Propagator,
    bots: Vec<Bot<Client>>,
    polling_timeout: Option<i64>,
    backoff: ExponentialBackoff<SystemClock>,
    allowed_updates: Vec<String>,
}

impl<Client, Propagator> Default for DispatcherBuilder<Client, Propagator>
where
    Propagator: Default,
{
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

impl<Client, Propagator> DispatcherBuilder<Client, Propagator> {
    /// Main router, whose service will propagate updates to the other routers and its observers
    #[must_use]
    pub fn main_router<Cfg, PropagatorService, InitPropagatorServiceError>(
        self,
        val: Propagator,
    ) -> Self
    where
        Propagator: ToServiceProvider<
            Config = Cfg,
            ServiceProvider = PropagatorService,
            InitError = InitPropagatorServiceError,
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
    /// Alias to [`DispatcherBuilder::main_router`] method
    #[must_use]
    pub fn router<Cfg, PropagatorService, InitPropagatorServiceError>(self, val: Propagator) -> Self
    where
        Propagator: ToServiceProvider<
            Config = Cfg,
            ServiceProvider = PropagatorService,
            InitError = InitPropagatorServiceError,
        >,
        PropagatorService: PropagateEvent<Client>,
    {
        self.main_router(val)
    }

    /// Bots that will be used for getting updates and sending requests.
    /// All bots use the same dispatcher, but each bot has the own polling process.
    /// Polling process gets updates and propagates them to the main propagator.
    /// # Notes
    /// You can add multiple bots using [`DispatcherBuilder::bots`] method
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
    /// You can add sinlge bot using [`DispatcherBuilder::bot`] method
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
    pub fn backoff(self, val: ExponentialBackoff<SystemClock>) -> Self {
        Self {
            backoff: val,
            ..self
        }
    }

    /// Update type you want your bot to receive.
    /// For example, specify `message` to only receive this update type.
    /// See [`crate::enums::UpdateType`] for a complete list of available update types.
    /// # Default
    /// All update types except [`crate::enums::UpdateType::ChatMember`] are enabled.
    /// # Notes
    /// You can use [`crate::enums::UpdateType`] and pass it to this method, because it implements [`Into<String>`]
    ///
    /// You can add multiple update types using [`DispatcherBuilder::allowed_updates`] method
    #[must_use]
    pub fn allowed_update(self, val: impl Into<String>) -> Self {
        Self {
            allowed_updates: self
                .allowed_updates
                .into_iter()
                .chain(Some(val.into()))
                .collect(),
            ..self
        }
    }

    /// List the types of updates you want your bot to receive.
    /// For example, specify `message`, `edited_channel_post`, `callback_query` to only receive updates of these types.
    /// See [`crate::enums::UpdateType`] for a complete list of available update types.
    /// # Default
    /// All update types except [`crate::enums::UpdateType::ChatMember`] are enabled.
    /// # Notes
    /// You can use [`crate::enums::UpdateType`] and pass it to this method, because it implements [`Into<String>`]
    ///
    /// You can add single update type using [`DispatcherBuilder::allowed_update`] method
    #[must_use]
    pub fn allowed_updates<T, I>(self, val: I) -> Self
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
    {
        Self {
            allowed_updates: self
                .allowed_updates
                .into_iter()
                .chain(val.into_iter().map(Into::into))
                .collect(),
            ..self
        }
    }

    /// Build [`Dispatcher`] with provided configuration
    #[must_use]
    pub fn build(self) -> Dispatcher<Client, Propagator> {
        Dispatcher {
            main_router: self.main_router,
            bots: self.bots,
            polling_timeout: self.polling_timeout,
            backoff: self.backoff,
            allowed_updates: self.allowed_updates,
        }
    }
}

/// This converts all dependencies to [`ServiceProvider`] and creates [`Arc<DispatcherService>`]
/// that contains converted [`ServiceProvider`]s.
impl<Client, PropagatorService, Propagator, Cfg, InitPropagatorServiceError> ToServiceProvider
    for Dispatcher<Client, Propagator>
where
    Client: Send + Sync + 'static,
    Propagator: ToServiceProvider<
        Config = Cfg,
        ServiceProvider = PropagatorService,
        InitError = InitPropagatorServiceError,
    >,
{
    type Config = Cfg;
    type ServiceProvider = Arc<DispatcherService<Client, PropagatorService>>;
    type InitError = InitPropagatorServiceError;

    #[allow(unknown_lints)]
    #[allow(clippy::arc_with_non_send_sync)]
    fn to_service_provider(
        self,
        config: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        let main_router = self.main_router.to_service_provider(config)?;

        Ok(Arc::new(DispatcherService {
            main_router,
            bots: self.bots,
            polling_timeout: self.polling_timeout,
            backoff: self.backoff,
            allowed_updates: self.allowed_updates,
        }))
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct DispatcherService<Client, PropagatorService> {
    main_router: PropagatorService,
    bots: Vec<Bot<Client>>,
    polling_timeout: Option<i64>,
    backoff: ExponentialBackoff<SystemClock>,
    allowed_updates: Vec<String>,
}

impl<Client, PropagatorService> ServiceProvider for DispatcherService<Client, PropagatorService> {}

impl<Client, PropagatorService> DispatcherService<Client, PropagatorService> {
    /// Main entry point for incoming updates.
    /// This method will propagate update to the main router.
    /// # Errors
    /// - [`UnknownUpdateTypeError`] if update type is unknown
    /// - [`EventErrorKind`] if propagation event returns error
    pub async fn feed_update<B, U>(
        self: Arc<Self>,
        bot: B,
        update: U,
    ) -> Result<Result<Response<Client>, EventErrorKind>, UnknownUpdateTypeError>
    where
        B: Into<Arc<Bot<Client>>>,
        U: Into<Arc<Update>>,
        Client: Send + Sync + 'static,
        PropagatorService: PropagateEvent<Client>,
    {
        self.feed_update_with_context(bot, update, Context::default())
            .await
    }

    /// Main entry point for incoming updates with user context.
    /// This method will propagate update to the main router.
    /// # Errors
    /// - [`UnknownUpdateTypeError`] if update type is unknown
    /// - [`EventErrorKind`] if propagation event returns error
    pub async fn feed_update_with_context<B, U, C>(
        self: Arc<Self>,
        bot: B,
        update: U,
        context: C,
    ) -> Result<Result<Response<Client>, EventErrorKind>, UnknownUpdateTypeError>
    where
        B: Into<Arc<Bot<Client>>>,
        U: Into<Arc<Update>>,
        C: Into<Arc<Context>>,
        Client: Send + Sync + 'static,
        PropagatorService: PropagateEvent<Client>,
    {
        let update: Arc<Update> = update.into();

        let update_type = match update.as_ref().try_into() {
            Ok(update_type) => update_type,
            Err(err) => {
                error!(target: module_path!(), "{err}");

                return Err(err);
            }
        };

        Ok(self
            .main_router
            .propagate_event(update_type, Request::new(bot, update, context.into()))
            .await)
    }

    /// Start listening updates for the bot.
    /// [`Update`] is sent to the [`Sender`] channel.
    /// # Errors
    /// If sender channel is disconnected
    async fn listen_updates<T, I>(
        bot: Arc<Bot<Client>>,
        polling_timeout: Option<i64>,
        allowed_updates: I,
        update_sender: Sender<Box<Update>>,
        mut backoff: ExponentialBackoff<SystemClock>,
    ) -> Result<(), ListenerError<Box<Update>>>
    where
        T: Into<String>,
        I: IntoIterator<Item = T>,
        Client: Session,
    {
        let mut method = GetUpdates::new()
            .limit(GET_UPDATES_SIZE)
            .timeout_option(polling_timeout)
            .allowed_updates(allowed_updates);

        // Flag for handling connection errors.
        // If it's true, we will use exponential backoff algorithm to next backoff.
        // If it's false, we will use default backoff algorithm.
        let mut failed = false;

        loop {
            let updates = match bot.send(&method, None).await {
                Ok(updates) => {
                    if updates.is_empty() {
                        continue;
                    }

                    updates
                }
                Err(err) => {
                    error!(target: module_path!(), "Failed to fetch updates: {err}");

                    failed = true;

                    if let Some(backoff) = backoff.next_backoff() {
                        warn!(target: module_path!(), "Sleep for {backoff:?} seconds and try again...");
                        tokio::time::sleep(backoff).await;
                    }
                    continue;
                }
            };

            // The `getUpdates` method returns the earliest 100 unconfirmed updates.
            // To confirm an update, use the offset parameter when calling `getUpdates`.
            // All updates with `update_id` less than or equal to `offset` will be marked.
            // as confirmed on the server and will no longer be returned.
            // So we need to set offset to the last update id + 1
            // `unwrap` is safe here, because we checked that updates isn't empty
            method.offset = Some(updates.last().unwrap().update_id + 1);

            for update in updates {
                // `Box` is used to avoid stack overflow, because `Update` is a big struct
                update_sender.send(Box::new(update)).await?;
            }

            // If we successfully connected to the server, we will reset backoff config
            if failed {
                info!(target: module_path!(), "Connection established successfully");

                failed = false;

                backoff.reset();
            }
        }
    }

    /// Internal polling process.
    /// Start listening updates for the bot and propagate them to the main router.
    /// Wait exit signal to stop polling.
    /// # Panics
    /// If failed to register exit signal handlers
    async fn polling(self: Arc<Self>, bot: Bot<Client>) -> PollingError
    where
        Client: Session + 'static,
        PropagatorService: PropagateEvent<Client> + 'static,
    {
        let bot = Arc::new(bot);

        let (sender_update, mut receiver_update) =
            mspc_channel(GET_UPDATES_SIZE.try_into().unwrap());

        let listen_updates_handle = tokio::spawn(Self::listen_updates(
            Arc::clone(&bot),
            self.polling_timeout,
            self.allowed_updates.clone(),
            sender_update,
            self.backoff.clone(),
        ));

        let receiver_updates_handle = tokio::spawn(async move {
            while let Some(update) = receiver_update.recv().await {
                let dispatcher = Arc::clone(&self);
                let bot = Arc::clone(&bot);

                tokio::spawn(dispatcher.feed_update(bot, update));
            }
        });

        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};

            let mut sigint = signal(SignalKind::interrupt()).expect(
                "Failed to register SIGINT handler. \
                This is a bug, please report it.",
            );
            let mut sigterm = signal(SignalKind::terminate()).expect(
                "Failed to register SIGTERM handler. \
                This is a bug, please report it.",
            );

            tokio::select! {
                _ = sigint.recv() => {
                    warn!(target: module_path!(), "SIGINT signal received");
                },
                _ = sigterm.recv() => {
                    warn!(target: module_path!(), "SIGTERM signal received");
                },
            }
        }
        #[cfg(windows)]
        {
            use tokio::signal::windows::{ctrl_break, ctrl_c};

            let mut ctrl_c = ctrl_c().expect(
                "Failed to register CTRL+C handler. \
                This is a bug, please report it.",
            );
            let mut ctrl_break = ctrl_break().expect(
                "Failed to register CTRL+BREAK handler. \
                This is a bug, please report it.",
            );

            tokio::select! {
                _ = ctrl_c.recv() => {
                    warn!(target: module_path!(), "CTRL+C signal received");
                },
                _ = ctrl_break.recv() => {
                    warn!(target: module_path!(), "CTRL+BREAK signal received");
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
            warn!(
                target: module_path!(),
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
    pub async fn run_polling(self: Arc<Self>) -> Result<(), EventErrorKind>
    where
        Client: Session + Clone + 'static,
        PropagatorService: PropagateEvent<Client> + 'static,
    {
        if let Err(err) = self.main_router.emit_startup().await {
            error!(target: module_path!(), "Error while emit startup: {err}");

            return Err(err.into());
        }

        let dispatcher = Arc::clone(&self);
        dispatcher.run_polling_without_startup_and_shutdown().await;

        self.emit_shutdown().await.map_err(|err| {
            error!(target: module_path!(), "Error while emit shutdown: {err}");

            err.into()
        })
    }

    /// External polling process runner for multiple bots
    /// # Panics
    /// If bots is empty
    pub async fn run_polling_without_startup_and_shutdown(self: Arc<Self>)
    where
        Client: Session + Clone + 'static,
        PropagatorService: PropagateEvent<Client> + 'static,
    {
        let bots = self.bots.clone();
        let bots_len = bots.len();

        assert!(
            bots_len > 0,
            "You must add at least one bot to the dispatcher"
        );

        let mut handles = Vec::with_capacity(bots_len);
        for bot in bots {
            let dispatcher = Arc::clone(&self);

            info!(target: module_path!(), "Polling is started for bot: {bot}");

            handles.push(tokio::spawn(dispatcher.polling(bot)));
        }

        for handle in handles {
            if let Err(err) = handle.await {
                error!(target: module_path!(), "Task failed to execute to completion: {err}");
            }
        }

        if bots_len == 1 {
            warn!(target: module_path!(), "Polling is finished");
        } else {
            warn!(target: module_path!(), "Polling is finished for all bots");
        }
    }

    /// Emit startup events.
    /// Use this method if you want to emit startup events manually
    /// # Notes
    /// This method is called automatically in `run_polling` method,
    /// but not in `run_polling_without_startup_and_shutdown` method
    /// # Errors
    /// If any startup observer returns error
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
        enums::UpdateType,
        event::bases::{EventReturn, PropagateEventResult},
        router::Router,
        types::Message,
    };

    use tokio;

    #[tokio::test]
    async fn test_feed_update() {
        let bot = Arc::new(Bot::<Reqwest>::default());
        let update = Arc::new(Update {
            message: Some(Message::default()),
            ..Default::default()
        });

        let router = Router::new("main");
        let dispatcher = Dispatcher::builder()
            .main_router(router)
            .build()
            .to_service_provider_default()
            .unwrap();

        let response = dispatcher
            .feed_update(Arc::clone(&bot), Arc::clone(&update))
            .await
            .unwrap()
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
            .unwrap()
            .unwrap();

        // Event should be handled
        match response.propagate_result {
            PropagateEventResult::Handled(_) => {}
            _ => panic!("Unexpected result"),
        }

        // Should return error, because `Update` is empty and `UpdateType` will be unknown
        let response = dispatcher.feed_update(bot, Update::default()).await;
        assert!(response.is_err());
    }

    #[test]
    fn test_builder() {
        let bot = Bot::<Reqwest>::default();

        let dispatcher = Dispatcher::builder()
            .main_router(Router::new("main"))
            .bot(bot.clone())
            .bots(vec![bot])
            .polling_timeout(123)
            .allowed_update(UpdateType::Message)
            .allowed_updates(vec![
                UpdateType::InlineQuery,
                UpdateType::ChosenInlineResult,
            ])
            .build();

        assert_eq!(dispatcher.bots.len(), 2);
        assert_eq!(dispatcher.polling_timeout, Some(123));
        assert_eq!(dispatcher.allowed_updates.len(), 3);
    }
}
