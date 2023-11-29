//! This example shows how to create a router tree.
//! Router tree is a tree of routers, where each router can have multiple children routers.
//! Each router can have multiple handlers for different update types and filters.
//! When update is received, it is passed to the main router, which will pass it to the first child router, which can handle this update.
//! If child router can't handle this update, it will pass it to the next child router, and so on.
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package router_tree
//! ```

use async_trait::async_trait;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use telers::{
    enums::ChatType as ChatTypeEnum,
    errors::EventErrorKind,
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    filters::{ChatType, Command},
    methods::{CopyMessage, SendMessage},
    middlewares::{outer::MiddlewareResponse, OuterMiddleware},
    router::{Request as RouterRequest, Router},
    types::Message,
    Bot, Context, Dispatcher,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

/// This middleware will count all incoming updates, which are handled by echo router.
#[derive(Default)]
struct IncomingEchoRouterUpdates {
    counter: AtomicUsize,
}

#[async_trait]
impl<Client> OuterMiddleware<Client> for IncomingEchoRouterUpdates
where
    Client: Send + Sync + 'static,
{
    async fn call(
        &self,
        request: RouterRequest<Client>,
    ) -> Result<MiddlewareResponse<Client>, EventErrorKind> {
        event!(Level::INFO, "Incoming echo router update");

        self.counter.fetch_add(1, Ordering::SeqCst);

        request.context.insert(
            "incoming_echo_router_updates_counter",
            Box::new(self.counter.load(Ordering::SeqCst)),
        );

        Ok((request, EventReturn::Finish))
    }
}

async fn start_private(bot: Bot, message: Message) -> HandlerResult {
    bot.send(SendMessage::new(
        message.chat().id(),
        "Hello! I'm echo bot that will repeat all your messages!",
    ))
    .await?;

    Ok(EventReturn::Finish)
}

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(CopyMessage::new(
        message.chat().id(),
        message.chat().id(),
        message.id(),
    ))
    .await?;

    Ok(EventReturn::Finish)
}

async fn stats_echo_router(bot: Bot, message: Message, context: Arc<Context>) -> HandlerResult {
    let text = format!(
        "Echo router updates stats\n\nIncoming updates: {}",
        context
            .get("incoming_echo_router_updates_counter")
            .unwrap()
            .downcast_ref::<usize>()
            .unwrap()
    );

    bot.send(SendMessage::new(message.chat().id(), text))
        .await?;

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("RUST_LOG"))
        .init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::new(bot_token);

    let mut main_router = Router::new("main");

    // This router will handle all private messages
    let mut private_router = Router::new("private");
    // Register filter for all private messages
    private_router
        .message
        .filter(ChatType::one(ChatTypeEnum::Private));
    // Register handler for private messages, which will send a greeting message
    private_router
        .message
        .register(start_private)
        .filter(Command::one("start"));

    // Include private router into main router, so all updates, which are not handled by main router will be passed to private router
    main_router.include(private_router);

    let mut echo_router = Router::new("echo");
    // Register stats middleware for echo router
    echo_router
        .update
        .outer_middlewares
        .register(IncomingEchoRouterUpdates::default());
    // Register handler for stats commands
    echo_router
        .message
        .register(stats_echo_router)
        .filter(Command::many(["stats", "statistics"]));

    echo_router.message.register(echo_handler);

    // Include echo router into main router, so all updates, which are not handled by main router or private router will be passed to echo router
    main_router.include(echo_router);

    let dispatcher = Dispatcher::builder()
        .allowed_updates(main_router.resolve_used_update_types())
        .router(main_router)
        .bot(bot)
        .build();

    match dispatcher
        .to_service_provider_default()
        .unwrap()
        .run_polling()
        .await
    {
        Ok(()) => event!(Level::INFO, "Bot stopped"),
        Err(err) => event!(Level::ERROR, error = %err, "Bot stopped"),
    }
}
