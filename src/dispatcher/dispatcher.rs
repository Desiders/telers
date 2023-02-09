use super::router::{Request, Response, Router, RouterInner};

use crate::{
    client::Bot,
    context::Context,
    dispatcher::event::{
        service::{ServiceProvider, ToServiceProvider},
        simple::HandlerResult as SimpleHandlerResult,
    },
    error::{AppErrorKind, UnknownUpdateTypeError},
    methods::GetUpdates,
    types::Update,
};

use backoff::{backoff::Backoff as _, exponential::ExponentialBackoff, SystemClock};
use log;
use std::sync::Arc;
use thiserror;
use tokio::{
    self, signal,
    sync::mpsc::{self, error::SendError, Sender},
};

/// Maximum size of the channel for listener updates
const GET_UPDATES_SIZE: usize = 100;
/// Default timeout for polling
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
pub struct Dispatcher {
    main_router: Router,
    bots: Vec<Bot>,
    polling_timeout: Option<i64>,
    backoff: ExponentialBackoff<SystemClock>,
    allowed_updates: Option<Vec<String>>,
}

impl Default for Dispatcher {
    #[must_use]
    fn default() -> Self {
        Self {
            main_router: Router::default(),
            bots: vec![],
            polling_timeout: Some(DEFAULT_POLLING_TIMEOUT),
            backoff: ExponentialBackoff::default(),
            allowed_updates: None,
        }
    }
}

impl Dispatcher {
    /// Create a new dispatcher
    /// # Arguments
    /// * `main_router` -
    /// Main router, which will be used for dispatching updates
    /// * `bots` -
    /// Bots, which will be used for getting updates.
    /// All bots use the same dispatcher, but each bot has the own polling process.
    /// * `polling_timeout` -
    /// Timeout in seconds for long polling.
    /// Short polling should be used for testing purposes only.
    /// * `allowed_updates` -
    /// Allowed updates for polling.
    /// List of the update types you want your bot to receive.
    /// Specify an empty list to receive all update types except `chat_member` (default).
    /// * `backoff` - Backoff used for handling server-side errors
    #[must_use]
    pub fn new<AllowedUpdate>(
        main_router: Router,
        bots: Vec<Bot>,
        polling_timeout: Option<i64>,
        backoff: ExponentialBackoff<SystemClock>,
        allowed_updates: Option<Vec<AllowedUpdate>>,
    ) -> Self
    where
        AllowedUpdate: Into<String>,
    {
        Self {
            main_router,
            bots,
            polling_timeout,
            backoff,
            allowed_updates: allowed_updates
                .map(|allowed_updates| allowed_updates.into_iter().map(Into::into).collect()),
        }
    }

    #[must_use]
    pub fn builder() -> DispatcherBuilder {
        DispatcherBuilder::default()
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct DispatcherBuilder {
    main_router: Router,
    bots: Vec<Bot>,
    polling_timeout: Option<i64>,
    backoff: ExponentialBackoff<SystemClock>,
    allowed_updates: Option<Vec<String>>,
}

impl Default for DispatcherBuilder {
    #[must_use]
    fn default() -> Self {
        Self {
            main_router: Router::default(),
            bots: vec![],
            polling_timeout: Some(DEFAULT_POLLING_TIMEOUT),
            backoff: ExponentialBackoff::default(),
            allowed_updates: None,
        }
    }
}

/// A block of consuming builder
impl DispatcherBuilder {
    /// Set main router, which will be used for dispatching updates
    #[must_use]
    pub fn main_router(mut self, val: Router) -> Self {
        self.main_router = val;
        self
    }

    /// Alias to [`DispatcherBuilder::main_router`] method
    #[must_use]
    pub fn router(self, val: Router) -> Self {
        self.main_router(val)
    }

    /// Set bot to dispatcher. Bot used for getting updates.
    /// You can use this method multiple times to add multiple bots or just use `bots` method.
    #[must_use]
    pub fn bot(mut self, val: Bot) -> Self {
        self.bots.push(val);
        self
    }

    /// Set bots to dispatcher. Bots used for getting updates.
    /// All bots use the same dispatcher, but each bot has the own polling process.
    /// This can be useful if you want to run multibots with a single dispatcher logic.
    #[must_use]
    pub fn bots(mut self, val: Vec<Bot>) -> Self {
        self.bots.extend(val);
        self
    }

    /// Set timeout in seconds for long polling.
    /// Short polling should be used for testing purposes only.
    #[must_use]
    pub fn polling_timeout(mut self, val: i64) -> Self {
        self.polling_timeout = Some(val);
        self
    }

    /// Set backoff strategy for polling.
    /// Backoff used for handling server-side errors.
    #[must_use]
    pub fn backoff(mut self, val: ExponentialBackoff<SystemClock>) -> Self {
        self.backoff = val;
        self
    }

    /// Set allowed update for polling.
    /// Update type you want your bot to receive.
    /// You can use this method multiple times to add multiple allowed updates or just use `allowed_updates` method.
    #[must_use]
    pub fn allowed_update<T: Into<String>>(mut self, val: T) -> Self {
        self.allowed_updates
            .get_or_insert_with(Vec::new)
            .push(val.into());
        self
    }

    /// Set allowed updates for polling.
    /// List of the update types you want your bot to receive.
    /// Specify an empty list to receive all update types except `chat_member` (default).
    #[must_use]
    pub fn allowed_updates<T: Into<String>>(mut self, val: Vec<T>) -> Self {
        self.allowed_updates = Some(val.into_iter().map(Into::into).collect());
        self
    }

    #[must_use]
    pub fn build(self) -> Dispatcher {
        Dispatcher {
            main_router: self.main_router,
            bots: self.bots,
            polling_timeout: self.polling_timeout,
            backoff: self.backoff,
            allowed_updates: self.allowed_updates,
        }
    }
}

impl ToServiceProvider for Dispatcher {
    type Config = ();
    type ServiceProvider = Arc<DispatcherInner>;
    type InitError = ();

    fn to_service_provider(
        self,
        config: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        let main_router = self.main_router.to_service_provider(config)?;

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
pub struct DispatcherInner {
    main_router: RouterInner,
    bots: Vec<Bot>,
    polling_timeout: Option<i64>,
    backoff: ExponentialBackoff<SystemClock>,
    allowed_updates: Option<Vec<String>>,
}

impl ServiceProvider for DispatcherInner {}

impl DispatcherInner {
    /// Main entry point for incoming updates
    /// # Arguments
    /// * `bot` - [`Bot`] which will be used for creating [`Request`]
    /// * `update` - [`Update`] which will be processed
    /// # Errors
    /// Returns [`UnknownUpdateTypeError`] if update type is not supported, or [`AppErrorKind`] if any of this error occurs:
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler.
    pub async fn feed_update<B, U>(
        self: Arc<Self>,
        bot: B,
        update: U,
    ) -> Result<Result<Response, AppErrorKind>, UnknownUpdateTypeError>
    where
        B: Into<Arc<Bot>>,
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
            .propagate_event(&update_type, Request::new(bot, update, Context::default()))
            .await)
    }

    /// Endless updates reader with correctly handling any server-side or connection errors.
    /// So you may not worry that the polling will stop working. \
    /// We use exponential backoff algorithm for handling server-side errors.
    /// # Arguments
    /// * `bot` - [`Bot`] which will be used for getting updates
    /// * `polling_timeout` -
    /// Timeout in seconds for long polling.
    /// Should be positive, short polling should be used for testing purposes only.
    /// * `allowed_updates` -
    /// List of the update types you want your bot to receive.
    /// Specify an empty list to receive all update types except `chat_member` (default).
    /// * `update_sender` - Sender for sending updates
    /// * `backoff` - Backoff used for handling server-side errors
    /// # Errors
    /// If sender channel is disconnected
    async fn listen_updates(
        bot: Arc<Bot>,
        polling_timeout: Option<i64>,
        allowed_updates: Option<Vec<String>>,
        update_sender: Sender<Box<Update>>,
        mut backoff: ExponentialBackoff<SystemClock>,
    ) -> Result<(), ListenerError<Box<Update>>> {
        let GetUpdates {
            mut offset,
            limit,
            timeout,
            allowed_updates,
        } = GetUpdates {
            offset: None,
            limit: Some(GET_UPDATES_SIZE.try_into().unwrap()),
            timeout: polling_timeout,
            allowed_updates,
        };

        // Flag for handling connection errors.
        // If it's true, we will use exponential backoff algorithm to next backoff.
        // If it's false, we will use default backoff algorithm.
        let mut failed = false;

        loop {
            let updates = match bot
                .get_updates(offset, limit, timeout, allowed_updates.clone(), None)
                .await
            {
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
            offset = Some(updates.last().map(|update| update.update_id + 1).unwrap());

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

    /// Internal polling process. \
    /// It will create a channel for sending updates and spawn a task for listening updates. \
    /// Wait for exit signal, which will stop polling process.
    /// # Arguments
    /// * `bot` -
    /// Bot which will be used for getting updates and creating [`Request`]. \
    /// Check methods [`DispatcherService::listen_updates`] and [`DispatcherService::feed_update`] for more info.
    /// * `polling_timeout` -
    /// Timeout in seconds for long polling.
    /// Defaults to 0. Should be positive, short polling should be used for testing purposes only.
    /// * `allowed_updates` -
    /// List of the update types you want your bot to receive.
    /// Specify an empty list to receive all update types except `chat_member` (default).
    /// * `backoff` - Backoff used for handling server-side errors.
    /// # Panics
    /// If failed to register exit signal handlers
    async fn polling(
        self: Arc<Self>,
        bot: Bot,
        polling_timeout: Option<i64>,
        allowed_updates: Option<Vec<String>>,
        backoff: ExponentialBackoff<SystemClock>,
    ) -> PollingError {
        let bot = Arc::new(bot);

        let (sender_update, mut receiver_update) = mpsc::channel(GET_UPDATES_SIZE);

        let listen_updates_handle = tokio::spawn(Self::listen_updates(
            Arc::clone(&bot),
            polling_timeout,
            allowed_updates,
            sender_update,
            backoff,
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
            use signal::unix::{signal, SignalKind};

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

    /// Polling and startup/shutdown events runner. \
    /// Run [`Dispatcher::polling`] method for each bot in `bots`. \
    /// Wait for exit signal, which will stop polling process for all bots. \
    /// Emit startup events before starting polling process and shutdown events after stopping polling process for all bots.
    /// # Errors
    /// - If any startup observer returns error
    /// - If any shutdown observer returns error
    /// # Panics
    /// - If `bots` is empty
    /// - If failed to register exit signal handlers
    pub async fn run_polling(self: Arc<Self>) -> Result<(), AppErrorKind> {
        if let Err(extraction_err) = self.main_router.emit_startup().await {
            log::error!("Error while emit startup: {extraction_err}");

            return Err(AppErrorKind::User(extraction_err.into()));
        }

        let dispatcher = Arc::clone(&self);
        dispatcher.run_polling_without_startup_and_shutdown().await;

        self.emit_shutdown().await.map_err(|event_err| {
            log::error!("Error while emit shutdown: {event_err}");

            AppErrorKind::User(event_err.into())
        })
    }

    /// Polling runner. \
    /// Run [`Dispatcher::polling`] method for each bot in `bots`. \
    /// Wait for exit signal, which will stop polling process for all bots.
    /// # Panics
    /// If `bots` is empty
    pub async fn run_polling_without_startup_and_shutdown(self: Arc<Self>) {
        let bots = self.bots.clone();
        let bots_len = bots.len();
        assert_ne!(bots_len, 0);

        let handles = bots
            .into_iter()
            .map(|bot| {
                let dispatcher = Arc::clone(&self);

                tokio::spawn(dispatcher.polling(
                    bot,
                    self.polling_timeout,
                    self.allowed_updates.clone(),
                    self.backoff.clone(),
                ))
            })
            .collect::<Vec<_>>();

        if bots_len == 1 {
            log::info!("Polling is started");
        } else {
            log::info!("Polling is started for all bots");
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

    /// Call startup events. \
    /// Use this method if you want to emit startup events manually
    /// or if you use [`Dispatcher::run_polling_without_startup_and_shutdown`] method
    /// # Notes
    /// This method is called automatically in [`Dispatcher::run_polling`] method
    /// # Errors
    /// If any startup observer returns error
    pub async fn emit_startup(&self) -> SimpleHandlerResult {
        self.main_router.emit_startup().await
    }

    /// Call shutdown events. \
    /// Use this method if you want to emit shutdown events manually
    /// or if you use [`Dispatcher::run_polling_without_startup_and_shutdown`] method
    /// # Notes
    /// This method is called automatically in [`Dispatcher::run_polling`] method
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
        dispatcher::event::bases::{EventReturn, PropagateEventResult},
        types::Message,
    };

    use tokio;

    #[tokio::test]
    async fn test_feed_update() {
        let bot = Arc::new(Bot::default());
        let update = Arc::new(Update {
            message: Some(Message::default()),
            ..Default::default()
        });

        let router = Router::new("main");
        let dispatcher = Dispatcher::builder()
            .main_router(router)
            .build()
            .to_service_provider(())
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
            .register_no_filters(|| async { Ok(EventReturn::Finish) });

        let dispatcher = Dispatcher::builder()
            .main_router(router)
            .build()
            .to_service_provider(())
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
}
