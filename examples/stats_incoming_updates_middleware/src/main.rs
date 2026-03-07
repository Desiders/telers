//! This example shows how to create a middleware that count incoming updates and processed handlers.
//! [`IncomingUpdates`] middleware counter increments when an update arrives.
//! [`ProcessedHandlers`] middleware counter increments when a handler successfully processed.
//! Every counterer is passes to the handler in the context.
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package stats_incoming_updates_middleware
//! ```

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use telers::{
    enums::UpdateType,
    errors::EventErrorKind,
    event::{
        telegram::{Handler, HandlerResponse, HandlerResult},
        EventReturn,
    },
    methods::SendMessage,
    middlewares::{outer::MiddlewareResponse, InnerMiddleware, Next, OuterMiddleware},
    types::Update,
    Bot, Context, Dispatcher, Request, Router,
};

#[derive(Default, Clone)]
struct IncomingUpdates {
    counter: Arc<AtomicUsize>,
}

impl OuterMiddleware for IncomingUpdates {
    async fn call(&mut self, mut request: Request) -> Result<MiddlewareResponse, EventErrorKind> {
        self.counter.fetch_add(1, Ordering::SeqCst);

        request.context.insert(
            "incoming_updates_counter",
            self.counter.load(Ordering::SeqCst),
        );

        Ok((request, EventReturn::Finish))
    }
}

/// # Warning
/// If the handler returns an error, the counter not increments
#[derive(Default, Clone)]
struct ProcessedHandlers {
    counter: Arc<AtomicUsize>,
}

impl InnerMiddleware for ProcessedHandlers {
    async fn call(
        &mut self,
        mut request: Request,
        next: Next,
    ) -> Result<HandlerResponse, EventErrorKind> {
        request.context.insert(
            "processed_handlers_counter",
            self.counter.load(Ordering::SeqCst),
        );

        let response = next(request).await?;

        self.counter.fetch_add(1, Ordering::SeqCst);

        Ok(response)
    }
}

async fn handler(bot: Bot, update: Update, context: Context) -> HandlerResult<()> {
    let text = format!(
        "Hello! Users sent me {} updates and I processed {} handlers successfully for them.",
        context.get::<usize>("incoming_updates_counter").unwrap(),
        context.get::<usize>("processed_handlers_counter").unwrap()
    );
    if let Some(chat) = update.chat() {
        bot.send(SendMessage::new(chat.id(), text)).await?;
    }
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let mut router = Router::new("main");

    // Register inner middleware for all telegram observers
    router
        .telegram_observers_mut()
        .iter_mut()
        .for_each(|observer| {
            observer
                .inner_middlewares
                .register(ProcessedHandlers::default());
        });

    // Register outer middleware for update
    router
        .update
        .outer_middlewares
        .register(IncomingUpdates::default());

    router.message.register(Handler::new(handler));

    let dispatcher = Dispatcher::builder()
        .main_router(router.configure_default())
        .bot(bot)
        .allowed_updates(UpdateType::all())
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
