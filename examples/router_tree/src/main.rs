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

    // This router will handle all private messages
    let private_router = Router::new("private").on_message(|observer| {
        observer
            // Register filter for all private messages
            .filter(ChatType::one(Private))
            // Register handler for private messages, which will send a greeting message
            .register(Handler::new(start_private).filter(Command::one("start")))
    });

    let echo_router = Router::new("echo")
        .on_update(|observer| {
            // Register stats middleware for echo router
            observer.register_outer_middleware(IncomingEchoRouterUpdates::default())
        })
        .on_message(|observer| {
            // Register handler for stats commands
            observer.registers([
                Handler::new(stats_echo_router).filter(Command::many(["stats", "statistics"])),
                Handler::new(echo_handler),
            ])
        });

    let main_router = Router::new("main")
        // Include private router into main router, so all updates, which are not handled by main router will be passed to private router
        .include(private_router)
        // Include echo router into main router, so all updates, which are not handled by main router or private router will be passed to echo router
        .include(echo_router);

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
