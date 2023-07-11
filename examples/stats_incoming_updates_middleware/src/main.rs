//! This example shows how to create a simple middleware count for incoming updates and processed handlers.
//! Every time a new update is received, the count is incremented. The count is also incremented every time a handler is called.
//! count is passes to the handler in context.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! cd examples
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --bin stats_incoming_updates_middleware
//! ```

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
    count: AtomicUsize,
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
        self.count.fetch_add(1, Ordering::SeqCst);

        request.context.insert(
            "incoming_updates_count",
            Box::new(self.count.load(Ordering::SeqCst)),
        );

        Ok((request, EventReturn::Finish))
    }
}

/// # Warning
/// If the handler returns an error, the count not increments
#[derive(Default)]
struct ProcessedHandlers {
    count: AtomicUsize,
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
            "processed_handlers_count",
            Box::new(self.count.load(Ordering::SeqCst)),
        );

        let response = next(request).await?;

        self.count.fetch_add(1, Ordering::SeqCst);

        Ok(response)
    }
}

async fn handler(bot: Bot, update: Update, context: Arc<Context>) -> HandlerResult {
    let text = format!(
        "Hello! Users sent me {} updates and I processed {} handlers successfully for them.",
        context
            .get("incoming_updates_count")
            .unwrap()
            .downcast_ref::<usize>()
            .unwrap(),
        context
            .get("processed_handlers_count")
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
        Ok(_) => log::info!("Bot stopped"),
        Err(err) => log::error!("Bot stopped with error: {err}"),
    }
}
