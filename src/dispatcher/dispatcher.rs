use super::router::{Request, Response, Router, RouterService};

use crate::{
    client::Bot,
    context::Context,
    dispatcher::event::service::{BoxFuture, Service, ServiceFactory},
    enums::update_type::UpdateType,
    error::app,
    types::Update,
};

use async_channel::{self, Receiver, RecvError, SendError, Sender, TryRecvError};
use log;
use std::sync::Arc;
use tokio::{
    self,
    signal::unix::{signal, SignalKind},
};

enum MessageForListener {
    NextUpdates,
}

enum ListenerError {
    SendError(SendError<Update>),
    RecvError(RecvError),
}

impl From<SendError<Update>> for ListenerError {
    fn from(error: SendError<Update>) -> Self {
        Self::SendError(error)
    }
}

impl From<RecvError> for ListenerError {
    fn from(error: RecvError) -> Self {
        Self::RecvError(error)
    }
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
    type Future = BoxFuture<Result<Self::Service, Self::InitError>>;

    fn new_service(&self, _: Self::Config) -> Self::Future {
        let main_router = self.main_router.new_service(());

        Box::pin(async move {
            let main_router = main_router.await?;

            Ok(Arc::new(DispatcherService { main_router }))
        })
    }
}

#[derive(Debug)]
#[allow(clippy::module_name_repetitions)]
pub struct DispatcherService {
    main_router: RouterService,
}

impl DispatcherService {
    /// Endless updates reader with correctly handling any server-side or connection errors.
    /// So you may not worry that the polling will stop working.
    /// # Arguments
    /// * `bot` - Bot which will be used for getting updates
    /// * `update_sender` - Sender for sending updates to the listener
    /// * `message_receiver` - Receiver for receiving messages from the listener
    async fn listen_updates(
        bot: Arc<Bot>,
        update_sender: Sender<Update>,
        message_receiver: Receiver<MessageForListener>,
    ) -> ! {
        loop {
            let updates: Vec<Update> = todo!(
                "Get updates from Telegram API. \
                 Use `bot.get_updates()` method for this. \
                 Don't forget to use `offset` parameter for getting only new updates"
            );

            for update in updates {
                // Send update to the listener
                update_sender.send(update).await.unwrap();
            }

            // Wait for a message from the listener
            message_receiver.recv().await.unwrap();
        }
    }

    /// Main entry point for incoming updates
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

        // Get update type for get observer
        let update_type = match update.as_ref().try_into() as Result<UpdateType, app::ErrorKind> {
            Ok(update_type) => update_type,
            Err(err) => {
                log::error!("{err:?}");

                return Err(err);
            }
        };

        // Create a context for the update
        let context = Context::default();

        // Create a request for the router
        let req = Request::new(bot, Arc::clone(&update), context);

        // Propagate event to the main router
        self.main_router.propagate_event(&update_type, req).await
    }

    /// Internal polling process
    async fn polling(self: Arc<Self>, bot: Arc<Bot>) -> ! {
        let (sender, receiver) = async_channel::unbounded();
        let (sender_for_listener, receiver_for_listener) = async_channel::unbounded();

        // Start listening updates
        tokio::spawn(Self::listen_updates(
            Arc::clone(&bot),
            sender,
            receiver_for_listener,
        ));

        loop {
            // Get update from the channel
            match receiver.try_recv() {
                Ok(update) => {
                    let dispatcher = Arc::clone(&self);
                    let bot = Arc::clone(&bot);

                    // Feed the update to the main router
                    tokio::spawn(dispatcher.feed_update(bot, update));
                }
                Err(err) => match err {
                    TryRecvError::Empty => {
                        sender_for_listener
                            .send(MessageForListener::NextUpdates)
                            .await
                            .unwrap();
                    }
                    TryRecvError::Closed => unreachable!(
                        "The channel for getting updates from the listener is disconnected"
                    ),
                },
            }
        }
    }

    /// Polling runner
    /// # Arguments
    /// * `bots` -
    /// Bots used to getting updates.
    /// You can use one bot or many.
    /// All bots use the same dispatcher, but each bot has a polling process.
    /// This can be useful if you want to run multibots with a single dispatcher logic.
    /// # Errors
    /// - If any startup observer returns error
    /// - If any shutdown observer returns error
    /// # Panics
    /// * [`signal`]:
    ///     - If there is no current reactor set, or if the `rt` feature flag is not enabled
    pub async fn run_polling<D, B>(self: Arc<Self>, bots: Vec<B>) -> Result<(), app::ErrorKind>
    where
        B: Into<Arc<Bot>>,
    {
        // Emit startup events
        if let Err(err) = self.main_router.emit_startup().await {
            log::error!("Error while emit startup: {err}");

            return Err(err);
        }
        log::info!("Startup events emitted");

        // Start polling for each bot
        let handles = bots
            .into_iter()
            .map(|bot| {
                let bot = bot.into();
                let dispatcher = Arc::clone(&self);

                log::info!("Start polling for bot {:?}", bot);

                // Start polling for the bot
                tokio::spawn(dispatcher.polling(bot))
            })
            .collect::<Vec<_>>();

        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sigint = signal(SignalKind::interrupt()).unwrap();

        let call_shutdown = async {
            // Abort handles
            handles.into_iter().for_each(|handle| handle.abort());

            // Emit shutdown events
            if let Err(err) = self.main_router.emit_shutdown().await {
                log::error!("Error while emit shutdown: {}", err);

                Err(err)
            } else {
                log::info!("Shutdown events emitted");

                Ok(())
            }
        };

        // Process the SIGINT and SIGTERM signal to emit shutdown events
        tokio::select! {
            _ = sigterm.recv() => {
                log::error!("SIGTERM signal received");

                call_shutdown.await
            },
            _ = sigint.recv() => {
                log::error!("SIGINT signal received");

                call_shutdown.await
            },
        }
    }
}

impl Service<()> for Arc<DispatcherService> {
    type Response = ();
    type Error = app::ErrorKind;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn call(&self, _: ()) -> Self::Future {
        log::error!("{:?}: Should not be called", self);

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
        let dispatcher_service = dispatcher.new_service(()).await.unwrap();

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
        let dispatcher_service = dispatcher.new_service(()).await.unwrap();

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
        let dispatcher_service = dispatcher.new_service(()).await.unwrap();

        // Should return error, because `Update` is empty and `Update::update_type()` will be unknown
        dispatcher_service.feed_update(bot, update).await.unwrap();
    }
}
