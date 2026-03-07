//! This example shows how to create a router tree.
//! Router tree is a tree of routers, where each router can have multiple children routers.
//! Each router can have multiple handlers for different update types and filters.
//! When update is received, it is passed to the main router, which will pass it to the first child router, which can handle this update.
//! If child router can't handle this update, it will pass it to the next child router, and so on.
//!
//! You can run this example by setting `BOT_TOKEN` and running:
//! ```bash
//! BOT_TOKEN={your_bot_token} cargo run --package router_tree
//! ```

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use telers::{
    enums::ChatType::Private,
    errors::EventErrorKind,
    event::{
        telegram::{Handler, HandlerResult},
        EventReturn,
    },
    filters::{ChatType, Command},
    methods::{CopyMessage, SendMessage},
    middlewares::{outer::MiddlewareResponse, OuterMiddleware},
    types::Message,
    Bot, Context, Dispatcher, Request, Router,
};

/// This middleware will count all incoming updates, which are handled by echo router.
#[derive(Default, Clone)]
struct IncomingEchoRouterUpdates {
    counter: Arc<AtomicUsize>,
}

impl OuterMiddleware for IncomingEchoRouterUpdates {
    async fn call(&mut self, mut request: Request) -> Result<MiddlewareResponse, EventErrorKind> {
        tracing::info!("Incoming echo router update");

        self.counter.fetch_add(1, Ordering::SeqCst);

        request.context.insert(
            "incoming_echo_router_updates_counter",
            self.counter.load(Ordering::SeqCst),
        );

        Ok((request, EventReturn::Finish))
    }
}

async fn start_private(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(SendMessage::new(
        message.chat().id(),
        "Hello! I'm echo bot that will repeat all your messages!",
    ))
    .await?;
    Ok(())
}

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult<()> {
    bot.send(CopyMessage::new(
        message.chat().id(),
        message.chat().id(),
        message.message_id(),
    ))
    .await?;
    Ok(())
}

async fn stats_echo_router(bot: Bot, message: Message, context: Context) -> HandlerResult<()> {
    let text = format!(
        "Echo router updates stats\n\nIncoming updates: {}",
        context
            .get::<usize>("incoming_echo_router_updates_counter")
            .unwrap()
    );
    bot.send(SendMessage::new(message.chat().id(), text))
        .await?;
    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();

    let bot = Bot::from_env();

    let mut main_router = Router::new("main");

    // This router will handle all private messages
    let mut private_router = Router::new("private");
    private_router
        .message
        // Register filter for all private messages
        .filter(ChatType::one(Private))
        // Register handler for private messages, which will send a greeting message
        .register(Handler::new(start_private).filter(Command::one("start")));

    // Include private router into main router, so all updates, which are not handled by main router will be passed to private router
    main_router.include(private_router);

    let mut echo_router = Router::new("echo");
    echo_router
        // Register stats middleware for echo router
        .update
        .outer_middlewares
        .register(IncomingEchoRouterUpdates::default());

    echo_router
        .message
        // Register handler for stats commands
        .registers([
            Handler::new(stats_echo_router).filter(Command::many(["stats", "statistics"])),
            Handler::new(echo_handler),
        ]);

    // Include echo router into main router, so all updates, which are not handled by main router or private router will be passed to echo router
    main_router.include(echo_router);

    let dispatcher = Dispatcher::builder()
        .allowed_updates(main_router.resolve_used_update_types())
        .router(main_router.configure_default())
        .bot(bot)
        .build();

    match dispatcher.run_polling().await {
        Ok(()) => tracing::info!("Bot stopped"),
        Err(err) => tracing::error!(error = %err, "Bot stopped"),
    }
}
