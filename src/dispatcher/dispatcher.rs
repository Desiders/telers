use super::router::{Request, Response, Router, RouterService};

use crate::{
    client::Bot,
    context::Context,
    dispatcher::event::service::{BoxFuture, Service, ServiceFactory},
    enums::update_type::UpdateType,
    error::app,
    methods::GetUpdates,
    types::Update,
};

use backoff::backoff::Backoff;
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
#[derive(Debug)]
pub struct Dispatcher {
    main_router: Router,
}

impl Dispatcher {
    /// Creates a new dispatcher
    /// # Arguments
    /// * `main_router` - Main router, which will be used for dispatching updates
    #[must_use]
    pub fn new(main_router: Router) -> Self {
        Self { main_router }
    }
}

impl ServiceFactory<()> for Dispatcher {
    type Response = ();
    type Error = app::ErrorKind;
    type Config = ();
    type Service = Arc<DispatcherService>;
    type InitError = ();

    fn new_service(&self, _: Self::Config) -> Result<Self::Service, Self::InitError> {
        let main_router = self.main_router.new_service(())?;

        Ok(Arc::new(DispatcherService { main_router }))
    }
}

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct DispatcherService {
    main_router: RouterService,
}

impl DispatcherService {
    /// Main entry point for incoming updates
    /// # Arguments
    /// * `bot` - Bot which will be used for creating [`Request`]
    /// # Returns
    /// - Ok([`Response`]) if the update was successfully processed
    /// - Err([`app::ErrorKind`]) if any error occurred while processing the update
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
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
    /// So you may not worry that the polling will stop working.
    /// We use exponential backoff algorithm for handling server-side errors. \
    ///
    /// `Box<Update>` is used to avoid stack overflow, because [`Update`] contains recursive structures.
    /// # Arguments
    /// * `bot` - Bot which will be used for getting updates
    /// * `update_sender` - Sender for sending updates
    /// * `backoff` - Backoff for handling server-side errors
    /// # Errors
    /// - If sender channel is disconnected
    async fn listen_updates<B: Backoff>(
        bot: Arc<Bot>,
        update_sender: Sender<Box<Update>>,
        mut backoff: B,
    ) -> Result<(), ListenerError<Box<Update>>> {
        let GetUpdates {
            mut offset,
            limit,
            timeout,
            allowed_updates,
        } = GetUpdates::default();

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
                    let updates_len = updates.len();

                    if updates_len == 0 {
                        log::trace!("Received 0 updates");
                        continue;
                    }
                    log::trace!("Received {updates_len} updates");

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
                log::trace!("Sending {update:?}");
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
    /// * `backoff` - Backoff for handling server-side errors
    /// # Panics
    /// - If failed to register SIGINT or SIGTERM signal handler
    async fn polling<B>(self: Arc<Self>, bot: Arc<Bot>, backoff: B) -> PollingError
    where
        B: Backoff + Send + 'static,
    {
        log::trace!("Starting polling");

        log::trace!("Creating channel for updates");
        let (sender_update, mut receiver_update) = mpsc::channel(GET_UPDATES_SIZE);
        log::trace!("Channel for updates created");

        log::trace!("Starting listener updates");
        let listen_updates_handle = tokio::spawn(Self::listen_updates(
            Arc::clone(&bot),
            sender_update,
            backoff,
        ));
        log::trace!("Listener updates started");

        log::trace!("Receiving updates started");
        let receiver_updates_handle = tokio::spawn(async move {
            while let Some(update) = receiver_update.recv().await {
                let dispatcher = Arc::clone(&self);
                let bot = Arc::clone(&bot);

                log::trace!("Spawning a task for processing the update");
                tokio::spawn(dispatcher.feed_update(bot, update));
                log::trace!("Task for processing the update spawned");
            }
        });
        log::trace!("Receiving updates stopped");

        log::trace!("Registering SIGINT and SIGTERM handlers");
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
                log::debug!("SIGINT signal received");
            },
            _ = sigterm.recv() => {
                log::debug!("SIGTERM signal received");
            },
        }

        log::trace!("Aborting listener updates");
        listen_updates_handle.abort();
        log::trace!("Listener updates aborted");

        log::trace!("Aborting receiver updates");
        receiver_updates_handle.abort();
        log::trace!("Receiver updates aborted");

        PollingError::Aborted
    }

    /// Polling runner
    /// # Arguments
    /// * `bots` -
    /// Bots used to getting updates. \
    /// You can use one bot or many. \
    /// All bots use the same dispatcher, but each bot has own polling process. \
    /// This can be useful if you want to run multibots with a single dispatcher logic.
    /// * `backoff` - Backoff config for handling server-side errors
    /// # Errors
    /// - If any startup observer returns error
    /// - If any shutdown observer returns error
    /// # Panics
    /// - If `bots` is empty
    /// - If failed to register SIGINT or SIGTERM signal handler
    pub async fn run_polling<B, C>(
        self: Arc<Self>,
        bots: Vec<B>,
        backoff: C,
    ) -> Result<(), app::ErrorKind>
    where
        B: Into<Arc<Bot>>,
        C: Backoff + Send + Clone + 'static,
    {
        let bots_len = bots.len();
        assert_ne!(bots_len, 0);

        log::trace!("Emitting startup events");
        if let Err(err) = self.main_router.emit_startup().await {
            log::error!("Error while emit startup: {err}");

            return Err(err);
        }
        log::trace!("Startup events emitted");

        let handles = bots
            .into_iter()
            .map(|bot| {
                let bot = bot.into();
                let dispatcher = Arc::clone(&self);

                log::trace!("Starting polling for bot");
                let handle = tokio::spawn(dispatcher.polling(bot, backoff.clone()));
                log::trace!("Polling for bot started");

                handle
            })
            .collect::<Vec<_>>();

        if bots_len == 1 {
            log::warn!("Polling is started");
        } else {
            log::warn!("Polling is started for all bots");
        }

        for handle in handles {
            log::trace!("Waiting polling for bot");
            handle.await.unwrap();
            log::trace!("Polling for bot finished");
        }

        if bots_len == 1 {
            log::warn!("Polling is finished");
        } else {
            log::warn!("Polling is finished for all bots");
        }

        log::trace!("Emitting shutdown events");
        if let Err(err) = self.main_router.emit_shutdown().await {
            log::error!("Error while emit shutdown: {err}");

            Err(err)
        } else {
            Ok(())
        }
    }
}

impl Service<()> for Arc<DispatcherService> {
    type Response = ();
    type Error = app::ErrorKind;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn call(&self, _: ()) -> Self::Future {
        log::error!("{self:?}: Should not be called");

        unimplemented!(
            "This method should not be called. \
             Use `Dispatcher::run_polling` method for running polling"
        );
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
        let dispatcher = Dispatcher::new(router);
        let dispatcher_service = dispatcher.new_service(()).unwrap();

        let res = dispatcher_service
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

        let dispatcher = Dispatcher::new(router);
        let dispatcher_service = dispatcher.new_service(()).unwrap();

        let res = dispatcher_service.feed_update(bot, update).await.unwrap();

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
        let dispatcher = Dispatcher::new(router);
        let dispatcher_service = dispatcher.new_service(()).unwrap();

        // Should return error, because `Update` is empty and `Update::update_type()` will be unknown
        dispatcher_service.feed_update(bot, update).await.unwrap();
    }
}
