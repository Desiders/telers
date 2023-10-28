//! This example shows how to create a router tree.
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
    enums::{ChatType as ChatTypeEnum, UpdateType},
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
        message.chat.id,
        "Hello! I'm echo bot that will repeat all your messages!",
    ))
    .await?;

    Ok(EventReturn::Finish)
}

async fn echo_handler(bot: Bot, message: Message) -> HandlerResult {
    bot.send(CopyMessage::new(
        message.chat.id,
        message.chat.id,
        message.message_id,
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

    bot.send(SendMessage::new(message.chat.id, text)).await?;

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

    let mut router = Router::new("main");

    let mut private_router = Router::new("private");
    private_router
        .message
        .register(start_private)
        .filter(Command::one("start"))
        .filter(ChatType::one(ChatTypeEnum::Private));

    // We include private router into main router, so all updates, which are not handled by main router, will be passed to private router,
    // and if they are not handled by private router, they will be passed to the next router in the chain.
    router.include(private_router);

    let mut echo_router = Router::new("echo");
    echo_router
        .update
        .outer_middlewares
        .register(IncomingEchoRouterUpdates::default());

    echo_router
        .message
        .register(stats_echo_router)
        .filter(Command::many(["stats", "statistics"]));
    // We register echo handler in the end, because we want to handle stats command before echo handler
    echo_router.message.register(echo_handler);

    // We include echo router into main router, so all updates, which are not handled by main router and private router, will be passed to echo router
    router.include(echo_router);

    let dispatcher = Dispatcher::builder()
        .main_router(router)
        .bot(bot)
        .allowed_update(UpdateType::Message)
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
