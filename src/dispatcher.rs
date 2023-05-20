use super::router::{Request, Response, Router, RouterInner};

use crate::{
    client::{Bot, Session},
    context::Context,
    error::{EventErrorKind, UnknownUpdateTypeError},
    event::{
        service::{ServiceProvider, ToServiceProvider},
        simple::HandlerResult as SimpleHandlerResult,
    },
    methods::GetUpdates,
    types::Update,
};

use backoff::{backoff::Backoff as _, exponential::ExponentialBackoff, SystemClock};
use log;
use std::sync::Arc;
use thiserror;
use tokio::{
    self,
    sync::mpsc::{self, error::SendError, Sender},
};

/// Maximum size of the channel for listener updates
const GET_UPDATES_SIZE: i64 = 100;
/// Default timeout for long polling
const DEFAULT_POLLING_TIMEOUT: i64 = 30;

/// Error which may occur while listening updates
#[derive(thiserror::Error, Debug)]
enum ListenerError<T> {
    #[error(transparent)]
    SendError(#[from] SendError<T>),
}

/// Error which may occur while polling
#[derive(thiserror::Error, Debug)]
enum PollingError {
    #[error("Polling was aborted by signal")]
    Aborted,
}

/// Dispatcher using to dispatch incoming updates to the routers
pub struct Dispatcher<Client> {
    /// Main router, which will be used for dispatching updates
    main_router: Router<Client>,
    /// Bots, which will be used for getting updates. \
    /// All bots use the same dispatcher, but each bot has the own polling process.
    bots: Vec<Bot<Client>>,
    /// Timeout in seconds for long polling
    polling_timeout: Option<i64>,
    /// Backoff used for handling server-side errors and network errors (like connection reset or telegram server is down, etc.)
    backoff: ExponentialBackoff<SystemClock>,
    /// Allowed updates for polling. \
    /// List of the update types you want your bot to receive,
    /// specify an empty list to receive all update types except `chat_member` (default).
    allowed_updates: Vec<String>,
}

impl<Client> Dispatcher<Client> {
    #[must_use]
    pub fn new(
        main_router: Router<Client>,
        bots: Vec<Bot<Client>>,
        polling_timeout: Option<i64>,
        backoff: ExponentialBackoff<SystemClock>,
        allowed_updates: Vec<impl Into<String>>,
    ) -> Self {
        Self {
            main_router,
            bots,
            polling_timeout,
            backoff,
            allowed_updates: allowed_updates.into_iter().map(Into::into).collect(),
        }
    }
}

impl<Client> Dispatcher<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    pub fn builder() -> DispatcherBuilder<Client> {
        DispatcherBuilder::default()
    }
}

impl<Client> Default for Dispatcher<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    fn default() -> Self {
        Self {
            main_router: Router::default(),
            bots: vec![],
            polling_timeout: Some(DEFAULT_POLLING_TIMEOUT),
            backoff: ExponentialBackoff::default(),
            allowed_updates: vec![],
        }
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct DispatcherBuilder<Client> {
    main_router: Router<Client>,
    bots: Vec<Bot<Client>>,
    polling_timeout: Option<i64>,
    backoff: ExponentialBackoff<SystemClock>,
    allowed_updates: Vec<String>,
}

impl<Client> Default for DispatcherBuilder<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    fn default() -> Self {
        Self {
            main_router: Router::default(),
            bots: vec![],
            polling_timeout: Some(DEFAULT_POLLING_TIMEOUT),
            backoff: ExponentialBackoff::default(),
            allowed_updates: vec![],
        }
    }
}

/// A block of consuming builder
impl<Client> DispatcherBuilder<Client> {
    #[must_use]
    pub fn main_router(self, val: Router<Client>) -> Self {
        Self {
            main_router: val,
            ..self
        }
    }

    /// Alias to [`DispatcherBuilder::main_router`] method
    #[must_use]
    pub fn router(self, val: Router<Client>) -> Self {
        self.main_router(val)
    }

    #[must_use]
    pub fn bot(self, val: Bot<Client>) -> Self {
        Self {
            bots: self.bots.into_iter().chain(Some(val)).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn bots(self, val: impl IntoIterator<Item = Bot<Client>>) -> Self {
        Self {
            bots: self.bots.into_iter().chain(val).collect(),
            ..self
        }
    }

    #[must_use]
    pub fn polling_timeout(self, val: i64) -> Self {
        Self {
            polling_timeout: Some(val),
            ..self
        }
    }

    #[must_use]
    pub fn backoff(self, val: ExponentialBackoff<SystemClock>) -> Self {
        Self {
            backoff: val,
            ..self
        }
    }

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

    #[must_use]
    pub fn build(self) -> Dispatcher<Client> {
        Dispatcher {
            main_router: self.main_router,
            bots: self.bots,
            polling_timeout: self.polling_timeout,
            backoff: self.backoff,
            allowed_updates: self.allowed_updates,
        }
    }
}

impl<Client> ToServiceProvider for Dispatcher<Client>
where
    Client: Send + Sync + 'static,
{
    type Config = ();
    type ServiceProvider = Arc<DispatcherInner<Client>>;
    type InitError = ();

    fn to_service_provider(
        self,
        _config: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        let main_router = self.main_router.to_service_provider_default()?;

        Ok(Arc::new(DispatcherInner {
            main_router,
            bots: self.bots,
            polling_timeout: self.polling_timeout,
            backoff: self.backoff,
            allowed_updates: self.allowed_updates,
        }))
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct DispatcherInner<Client> {
    main_router: RouterInner<Client>,
    bots: Vec<Bot<Client>>,
    polling_timeout: Option<i64>,
    backoff: ExponentialBackoff<SystemClock>,
    allowed_updates: Vec<String>,
}

impl<Client> ServiceProvider for DispatcherInner<Client> {}

impl<Client> DispatcherInner<Client>
where
    Client: Session + 'static,
{
    /// Main entry point for incoming updates
    /// # Errors
    /// Returns [`UnknownUpdateTypeError`] if update type isn't supported, or [`EventErrorKind`] if any of this error occurs:
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler.
    pub async fn feed_update<B, U>(
        self: Arc<Self>,
        bot: B,
        update: U,
    ) -> Result<Result<Response<Client>, EventErrorKind>, UnknownUpdateTypeError>
    where
        B: Into<Arc<Bot<Client>>>,
        U: Into<Arc<Update>>,
    {
        let update: Arc<Update> = update.into();

        let update_type = match update.as_ref().try_into() {
            Ok(update_type) => update_type,
            Err(err) => {
                log::error!("{err}");

                return Err(err);
            }
        };

        Ok(self
            .main_router
            .propagate_event(update_type, Request::new(bot, update, Context::default()))
            .await)
    }

    /// Main entry point for incoming updates with user context
    /// # Errors
    /// Returns [`UnknownUpdateTypeError`] if update type is not supported, or [`EventErrorKind`] if any of this error occurs:
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler.
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
    {
        let update: Arc<Update> = update.into();

        let update_type = match update.as_ref().try_into() {
            Ok(update_type) => update_type,
            Err(err) => {
                log::error!("{err}");

                return Err(err);
            }
        };

        Ok(self
            .main_router
            .propagate_event(update_type, Request::new(bot, update, context.into()))
            .await)
    }

    /// Endless updates reader with correctly handling any server-side or connection errors.
    /// So you may not worry that the polling will stop working.
    /// We use exponential backoff algorithm for handling server-side errors.
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
                    log::error!("{err}");

                    failed = true;

                    if let Some(backoff) = backoff.next_backoff() {
                        log::error!("Failed to fetch updates");

                        log::warn!("Sleep for {backoff:?} seconds and try again...");
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
                log::info!("Connection established successfully");

                failed = false;

                backoff.reset();
            }
        }
    }

    /// Internal polling process
    ///
    /// It will create a channel for sending updates and spawn a task for listening updates. \
    /// Wait for exit signal, which will stop polling process.
    /// # Panics
    /// If failed to register exit signal handlers
    async fn polling(self: Arc<Self>, bot: Bot<Client>) -> PollingError {
        let bot = Arc::new(bot);

        let (sender_update, mut receiver_update) =
            mpsc::channel(GET_UPDATES_SIZE.try_into().unwrap());

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
                    log::warn!("SIGINT signal received");
                },
                _ = sigterm.recv() => {
                    log::warn!("SIGTERM signal received");
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
                    log::warn!("CTRL+C signal received");
                },
                _ = ctrl_break.recv() => {
                    log::warn!("CTRL+BREAK signal received");
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
            log::warn!(
                "Exit signals of this platform are not supported, \
                so polling process will never stop by signal and shutdown events will never be emitted.",
            );

            listen_updates_handle.await;
            receiver_updates_handle.await;

            unimplemented!("Exit signals of this platform are not supported");
        }
    }
}

impl<Client> DispatcherInner<Client>
where
    Client: Session + Clone + 'static,
{
    /// External polling process runner for multiple bots and emit startup and shutdown observers
    /// # Errors
    /// - If any startup observer returns error
    /// - If any shutdown observer returns error
    /// # Panics
    /// - If `bots` is empty
    /// - If failed to register exit signal handlers
    pub async fn run_polling(self: Arc<Self>) -> Result<(), EventErrorKind> {
        if let Err(err) = self.main_router.emit_startup().await {
            log::error!("Error while emit startup: {err}");

            return Err(err.into());
        }

        let dispatcher = Arc::clone(&self);
        dispatcher.run_polling_without_startup_and_shutdown().await;

        self.emit_shutdown().await.map_err(|err| {
            log::error!("Error while emit shutdown: {err}");

            err.into()
        })
    }

    /// External polling process runner for multiple bots
    /// # Panics
    /// If `bots` is empty
    pub async fn run_polling_without_startup_and_shutdown(self: Arc<Self>) {
        let bots = self.bots.clone();
        let bots_len = bots.len();

        assert_ne!(bots_len, 0);

        let mut handles = Vec::with_capacity(bots_len);
        for bot in bots {
            let dispatcher = Arc::clone(&self);

            log::info!("Polling is started for bot: {bot}");

            handles.push(tokio::spawn(dispatcher.polling(bot)));
        }

        for handle in handles {
            if let Err(err) = handle.await {
                log::error!("Task failed to execute to completion: {err}");
            }
        }

        if bots_len == 1 {
            log::warn!("Polling is finished");
        } else {
            log::warn!("Polling is finished for all bots");
        }
    }
}

impl<Client> DispatcherInner<Client> {
    /// Emit startup events
    ///
    /// Use this method if you want to emit startup events manually
    /// # Notes
    /// This method is called automatically in `run_polling` method,
    /// but not in `run_polling_without_startup_and_shutdown` method
    /// # Errors
    /// If any startup observer returns error
    pub async fn emit_startup(&self) -> SimpleHandlerResult {
        self.main_router.emit_startup().await
    }

    /// Emit shutdown events
    ///
    /// Use this method if you want to emit shutdown events manually
    /// # Notes
    /// This method is called automatically in `run_polling` method,
    /// but not in `run_polling_without_startup_and_shutdown` method
    /// # Errors
    /// If any shutdown observer returns error
    pub async fn emit_shutdown(&self) -> SimpleHandlerResult {
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
