use super::router::{Request, Response, Router, RouterService};

use crate::{
    client::Bot,
    context::Context,
    dispatcher::event::service::{BoxFuture, Service, ServiceFactory},
    enums::update_type::UpdateType,
    error::app,
    types::{Message, Update},
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

const GET_UPDATES_SIZE: usize = 100;

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

/// Dispatcher using to dispatch incoming updates to the routers
#[derive(Debug)]
pub struct Dispatcher {
    main_router: Router,
}

impl Dispatcher {
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
                log::error!("{err:#?}");

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
    /// # Errors
    /// - If sender channel is disconnected
    async fn listen_updates<C>(
        bot: Arc<Bot>,
        update_sender: Sender<Box<Update>>,
        mut backoff_config: C,
    ) -> Result<(), ListenerError<Box<Update>>>
    where
        C: Backoff,
    {
        let mut failed = false;

        loop {
            // TOOD: Add `GetUpdates` request
            let mut updates: Vec<Update> = vec![];
            for _ in 0..100 {
                updates.push(Update {
                    message: Some(Message::default()),
                    ..Default::default()
                });
            }

            for update in updates {
                log::trace!("Sending {update:?}");
                update_sender.send(Box::new(update)).await?;
            }

            // TODO: Add backoff, if we get error in `GetUpdates` request
            // if let Some(backoff) = backoff_config.next_backoff() {
            //     log::trace!("Waiting for next update: {backoff:?}");
            //     tokio::time::sleep(backoff).await;
            // }
        }
    }

    /// Internal polling process
    /// # Arguments
    /// * `bot` - Bot which will be used for getting updates and creating [`Request`]. \
    /// Check methods [`listen_updates`](DispatcherService::listen_updates) and [`feed_update`](DispatcherService::feed_update) for more info.
    /// # Panics
    /// - If failed to register SIGINT or SIGTERM signal handler
    async fn polling<C>(self: Arc<Self>, bot: Arc<Bot>, backoff_config: C) -> PollingError
    where
        C: Backoff + Send + 'static,
    {
        log::trace!("Starting polling");

        log::trace!("Creating channel for updates");
        let (sender_update, mut receiver_update) = mpsc::channel(GET_UPDATES_SIZE);
        log::trace!("Channel for updates created");

        log::trace!("Starting listener updates");
        let listen_updates_handle = tokio::spawn(Self::listen_updates(
            Arc::clone(&bot),
            sender_update,
            backoff_config,
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
    /// * `backoff_config` - Backoff configuration for polling
    /// # Errors
    /// - If any startup observer returns error
    /// - If any shutdown observer returns error
    /// # Panics
    /// - If `bots` is empty
    /// - If failed to register SIGINT or SIGTERM signal handler
    pub async fn run_polling<B, C>(
        self: Arc<Self>,
        bots: Vec<B>,
        backoff_config: C,
    ) -> Result<(), app::ErrorKind>
    where
        B: Into<Arc<Bot>>,
        C: Backoff + Send + Clone + 'static,
    {
        assert!(!bots.is_empty());

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

                log::info!("Starting polling for {bot:#?}");
                tokio::spawn(dispatcher.polling(bot, backoff_config.clone()))
            })
            .collect::<Vec<_>>();

        for handle in handles {
            log::trace!("Waiting for polling to finish");
            handle.await.unwrap();
            log::trace!("Polling finished");
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
        router.message.register(|| async {}, vec![]);

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
