use super::router::{Request, Response, Router, RouterService};

use crate::{
    client::Bot,
    context::Context,
    dispatcher::event::service::{ServiceFactory, ServiceProvider, ToServiceProvider},
    enums::update_type::UpdateType,
    error::app,
    methods::GetUpdates,
    types::Update,
};

use backoff::{backoff::Backoff, exponential::ExponentialBackoff, SystemClock};
use log;
use std::sync::Arc;
use thiserror;
use tokio::{
    self,
    signal::unix::{signal, SignalKind},
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
    bots: Vec<Arc<Bot>>,
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
    /// * `main_router` - Main router, which will be used for dispatching updates
    /// * bots - Bots, which will be used for getting updates
    /// * `polling_timeout` - *Optional*. Timeout in seconds for long polling.
    /// Defaults to 0. Should be positive, short polling should be used for testing purposes only.
    /// * `allowed_updates` - *Optional*. List of the update types you want your bot to receive.
    /// Specify an empty list to receive all update types except `chat_member` (default).
    /// * `backoff` - Backoff for handling server-side errors
    #[must_use]
    pub fn new(
        main_router: Router,
        bots: Vec<Arc<Bot>>,
        polling_timeout: Option<i64>,
        backoff: ExponentialBackoff<SystemClock>,
        allowed_updates: Option<Vec<String>>,
    ) -> Self {
        Self {
            main_router,
            bots,
            polling_timeout,
            backoff,
            allowed_updates,
        }
    }

    #[must_use]
    pub fn builder() -> DispatcherBuilder {
        DispatcherBuilder::default()
    }
}

pub struct DispatcherBuilder {
    main_router: Router,
    bots: Vec<Arc<Bot>>,
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
    pub fn bot<B: Into<Arc<Bot>>>(mut self, val: B) -> Self {
        self.bots.push(val.into());
        self
    }

    /// Set bots to dispatcher. Bots used for getting updates.
    /// All bots use the same dispatcher, but each bot has the own polling process.
    /// This can be useful if you want to run multibots with a single dispatcher logic.
    #[must_use]
    pub fn bots<T: Into<Arc<Bot>>>(mut self, val: Vec<T>) -> Self {
        self.bots.extend(val.into_iter().map(Into::into));
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
    pub fn backoff<T>(mut self, val: T) -> Self
    where
        T: Into<ExponentialBackoff<SystemClock>>,
    {
        self.backoff = val.into();
        self
    }

    /// Set allowed update for polling.
    /// Update type you want your bot to receive.
    /// You can use this method multiple times to add multiple allowed updates or just use `allowed_updates` method.
    #[must_use]
    pub fn allowed_update<T: Into<String>>(mut self, val: T) -> Self {
        let allowed_update = val.into();

        match self.allowed_updates {
            Some(ref mut allowed_updates) => allowed_updates.push(allowed_update),
            None => {
                self.allowed_updates = Some(vec![allowed_update]);
            }
        }

        self
    }

    /// Set allowed updates for polling.
    /// List of the update types you want your bot to receive.
    /// Specify an empty list to receive all update types except `chat_member` (default).
    #[must_use]
    pub fn allowed_updates<T, AllowedUpdate>(mut self, val: T) -> Self
    where
        T: Into<Vec<AllowedUpdate>>,
        AllowedUpdate: Into<String>,
    {
        self.allowed_updates = Some(
            (val.into() as Vec<AllowedUpdate>)
                .into_iter()
                .map(Into::into)
                .collect(),
        );
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
        cfg: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        let main_router = self.main_router.new_service(cfg)?;

        Ok(Arc::new(DispatcherInner {
            main_router,
            bots: self.bots,
            polling_timeout: self.polling_timeout,
            backoff: self.backoff,
            allowed_updates: self.allowed_updates,
        }))
    }
}

pub struct DispatcherInner {
    main_router: RouterService,
    bots: Vec<Arc<Bot>>,
    polling_timeout: Option<i64>,
    backoff: ExponentialBackoff<SystemClock>,
    allowed_updates: Option<Vec<String>>,
}

impl ServiceProvider for DispatcherInner {}

impl DispatcherInner {
    /// Main entry point for incoming updates
    /// # Arguments
    /// * `bot` - Bot which will be used for creating [`Request`]
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler.
    /// - If update type is not supported
    pub async fn feed_update<B, U>(
        self: Arc<Self>,
        bot: B,
        update: U,
    ) -> Result<Response, app::ErrorKind>
    where
        B: Into<Arc<Bot>>,
        U: Into<Arc<Update>>,
    {
        let update: Arc<Update> = update.into();

        let update_type = match update.as_ref().try_into() as Result<UpdateType, app::ErrorKind> {
            Ok(update_type) => update_type,
            Err(err) => {
                log::error!("{err}");

                return Err(err);
            }
        };

        log::trace!("Propagating event");
        self.main_router
            .propagate_event(
                &update_type,
                Request::new(bot, Arc::clone(&update), Context::default()),
            )
            .await
    }

    /// Endless updates reader with correctly handling any server-side or connection errors.
    /// So you may not worry that the polling will stop working. \
    /// We use exponential backoff algorithm for handling server-side errors.
    /// # Arguments
    /// * `bot` - Bot which will be used for getting updates
    /// * `polling_timeout` - *Optional*. Timeout in seconds for long polling.
    /// Defaults to 0. Should be positive, short polling should be used for testing purposes only.
    /// * `allowed_updates` - *Optional*. List of the update types you want your bot to receive.
    /// Specify an empty list to receive all update types except `chat_member` (default).
    /// * `update_sender` - Sender for sending updates
    /// * `backoff` - Backoff for handling server-side errors
    /// # Errors
    /// - If sender channel is disconnected
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
            let updates: Vec<Update> = match bot
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
                // `Box` is used to avoid stack overflow
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
    /// # Arguments
    /// * `bot` - Bot which will be used for getting updates and creating [`Request`]. \
    /// Check methods [`listen_updates`](DispatcherService::listen_updates) and [`feed_update`](DispatcherService::feed_update) for more info.
    /// * `polling_timeout` - *Optional*. Timeout in seconds for long polling.
    /// Defaults to 0. Should be positive, short polling should be used for testing purposes only.
    /// * `allowed_updates` - *Optional*. List of the update types you want your bot to receive.
    /// Specify an empty list to receive all update types except `chat_member` (default).
    /// * `backoff` - Backoff for handling server-side errors
    /// # Panics
    /// - If failed to register SIGINT or SIGTERM signal handler
    async fn polling(
        self: Arc<Self>,
        bot: Arc<Bot>,
        polling_timeout: Option<i64>,
        allowed_updates: Option<Vec<String>>,
        backoff: ExponentialBackoff<SystemClock>,
    ) -> PollingError {
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

        listen_updates_handle.abort();
        receiver_updates_handle.abort();

        PollingError::Aborted
    }

    /// Polling runner
    /// # Errors
    /// - If any startup observer returns error
    /// - If any shutdown observer returns error
    /// # Panics
    /// - If `bots` is empty
    /// - If failed to register SIGINT or SIGTERM signal handler
    pub async fn run_polling(self: Arc<Self>) -> Result<(), app::ErrorKind> {
        let bots = self.bots.clone();
        let bots_len = bots.len();
        assert_ne!(bots_len, 0);

        if let Err(err) = self.main_router.emit_startup().await {
            log::error!("Error while emit startup: {err}");

            return Err(err);
        }

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
            handle.await.unwrap();
        }

        if bots_len == 1 {
            log::warn!("Polling is finished");
        } else {
            log::warn!("Polling is finished for all bots");
        }

        if let Err(err) = self.main_router.emit_shutdown().await {
            log::error!("Error while emit shutdown: {err}");

            Err(err)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dispatcher::event::bases::PropagateEventResult, types::Message};

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

        let res = dispatcher
            .feed_update(Arc::clone(&bot), Arc::clone(&update))
            .await
            .unwrap();

        // Event shouldn't be handled, because there is no any handler registered
        match res.response() {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        let mut router = Router::new("main");
        router.message.register_no_filters(|| async {});

        let dispatcher = Dispatcher::builder()
            .main_router(router)
            .build()
            .to_service_provider(())
            .unwrap();

        let res = dispatcher.feed_update(bot, update).await.unwrap();

        // Event should be handled
        match res.response() {
            PropagateEventResult::Handled(_) => {}
            _ => panic!("Unexpected result"),
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn test_feed_update_panic() {
        let bot = Bot::default();
        let update = Update::default();

        let router = Router::new("main");
        let dispatcher = Dispatcher::builder()
            .main_router(router)
            .build()
            .to_service_provider(())
            .unwrap();

        // Should return error, because `Update` is empty and `Update::update_type()` will be unknown
        dispatcher.feed_update(bot, update).await.unwrap();
    }
}
