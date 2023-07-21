//! This example shows how to create a simple middleware that counter incoming updates and processed handlers.
//! [`IncomingUpdates`] middleware counter increments when an update arrives.
//! [`ProcessedHandlers`] middleware counter increments when a handler successfully processed.
//! Every counterer is passes to the handler in the context.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package stats_incoming_updates_middleware
//! ```

use log::{error, info};
use telers::{
    enums::UpdateType,
    error::EventErrorKind,
    event::{
        telegram::{HandlerRequest, HandlerResponse, HandlerResult},
        EventReturn, ToServiceProvider as _,
    },
    methods::SendMessage,
    middlewares::{outer::MiddlewareResponse, InnerMiddleware, Next, OuterMiddleware},
    router::{Request as RouterRequest, Router},
    types::Update,
    Bot, Context, Dispatcher,
};

use async_trait::async_trait;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Default)]
struct IncomingUpdates {
    counter: AtomicUsize,
}

#[async_trait]
impl<Client> OuterMiddleware<Client> for IncomingUpdates
where
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: RouterRequest<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        self.counter.fetch_add(1, Ordering::SeqCst);

        request.context.insert(
            "incoming_updates_counter",
            Box::new(self.counter.load(Ordering::SeqCst)),
        );

        Ok((request, EventReturn::Finish))
    }
}

/// # Warning
/// If the handler returns an error, the counter not increments
#[derive(Default)]
struct ProcessedHandlers {
    counter: AtomicUsize,
}

#[async_trait]
impl<Client> InnerMiddleware<Client> for ProcessedHandlers
where
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: HandlerRequest<Client>,
        next: Next<Client>,
    ) -> Result<HandlerResponse<Client>, EventErrorKind> {
        request.context.insert(
            "processed_handlers_counter",
            Box::new(self.counter.load(Ordering::SeqCst)),
        );

        let response = next(request).await?;

        self.counter.fetch_add(1, Ordering::SeqCst);

        Ok(response)
    }
}

async fn handler(bot: Bot, update: Update, context: Arc<Context>) -> HandlerResult {
    let text = format!(
        "Hello! Users sent me {} updates and I processed {} handlers successfully for them.",
        context
            .get("incoming_updates_counter")
            .unwrap()
            .downcast_ref::<usize>()
            .unwrap(),
        context
            .get("processed_handlers_counter")
            .unwrap()
            .downcast_ref::<usize>()
            .unwrap()
    );

    if let Some(chat) = update.chat() {
        bot.send(&SendMessage::new(chat.id, text), None).await?;
    }

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::new(bot_token);

    let mut router = Router::new("main");
    // Register outer middleware for update
    router
        .update
        .outer_middlewares
        .register(IncomingUpdates::default());
    // Register inner middleware for all telegram observers
    router
        .telegram_observers_mut()
        .iter_mut()
        .for_each(|observer| {
            observer
                .inner_middlewares
                .register(ProcessedHandlers::default());
        });
    router.message.register(handler);

    let dispatcher = Dispatcher::builder()
        .main_router(router)
        .bot(bot)
        .allowed_updates(UpdateType::all())
        .build();

    match dispatcher
        .to_service_provider_default()
        .unwrap()
        .run_polling()
        .await
    {
        Ok(_) => info!("Bot stopped"),
        Err(err) => error!("Bot stopped with error: {err}"),
    }
}
